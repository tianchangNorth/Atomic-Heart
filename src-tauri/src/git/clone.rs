use crate::git::auth::AuthManager;
use crate::git::types::{
    CloneOptions, CloneProgress, CloneResult, CloneStage, CloneStats, GitError, NetworkProgress,
    RepositoryInfo,
};
use crate::utils::system_command::create_hidden_command;
use git2::{FetchOptions, Progress, RemoteCallbacks, Repository};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tauri::{Emitter, Window};
use uuid::Uuid;

/// Git 克隆管理器
pub struct CloneManager {
    window: Window,
}

impl CloneManager {
    pub fn new(window: Window) -> Self {
        Self { window }
    }

    /// 执行仓库克隆（同步版本）
    pub fn clone_repository_sync(&self, options: CloneOptions) -> Result<CloneResult, GitError> {
        // 首先尝试libgit2克隆
        match self.clone_with_libgit2(&options) {
            Ok(result) => Ok(result),
            Err(GitError::Git(e)) if self.is_ssh_hostkey_error(&e) => {
                log::warn!("libgit2 SSH失败，回退到系统Git: {}", e);
                self.clone_with_system_git(&options)
            }
            Err(e) => Err(e),
        }
    }

    /// 使用libgit2进行克隆
    fn clone_with_libgit2(&self, options: &CloneOptions) -> Result<CloneResult, GitError> {
        let start_time = Instant::now();
        let clone_id = Uuid::new_v4().to_string();

        // 验证输入参数
        self.validate_clone_options(options)?;

        // 发送初始化进度
        self.emit_progress(&clone_id, CloneStage::Initializing, 0, "准备克隆仓库...");

        // 检查目标目录
        let target_path = Path::new(&options.directory);
        if target_path.exists()
            && target_path
                .read_dir()
                .map_or(false, |mut i| i.next().is_some())
        {
            return Err(GitError::DirectoryExists {
                path: options.directory.clone(),
            });
        }

        // 创建目标目录
        if let Some(parent) = target_path.parent() {
            std::fs::create_dir_all(parent).map_err(GitError::Io)?;
        }

        // 设置进度回调
        let progress_data = Arc::new(Mutex::new(ProgressData::new()));
        let progress_clone = Arc::clone(&progress_data);
        let window_clone = self.window.clone();
        let clone_id_clone = clone_id.clone();

        let mut callbacks = RemoteCallbacks::new();

        // 设置认证回调
        if let Some(auth) = &options.auth {
            let auth_clone = auth.clone();
            let url_clone = options.url.clone();

            callbacks.credentials(move |url, username_from_url, allowed_types| {
                log::debug!(
                    "认证回调: url={}, username={:?}, allowed_types={:?}",
                    url,
                    username_from_url,
                    allowed_types
                );

                match AuthManager::create_credentials(
                    &auth_clone,
                    &url_clone,
                    username_from_url,
                    allowed_types,
                ) {
                    Ok(cred) => Ok(cred),
                    Err(e) => {
                        log::error!("认证失败: {}", e);
                        Err(git2::Error::from_str(&e.to_string()))
                    }
                }
            });
        }

        // 设置证书检查回调（用于 SSH 主机密钥验证）
        callbacks.certificate_check(|_cert, valid| {
            log::debug!("证书检查: valid={}", valid);

            // 对于开发环境，我们接受所有证书
            // 这解决了 GitHub 等平台的新 SSH 算法兼容性问题
            log::info!("接受证书验证以解决 SSH 主机密钥算法兼容性问题");

            // 返回接受状态
            Ok(git2::CertificateCheckStatus::CertificateOk)
        });

        // 设置进度回调
        callbacks.transfer_progress(move |progress| {
            let mut data = progress_clone.lock().unwrap();
            data.update_network_progress(progress);

            let clone_progress = CloneProgress {
                id: clone_id_clone.clone(),
                stage: CloneStage::Downloading,
                progress: data.calculate_progress(),
                message: data.get_message(),
                network_progress: Some(data.network_progress.clone()),
                checkout_progress: None,
            };

            // 发送进度更新
            let _ = window_clone.emit("clone-progress", &clone_progress);

            true
        });

        // 设置更新引用回调
        callbacks.update_tips(|refname, a, b| {
            log::debug!("更新引用: {} {} -> {}", refname, a, b);
            true
        });

        // 配置获取选项
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        // 配置克隆选项
        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fetch_options);

        if let Some(branch) = &options.branch {
            builder.branch(branch);
        }

        if let Some(_depth) = options.depth {
            // git2 不直接支持浅克隆，需要通过其他方式实现
            log::warn!("浅克隆功能暂未实现，将执行完整克隆");
        }

        // 发送连接进度
        self.emit_progress(&clone_id, CloneStage::Connecting, 10, "连接到远程仓库...");

        // 执行克隆
        let repo = match builder.clone(&options.url, target_path) {
            Ok(repo) => repo,
            Err(e) => {
                let error_msg = format!("克隆失败: {}", e);
                log::error!("{}", error_msg);

                // 检查是否是 SSH 相关错误
                let error_str = e.to_string();
                if error_str.contains("hostkey") || error_str.contains("SSH") {
                    let ssh_help_msg = "SSH 错误提示：请检查 SSH 密钥配置或尝试使用 HTTPS URL";
                    log::error!("{}", ssh_help_msg);
                    self.emit_progress(
                        &clone_id,
                        CloneStage::Error,
                        0,
                        &format!("{}\n{}", error_msg, ssh_help_msg),
                    );
                } else {
                    self.emit_progress(&clone_id, CloneStage::Error, 0, &error_msg);
                }

                return Err(GitError::Git(e));
            }
        };

        // 发送检出进度
        self.emit_progress(&clone_id, CloneStage::CheckingOut, 80, "检出文件...");

        // 处理子模块（如果需要）
        if options.recursive {
            self.clone_submodules(&repo, &clone_id)?;
        }

        // 获取仓库信息
        let repo_info = self.get_repository_info(&repo, &options)?;

        // 计算统计信息
        let duration = start_time.elapsed();
        let stats = self.calculate_stats(&repo, duration)?;

        // 发送完成进度
        self.emit_progress(&clone_id, CloneStage::Completed, 100, "克隆完成！");

        Ok(CloneResult {
            success: true,
            repository_path: Some(options.directory.clone()),
            error: None,
            branch: Some(repo_info.current_branch),
            last_commit_sha: self.get_last_commit_sha(&repo)?,
            stats: Some(stats),
        })
    }

    /// 验证克隆选项
    fn validate_clone_options(&self, options: &CloneOptions) -> Result<(), GitError> {
        // 验证 URL 格式
        if options.url.is_empty() {
            return Err(GitError::InvalidUrl {
                url: "URL 不能为空".to_string(),
            });
        }

        // 简单的 URL 格式验证
        if !options.url.starts_with("http://")
            && !options.url.starts_with("https://")
            && !options.url.starts_with("git@")
            && !options.url.starts_with("ssh://")
        {
            return Err(GitError::InvalidUrl {
                url: options.url.clone(),
            });
        }

        // 验证目录路径
        if options.directory.is_empty() {
            return Err(GitError::Unknown {
                message: "目标目录不能为空".to_string(),
            });
        }

        Ok(())
    }

    /// 克隆子模块
    fn clone_submodules(&self, repo: &Repository, clone_id: &str) -> Result<(), GitError> {
        self.emit_progress(clone_id, CloneStage::CheckingOut, 85, "处理子模块...");

        // 获取子模块
        let mut submodules = repo.submodules().map_err(GitError::Git)?;

        for submodule in &mut submodules {
            let name = submodule.name().unwrap_or("unknown");
            self.emit_progress(
                clone_id,
                CloneStage::CheckingOut,
                90,
                &format!("克隆子模块: {}", name),
            );

            submodule.clone(None).map_err(GitError::Git)?;
        }

        Ok(())
    }

    /// 获取仓库信息
    fn get_repository_info(
        &self,
        repo: &Repository,
        options: &CloneOptions,
    ) -> Result<RepositoryInfo, GitError> {
        let head = repo.head().map_err(GitError::Git)?;
        let branch_name = head.shorthand().unwrap_or("main").to_string();

        let repo_name = Path::new(&options.directory)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("repository")
            .to_string();

        let now = chrono::Utc::now();

        Ok(RepositoryInfo {
            id: Uuid::new_v4().to_string(),
            name: repo_name,
            path: options.directory.clone(),
            remote_url: Some(options.url.clone()),
            current_branch: branch_name,
            created_at: now,
            updated_at: now,
        })
    }

    /// 获取最后一次提交的 SHA
    fn get_last_commit_sha(&self, repo: &Repository) -> Result<Option<String>, GitError> {
        match repo.head() {
            Ok(head) => {
                if let Some(oid) = head.target() {
                    Ok(Some(oid.to_string()))
                } else {
                    Ok(None)
                }
            }
            Err(_) => Ok(None),
        }
    }

    /// 计算统计信息
    fn calculate_stats(
        &self,
        repo: &Repository,
        duration: std::time::Duration,
    ) -> Result<CloneStats, GitError> {
        let odb = repo.odb().map_err(GitError::Git)?;

        // 计算对象数量（这是一个近似值）
        let mut object_count = 0;
        odb.foreach(|_| {
            object_count += 1;
            true
        })
        .map_err(GitError::Git)?;

        // 计算文件数量
        let mut file_count = 0;
        let workdir = repo.workdir().ok_or_else(|| GitError::Unknown {
            message: "无法获取工作目录".to_string(),
        })?;

        if let Ok(entries) = std::fs::read_dir(workdir) {
            file_count = entries.count();
        }

        Ok(CloneStats {
            duration_ms: duration.as_millis() as u64,
            downloaded_bytes: 0, // 这个值需要在进度回调中累计
            object_count,
            file_count,
        })
    }

    /// 发送进度更新
    fn emit_progress(&self, id: &str, stage: CloneStage, progress: u32, message: &str) {
        let clone_progress = CloneProgress {
            id: id.to_string(),
            stage,
            progress,
            message: message.to_string(),
            network_progress: None,
            checkout_progress: None,
        };

        let _ = self.window.emit("clone-progress", &clone_progress);
    }

    /// 检查是否是SSH主机密钥错误
    fn is_ssh_hostkey_error(&self, error: &git2::Error) -> bool {
        let error_str = error.to_string();
        error_str.contains("hostkey preference")
            || error_str.contains("The requested method(s) are not currently supported")
            || error_str.contains("class=Ssh")
    }

    /// 使用系统Git进行克隆
    fn clone_with_system_git(&self, options: &CloneOptions) -> Result<CloneResult, GitError> {
        let start_time = std::time::Instant::now();
        let clone_id = uuid::Uuid::new_v4().to_string();

        log::info!("使用系统Git克隆: {}", options.url);
        self.emit_progress(&clone_id, CloneStage::Initializing, 0, "使用系统Git克隆...");

        // 检查系统Git是否可用
        let git_version = self.check_system_git()?;
        log::info!("检测到系统Git版本: {}", git_version);

        // 构建Git命令
        let mut cmd = create_hidden_command("git");
        cmd.arg("clone");

        // 添加分支参数
        if let Some(branch) = &options.branch {
            cmd.args(&["--branch", branch]);
        }

        // 添加深度参数
        if let Some(depth) = options.depth {
            cmd.args(&["--depth", &depth.to_string()]);
        }

        // 添加递归参数
        if options.recursive {
            cmd.arg("--recursive");
        }

        cmd.arg(&options.url);
        cmd.arg(&options.directory);

        // 设置SSH配置环境变量
        if let Some(auth) = &options.auth {
            if matches!(auth.auth_type, crate::git::AuthType::Ssh) {
                if let Some(ssh_key) = &auth.ssh_key_path {
                    // 设置SSH命令使用指定的密钥
                    let ssh_cmd =
                        format!("ssh -i \"{}\" -o StrictHostKeyChecking=accept-new", ssh_key);
                    cmd.env("GIT_SSH_COMMAND", ssh_cmd);
                }
            }
        }

        self.emit_progress(&clone_id, CloneStage::Connecting, 20, "连接到远程仓库...");

        // 执行Git命令
        let output = cmd.output().map_err(|e| {
            log::error!("执行git命令失败: {}", e);
            GitError::SystemGitNotFound
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("Git克隆失败: {}", stderr);
            return Err(GitError::SystemGitFailed {
                message: stderr.to_string(),
            });
        }

        self.emit_progress(&clone_id, CloneStage::Completed, 100, "克隆完成！");

        // 获取仓库信息
        let repo = git2::Repository::open(&options.directory).map_err(GitError::Git)?;
        let repo_info = self.get_repository_info(&repo, options)?;
        let stats = self.calculate_stats(&repo, start_time.elapsed())?;

        Ok(CloneResult {
            success: true,
            repository_path: Some(options.directory.clone()),
            error: None,
            branch: Some(repo_info.current_branch),
            last_commit_sha: self.get_last_commit_sha(&repo)?,
            stats: Some(stats),
        })
    }

    /// 检查系统Git是否可用
    fn check_system_git(&self) -> Result<String, GitError> {
        let mut cmd = create_hidden_command("git");
        cmd.args(&["--version"]);

        let output = cmd.output().map_err(|_| GitError::SystemGitNotFound)?;

        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout);
            Ok(version.trim().to_string())
        } else {
            Err(GitError::SystemGitNotFound)
        }
    }
}

/// 进度数据
struct ProgressData {
    network_progress: NetworkProgress,
    last_update: Instant,
}

impl ProgressData {
    fn new() -> Self {
        Self {
            network_progress: NetworkProgress {
                received_bytes: 0,
                received_objects: 0,
                total_objects: 0,
                indexed_objects: 0,
            },
            last_update: Instant::now(),
        }
    }

    fn update_network_progress(&mut self, progress: Progress) {
        self.network_progress = NetworkProgress {
            received_bytes: progress.received_bytes(),
            received_objects: progress.received_objects(),
            total_objects: progress.total_objects(),
            indexed_objects: progress.indexed_objects(),
        };
        self.last_update = Instant::now();
    }

    fn calculate_progress(&self) -> u32 {
        if self.network_progress.total_objects == 0 {
            return 0;
        }

        let download_progress = (self.network_progress.received_objects as f64
            / self.network_progress.total_objects as f64)
            * 70.0;
        let index_progress = (self.network_progress.indexed_objects as f64
            / self.network_progress.total_objects as f64)
            * 20.0;

        (10.0 + download_progress + index_progress) as u32
    }

    fn get_message(&self) -> String {
        if self.network_progress.total_objects == 0 {
            return "准备下载...".to_string();
        }

        format!(
            "下载对象: {}/{} ({:.1} MB)",
            self.network_progress.received_objects,
            self.network_progress.total_objects,
            self.network_progress.received_bytes as f64 / 1024.0 / 1024.0
        )
    }
}

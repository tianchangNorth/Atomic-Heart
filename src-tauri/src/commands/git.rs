use crate::git::{
    AuthConfig, AuthManager, CloneManager, CloneOptions, CloneResult, CommitHistoryItem,
    RepositoryStatus,
};
use git2::Repository;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use tauri::{command, State, Window};

/// Git 命令状态管理
pub struct GitState {
    pub clone_operations: Mutex<HashMap<String, bool>>,
}

impl Default for GitState {
    fn default() -> Self {
        Self {
            clone_operations: Mutex::new(HashMap::new()),
        }
    }
}

/// 克隆 Git 仓库
#[command]
pub async fn clone_repository(
    window: Window,
    options: CloneOptions,
) -> Result<CloneResult, String> {
    log::info!("开始克隆仓库: {}", options.url);

    // 在阻塞任务中执行克隆操作以避免 Send 问题
    let result = tokio::task::spawn_blocking(move || {
        let clone_manager = CloneManager::new(window);
        clone_manager.clone_repository_sync(options)
    })
    .await;

    match result {
        Ok(Ok(result)) => {
            log::info!("仓库克隆成功: {:?}", result.repository_path);
            Ok(result)
        }
        Ok(Err(e)) => {
            log::error!("仓库克隆失败: {}", e);
            Err(e.to_string())
        }
        Err(e) => {
            log::error!("任务执行失败: {}", e);
            Err(format!("任务执行失败: {}", e))
        }
    }
}

/// 验证仓库 URL
#[command]
pub async fn validate_repository_url(url: String) -> Result<bool, String> {
    log::debug!("验证仓库 URL: {}", url);

    // 基本 URL 格式验证
    if url.is_empty() {
        return Ok(false);
    }

    // 检查 URL 格式
    let is_valid = url.starts_with("http://")
        || url.starts_with("https://")
        || url.starts_with("git@")
        || url.starts_with("ssh://");

    if !is_valid {
        return Ok(false);
    }

    // 可以添加更复杂的验证逻辑，比如实际连接测试
    // 这里先返回基本验证结果
    Ok(true)
}

/// 检测认证类型
#[command]
pub async fn detect_auth_type(url: String) -> Result<String, String> {
    log::debug!("检测认证类型: {}", url);

    let auth_type = AuthManager::detect_auth_type(&url);
    let auth_type_str = match auth_type {
        crate::git::AuthType::None => "none",
        crate::git::AuthType::Password => "password",
        crate::git::AuthType::Token => "token",
        crate::git::AuthType::Ssh => "ssh",
    };

    Ok(auth_type_str.to_string())
}

/// 获取默认 SSH 密钥
#[command]
pub async fn get_default_ssh_keys() -> Result<Vec<String>, String> {
    log::debug!("获取默认 SSH 密钥");

    let keys = AuthManager::get_default_ssh_keys();
    Ok(keys)
}

/// 验证 SSH 密钥
#[command]
pub async fn validate_ssh_key(
    private_key_path: String,
    passphrase: Option<String>,
) -> Result<bool, String> {
    log::debug!("验证 SSH 密钥: {}", private_key_path);

    match AuthManager::validate_ssh_key(&private_key_path, passphrase.as_deref()) {
        Ok(is_valid) => Ok(is_valid),
        Err(e) => {
            log::error!("SSH 密钥验证失败: {}", e);
            Err(e.to_string())
        }
    }
}

/// 存储认证凭据
#[command]
pub async fn store_credentials(url: String, auth: AuthConfig) -> Result<(), String> {
    log::debug!("存储认证凭据: {}", url);

    match AuthManager::store_credentials(&url, &auth) {
        Ok(()) => {
            log::info!("凭据存储成功");
            Ok(())
        }
        Err(e) => {
            log::error!("凭据存储失败: {}", e);
            Err(e.to_string())
        }
    }
}

/// 加载认证凭据
#[command]
pub async fn load_credentials(url: String) -> Result<Option<AuthConfig>, String> {
    log::debug!("加载认证凭据: {}", url);

    match AuthManager::load_credentials(&url) {
        Ok(auth) => Ok(auth),
        Err(e) => {
            log::error!("凭据加载失败: {}", e);
            Err(e.to_string())
        }
    }
}

/// 删除认证凭据
#[command]
pub async fn delete_credentials(url: String) -> Result<(), String> {
    log::debug!("删除认证凭据: {}", url);

    match AuthManager::delete_credentials(&url) {
        Ok(()) => {
            log::info!("凭据删除成功");
            Ok(())
        }
        Err(e) => {
            log::error!("凭据删除失败: {}", e);
            Err(e.to_string())
        }
    }
}

/// 从 URL 提取用户名
#[command]
pub async fn extract_username_from_url(url: String) -> Result<Option<String>, String> {
    log::debug!("从 URL 提取用户名: {}", url);

    let username = AuthManager::extract_username_from_url(&url);
    Ok(username)
}

/// 取消克隆操作
#[command]
pub async fn cancel_clone_operation(
    operation_id: String,
    state: State<'_, GitState>,
) -> Result<(), String> {
    log::info!("取消克隆操作: {}", operation_id);

    let mut operations = state.clone_operations.lock().unwrap();
    operations.insert(operation_id, true); // true 表示已取消

    Ok(())
}

/// 获取克隆操作状态
#[command]
pub async fn get_clone_operation_status(
    operation_id: String,
    state: State<'_, GitState>,
) -> Result<bool, String> {
    let operations = state.clone_operations.lock().unwrap();
    let is_cancelled = operations.get(&operation_id).copied().unwrap_or(false);
    Ok(is_cancelled)
}

/// 清理完成的克隆操作
#[command]
pub async fn cleanup_clone_operation(
    operation_id: String,
    state: State<'_, GitState>,
) -> Result<(), String> {
    log::debug!("清理克隆操作: {}", operation_id);

    let mut operations = state.clone_operations.lock().unwrap();
    operations.remove(&operation_id);

    Ok(())
}

/// 选择目录
#[command]
pub async fn select_directory() -> Result<Option<String>, String> {
    // 这个功能需要在前端使用 @tauri-apps/plugin-dialog 实现
    // 这里提供一个占位符实现
    log::debug!("选择目录命令被调用");
    Ok(None)
}

/// 选择 SSH 密钥文件
#[command]
pub async fn select_ssh_key_file() -> Result<Option<String>, String> {
    // 这个功能需要在前端使用 @tauri-apps/plugin-dialog 实现
    // 这里提供一个占位符实现
    log::debug!("选择 SSH 密钥文件命令被调用");
    Ok(None)
}

/// 验证目录是否可用于克隆
#[command]
pub async fn validate_clone_directory(
    directory_path: String,
) -> Result<DirectoryValidation, String> {
    let path = Path::new(&directory_path);

    // 检查目录是否存在
    if !path.exists() {
        // 尝试创建目录
        match std::fs::create_dir_all(path) {
            Ok(_) => {
                return Ok(DirectoryValidation {
                    is_valid: true,
                    is_empty: true,
                    is_writable: true,
                    message: "目录已创建".to_string(),
                })
            }
            Err(e) => {
                return Ok(DirectoryValidation {
                    is_valid: false,
                    is_empty: false,
                    is_writable: false,
                    message: format!("无法创建目录: {}", e),
                })
            }
        }
    }

    // 检查是否为目录
    if !path.is_dir() {
        return Ok(DirectoryValidation {
            is_valid: false,
            is_empty: false,
            is_writable: false,
            message: "路径不是目录".to_string(),
        });
    }

    // 检查是否为空
    let is_empty = match std::fs::read_dir(path) {
        Ok(mut entries) => entries.next().is_none(),
        Err(_) => false,
    };

    // 检查是否可写
    let is_writable = path
        .metadata()
        .map(|m| !m.permissions().readonly())
        .unwrap_or(false);

    let message = if !is_empty {
        "目录不为空，克隆可能会覆盖现有文件".to_string()
    } else if !is_writable {
        "目录不可写".to_string()
    } else {
        "目录可用".to_string()
    };

    Ok(DirectoryValidation {
        is_valid: is_empty && is_writable,
        is_empty,
        is_writable,
        message,
    })
}

/// 目录验证结果
#[derive(serde::Serialize)]
pub struct DirectoryValidation {
    pub is_valid: bool,
    pub is_empty: bool,
    pub is_writable: bool,
    pub message: String,
}

/// 仓库信息结构
#[derive(serde::Serialize)]
pub struct RepositoryInfo {
    pub name: String,
    pub remote_url: Option<String>,
    pub current_branch: Option<String>,
    pub is_valid: bool,
}

/// 验证指定路径是否为有效的 Git 仓库
#[command]
pub async fn is_git_repository(path: String) -> Result<bool, String> {
    log::debug!("验证 Git 仓库: {}", path);

    let repo_path = Path::new(&path);

    // 检查路径是否存在
    if !repo_path.exists() {
        return Ok(false);
    }

    // 尝试打开 Git 仓库
    match Repository::open(repo_path) {
        Ok(_) => {
            log::debug!("路径是有效的 Git 仓库: {}", path);
            Ok(true)
        }
        Err(e) => {
            log::debug!("路径不是有效的 Git 仓库: {}, 错误: {}", path, e);
            Ok(false)
        }
    }
}

/// 获取仓库基本信息
#[command]
pub async fn get_repository_info(path: String) -> Result<RepositoryInfo, String> {
    log::debug!("获取仓库信息: {}", path);

    let repo_path = Path::new(&path);

    // 从路径提取仓库名称
    let repo_name = repo_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("Unknown Repository")
        .to_string();

    // 尝试打开 Git 仓库
    match Repository::open(repo_path) {
        Ok(repo) => {
            // 获取远程 URL
            let remote_url = get_remote_url_internal(&repo);

            // 获取当前分支
            let current_branch = get_current_branch_internal(&repo);

            Ok(RepositoryInfo {
                name: repo_name,
                remote_url,
                current_branch,
                is_valid: true,
            })
        }
        Err(e) => {
            log::warn!("无法打开 Git 仓库: {}, 错误: {}", path, e);
            Ok(RepositoryInfo {
                name: repo_name,
                remote_url: None,
                current_branch: None,
                is_valid: false,
            })
        }
    }
}

/// 获取仓库当前分支名称
#[command]
pub async fn get_current_branch(path: String) -> Result<Option<String>, String> {
    log::debug!("获取当前分支: {}", path);

    let repo_path = Path::new(&path);

    match Repository::open(repo_path) {
        Ok(repo) => {
            let branch = get_current_branch_internal(&repo);
            Ok(branch)
        }
        Err(e) => {
            log::error!("无法打开 Git 仓库: {}, 错误: {}", path, e);
            Err(format!("无法打开 Git 仓库: {}", e))
        }
    }
}

/// 获取仓库的远程 URL
#[command]
pub async fn get_remote_url(path: String) -> Result<Option<String>, String> {
    log::debug!("获取远程 URL: {}", path);

    let repo_path = Path::new(&path);

    match Repository::open(repo_path) {
        Ok(repo) => {
            let url = get_remote_url_internal(&repo);
            Ok(url)
        }
        Err(e) => {
            log::error!("无法打开 Git 仓库: {}, 错误: {}", path, e);
            Err(format!("无法打开 Git 仓库: {}", e))
        }
    }
}

/// 在文件管理器中打开指定文件夹
#[command]
pub async fn open_folder(path: String) -> Result<(), String> {
    log::debug!("打开文件夹: {}", path);

    let folder_path = Path::new(&path);

    // 检查路径是否存在
    if !folder_path.exists() {
        return Err(format!("路径不存在: {}", path));
    }

    // 使用系统默认程序打开文件夹
    match open::that(&path) {
        Ok(_) => {
            log::info!("成功打开文件夹: {}", path);
            Ok(())
        }
        Err(e) => {
            log::error!("打开文件夹失败: {}, 错误: {}", path, e);
            Err(format!("打开文件夹失败: {}", e))
        }
    }
}

// 内部辅助函数

/// 获取远程 URL（内部函数）
fn get_remote_url_internal(repo: &Repository) -> Option<String> {
    // 尝试获取 origin 远程
    match repo.find_remote("origin") {
        Ok(remote) => {
            if let Some(url) = remote.url() {
                return Some(url.to_string());
            }
        }
        Err(_) => {
            // 如果没有 origin，尝试获取第一个远程
            if let Ok(remotes) = repo.remotes() {
                for remote_name in remotes.iter() {
                    if let Some(name) = remote_name {
                        if let Ok(remote) = repo.find_remote(name) {
                            if let Some(url) = remote.url() {
                                return Some(url.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// 获取当前分支（内部函数）
fn get_current_branch_internal(repo: &Repository) -> Option<String> {
    match repo.head() {
        Ok(head) => {
            if let Some(branch_name) = head.shorthand() {
                return Some(branch_name.to_string());
            }
        }
        Err(_) => {
            // 如果无法获取 HEAD，尝试获取默认分支
            if let Ok(branches) = repo.branches(Some(git2::BranchType::Local)) {
                for branch_result in branches {
                    if let Ok((branch, _)) = branch_result {
                        if let Some(name) = branch.name().unwrap_or(None) {
                            return Some(name.to_string());
                        }
                    }
                }
            }
        }
    }
    None
}

/// 获取仓库状态
#[command]
pub async fn get_repository_status(repo_path: String) -> Result<RepositoryStatus, String> {
    log::debug!("获取仓库状态: {}", repo_path);

    match crate::git::operations::get_repository_status(&repo_path) {
        Ok(status) => Ok(status),
        Err(e) => {
            log::error!("获取仓库状态失败: {}", e);
            Err(e.to_string())
        }
    }
}

/// 暂存文件
#[command]
pub async fn stage_files(repo_path: String, file_paths: Vec<String>) -> Result<(), String> {
    log::debug!("暂存文件: {:?} in {}", file_paths, repo_path);

    match crate::git::operations::stage_files(&repo_path, &file_paths) {
        Ok(()) => Ok(()),
        Err(e) => {
            log::error!("暂存文件失败: {}", e);
            Err(e.to_string())
        }
    }
}

/// 取消暂存文件
#[command]
pub async fn unstage_files(repo_path: String, file_paths: Vec<String>) -> Result<(), String> {
    log::debug!("取消暂存文件: {:?} in {}", file_paths, repo_path);

    match crate::git::operations::unstage_files(&repo_path, &file_paths) {
        Ok(()) => Ok(()),
        Err(e) => {
            log::error!("取消暂存文件失败: {}", e);
            Err(e.to_string())
        }
    }
}

/// 创建提交
#[command]
pub async fn create_commit(
    repo_path: String,
    message: String,
    description: Option<String>,
    author_name: Option<String>,
    author_email: Option<String>,
    amend: Option<bool>,
    signoff: Option<bool>,
) -> Result<String, String> {
    log::debug!("创建提交: {} in {}", message, repo_path);

    let commit_options = crate::git::types::CommitOptions {
        message,
        description,
        author_name,
        author_email,
        amend: amend.unwrap_or(false),
        signoff: signoff.unwrap_or(false),
    };

    match crate::git::operations::create_commit(&repo_path, &commit_options) {
        Ok(commit_sha) => Ok(commit_sha),
        Err(e) => {
            log::error!("创建提交失败: {}", e);
            Err(e.to_string())
        }
    }
}

/// 获取提交历史
#[command]
pub async fn get_commit_history(
    repo_path: String,
    limit: Option<usize>,
    skip: Option<usize>,
) -> Result<Vec<CommitHistoryItem>, String> {
    log::debug!(
        "获取提交历史: {} (limit: {:?}, skip: {:?})",
        repo_path,
        limit,
        skip
    );

    match crate::git::operations::get_commit_history(
        &repo_path,
        limit.unwrap_or(50),
        skip.unwrap_or(0),
    ) {
        Ok(commits) => Ok(commits),
        Err(e) => {
            log::error!("获取提交历史失败: {}", e);
            Err(e.to_string())
        }
    }
}

/// 获取文件差异
#[command]
pub async fn get_file_diff(
    repo_path: String,
    file_path: String,
    staged: Option<bool>,
) -> Result<String, String> {
    log::debug!(
        "获取文件差异: {} in {} (staged: {:?})",
        file_path,
        repo_path,
        staged
    );

    match crate::git::operations::get_file_diff(&repo_path, &file_path, staged.unwrap_or(false)) {
        Ok(diff) => Ok(diff),
        Err(e) => {
            log::error!("获取文件差异失败: {}", e);
            Err(e.to_string())
        }
    }
}

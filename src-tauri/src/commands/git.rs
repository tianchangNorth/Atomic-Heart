use crate::git::{
    AuthConfig, AuthManager, CloneManager, CloneOptions, CloneResult, CommitHistoryItem,
    RepositoryStatus,
};
use git2::Repository;
use std::collections::HashMap;
use std::path::Path;

use std::sync::Mutex;
use tauri::{command, AppHandle, State, Window};
use tauri_plugin_store::StoreBuilder;

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
    // 首先尝试获取默认远程名称
    if let Ok(default_remote) = crate::git::operations::get_default_remote_name(repo) {
        if let Ok(remote) = repo.find_remote(&default_remote) {
            if let Some(url) = remote.url() {
                return Some(url.to_string());
            }
        }
    }

    // 如果默认远程获取失败，尝试获取第一个可用的远程
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

/// 获取远程变更（fetch操作）
#[command]
pub async fn fetch_remote(
    repo_path: String,
    remote_name: Option<String>,
) -> Result<crate::git::types::SyncResult, String> {
    log::debug!("获取远程变更: {} (remote: {:?})", repo_path, remote_name);

    match crate::git::operations::fetch_remote(&repo_path, remote_name.as_deref()) {
        Ok(result) => Ok(result),
        Err(e) => {
            log::error!("获取远程变更失败: {}", e);
            Err(e.to_string())
        }
    }
}

/// 智能获取远程变更（支持Token认证）
#[command]
pub async fn smart_fetch_remote(
    app_handle: AppHandle,
    repo_path: String,
    remote_name: Option<String>,
) -> Result<crate::git::types::SyncResult, String> {
    log::debug!(
        "智能获取远程变更: {} (remote: {:?})",
        repo_path,
        remote_name
    );

    // 获取远程URL并提取域名
    let token_cache = match get_token_for_repository(&app_handle, &repo_path).await {
        Ok(token) => token,
        Err(e) => {
            log::warn!("获取Token失败，使用默认认证: {}", e);
            None
        }
    };

    if let Some(ref _token) = token_cache {
        log::debug!("使用Token认证进行fetch操作");
    } else {
        log::debug!("使用默认认证进行fetch操作");
    }

    match crate::git::operations::fetch_remote_with_token(
        &repo_path,
        remote_name.as_deref(),
        token_cache,
    ) {
        Ok(result) => Ok(result),
        Err(e) => {
            log::error!("智能获取远程变更失败: {}", e);
            Err(e.to_string())
        }
    }
}

/// 拉取远程变更（pull操作）
#[command]
pub async fn pull_remote(
    repo_path: String,
    strategy: String,
) -> Result<crate::git::types::SyncResult, String> {
    log::debug!("拉取远程变更: {} (strategy: {})", repo_path, strategy);

    let pull_strategy = match strategy.as_str() {
        "merge" => crate::git::types::PullStrategy::Merge,
        "rebase" => crate::git::types::PullStrategy::Rebase,
        _ => {
            return Err("无效的拉取策略，支持: merge, rebase".to_string());
        }
    };

    match crate::git::operations::pull_remote(&repo_path, pull_strategy) {
        Ok(result) => Ok(result),
        Err(e) => {
            log::error!("拉取远程变更失败: {}", e);
            Err(e.to_string())
        }
    }
}

/// 推送本地变更（push操作）
#[command]
pub async fn push_remote(
    repo_path: String,
    remote_name: Option<String>,
    force: Option<bool>,
) -> Result<crate::git::types::SyncResult, String> {
    log::debug!(
        "推送本地变更: {} (remote: {:?}, force: {:?})",
        repo_path,
        remote_name,
        force
    );

    match crate::git::operations::push_remote(
        &repo_path,
        remote_name.as_deref(),
        force.unwrap_or(false),
    ) {
        Ok(result) => Ok(result),

        Err(e) => {
            log::error!("推送本地变更失败: {}", e);
            Err(e.to_string())
        }
    }
}

/// 智能推送本地变更（支持Token认证）
#[command]
pub async fn smart_push_remote(
    app_handle: AppHandle,
    repo_path: String,
    remote_name: Option<String>,
    force: Option<bool>,
) -> Result<crate::git::types::SyncResult, String> {
    log::debug!(
        "智能推送本地变更: {} (remote: {:?}, force: {:?})",
        repo_path,
        remote_name,
        force
    );

    // 获取远程URL并提取域名
    let token_cache = match get_token_for_repository(&app_handle, &repo_path).await {
        Ok(token) => token,
        Err(e) => {
            log::warn!("获取Token失败，使用默认认证: {}", e);
            None
        }
    };

    if let Some(ref _token) = token_cache {
        log::debug!("使用Token认证进行push操作");
    } else {
        log::debug!("使用默认认证进行push操作");
    }

    match crate::git::operations::push_remote_with_token(
        &repo_path,
        remote_name.as_deref(),
        force.unwrap_or(false),
        token_cache,
    ) {
        Ok(result) => Ok(result),
        Err(e) => {
            log::error!("智能推送本地变更失败: {}", e);
            Err(e.to_string())
        }
    }
}

/// 获取远程仓库信息
#[command]
pub async fn get_remote_info(
    repo_path: String,
) -> Result<crate::git::types::RemoteBranchInfo, String> {
    log::debug!("获取远程仓库信息: {}", repo_path);

    match crate::git::operations::get_remote_info(&repo_path) {
        Ok(info) => Ok(info),
        Err(e) => {
            log::error!("获取远程仓库信息失败: {}", e);
            Err(e.to_string())
        }
    }
}

// ==================== 新增：双协议认证系统 ====================

/// 协议类型
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum ProtocolType {
    Https,
    Ssh,
}

/// Token配置
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TokenConfig {
    pub domain: String,
    pub token: String,
    pub username: Option<String>,
    pub created_at: i64,
    pub last_used: Option<i64>,
}

/// 检测远程仓库URL的协议类型
#[command]
pub async fn detect_repository_protocol(repo_path: String) -> Result<String, String> {
    log::debug!("检测仓库协议类型: {}", repo_path);

    // 获取远程URL
    match get_remote_url(repo_path).await? {
        Some(url) => {
            let protocol = if url.starts_with("https://") || url.starts_with("http://") {
                "https"
            } else if url.starts_with("git@") || url.starts_with("ssh://") {
                "ssh"
            } else {
                "unknown"
            };

            log::debug!("检测到协议类型: {} for URL: {}", protocol, url);
            Ok(protocol.to_string())
        }
        None => {
            log::warn!("未找到远程URL");
            Err("未找到远程URL".to_string())
        }
    }
}

/// 从URL提取域名
#[command]
pub async fn extract_domain_from_url(url: String) -> Result<String, String> {
    log::debug!("从URL提取域名: {}", url);

    // 处理HTTPS URL
    if url.starts_with("https://") || url.starts_with("http://") {
        if let Ok(parsed_url) = url::Url::parse(&url) {
            if let Some(host) = parsed_url.host_str() {
                return Ok(host.to_string());
            }
        }
    }

    // 处理SSH URL (git@github.com:user/repo.git)
    if url.starts_with("git@") {
        if let Some(at_pos) = url.find('@') {
            if let Some(colon_pos) = url[at_pos..].find(':') {
                let domain = &url[at_pos + 1..at_pos + colon_pos];
                return Ok(domain.to_string());
            }
        }
    }

    // 处理SSH URL (ssh://git@github.com/user/repo.git)
    if url.starts_with("ssh://") {
        if let Ok(parsed_url) = url::Url::parse(&url) {
            if let Some(host) = parsed_url.host_str() {
                return Ok(host.to_string());
            }
        }
    }

    Err("无法从URL提取域名".to_string())
}

/// 存储Personal Access Token
#[command]
pub async fn store_access_token(
    app_handle: AppHandle,
    domain: String,
    token: String,
    username: Option<String>,
) -> Result<(), String> {
    log::debug!("存储访问令牌: {}", domain);

    let token_config = TokenConfig {
        domain: domain.clone(),
        token,
        username,
        created_at: chrono::Utc::now().timestamp(),
        last_used: None,
    };

    // 使用Tauri Store API存储token
    let store = StoreBuilder::new(&app_handle, "tokens.dat").build();

    match store {
        Ok(store) => {
            store.set(domain.clone(), serde_json::to_value(&token_config).unwrap());
            match store.save() {
                Ok(_) => {
                    log::info!("Token存储成功: {}", domain);
                    Ok(())
                }
                Err(e) => {
                    log::error!("Token存储失败: {}", e);
                    Err(format!("Token存储失败: {}", e))
                }
            }
        }
        Err(e) => {
            log::error!("无法创建存储: {}", e);
            Err(format!("无法创建存储: {}", e))
        }
    }
}

/// 获取Personal Access Token
#[command]
pub async fn get_access_token(
    app_handle: AppHandle,
    domain: String,
) -> Result<Option<TokenConfig>, String> {
    log::debug!("获取访问令牌: {}", domain);

    let store = StoreBuilder::new(&app_handle, "tokens.dat").build();

    match store {
        Ok(store) => {
            if let Some(value) = store.get(&domain) {
                match serde_json::from_value::<TokenConfig>(value.clone()) {
                    Ok(token_config) => {
                        log::debug!("找到Token: {}", domain);
                        Ok(Some(token_config))
                    }
                    Err(e) => {
                        log::error!("Token反序列化失败: {}", e);
                        Err(format!("Token反序列化失败: {}", e))
                    }
                }
            } else {
                log::debug!("未找到Token: {}", domain);
                Ok(None)
            }
        }
        Err(e) => {
            log::error!("无法读取存储: {}", e);
            Err(format!("无法读取存储: {}", e))
        }
    }
}

/// 删除Personal Access Token
#[command]
pub async fn delete_access_token(app_handle: AppHandle, domain: String) -> Result<(), String> {
    log::debug!("删除访问令牌: {}", domain);

    let store = StoreBuilder::new(&app_handle, "tokens.dat").build();

    match store {
        Ok(store) => {
            store.delete(&domain);
            match store.save() {
                Ok(_) => {
                    log::info!("Token删除成功: {}", domain);
                    Ok(())
                }
                Err(e) => {
                    log::error!("Token删除失败: {}", e);
                    Err(format!("Token删除失败: {}", e))
                }
            }
        }
        Err(e) => {
            log::error!("无法创建存储: {}", e);
            Err(format!("无法创建存储: {}", e))
        }
    }
}

/// 获取所有存储的Token
#[command]
pub async fn get_all_tokens(app_handle: AppHandle) -> Result<Vec<TokenConfig>, String> {
    log::debug!("获取所有访问令牌");

    let store = StoreBuilder::new(&app_handle, "tokens.dat").build();

    match store {
        Ok(store) => {
            let mut tokens = Vec::new();

            for (_, value) in store.entries() {
                match serde_json::from_value::<TokenConfig>(value.clone()) {
                    Ok(token_config) => tokens.push(token_config),
                    Err(e) => log::warn!("跳过无效的Token配置: {}", e),
                }
            }

            log::debug!("找到 {} 个Token", tokens.len());
            Ok(tokens)
        }
        Err(e) => {
            log::error!("无法读取存储: {}", e);
            Err(format!("无法读取存储: {}", e))
        }
    }
}

/// 更新Token的最后使用时间
#[command]
pub async fn update_token_last_used(app_handle: AppHandle, domain: String) -> Result<(), String> {
    log::debug!("更新Token最后使用时间: {}", domain);

    // 获取现有Token
    if let Ok(Some(mut token_config)) = get_access_token(app_handle.clone(), domain.clone()).await {
        token_config.last_used = Some(chrono::Utc::now().timestamp());

        // 重新存储
        store_access_token(
            app_handle,
            token_config.domain,
            token_config.token,
            token_config.username,
        )
        .await
    } else {
        Err("Token不存在".to_string())
    }
}

// ==================== 系统Git命令实现 ====================

/// 使用系统Git命令执行fetch操作（用于SSH协议）
#[command]
pub async fn fetch_remote_with_system_git(
    repo_path: String,
    remote_name: Option<String>,
    ssh_key_path: Option<String>,
) -> Result<crate::git::types::SyncResult, String> {
    log::debug!(
        "使用系统Git执行fetch: {} (remote: {:?}, ssh_key: {:?})",
        repo_path,
        remote_name,
        ssh_key_path
    );

    let remote = if let Some(name) = remote_name {
        name
    } else {
        // 动态检测默认远程名称
        match detect_default_remote_name(&repo_path).await {
            Ok(name) => name,
            Err(e) => {
                log::error!("检测默认远程名称失败: {}", e);
                return Err(format!("检测默认远程名称失败: {}", e));
            }
        }
    };

    let mut cmd = crate::utils::system_command::create_hidden_command_async("git");

    // 如果指定了SSH密钥，设置GIT_SSH_COMMAND环境变量
    if let Some(ssh_key) = ssh_key_path {
        let ssh_command = format!(
            "ssh -i \"{}\" -o StrictHostKeyChecking=no -o ConnectTimeout=10 -o BatchMode=yes",
            ssh_key
        );
        cmd.env("GIT_SSH_COMMAND", ssh_command);
        log::debug!("使用SSH密钥: {}", ssh_key);
    } else {
        // 即使没有指定密钥，也设置超时和批处理模式
        cmd.env(
            "GIT_SSH_COMMAND",
            "ssh -o ConnectTimeout=10 -o BatchMode=yes",
        );
    }

    cmd.arg("fetch").arg(&remote).current_dir(&repo_path);

    // 添加30秒超时
    let output = tokio::time::timeout(std::time::Duration::from_secs(30), cmd.output()).await;

    match output {
        Ok(Ok(output)) => {
            if output.status.success() {
                // 获取ahead/behind状态
                let (ahead, behind) = get_ahead_behind_with_git(&repo_path).await?;

                Ok(crate::git::types::SyncResult {
                    success: true,
                    message: "成功获取远程变更".to_string(),
                    has_conflicts: false,
                    conflict_files: vec![],
                    ahead: ahead.max(0) as u32,
                    behind: behind.max(0) as u32,
                })
            } else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                log::error!("Git fetch失败: {}", error_msg);
                Err(format!("Git fetch失败: {}", error_msg))
            }
        }
        Ok(Err(e)) => {
            log::error!("执行Git命令失败: {}", e);
            Err(format!("执行Git命令失败: {}", e))
        }
        Err(_) => {
            log::error!("Git fetch操作超时 (30秒)");
            Err("Git fetch操作超时，可能是SSH密钥认证失败或网络问题".to_string())
        }
    }
}

/// 使用系统Git命令执行push操作（用于SSH协议）
#[command]
pub async fn push_remote_with_system_git(
    repo_path: String,
    remote_name: Option<String>,
    force: Option<bool>,
    ssh_key_path: Option<String>,
) -> Result<crate::git::types::SyncResult, String> {
    log::debug!(
        "使用系统Git执行push: {} (remote: {:?}, force: {:?}, ssh_key: {:?})",
        repo_path,
        remote_name,
        force,
        ssh_key_path
    );

    let remote = if let Some(name) = remote_name {
        name
    } else {
        // 动态检测默认远程名称
        match detect_default_remote_name(&repo_path).await {
            Ok(name) => name,
            Err(e) => {
                log::error!("检测默认远程名称失败: {}", e);
                return Err(format!("检测默认远程名称失败: {}", e));
            }
        }
    };
    let force_flag = force.unwrap_or(false);

    // 获取当前分支
    let current_branch = get_current_branch_with_git(&repo_path).await?;

    let mut cmd = crate::utils::system_command::create_hidden_command_async("git");

    // 如果指定了SSH密钥，设置GIT_SSH_COMMAND环境变量
    if let Some(ssh_key) = ssh_key_path {
        let ssh_command = format!(
            "ssh -i \"{}\" -o StrictHostKeyChecking=no -o ConnectTimeout=10 -o BatchMode=yes",
            ssh_key
        );
        cmd.env("GIT_SSH_COMMAND", ssh_command);
        log::debug!("使用SSH密钥: {}", ssh_key);
    } else {
        // 即使没有指定密钥，也设置超时和批处理模式
        cmd.env(
            "GIT_SSH_COMMAND",
            "ssh -o ConnectTimeout=10 -o BatchMode=yes",
        );
    }

    cmd.arg("push");

    if force_flag {
        cmd.arg("--force");
    }

    cmd.arg(&remote)
        .arg(&current_branch)
        .current_dir(&repo_path);

    // 添加30秒超时
    let output = tokio::time::timeout(std::time::Duration::from_secs(30), cmd.output()).await;

    match output {
        Ok(Ok(output)) => {
            if output.status.success() {
                // 获取ahead/behind状态
                let (ahead, behind) = get_ahead_behind_with_git(&repo_path).await?;

                Ok(crate::git::types::SyncResult {
                    success: true,
                    message: "成功推送到远程仓库".to_string(),
                    has_conflicts: false,
                    conflict_files: vec![],
                    ahead: ahead.max(0) as u32,
                    behind: behind.max(0) as u32,
                })
            } else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                log::error!("Git push失败: {}", error_msg);
                Err(format!("Git push失败: {}", error_msg))
            }
        }
        Ok(Err(e)) => {
            log::error!("执行Git命令失败: {}", e);
            Err(format!("执行Git命令失败: {}", e))
        }
        Err(_) => {
            log::error!("Git push操作超时 (30秒)");
            Err("Git push操作超时，可能是SSH密钥认证失败或网络问题".to_string())
        }
    }
}

/// 使用系统Git命令执行pull操作（用于SSH协议）
#[command]
pub async fn pull_remote_with_system_git(
    repo_path: String,
    strategy: String,
    ssh_key_path: Option<String>,
) -> Result<crate::git::types::SyncResult, String> {
    log::debug!(
        "使用系统Git执行pull: {} (strategy: {}, ssh_key: {:?})",
        repo_path,
        strategy,
        ssh_key_path
    );

    let mut cmd = crate::utils::system_command::create_hidden_command_async("git");

    // 如果指定了SSH密钥，设置GIT_SSH_COMMAND环境变量
    if let Some(ssh_key) = ssh_key_path {
        let ssh_command = format!(
            "ssh -i \"{}\" -o StrictHostKeyChecking=no -o ConnectTimeout=10 -o BatchMode=yes",
            ssh_key
        );
        cmd.env("GIT_SSH_COMMAND", ssh_command);
        log::debug!("使用SSH密钥: {}", ssh_key);
    } else {
        // 即使没有指定密钥，也设置超时和批处理模式
        cmd.env(
            "GIT_SSH_COMMAND",
            "ssh -o ConnectTimeout=10 -o BatchMode=yes",
        );
    }

    cmd.arg("pull");

    match strategy.as_str() {
        "rebase" => {
            cmd.arg("--rebase");
        }
        "merge" => {
            // 默认行为，不需要额外参数
        }
        _ => {
            return Err("无效的拉取策略，支持: merge, rebase".to_string());
        }
    }

    cmd.current_dir(&repo_path);

    // 添加30秒超时
    let output = tokio::time::timeout(std::time::Duration::from_secs(30), cmd.output()).await;

    match output {
        Ok(Ok(output)) => {
            if output.status.success() {
                // 获取ahead/behind状态
                let (ahead, behind) = get_ahead_behind_with_git(&repo_path).await?;

                Ok(crate::git::types::SyncResult {
                    success: true,
                    message: "成功拉取远程变更".to_string(),
                    has_conflicts: false,
                    conflict_files: vec![],
                    ahead: ahead.max(0) as u32,
                    behind: behind.max(0) as u32,
                })
            } else {
                let error_msg = String::from_utf8_lossy(&output.stderr);

                // 检查是否有冲突
                if error_msg.contains("CONFLICT") || error_msg.contains("conflict") {
                    // 获取冲突文件列表
                    let conflict_files = get_conflict_files_with_git(&repo_path).await?;

                    Ok(crate::git::types::SyncResult {
                        success: false,
                        message: "拉取时发现冲突".to_string(),
                        has_conflicts: true,
                        conflict_files,
                        ahead: 0,
                        behind: 0,
                    })
                } else {
                    log::error!("Git pull失败: {}", error_msg);
                    Err(format!("Git pull失败: {}", error_msg))
                }
            }
        }
        Ok(Err(e)) => {
            log::error!("执行Git命令失败: {}", e);
            Err(format!("执行Git命令失败: {}", e))
        }
        Err(_) => {
            log::error!("Git pull操作超时 (30秒)");
            Err("Git pull操作超时，可能是SSH密钥认证失败或网络问题".to_string())
        }
    }
}

// ==================== 辅助函数 ====================

/// 使用系统Git命令获取ahead/behind状态
async fn get_ahead_behind_with_git(repo_path: &str) -> Result<(i32, i32), String> {
    let mut cmd = crate::utils::system_command::create_hidden_command_async("git");
    cmd.arg("rev-list")
        .arg("--left-right")
        .arg("--count")
        .arg("HEAD...@{upstream}")
        .current_dir(repo_path);

    let output = cmd.output().await;

    match output {
        Ok(output) => {
            if output.status.success() {
                let result = String::from_utf8_lossy(&output.stdout);
                let parts: Vec<&str> = result.trim().split('\t').collect();

                if parts.len() == 2 {
                    let ahead = parts[0].parse::<i32>().unwrap_or(0);
                    let behind = parts[1].parse::<i32>().unwrap_or(0);
                    Ok((ahead, behind))
                } else {
                    Ok((0, 0))
                }
            } else {
                // 如果没有upstream，返回0
                Ok((0, 0))
            }
        }
        Err(_) => Ok((0, 0)),
    }
}

/// 使用系统Git命令获取当前分支
async fn get_current_branch_with_git(repo_path: &str) -> Result<String, String> {
    let mut cmd = crate::utils::system_command::create_hidden_command_async("git");
    cmd.arg("branch")
        .arg("--show-current")
        .current_dir(repo_path);

    let output = cmd.output().await;

    match output {
        Ok(output) => {
            if output.status.success() {
                let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if branch.is_empty() {
                    Err("无法获取当前分支".to_string())
                } else {
                    Ok(branch)
                }
            } else {
                Err("获取当前分支失败".to_string())
            }
        }
        Err(e) => Err(format!("执行Git命令失败: {}", e)),
    }
}

/// 使用系统Git命令获取冲突文件列表
async fn get_conflict_files_with_git(repo_path: &str) -> Result<Vec<String>, String> {
    let mut cmd = crate::utils::system_command::create_hidden_command_async("git");
    cmd.arg("diff")
        .arg("--name-only")
        .arg("--diff-filter=U")
        .current_dir(repo_path);

    let output = cmd.output().await;

    match output {
        Ok(output) => {
            if output.status.success() {
                let files = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .map(|line| line.trim().to_string())
                    .filter(|line| !line.is_empty())
                    .collect();
                Ok(files)
            } else {
                Ok(vec![])
            }
        }
        Err(_) => Ok(vec![]),
    }
}

// ==================== 分支管理命令 ====================

/// 获取分支列表
#[command]
pub async fn list_branches(
    repo_path: String,
) -> Result<Vec<crate::git::operations::BranchInfo>, String> {
    log::debug!("获取分支列表: {}", repo_path);

    match crate::git::operations::list_branches(&repo_path) {
        Ok(branches) => {
            log::debug!("成功获取 {} 个分支", branches.len());
            Ok(branches)
        }
        Err(e) => {
            log::error!("获取分支列表失败: {}", e);
            Err(e.to_string())
        }
    }
}

/// 创建新分支
#[command]
pub async fn create_branch(
    repo_path: String,
    branch_name: String,
    from_commit: Option<String>,
    checkout: Option<bool>,
) -> Result<crate::git::operations::SwitchResult, String> {
    log::debug!(
        "创建分支: {} (from: {:?}, checkout: {:?})",
        repo_path,
        from_commit,
        checkout
    );

    match crate::git::operations::create_branch(
        &repo_path,
        &branch_name,
        from_commit.as_deref(),
        checkout.unwrap_or(false),
    ) {
        Ok(result) => {
            log::debug!("分支创建成功: {}", branch_name);
            Ok(result)
        }
        Err(e) => {
            log::error!("创建分支失败: {}", e);
            Err(e.to_string())
        }
    }
}

/// 切换分支
#[command]
pub async fn switch_branch(
    repo_path: String,
    branch_name: String,
) -> Result<crate::git::operations::SwitchResult, String> {
    log::debug!("切换分支: {} -> {}", repo_path, branch_name);

    match crate::git::operations::switch_branch(&repo_path, &branch_name) {
        Ok(result) => {
            log::debug!("分支切换结果: {:?}", result);
            Ok(result)
        }
        Err(e) => {
            log::error!("切换分支失败: {}", e);
            Err(e.to_string())
        }
    }
}

/// 删除分支
#[command]
pub async fn delete_branch(
    repo_path: String,
    branch_name: String,
    force: Option<bool>,
) -> Result<crate::git::operations::SwitchResult, String> {
    log::debug!(
        "删除分支: {} (branch: {}, force: {:?})",
        repo_path,
        branch_name,
        force
    );

    match crate::git::operations::delete_branch(&repo_path, &branch_name, force.unwrap_or(false)) {
        Ok(result) => {
            log::debug!("分支删除成功: {}", branch_name);
            Ok(result)
        }
        Err(e) => {
            log::error!("删除分支失败: {}", e);
            Err(e.to_string())
        }
    }
}

/// 检出远程分支
#[command]
pub async fn checkout_remote_branch(
    repo_path: String,
    remote_branch_name: String,
    local_branch_name: Option<String>,
) -> Result<crate::git::operations::SwitchResult, String> {
    log::debug!(
        "检出远程分支: {} -> {:?}",
        remote_branch_name,
        local_branch_name
    );

    match crate::git::operations::checkout_remote_branch(
        &repo_path,
        &remote_branch_name,
        local_branch_name.as_deref(),
    ) {
        Ok(result) => {
            log::debug!("远程分支检出成功: {}", remote_branch_name);
            Ok(result)
        }
        Err(e) => {
            log::error!("检出远程分支失败: {}", e);
            Err(e.to_string())
        }
    }
}

/// 检测并验证仓库的远程配置
#[command]
pub async fn detect_repository_remotes(repo_path: String) -> Result<Vec<String>, String> {
    log::debug!("检测仓库远程配置: {}", repo_path);

    match git2::Repository::open(&repo_path) {
        Ok(repo) => match repo.remotes() {
            Ok(remotes) => {
                let remote_list: Vec<String> =
                    remotes.iter().flatten().map(|s| s.to_string()).collect();
                log::debug!("找到远程列表: {:?}", remote_list);

                if remote_list.is_empty() {
                    Err("仓库中没有配置任何远程仓库".to_string())
                } else {
                    Ok(remote_list)
                }
            }
            Err(e) => {
                log::error!("获取远程列表失败: {}", e);
                Err(format!("获取远程列表失败: {}", e))
            }
        },
        Err(e) => {
            log::error!("打开仓库失败: {}", e);
            Err(format!("打开仓库失败: {}", e))
        }
    }
}

/// 获取仓库的默认远程名称
#[command]
pub async fn get_default_remote_name_command(repo_path: String) -> Result<String, String> {
    log::debug!("获取默认远程名称: {}", repo_path);

    match detect_default_remote_name(&repo_path).await {
        Ok(remote_name) => {
            log::debug!("默认远程名称: {}", remote_name);
            Ok(remote_name)
        }
        Err(e) => {
            log::error!("获取默认远程名称失败: {}", e);
            Err(e)
        }
    }
}

// ==================== 远程名称检测辅助函数 ====================

/// 检测仓库的默认远程名称
async fn detect_default_remote_name(repo_path: &str) -> Result<String, String> {
    log::debug!("检测仓库默认远程名称: {}", repo_path);

    // 使用git2库检测远程名称
    match git2::Repository::open(repo_path) {
        Ok(repo) => match crate::git::operations::get_default_remote_name(&repo) {
            Ok(remote_name) => {
                log::debug!("检测到默认远程名称: {}", remote_name);
                Ok(remote_name)
            }
            Err(e) => {
                log::error!("使用git2检测远程名称失败: {}", e);
                Err(format!("检测远程名称失败: {}", e))
            }
        },
        Err(e) => {
            log::error!("打开仓库失败: {}", e);
            Err(format!("打开仓库失败: {}", e))
        }
    }
}

// ==================== Token认证辅助函数 ====================

/// 获取仓库的远程URL（内部辅助函数）
async fn get_repository_remote_url(repo_path: String) -> Result<Option<String>, String> {
    match crate::git::operations::get_remote_info(&repo_path) {
        Ok(remote_info) => {
            // 这里需要实际获取远程URL，暂时返回None
            // 在实际实现中，应该从git2库获取远程URL
            log::debug!("获取远程信息成功: {:?}", remote_info);

            // 尝试使用git2获取远程URL
            match git2::Repository::open(&repo_path) {
                Ok(repo) => {
                    if let Ok(remote) = repo.find_remote(&remote_info.remote_name) {
                        if let Some(url) = remote.url() {
                            return Ok(Some(url.to_string()));
                        }
                    }
                    Ok(None)
                }
                Err(_) => Ok(None),
            }
        }
        Err(e) => {
            log::error!("获取远程信息失败: {}", e);
            Err(e.to_string())
        }
    }
}

/// 为指定仓库获取Token
async fn get_token_for_repository(
    app_handle: &AppHandle,
    repo_path: &str,
) -> Result<Option<String>, String> {
    // 获取远程URL
    let remote_url = match get_repository_remote_url(repo_path.to_string()).await? {
        Some(url) => url,
        None => {
            log::debug!("未找到远程URL");
            return Ok(None);
        }
    };

    log::debug!("仓库远程URL: {}", remote_url);

    // 检查是否是HTTPS协议
    if !remote_url.starts_with("https://") && !remote_url.starts_with("http://") {
        log::debug!("非HTTPS协议，不需要Token认证");
        return Ok(None);
    }

    // 从URL提取域名
    let domain = match extract_domain_from_url(remote_url).await {
        Ok(domain) => domain,
        Err(e) => {
            log::warn!("提取域名失败: {}", e);
            return Ok(None);
        }
    };

    log::debug!("提取到域名: {}", domain);

    // 获取对应的Token
    match get_access_token(app_handle.clone(), domain.clone()).await {
        Ok(Some(token_config)) => {
            log::debug!("找到Token配置: {}", domain);

            // 更新最后使用时间
            let _ = update_token_last_used(app_handle.clone(), domain).await;

            Ok(Some(token_config.token))
        }
        Ok(None) => {
            log::debug!("未找到Token配置: {}", domain);
            Ok(None)
        }
        Err(e) => {
            log::error!("获取Token失败: {}", e);
            Err(e)
        }
    }
}

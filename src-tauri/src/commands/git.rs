use crate::git::{AuthConfig, AuthManager, CloneManager, CloneOptions, CloneResult};
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

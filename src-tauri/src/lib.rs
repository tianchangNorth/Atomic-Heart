// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{ Emitter };
mod http_client; // 导入新模块
mod git;
mod commands;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use std::sync::{Arc, Mutex};
use warp::Filter;
// 全局服务器状态
static SERVER_STATE: Mutex<Option<tokio::task::JoinHandle<()>>> = Mutex::new(None);

#[tauri::command]
async fn start_oauth_callback_server(app_handle: tauri::AppHandle) -> Result<String, String> {
    // 检查是否已有服务器在运行
    {
        let mut state = SERVER_STATE.lock().unwrap();
        if let Some(handle) = state.take() {
            handle.abort(); // 停止之前的服务器
        }
    }
    
    let app_handle = Arc::new(app_handle);
    
    // 创建回调路由
    let callback = warp::path("callback")
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .and(warp::any().map(move || app_handle.clone()))
        .and_then(|params: std::collections::HashMap<String, String>, app_handle: Arc<tauri::AppHandle>| async move {
            let code = params.get("code").cloned();
            let state = params.get("state").cloned();
            
            if let (Some(code), Some(state)) = (code, state) {
                let _ = app_handle.emit("oauth-callback", serde_json::json!({
                    "code": code,
                    "state": state
                }));
            }
            
            Ok::<_, warp::Rejection>(warp::reply::html(
                "<html><body><h1>授权成功！</h1><p>您可以关闭此页面并返回应用。</p><script>setTimeout(() => window.close(), 2000);</script></body></html>"
            ))
        });
    
    let port = 8080;
    let handle = tokio::spawn(async move {
        warp::serve(callback)
            .run(([127, 0, 0, 1], port))
            .await;
    });
    
    // 保存服务器句柄
    {
        let mut state = SERVER_STATE.lock().unwrap();
        *state = Some(handle);
    }
    
    Ok(format!("http://localhost:{}/callback", port))
}

#[tauri::command]
async fn open_url(url: String) -> Result<(), String> {
    open::that(url).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_oauth::init())
        .manage(commands::git::GitState::default())
        .invoke_handler(tauri::generate_handler![
            greet,
            open_url,
            start_oauth_callback_server, // 添加新命令
            http_client::http_get,
            http_client::http_post,
            // Git 命令
            commands::git::clone_repository,
            commands::git::validate_repository_url,
            commands::git::detect_auth_type,
            commands::git::get_default_ssh_keys,
            commands::git::validate_ssh_key,
            commands::git::store_credentials,
            commands::git::load_credentials,
            commands::git::delete_credentials,
            commands::git::extract_username_from_url,
            commands::git::cancel_clone_operation,
            commands::git::get_clone_operation_status,
            commands::git::cleanup_clone_operation,
            commands::git::select_directory,
            commands::git::select_ssh_key_file,
            commands::git::validate_clone_directory,
            // 新增的 Git 信息获取命令
            commands::git::is_git_repository,
            commands::git::get_repository_info,
            commands::git::get_current_branch,
            commands::git::get_remote_url,
            commands::git::open_folder,
            // Git 操作命令
            commands::git::get_repository_status,
            commands::git::stage_files,
            commands::git::unstage_files,
            commands::git::create_commit,
            commands::git::get_commit_history,
            commands::git::get_file_diff,
            // 同步操作命令
            commands::git::fetch_remote,
            commands::git::pull_remote,
            commands::git::push_remote,
            commands::git::get_remote_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

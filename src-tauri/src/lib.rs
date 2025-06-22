// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{Emitter, Manager};
mod commands;
mod git;
mod http_client; // 导入新模块
mod utils;

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

            // 发送事件到前端
            if let (Some(ref code_val), Some(ref state_val)) = (&code, &state) {
                let _ = app_handle.emit("oauth-callback", serde_json::json!({
                    "code": code_val,
                    "state": state_val
                }));
            }

            // 准备用于HTML的值
            let code_str = code.as_deref().unwrap_or("");
            let state_str = state.as_deref().unwrap_or("");

            Ok::<_, warp::Rejection>(warp::reply::html(
                format!(
                    r#"<html>
                    <head><title>授权成功</title></head>
                    <body>
                        <h1>授权成功！</h1>
                        <p>正在返回应用...</p>
                        <script>
                            // 尝试通过深度链接唤起应用
                            const deepLink = 'atomic-heart://auth/callback?code={}&state={}';
                            window.location.href = deepLink;

                            // 如果深度链接失败，显示手动操作提示
                            setTimeout(() => {{
                                document.body.innerHTML = '<h1>授权成功！</h1><p>请返回应用继续操作。如果应用没有自动打开，请手动打开应用。</p>';
                            }}, 3000);
                        </script>
                    </body>
                    </html>"#,
                    code_str,
                    state_str
                )
            ))
        });

    let port = 8080;
    let handle = tokio::spawn(async move {
        warp::serve(callback).run(([127, 0, 0, 1], port)).await;
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

// 处理深度链接的命令
#[tauri::command]
async fn handle_deep_link(app_handle: tauri::AppHandle, url: String) -> Result<(), String> {
    println!("收到深度链接: {}", url);

    // 解析URL参数
    if let Ok(parsed_url) = url::Url::parse(&url) {
        let mut params = std::collections::HashMap::new();

        // 提取查询参数
        for (key, value) in parsed_url.query_pairs() {
            params.insert(key.to_string(), value.to_string());
        }

        // 提取路径
        let path = parsed_url.path();

        // 发送事件到前端
        let _ = app_handle.emit(
            "deep-link-received",
            serde_json::json!({
                "url": url,
                "path": path,
                "params": params
            }),
        );

        // 如果窗口被最小化或隐藏，则显示并聚焦
        if let Some(window) = app_handle.get_webview_window("main") {
            let _ = window.show();
            let _ = window.set_focus();
            let _ = window.unminimize();
        }

        Ok(())
    } else {
        Err("无效的URL格式".to_string())
    }
}

// 注册自定义协议的命令
#[tauri::command]
async fn register_protocol_handler() -> Result<String, String> {
    // 在Windows上，协议会在安装时自动注册
    // 这个命令主要用于获取协议信息
    Ok("atomic-heart://".to_string())
}

// 获取启动参数的命令
#[tauri::command]
async fn get_startup_args() -> Result<Vec<String>, String> {
    let args: Vec<String> = std::env::args().collect();
    Ok(args)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    // 配置单实例模式（仅桌面平台）
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
            println!("检测到新实例启动，参数: {argv:?}");

            // 将现有窗口置于前台
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
                let _ = window.unminimize();
            }

            // 处理深度链接参数
            for arg in argv.iter() {
                if arg.starts_with("atomic-heart://") {
                    // 发送深度链接事件到现有实例
                    let _ = app.emit(
                        "deep-link-received",
                        serde_json::json!({
                            "url": arg,
                            "source": "new_instance"
                        }),
                    );
                    break;
                }
            }
        }));
    }

    builder
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_oauth::init())
        .plugin(tauri_plugin_deep_link::init())
        .manage(commands::git::GitState::default())
        .setup(|app| {
            // 设置深度链接处理
            use tauri_plugin_deep_link::DeepLinkExt;

            // 监听深度链接事件
            app.deep_link().on_open_url(|event| {
                println!("深度链接 URLs: {:?}", event.urls());
                // 这里的事件会被单实例插件处理，主要用于日志记录
            });

            // 在开发模式下注册深度链接（仅限 Windows 和 Linux）
            #[cfg(any(windows, target_os = "linux"))]
            {
                if let Err(e) = app.deep_link().register_all() {
                    eprintln!("注册深度链接失败: {}", e);
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            open_url,
            start_oauth_callback_server, // 添加新命令
            handle_deep_link,
            register_protocol_handler,
            get_startup_args,
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
            commands::git::get_remote_info,
            // 智能Git操作（支持Token认证）
            commands::git::smart_fetch_remote,
            commands::git::smart_push_remote,
            // 双协议认证系统
            commands::git::detect_repository_protocol,
            commands::git::extract_domain_from_url,
            commands::git::store_access_token,
            commands::git::get_access_token,
            commands::git::delete_access_token,
            commands::git::get_all_tokens,
            commands::git::update_token_last_used,
            // 系统Git命令
            commands::git::fetch_remote_with_system_git,
            commands::git::push_remote_with_system_git,
            commands::git::pull_remote_with_system_git,
            // 远程名称检测
            commands::git::detect_repository_remotes,
            commands::git::get_default_remote_name_command,
            // 分支管理
            commands::git::list_branches,
            commands::git::create_branch,
            commands::git::switch_branch,
            commands::git::delete_branch,
            commands::git::checkout_remote_branch
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

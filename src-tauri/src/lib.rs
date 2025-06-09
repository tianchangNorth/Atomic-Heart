// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{command, Emitter, Window};
mod http_client; // 导入新模块

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 添加 OAuth 相关命令
#[command]
async fn start_oauth_server(window: Window, url: String) -> Result<u16, String> {
    // 先尝试关闭可能存在的之前的 OAuth 服务器
    let _ = tauri_plugin_oauth::cancel(5800); // 尝试关闭5800端口上

    // 使用固定端口配置
    let config = tauri_plugin_oauth::OauthConfig {
        ports: Some(vec![5800]), // 只尝试使用5800端口
        response: Some("Authorization completed. You can return to the application now.".into()),
    };

    let port = tauri_plugin_oauth::start_with_config(config, move |redirect_url| {
        let _ = window.emit("oauth://callback", redirect_url);
    })
    .map_err(|err| err.to_string())?;

    // 使用系统默认浏览器打开授权 URL
    open::that(url).map_err(|err| err.to_string())?;

    Ok(port)
}

// 在 run 函数中调用并处理错误
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_oauth::init()) // 初始化 OAuth 插件
        .plugin(tauri_plugin_opener::init()) // 保留原有的 opener 插件
        .invoke_handler(tauri::generate_handler![
            greet,
            start_oauth_server,
            http_client::http_get,
            http_client::http_post,
        ]) // 添加 start_oauth_server 命令
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

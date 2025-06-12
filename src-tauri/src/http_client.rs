use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::command;

// 创建一个HTTP客户端单例
lazy_static::lazy_static! {
    static ref HTTP_CLIENT: Client = Client::builder()
        .build()
        .expect("Failed to create HTTP client");
}

// 通用响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub code: u32,
    pub message: Option<String>,
    pub data: Option<T>,
}

// OAuth令牌响应
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
}

// 通用HTTP GET请求
#[command]
pub async fn http_get(
    url: String,
    headers: Option<HashMap<String, String>>,
    data: Option<HashMap<String, serde_json::Value>>, // 新增参数
) -> Result<ApiResponse<serde_json::Value>, String> {
    let mut request = HTTP_CLIENT.get(&url);

    // 添加查询参数
    if let Some(params_map) = data {
        request = request.query(&params_map);
    }

    if let Some(headers_map) = headers {
        for (key, value) in headers_map {
            request = request.header(key, value);
        }
    }

    let response = request.send().await.map_err(|e| e.to_string())?;

    handle_response(response).await
}

// 通用HTTP POST请求
#[command]
pub async fn http_post(
    url: String,
    data: Option<serde_json::Value>,
    headers: Option<HashMap<String, String>>,
) -> Result<ApiResponse<serde_json::Value>, String> {
    // 使用具体类型 serde_json::Value
    let mut request = HTTP_CLIENT.post(&url);

    if let Some(headers_map) = headers {
        for (key, value) in headers_map {
            request = request.header(key, value);
        }
    }

    if let Some(json_body) = data {
        request = request.json(&json_body);
    }

    let response = request.send().await.map_err(|e| e.to_string())?;

    handle_response(response).await
}

// 处理HTTP响应
async fn handle_response(response: Response) -> Result<ApiResponse<serde_json::Value>, String> {
    let status = response.status();

    if status.is_success() {
        match response.json::<serde_json::Value>().await {
            Ok(data) => Ok(ApiResponse {
                success: true,
                code: 0,
                message: Some("Request successful".to_string()),
                data: Some(data),
            }),
            Err(e) => Ok(ApiResponse {
                success: false,
                code: 1001,
                message: Some(format!("Failed to parse response: {}", e)),
                data: None,
            }),
        }
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Ok(ApiResponse {
            success: false,
            code: status.as_u16() as u32,
            message: Some(format!(
                "Request failed with status {}: {}",
                status, error_text
            )),
            data: None,
        })
    }
}

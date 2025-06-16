use crate::git::types::{AuthConfig, AuthType, GitError};
use git2::{Cred, CredentialType};
use keyring::Entry;
use std::path::Path;

/// 认证管理器
pub struct AuthManager;

impl AuthManager {
    /// 创建 Git 凭据
    pub fn create_credentials(
        auth: &AuthConfig,
        _url: &str,
        username_from_url: Option<&str>,
        allowed_types: CredentialType,
    ) -> Result<Cred, GitError> {
        match auth.auth_type {
            AuthType::None => {
                // 尝试默认凭据
                if allowed_types.contains(CredentialType::DEFAULT) {
                    Cred::default().map_err(GitError::Git)
                } else {
                    Err(GitError::AuthenticationFailed {
                        message: "需要认证但未提供凭据".to_string(),
                    })
                }
            }
            AuthType::Password => {
                if !allowed_types.contains(CredentialType::USER_PASS_PLAINTEXT) {
                    return Err(GitError::AuthenticationFailed {
                        message: "服务器不支持用户名/密码认证".to_string(),
                    });
                }

                let username = auth
                    .username
                    .as_deref()
                    .or(username_from_url)
                    .ok_or_else(|| GitError::AuthenticationFailed {
                        message: "缺少用户名".to_string(),
                    })?;

                let password =
                    auth.password
                        .as_ref()
                        .ok_or_else(|| GitError::AuthenticationFailed {
                            message: "缺少密码".to_string(),
                        })?;

                Cred::userpass_plaintext(username, password).map_err(GitError::Git)
            }
            AuthType::Token => {
                if !allowed_types.contains(CredentialType::USER_PASS_PLAINTEXT) {
                    return Err(GitError::AuthenticationFailed {
                        message: "服务器不支持 Token 认证".to_string(),
                    });
                }

                let token = auth
                    .token
                    .as_ref()
                    .ok_or_else(|| GitError::AuthenticationFailed {
                        message: "缺少访问令牌".to_string(),
                    })?;

                // 对于 GitHub 等服务，使用 token 作为密码，用户名可以是任意值
                let username = auth
                    .username
                    .as_deref()
                    .or(username_from_url)
                    .unwrap_or("token");

                Cred::userpass_plaintext(username, token).map_err(GitError::Git)
            }
            AuthType::Ssh => {
                if !allowed_types.contains(CredentialType::SSH_KEY) {
                    return Err(GitError::AuthenticationFailed {
                        message: "服务器不支持 SSH 密钥认证".to_string(),
                    });
                }

                let username = auth
                    .username
                    .as_deref()
                    .or(username_from_url)
                    .unwrap_or("git");

                if let Some(key_path) = &auth.ssh_key_path {
                    let public_key_path = Self::get_public_key_path(key_path);
                    let passphrase = auth.ssh_key_passphrase.as_deref();

                    Cred::ssh_key(
                        username,
                        public_key_path.as_ref().map(|p| Path::new(p)),
                        Path::new(key_path),
                        passphrase,
                    )
                    .map_err(GitError::Git)
                } else {
                    // 尝试使用 SSH Agent
                    Cred::ssh_key_from_agent(username).map_err(GitError::Git)
                }
            }
        }
    }

    /// 获取公钥路径
    fn get_public_key_path(private_key_path: &str) -> Option<String> {
        let _path = Path::new(private_key_path);

        // 尝试常见的公钥扩展名
        let extensions = [".pub", ""];
        for ext in &extensions {
            let public_path = if ext.is_empty() {
                format!("{}.pub", private_key_path)
            } else {
                private_key_path.replace(".pem", ext).replace(".key", ext) + ext
            };

            if Path::new(&public_path).exists() {
                return Some(public_path);
            }
        }

        None
    }

    /// 存储凭据到系统密钥环
    pub fn store_credentials(url: &str, auth: &AuthConfig) -> Result<(), GitError> {
        let service = "AtomDesk";
        let account = format!("git:{}", url);

        match Entry::new(service, &account) {
            Ok(entry) => {
                let credentials = serde_json::to_string(auth).map_err(|e| GitError::Unknown {
                    message: format!("序列化凭据失败: {}", e),
                })?;

                entry
                    .set_password(&credentials)
                    .map_err(|e| GitError::Unknown {
                        message: format!("存储凭据失败: {}", e),
                    })?;

                Ok(())
            }
            Err(e) => Err(GitError::Unknown {
                message: format!("创建密钥环条目失败: {}", e),
            }),
        }
    }

    /// 从系统密钥环加载凭据
    pub fn load_credentials(url: &str) -> Result<Option<AuthConfig>, GitError> {
        let service = "AtomDesk";
        let account = format!("git:{}", url);

        match Entry::new(service, &account) {
            Ok(entry) => match entry.get_password() {
                Ok(password) => {
                    let auth: AuthConfig =
                        serde_json::from_str(&password).map_err(|e| GitError::Unknown {
                            message: format!("反序列化凭据失败: {}", e),
                        })?;
                    Ok(Some(auth))
                }
                Err(keyring::Error::NoEntry) => Ok(None),
                Err(e) => Err(GitError::Unknown {
                    message: format!("加载凭据失败: {}", e),
                }),
            },
            Err(e) => Err(GitError::Unknown {
                message: format!("创建密钥环条目失败: {}", e),
            }),
        }
    }

    /// 删除存储的凭据
    pub fn delete_credentials(url: &str) -> Result<(), GitError> {
        let service = "AtomDesk";
        let account = format!("git:{}", url);

        match Entry::new(service, &account) {
            Ok(entry) => {
                entry.delete_password().map_err(|e| GitError::Unknown {
                    message: format!("删除凭据失败: {}", e),
                })?;
                Ok(())
            }
            Err(e) => Err(GitError::Unknown {
                message: format!("创建密钥环条目失败: {}", e),
            }),
        }
    }

    /// 验证 SSH 密钥
    pub fn validate_ssh_key(
        private_key_path: &str,
        _passphrase: Option<&str>,
    ) -> Result<bool, GitError> {
        let path = Path::new(private_key_path);

        if !path.exists() {
            return Ok(false);
        }

        // 尝试加载私钥来验证
        match std::fs::read_to_string(path) {
            Ok(content) => {
                // 简单检查是否是有效的 SSH 私钥格式
                let is_valid = content.contains("-----BEGIN")
                    && (content.contains("PRIVATE KEY") || content.contains("OPENSSH PRIVATE KEY"));
                Ok(is_valid)
            }
            Err(_) => Ok(false),
        }
    }

    /// 获取默认 SSH 密钥路径
    pub fn get_default_ssh_keys() -> Vec<String> {
        let mut keys = Vec::new();

        if let Some(home_dir) = dirs::home_dir() {
            let ssh_dir = home_dir.join(".ssh");

            // 常见的 SSH 密钥文件名
            let key_names = ["id_rsa", "id_ed25519", "id_ecdsa", "id_dsa"];

            for key_name in &key_names {
                let key_path = ssh_dir.join(key_name);
                if key_path.exists() {
                    if let Some(path_str) = key_path.to_str() {
                        keys.push(path_str.to_string());
                    }
                }
            }
        }

        keys
    }

    /// 检测 URL 的认证类型
    pub fn detect_auth_type(url: &str) -> AuthType {
        if url.starts_with("git@") || url.starts_with("ssh://") {
            AuthType::Ssh
        } else if url.starts_with("https://") || url.starts_with("http://") {
            // 对于 HTTPS，默认尝试 Token 认证（更安全）
            AuthType::Token
        } else {
            AuthType::None
        }
    }

    /// 从 URL 提取用户名
    pub fn extract_username_from_url(url: &str) -> Option<String> {
        // 处理 SSH URL 格式 (git@github.com:user/repo.git)
        if url.starts_with("git@") {
            if let Some(at_pos) = url.find('@') {
                return Some(url[..at_pos].to_string());
            }
        }

        // 处理 HTTP/HTTPS URL
        if let Ok(parsed_url) = url::Url::parse(url) {
            if !parsed_url.username().is_empty() {
                return Some(parsed_url.username().to_string());
            }
        }

        None
    }
}

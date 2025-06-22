use serde::{Deserialize, Serialize};

/// Git 克隆选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneOptions {
    /// 仓库 URL
    pub url: String,
    /// 目标目录
    pub directory: String,
    /// 指定分支（可选）
    pub branch: Option<String>,
    /// 克隆深度（可选）
    pub depth: Option<u32>,
    /// 是否递归克隆子模块
    pub recursive: bool,
    /// 认证信息
    pub auth: Option<AuthConfig>,
}

/// 认证配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// 认证类型
    pub auth_type: AuthType,
    /// 用户名（HTTP 认证）
    pub username: Option<String>,
    /// 密码（HTTP 认证）
    pub password: Option<String>,
    /// Personal Access Token
    pub token: Option<String>,
    /// SSH 密钥路径
    pub ssh_key_path: Option<String>,
    /// SSH 密钥密码
    pub ssh_key_passphrase: Option<String>,
}

/// 认证类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthType {
    /// HTTP 用户名/密码认证
    Password,
    /// Personal Access Token 认证
    Token,
    /// SSH 密钥认证
    Ssh,
    /// 无认证（公开仓库）
    None,
}

/// 克隆进度信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneProgress {
    /// 操作 ID
    pub id: String,
    /// 当前阶段
    pub stage: CloneStage,
    /// 进度百分比 (0-100)
    pub progress: u32,
    /// 当前消息
    pub message: String,
    /// 网络进度（可选）
    pub network_progress: Option<NetworkProgress>,
    /// 检出进度（可选）
    pub checkout_progress: Option<CheckoutProgress>,
}

/// 克隆阶段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CloneStage {
    /// 初始化
    Initializing,
    /// 连接远程仓库
    Connecting,
    /// 下载对象
    Downloading,
    /// 解压对象
    Unpacking,
    /// 检出文件
    CheckingOut,
    /// 完成
    Completed,
    /// 错误
    Error,
}

/// 网络进度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkProgress {
    /// 已接收字节数
    pub received_bytes: usize,
    /// 已接收对象数
    pub received_objects: usize,
    /// 总对象数
    pub total_objects: usize,
    /// 已索引对象数
    pub indexed_objects: usize,
}

/// 检出进度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutProgress {
    /// 已完成步骤
    pub completed_steps: usize,
    /// 总步骤数
    pub total_steps: usize,
    /// 当前路径
    pub current_path: Option<String>,
}

/// 克隆结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneResult {
    /// 是否成功
    pub success: bool,
    /// 仓库路径
    pub repository_path: Option<String>,
    /// 错误信息
    pub error: Option<String>,
    /// 克隆的分支
    pub branch: Option<String>,
    /// 最后一次提交的 SHA
    pub last_commit_sha: Option<String>,
    /// 克隆统计信息
    pub stats: Option<CloneStats>,
}

/// 克隆统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneStats {
    /// 总耗时（毫秒）
    pub duration_ms: u64,
    /// 下载的字节数
    pub downloaded_bytes: usize,
    /// 对象数量
    pub object_count: usize,
    /// 文件数量
    pub file_count: usize,
}

/// Git 错误类型
#[derive(Debug, thiserror::Error)]
pub enum GitError {
    #[error("Git 操作失败: {0}")]
    Git(#[from] git2::Error),

    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("认证失败: {message}")]
    AuthenticationFailed { message: String },

    #[error("无效的仓库 URL: {url}")]
    InvalidUrl { url: String },

    #[error("目标目录已存在: {path}")]
    DirectoryExists { path: String },

    #[error("未知错误: {message}")]
    Unknown { message: String },

    #[error("系统Git未找到")]
    SystemGitNotFound,

    #[error("系统Git执行失败: {message}")]
    SystemGitFailed { message: String },
}

impl Serialize for GitError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("GitError", 2)?;
        state.serialize_field("type", &self.error_type())?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}

impl GitError {
    pub fn error_type(&self) -> &'static str {
        match self {
            GitError::Git(_) => "git",
            GitError::Io(_) => "io",
            GitError::AuthenticationFailed { .. } => "authentication",
            GitError::InvalidUrl { .. } => "invalid_url",
            GitError::DirectoryExists { .. } => "directory_exists",
            GitError::Unknown { .. } => "unknown",
            GitError::SystemGitNotFound => "system_git_not_found",
            GitError::SystemGitFailed { .. } => "system_git_failed",
        }
    }
}

/// 仓库信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryInfo {
    /// 仓库 ID
    pub id: String,
    /// 仓库名称
    pub name: String,
    /// 本地路径
    pub path: String,
    /// 远程 URL
    pub remote_url: Option<String>,
    /// 当前分支
    pub current_branch: String,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 最后更新时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// 仓库状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryStatus {
    /// 当前分支
    pub current_branch: String,
    /// 文件状态列表
    pub files: Vec<FileStatus>,
    /// 领先提交数
    pub ahead: u32,
    /// 落后提交数
    pub behind: u32,
    /// 是否干净（无变更）
    pub is_clean: bool,
}

/// 文件状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStatus {
    /// 文件路径
    pub path: String,
    /// 状态类型
    pub status: String,
    /// 是否已暂存
    pub staged: bool,
    /// 新增行数
    pub additions: u32,
    /// 删除行数
    pub deletions: u32,
}

/// 提交选项
#[derive(Debug, Clone)]
pub struct CommitOptions {
    /// 提交消息
    pub message: String,
    /// 详细描述
    pub description: Option<String>,
    /// 作者姓名
    pub author_name: Option<String>,
    /// 作者邮箱
    pub author_email: Option<String>,
    /// 是否修正上次提交
    pub amend: bool,
    /// 是否添加签名
    pub signoff: bool,
}

/// 提交历史项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitHistoryItem {
    /// 提交SHA
    pub sha: String,
    /// 提交消息
    pub message: String,
    /// 作者姓名
    pub author_name: String,
    /// 作者邮箱
    pub author_email: String,
    /// 作者时间戳
    pub author_date: i64,
    /// 提交者姓名
    pub committer_name: String,
    /// 提交者邮箱
    pub committer_email: String,
    /// 提交者时间戳
    pub committer_date: i64,
    /// 父提交数量
    pub parent_count: usize,
}

/// 同步操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    /// 操作是否成功
    pub success: bool,
    /// 操作消息
    pub message: String,
    /// 是否有冲突
    pub has_conflicts: bool,
    /// 冲突文件列表
    pub conflict_files: Vec<String>,
    /// 更新后的ahead/behind状态
    pub ahead: u32,
    pub behind: u32,
}

/// Pull策略
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PullStrategy {
    /// 合并策略
    Merge,
    /// 变基策略
    Rebase,
}

/// 远程分支信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteBranchInfo {
    /// 远程名称
    pub remote_name: String,
    /// 分支名称
    pub branch_name: String,
    /// 本地领先提交数
    pub ahead: u32,
    /// 本地落后提交数
    pub behind: u32,
    /// 最后同步时间
    pub last_sync: Option<i64>,
}

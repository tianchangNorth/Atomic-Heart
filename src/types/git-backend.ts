// Git 后端 API 类型定义

/// Git 克隆选项
export interface CloneOptions {
  /// 仓库 URL
  url: string;
  /// 目标目录
  directory: string;
  /// 指定分支（可选）
  branch?: string;
  /// 克隆深度（可选）
  depth?: number;
  /// 是否递归克隆子模块
  recursive: boolean;
  /// 认证信息
  auth?: AuthConfig;
}

/// 认证配置
export interface AuthConfig {
  /// 认证类型
  auth_type: AuthType;
  /// 用户名（HTTP 认证）
  username?: string;
  /// 密码（HTTP 认证）
  password?: string;
  /// Personal Access Token
  token?: string;
  /// SSH 密钥路径
  ssh_key_path?: string;
  /// SSH 密钥密码
  ssh_key_passphrase?: string;
}

/// 认证类型
export type AuthType = 'none' | 'password' | 'token' | 'ssh';

/// 克隆进度信息
export interface CloneProgress {
  /// 操作 ID
  id: string;
  /// 当前阶段
  stage: CloneStage;
  /// 进度百分比 (0-100)
  progress: number;
  /// 当前消息
  message: string;
  /// 网络进度（可选）
  network_progress?: NetworkProgress;
  /// 检出进度（可选）
  checkout_progress?: CheckoutProgress;
}

/// 克隆阶段
export type CloneStage =
  | 'Initializing'
  | 'Connecting'
  | 'Downloading'
  | 'Unpacking'
  | 'CheckingOut'
  | 'Completed'
  | 'Error';

/// 网络进度
export interface NetworkProgress {
  /// 已接收字节数
  received_bytes: number;
  /// 已接收对象数
  received_objects: number;
  /// 总对象数
  total_objects: number;
  /// 已索引对象数
  indexed_objects: number;
}

/// 检出进度
export interface CheckoutProgress {
  /// 已完成步骤
  completed_steps: number;
  /// 总步骤数
  total_steps: number;
  /// 当前路径
  current_path?: string;
}

/// 克隆结果
export interface CloneResult {
  /// 是否成功
  success: boolean;
  /// 仓库路径
  repository_path?: string;
  /// 错误信息
  error?: string;
  /// 克隆的分支
  branch?: string;
  /// 最后一次提交的 SHA
  last_commit_sha?: string;
  /// 克隆统计信息
  stats?: CloneStats;
}

/// 克隆统计信息
export interface CloneStats {
  /// 总耗时（毫秒）
  duration_ms: number;
  /// 下载的字节数
  downloaded_bytes: number;
  /// 对象数量
  object_count: number;
  /// 文件数量
  file_count: number;
}

/// Git 错误信息
export interface GitError {
  type: string;
  message: string;
}

/// 仓库信息
export interface RepositoryInfo {
  /// 仓库 ID
  id: string;
  /// 仓库名称
  name: string;
  /// 本地路径
  path: string;
  /// 远程 URL
  remote_url?: string;
  /// 当前分支
  current_branch: string;
  /// 创建时间
  created_at: string;
  /// 最后更新时间
  updated_at: string;
}

/// 目录验证结果
export interface DirectoryValidation {
  /// 是否有效
  is_valid: boolean;
  /// 是否为空
  is_empty: boolean;
  /// 是否可写
  is_writable: boolean;
  /// 验证消息
  message: string;
}

/// 克隆进度回调函数类型
export type CloneProgressCallback = (progress: CloneProgress) => void;

/// 克隆选项构建器
export class CloneOptionsBuilder {
  private options: CloneOptions;

  constructor(url: string, directory: string) {
    this.options = {
      url,
      directory,
      recursive: false,
    };
  }

  branch(branch: string): CloneOptionsBuilder {
    this.options.branch = branch;
    return this;
  }

  depth(depth: number): CloneOptionsBuilder {
    this.options.depth = depth;
    return this;
  }

  recursive(recursive: boolean = true): CloneOptionsBuilder {
    this.options.recursive = recursive;
    return this;
  }

  auth(auth: AuthConfig): CloneOptionsBuilder {
    this.options.auth = auth;
    return this;
  }

  build(): CloneOptions {
    return { ...this.options };
  }
}

/// 认证配置构建器
export class AuthConfigBuilder {
  private config: AuthConfig;

  constructor(authType: AuthType) {
    this.config = {
      auth_type: authType,
    };
  }

  username(username: string): AuthConfigBuilder {
    this.config.username = username;
    return this;
  }

  password(password: string): AuthConfigBuilder {
    this.config.password = password;
    return this;
  }

  token(token: string): AuthConfigBuilder {
    this.config.token = token;
    return this;
  }

  sshKey(keyPath: string, passphrase?: string): AuthConfigBuilder {
    this.config.ssh_key_path = keyPath;
    this.config.ssh_key_passphrase = passphrase;
    return this;
  }

  build(): AuthConfig {
    // Convert auth_type to PascalCase for backend compatibility
    const pascalCaseType = this.config.auth_type === 'none' ? 'None' :
      this.config.auth_type === 'password' ? 'Password' :
        this.config.auth_type === 'token' ? 'Token' : 'Ssh';

    return {
      ...this.config,
      auth_type: pascalCaseType as AuthType
    };
  }
}

/// 克隆阶段的中文描述
export const CloneStageDescriptions: Record<CloneStage, string> = {
  Initializing: '初始化',
  Connecting: '连接远程仓库',
  Downloading: '下载对象',
  Unpacking: '解压对象',
  CheckingOut: '检出文件',
  Completed: '完成',
  Error: '错误',
};

/// 认证类型的中文描述
export const AuthTypeDescriptions: Record<AuthType, string> = {
  none: '无认证',
  password: '用户名/密码',
  token: '访问令牌',
  ssh: 'SSH 密钥',
};

/// 格式化文件大小
export function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B';

  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

/// 格式化持续时间
export function formatDuration(ms: number): string {
  if (ms < 1000) return `${ms}ms`;
  if (ms < 60000) return `${(ms / 1000).toFixed(1)}s`;
  if (ms < 3600000) return `${(ms / 60000).toFixed(1)}m`;
  return `${(ms / 3600000).toFixed(1)}h`;
}

/// 验证 Git URL 格式
export function isValidGitUrl(url: string): boolean {
  if (!url || url.trim().length === 0) return false;

  const patterns = [
    /^https?:\/\/.+\.git$/,
    /^https?:\/\/.+$/,
    /^git@.+:.+\.git$/,
    /^ssh:\/\/.+\.git$/,
  ];

  return patterns.some(pattern => pattern.test(url.trim()));
}

/// 从 Git URL 提取仓库名称
export function extractRepoNameFromUrl(url: string): string {
  try {
    // 移除 .git 后缀
    let cleanUrl = url.replace(/\.git$/, '');

    // 提取最后一部分作为仓库名
    const parts = cleanUrl.split('/');
    const repoName = parts[parts.length - 1];

    // 处理 SSH URL 格式
    if (repoName.includes(':')) {
      return repoName.split(':').pop() || 'repository';
    }

    return repoName || 'repository';
  } catch {
    return 'repository';
  }
}

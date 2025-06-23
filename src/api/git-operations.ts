import { invoke } from '@tauri-apps/api/core';

// 类型定义
export interface RepositoryStatus {
  current_branch: string;
  files: FileStatus[];
  ahead: number;
  behind: number;
  is_clean: boolean;
}

export interface FileStatus {
  path: string;
  status: string;
  staged: boolean;
  additions: number;
  deletions: number;
}

export interface CommitHistoryItem {
  sha: string;
  message: string;
  author_name: string;
  author_email: string;
  author_date: number;
  committer_name: string;
  committer_email: string;
  committer_date: number;
  parent_count: number;
}

export interface SyncResult {
  success: boolean;
  message: string;
  has_conflicts: boolean;
  conflict_files: string[];
  ahead: number;
  behind: number;
}

export interface SwitchResult {
  success: boolean;
  message: string;
  has_uncommitted_changes: boolean;
  uncommitted_files: string[];
}

export type PullStrategy = 'merge' | 'rebase';

export interface RemoteBranchInfo {
  remote_name: string;
  branch_name: string;
  ahead: number;
  behind: number;
  last_sync: number | null;
}

export interface CommitOptions {
  message: string;
  description?: string;
  author_name?: string;
  author_email?: string;
  amend?: boolean;
  signoff?: boolean;
}

// 新增：双协议认证系统类型定义
export interface TokenConfig {
  domain: string;
  token: string;
  username?: string;
  created_at: number;
  last_used?: number;
}

export type ProtocolType = 'https' | 'ssh' | 'unknown';

// Git 操作 API
export class GitOperationsApi {
  /**
   * 获取仓库状态
   */
  async getRepositoryStatus(repoPath: string): Promise<RepositoryStatus> {
    try {
      const status = await invoke<RepositoryStatus>('get_repository_status', {
        repoPath
      });
      return status;
    } catch (error) {
      console.error('获取仓库状态失败:', error);
      throw new Error(`获取仓库状态失败: ${error}`);
    }
  }

  /**
   * 暂存文件
   */
  async stageFiles(repoPath: string, filePaths: string[]): Promise<void> {
    try {
      await invoke('stage_files', {
        repoPath,
        filePaths
      });
    } catch (error) {
      console.error('暂存文件失败:', error);
      throw new Error(`暂存文件失败: ${error}`);
    }
  }

  /**
   * 取消暂存文件
   */
  async unstageFiles(repoPath: string, filePaths: string[]): Promise<void> {
    try {
      await invoke('unstage_files', {
        repoPath,
        filePaths
      });
    } catch (error) {
      console.error('取消暂存文件失败:', error);
      throw new Error(`取消暂存文件失败: ${error}`);
    }
  }

  /**
   * 创建提交
   */
  async createCommit(repoPath: string, options: CommitOptions): Promise<string> {
    try {
      const commitSha = await invoke<string>('create_commit', {
        repoPath,
        message: options.message,
        description: options.description,
        authorName: options.author_name,
        authorEmail: options.author_email,
        amend: options.amend,
        signoff: options.signoff
      });
      return commitSha;
    } catch (error) {
      console.error('创建提交失败:', error);
      throw new Error(`创建提交失败: ${error}`);
    }
  }

  /**
   * 获取提交历史
   */
  async getCommitHistory(
    repoPath: string,
    limit?: number,
    skip?: number
  ): Promise<CommitHistoryItem[]> {
    try {
      const commits = await invoke<CommitHistoryItem[]>('get_commit_history', {
        repoPath,
        limit,
        skip
      });
      return commits;
    } catch (error) {
      console.error('获取提交历史失败:', error);
      throw new Error(`获取提交历史失败: ${error}`);
    }
  }

  /**
   * 获取文件差异
   */
  async getFileDiff(
    repoPath: string,
    filePath: string,
    staged?: boolean
  ): Promise<string> {
    try {
      const diff = await invoke<string>('get_file_diff', {
        repoPath,
        filePath,
        staged
      });
      return diff;
    } catch (error) {
      console.error('获取文件差异失败:', error);
      throw new Error(`获取文件差异失败: ${error}`);
    }
  }

  /**
   * 暂存单个文件
   */
  async stageFile(repoPath: string, filePath: string): Promise<void> {
    return this.stageFiles(repoPath, [filePath]);
  }

  /**
   * 取消暂存单个文件
   */
  async unstageFile(repoPath: string, filePath: string): Promise<void> {
    return this.unstageFiles(repoPath, [filePath]);
  }

  /**
   * 暂存所有文件
   */
  async stageAllFiles(repoPath: string, files: FileStatus[]): Promise<void> {
    const unstagedFiles = files.filter(f => !f.staged).map(f => f.path);
    if (unstagedFiles.length > 0) {
      return this.stageFiles(repoPath, unstagedFiles);
    }
  }

  /**
   * 取消暂存所有文件
   */
  async unstageAllFiles(repoPath: string, files: FileStatus[]): Promise<void> {
    const stagedFiles = files.filter(f => f.staged).map(f => f.path);
    if (stagedFiles.length > 0) {
      return this.unstageFiles(repoPath, stagedFiles);
    }
  }

  /**
   * 获取远程变更（fetch操作）
   */
  async fetchRemote(repoPath: string, remoteName?: string): Promise<SyncResult> {
    try {
      const result = await invoke<SyncResult>('fetch_remote', {
        repoPath: repoPath,
        remoteName: remoteName
      });
      return result;
    } catch (error) {
      console.error('获取远程变更失败:', error);
      throw new Error(`获取远程变更失败: ${error}`);
    }
  }

  /**
   * 拉取远程变更（pull操作）
   */
  async pullRemote(repoPath: string, strategy: PullStrategy): Promise<SyncResult> {
    try {
      const result = await invoke<SyncResult>('pull_remote', {
        repoPath: repoPath,
        strategy
      });
      return result;
    } catch (error) {
      console.error('拉取远程变更失败:', error);
      throw new Error(`拉取远程变更失败: ${error}`);
    }
  }

  /**
   * 推送本地变更（push操作）
   */
  async pushRemote(repoPath: string, remoteName?: string, force?: boolean): Promise<SyncResult> {
    try {
      const result = await invoke<SyncResult>('push_remote', {
        repoPath: repoPath,
        remote_name: remoteName,
        force
      });
      return result;
    } catch (error) {
      console.error('推送本地变更失败:', error);
      throw new Error(`推送本地变更失败: ${error}`);
    }
  }

  /**
   * 获取远程仓库信息
   */
  async getRemoteInfo(repoPath: string): Promise<RemoteBranchInfo> {
    try {
      const result = await invoke<RemoteBranchInfo>('get_remote_info', {
        repoPath: repoPath
      });
      return result;
    } catch (error) {
      console.error('获取远程仓库信息失败:', error);
      throw new Error(`获取远程仓库信息失败: ${error}`);
    }
  }

  // ==================== 双协议认证系统 ====================

  /**
   * 检测仓库协议类型
   */
  async detectRepositoryProtocol(repoPath: string): Promise<ProtocolType> {
    try {
      const protocol = await invoke<string>('detect_repository_protocol', {
        repoPath
      });
      return protocol as ProtocolType;
    } catch (error) {
      console.error('检测仓库协议失败:', error);
      return 'unknown';
    }
  }

  /**
   * 从URL提取域名
   */
  async extractDomainFromUrl(url: string): Promise<string> {
    try {
      const domain = await invoke<string>('extract_domain_from_url', {
        url
      });
      return domain;
    } catch (error) {
      console.error('提取域名失败:', error);
      throw new Error(`提取域名失败: ${error}`);
    }
  }

  /**
   * 存储Personal Access Token
   */
  async storeAccessToken(domain: string, token: string, username?: string): Promise<void> {
    try {
      await invoke('store_access_token', {
        domain,
        token,
        username
      });
    } catch (error) {
      console.error('存储Token失败:', error);
      throw new Error(`存储Token失败: ${error}`);
    }
  }

  /**
   * 获取Personal Access Token
   */
  async getAccessToken(domain: string): Promise<TokenConfig | null> {
    try {
      const token = await invoke<TokenConfig | null>('get_access_token', {
        domain
      });
      return token;
    } catch (error) {
      console.error('获取Token失败:', error);
      throw new Error(`获取Token失败: ${error}`);
    }
  }

  /**
   * 删除Personal Access Token
   */
  async deleteAccessToken(domain: string): Promise<void> {
    try {
      await invoke('delete_access_token', {
        domain
      });
    } catch (error) {
      console.error('删除Token失败:', error);
      throw new Error(`删除Token失败: ${error}`);
    }
  }

  /**
   * 获取所有存储的Token
   */
  async getAllTokens(): Promise<TokenConfig[]> {
    try {
      const tokens = await invoke<TokenConfig[]>('get_all_tokens');
      return tokens;
    } catch (error) {
      console.error('获取所有Token失败:', error);
      throw new Error(`获取所有Token失败: ${error}`);
    }
  }

  /**
   * 更新Token最后使用时间
   */
  async updateTokenLastUsed(domain: string): Promise<void> {
    try {
      await invoke('update_token_last_used', {
        domain
      });
    } catch (error) {
      console.error('更新Token使用时间失败:', error);
      // 这个错误不需要抛出，因为不影响主要功能
    }
  }

  // ==================== 远程名称检测 ====================

  /**
   * 检测仓库的远程配置
   */
  async detectRepositoryRemotes(repoPath: string): Promise<string[]> {
    try {
      const remotes = await invoke<string[]>('detect_repository_remotes', {
        repoPath
      });
      return remotes;
    } catch (error) {
      console.error('检测远程配置失败:', error);
      throw new Error(`检测远程配置失败: ${error}`);
    }
  }

  /**
   * 获取默认远程名称
   */
  async getDefaultRemoteName(repoPath: string): Promise<string> {
    try {
      const remoteName = await invoke<string>('get_default_remote_name_command', {
        repoPath
      });
      return remoteName;
    } catch (error) {
      console.error('获取默认远程名称失败:', error);
      throw new Error(`获取默认远程名称失败: ${error}`);
    }
  }

  /**
   * 获取远程URL
   */
  async getRemoteUrl(repoPath: string): Promise<string | null> {
    try {
      const url = await invoke<string | null>('get_remote_url', {
        path: repoPath
      });
      return url;
    } catch (error) {
      console.error('获取远程URL失败:', error);
      throw new Error(`获取远程URL失败: ${error}`);
    }
  }

  // ==================== 分支操作 ====================

  /**
   * 创建新分支
   */
  async createBranch(
    repoPath: string,
    branchName: string,
    fromCommit?: string,
    checkout?: boolean
  ): Promise<SwitchResult> {
    try {
      const result = await invoke<SwitchResult>('create_branch', {
        repoPath,
        branchName,
        fromCommit,
        checkout
      });
      return result;
    } catch (error) {
      console.error('创建分支失败:', error);
      throw new Error(`创建分支失败: ${error}`);
    }
  }

  /**
   * 切换分支
   */
  async switchBranch(repoPath: string, branchName: string): Promise<SwitchResult> {
    try {
      const result = await invoke<SwitchResult>('switch_branch', {
        repoPath,
        branchName
      });
      return result;
    } catch (error) {
      console.error('切换分支失败:', error);
      throw new Error(`切换分支失败: ${error}`);
    }
  }

  /**
   * 删除分支
   */
  async deleteBranch(
    repoPath: string,
    branchName: string,
    force?: boolean
  ): Promise<SwitchResult> {
    try {
      const result = await invoke<SwitchResult>('delete_branch', {
        repoPath,
        branchName,
        force
      });
      return result;
    } catch (error) {
      console.error('删除分支失败:', error);
      throw new Error(`删除分支失败: ${error}`);
    }
  }

  /**
   * 检出远程分支
   */
  async checkoutRemoteBranch(
    repoPath: string,
    remoteBranchName: string,
    localBranchName?: string
  ): Promise<SwitchResult> {
    try {
      const result = await invoke<SwitchResult>('checkout_remote_branch', {
        repoPath,
        remoteBranchName,
        localBranchName
      });
      return result;
    } catch (error) {
      console.error('检出远程分支失败:', error);
      throw new Error(`检出远程分支失败: ${error}`);
    }
  }

  // ==================== 智能协议选择的Git操作 ====================

  /**
   * 智能fetch操作（自动选择协议）
   */
  async smartFetchRemote(repoPath: string, remoteName?: string, sshKeyPath?: string): Promise<SyncResult> {
    const protocol = await this.detectRepositoryProtocol(repoPath);

    if (protocol === 'ssh') {
      return this.fetchRemoteWithSystemGit(repoPath, remoteName, sshKeyPath);
    } else if (protocol === 'https') {
      // 使用支持Token认证的智能fetch
      return this.smartFetchRemoteWithToken(repoPath, remoteName);
    } else {
      // 默认使用git2
      return this.fetchRemote(repoPath, remoteName);
    }
  }

  /**
   * 智能fetch操作（支持Token认证）
   */
  async smartFetchRemoteWithToken(repoPath: string, remoteName?: string): Promise<SyncResult> {
    try {
      const result = await invoke<SyncResult>('smart_fetch_remote', {
        repoPath,
        remoteName
      });
      return result;
    } catch (error) {
      console.error('智能fetch操作失败:', error);
      throw new Error(`智能fetch操作失败: ${error}`);
    }
  }

  /**
   * 智能push操作（自动选择协议）
   */
  async smartPushRemote(repoPath: string, remoteName?: string, force?: boolean, sshKeyPath?: string): Promise<SyncResult> {
    const protocol = await this.detectRepositoryProtocol(repoPath);

    if (protocol === 'ssh') {
      return this.pushRemoteWithSystemGit(repoPath, remoteName, force, sshKeyPath);
    } else if (protocol === 'https') {
      // 使用支持Token认证的智能push
      return this.smartPushRemoteWithToken(repoPath, remoteName, force);
    } else {
      // 默认使用git2
      return this.pushRemote(repoPath, remoteName, force);
    }
  }

  /**
   * 智能push操作（支持Token认证）
   */
  async smartPushRemoteWithToken(repoPath: string, remoteName?: string, force?: boolean): Promise<SyncResult> {
    try {
      const result = await invoke<SyncResult>('smart_push_remote', {
        repoPath,
        remoteName,
        force
      });
      return result;
    } catch (error) {
      console.error('智能push操作失败:', error);
      throw new Error(`智能push操作失败: ${error}`);
    }
  }

  /**
   * 智能pull操作（自动选择协议）
   */
  async smartPullRemote(repoPath: string, strategy: PullStrategy, sshKeyPath?: string): Promise<SyncResult> {
    const protocol = await this.detectRepositoryProtocol(repoPath);

    if (protocol === 'ssh') {
      return this.pullRemoteWithSystemGit(repoPath, strategy, sshKeyPath);
    } else if (protocol === 'https') {
      // 检查并更新token认证
      await this.ensureHttpsAuthentication(repoPath);
      return this.pullRemote(repoPath, strategy);
    } else {
      // 默认使用git2
      return this.pullRemote(repoPath, strategy);
    }
  }

  // ==================== 系统Git命令操作 ====================

  /**
   * 使用系统Git执行fetch操作
   */
  async fetchRemoteWithSystemGit(repoPath: string, remoteName?: string, sshKeyPath?: string): Promise<SyncResult> {
    try {
      const result = await invoke<SyncResult>('fetch_remote_with_system_git', {
        repoPath,
        remoteName,
        sshKeyPath
      });
      return result;
    } catch (error) {
      console.error('系统Git fetch失败:', error);
      throw new Error(`系统Git fetch失败: ${error}`);
    }
  }

  /**
   * 使用系统Git执行push操作
   */
  async pushRemoteWithSystemGit(repoPath: string, remoteName?: string, force?: boolean, sshKeyPath?: string): Promise<SyncResult> {
    try {
      const result = await invoke<SyncResult>('push_remote_with_system_git', {
        repoPath,
        remoteName,
        force,
        sshKeyPath
      });
      return result;
    } catch (error) {
      console.error('系统Git push失败:', error);
      throw new Error(`系统Git push失败: ${error}`);
    }
  }

  /**
   * 使用系统Git执行pull操作
   */
  async pullRemoteWithSystemGit(repoPath: string, strategy: PullStrategy, sshKeyPath?: string): Promise<SyncResult> {
    try {
      const result = await invoke<SyncResult>('pull_remote_with_system_git', {
        repoPath,
        strategy,
        sshKeyPath
      });
      return result;
    } catch (error) {
      console.error('系统Git pull失败:', error);
      throw new Error(`系统Git pull失败: ${error}`);
    }
  }

  // ==================== 私有辅助方法 ====================

  /**
   * 确保HTTPS协议的认证配置
   */
  private async ensureHttpsAuthentication(repoPath: string): Promise<void> {
    try {
      // 获取远程URL
      const remoteInfo = await this.getRemoteInfo(repoPath);
      if (!remoteInfo.remote_name) return;

      // 这里可以添加获取远程URL的逻辑
      // 然后检查是否有对应的token
      // 如果没有，可以触发token配置流程
    } catch (error) {
      console.warn('检查HTTPS认证失败:', error);
    }
  }
}

// 导出单例实例
export const gitOperationsApi = new GitOperationsApi();

// 默认导出
export default gitOperationsApi;

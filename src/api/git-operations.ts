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
}

// 导出单例实例
export const gitOperationsApi = new GitOperationsApi();

// 默认导出
export default gitOperationsApi;

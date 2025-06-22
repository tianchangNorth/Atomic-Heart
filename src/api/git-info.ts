/**
 * Git 仓库信息获取 API
 */

import { invoke } from '@tauri-apps/api/core';

/**
 * 仓库信息接口
 */
export interface RepositoryInfo {
  name: string;
  remote_url?: string;
  current_branch?: string;
  is_valid: boolean;
}

/**
 * Git 仓库信息 API 类
 */
export class GitInfoApi {
  /**
   * 验证指定路径是否为有效的 Git 仓库
   */
  static async isGitRepository(path: string): Promise<boolean> {
    try {
      const result = await invoke<boolean>('is_git_repository', { path });
      return result;
    } catch (error) {
      console.error('验证 Git 仓库失败:', error);
      return false;
    }
  }

  /**
   * 获取仓库基本信息
   */
  static async getRepositoryInfo(path: string): Promise<RepositoryInfo | null> {
    try {
      const result = await invoke<RepositoryInfo>('get_repository_info', { path });
      return result;
    } catch (error) {
      console.error('获取仓库信息失败:', error);
      return null;
    }
  }

  /**
   * 获取仓库当前分支名称
   */
  static async getCurrentBranch(path: string): Promise<string | null> {
    try {
      const result = await invoke<string | null>('get_current_branch', { path });
      return result;
    } catch (error) {
      console.error('获取当前分支失败:', error);
      return null;
    }
  }

  /**
   * 获取仓库的远程 URL
   */
  static async getRemoteUrl(path: string): Promise<string | null> {
    try {
      const result = await invoke<string | null>('get_remote_url', { path });
      return result;
    } catch (error) {
      console.error('获取远程 URL 失败:', error);
      return null;
    }
  }

  /**
   * 在文件管理器中打开指定文件夹
   */
  static async openFolder(path: string): Promise<boolean> {
    try {
      await invoke('open_folder', { path });
      return true;
    } catch (error) {
      console.error('打开文件夹失败:', error);
      return false;
    }
  }

  /**
   * 批量验证多个仓库路径
   */
  static async validateRepositories(paths: string[]): Promise<Map<string, boolean>> {
    const results = new Map<string, boolean>();
    
    // 并行验证所有路径
    const promises = paths.map(async (path) => {
      const isValid = await this.isGitRepository(path);
      return { path, isValid };
    });

    const validationResults = await Promise.all(promises);
    
    validationResults.forEach(({ path, isValid }) => {
      results.set(path, isValid);
    });

    return results;
  }

  /**
   * 批量获取多个仓库的信息
   */
  static async getRepositoriesInfo(paths: string[]): Promise<Map<string, RepositoryInfo | null>> {
    const results = new Map<string, RepositoryInfo | null>();
    
    // 并行获取所有仓库信息
    const promises = paths.map(async (path) => {
      const info = await this.getRepositoryInfo(path);
      return { path, info };
    });

    const infoResults = await Promise.all(promises);
    
    infoResults.forEach(({ path, info }) => {
      results.set(path, info);
    });

    return results;
  }

  /**
   * 刷新仓库信息（重新获取最新状态）
   */
  static async refreshRepositoryInfo(path: string): Promise<RepositoryInfo | null> {
    // 这里可以添加缓存清理逻辑
    return await this.getRepositoryInfo(path);
  }

  /**
   * 检查仓库是否有远程配置
   */
  static async hasRemoteRepository(path: string): Promise<boolean> {
    try {
      const remoteUrl = await this.getRemoteUrl(path);
      return remoteUrl !== null && remoteUrl.trim().length > 0;
    } catch (error) {
      console.error('检查远程仓库失败:', error);
      return false;
    }
  }

  /**
   * 从仓库路径提取仓库名称（作为备用方案）
   */
  static extractRepositoryNameFromPath(path: string): string {
    try {
      const normalizedPath = path.replace(/\\/g, '/');
      const parts = normalizedPath.split('/');
      const name = parts[parts.length - 1];
      return name || 'Unknown Repository';
    } catch (error) {
      console.error('从路径提取仓库名称失败:', error);
      return 'Unknown Repository';
    }
  }

  /**
   * 验证并获取仓库完整信息（组合方法）
   */
  static async validateAndGetInfo(path: string): Promise<{
    isValid: boolean;
    info: RepositoryInfo | null;
  }> {
    try {
      // 首先验证是否为 Git 仓库
      const isValid = await this.isGitRepository(path);
      
      if (!isValid) {
        return {
          isValid: false,
          info: null
        };
      }

      // 获取详细信息
      const info = await this.getRepositoryInfo(path);
      
      return {
        isValid: true,
        info
      };
    } catch (error) {
      console.error('验证并获取仓库信息失败:', error);
      return {
        isValid: false,
        info: null
      };
    }
  }
}

// 导出默认实例
export const gitInfoApi = GitInfoApi;

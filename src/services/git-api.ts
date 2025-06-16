import { invoke } from '@tauri-apps/api/core';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import type {
  CloneOptions,
  CloneResult,
  CloneProgress,
  AuthConfig,
  CloneProgressCallback,
  DirectoryValidation,
} from '@/types/git-backend';

/**
 * Git API 服务类
 * 封装所有与 Rust 后端的 Git 操作交互
 */
export class GitApiService {
  private static instance: GitApiService;
  private progressListeners: Map<string, UnlistenFn> = new Map();

  private constructor() { }

  public static getInstance(): GitApiService {
    if (!GitApiService.instance) {
      GitApiService.instance = new GitApiService();
    }
    return GitApiService.instance;
  }

  /**
   * 克隆 Git 仓库
   */
  async cloneRepository(options: CloneOptions): Promise<CloneResult> {
    try {
      const result = await invoke<CloneResult>('clone_repository', { options });
      return result;
    } catch (error) {
      throw new Error(`克隆仓库失败: ${error}`);
    }
  }

  /**
   * 监听克隆进度
   */
  async listenCloneProgress(callback: CloneProgressCallback): Promise<UnlistenFn> {
    const unlisten = await listen<CloneProgress>('clone-progress', (event) => {
      callback(event.payload);
    });

    return unlisten;
  }

  /**
   * 验证仓库 URL
   */
  async validateRepositoryUrl(url: string): Promise<boolean> {
    try {
      return await invoke<boolean>('validate_repository_url', { url });
    } catch (error) {
      console.error('验证仓库 URL 失败:', error);
      return false;
    }
  }

  /**
   * 检测认证类型
   */
  async detectAuthType(url: string): Promise<string> {
    try {
      return await invoke<string>('detect_auth_type', { url });
    } catch (error) {
      console.error('检测认证类型失败:', error);
      return 'none';
    }
  }

  /**
   * 获取默认 SSH 密钥
   */
  async getDefaultSshKeys(): Promise<string[]> {
    try {
      return await invoke<string[]>('get_default_ssh_keys');
    } catch (error) {
      console.error('获取默认 SSH 密钥失败:', error);
      return [];
    }
  }

  /**
   * 验证 SSH 密钥
   */
  async validateSshKey(privateKeyPath: string, passphrase?: string): Promise<boolean> {
    try {
      return await invoke<boolean>('validate_ssh_key', {
        privateKeyPath,
        passphrase,
      });
    } catch (error) {
      console.error('验证 SSH 密钥失败:', error);
      return false;
    }
  }

  /**
   * 存储认证凭据
   */
  async storeCredentials(url: string, auth: AuthConfig): Promise<void> {
    try {
      await invoke('store_credentials', { url, auth });
    } catch (error) {
      throw new Error(`存储凭据失败: ${error}`);
    }
  }

  /**
   * 加载认证凭据
   */
  async loadCredentials(url: string): Promise<AuthConfig | null> {
    try {
      return await invoke<AuthConfig | null>('load_credentials', { url });
    } catch (error) {
      console.error('加载凭据失败:', error);
      return null;
    }
  }

  /**
   * 删除认证凭据
   */
  async deleteCredentials(url: string): Promise<void> {
    try {
      await invoke('delete_credentials', { url });
    } catch (error) {
      throw new Error(`删除凭据失败: ${error}`);
    }
  }

  /**
   * 从 URL 提取用户名
   */
  async extractUsernameFromUrl(url: string): Promise<string | null> {
    try {
      return await invoke<string | null>('extract_username_from_url', { url });
    } catch (error) {
      console.error('提取用户名失败:', error);
      return null;
    }
  }

  /**
   * 取消克隆操作
   */
  async cancelCloneOperation(operationId: string): Promise<void> {
    try {
      await invoke('cancel_clone_operation', { operationId });
    } catch (error) {
      throw new Error(`取消克隆操作失败: ${error}`);
    }
  }

  /**
   * 获取克隆操作状态
   */
  async getCloneOperationStatus(operationId: string): Promise<boolean> {
    try {
      return await invoke<boolean>('get_clone_operation_status', { operationId });
    } catch (error) {
      console.error('获取克隆操作状态失败:', error);
      return false;
    }
  }

  /**
   * 清理完成的克隆操作
   */
  async cleanupCloneOperation(operationId: string): Promise<void> {
    try {
      await invoke('cleanup_clone_operation', { operationId });
    } catch (error) {
      console.error('清理克隆操作失败:', error);
    }
  }

  /**
   * 选择目录
   */
  async selectDirectory(): Promise<string | null> {
    try {
      return await invoke<string | null>('select_directory');
    } catch (error) {
      console.error('选择目录失败:', error);
      return null;
    }
  }

  /**
   * 选择 SSH 密钥文件
   */
  async selectSshKeyFile(): Promise<string | null> {
    try {
      return await invoke<string | null>('select_ssh_key_file');
    } catch (error) {
      console.error('选择 SSH 密钥文件失败:', error);
      return null;
    }
  }

  /**
   * 验证克隆目录
   */
  async validateCloneDirectory(directoryPath: string): Promise<DirectoryValidation | null> {
    try {
      return await invoke<DirectoryValidation>('validate_clone_directory', { directoryPath });
    } catch (error) {
      console.error('验证克隆目录失败:', error);
      return null;
    }
  }

  /**
   * 清理所有进度监听器
   */
  async cleanup(): Promise<void> {
    for (const [, unlisten] of this.progressListeners) {
      unlisten();
    }
    this.progressListeners.clear();
  }
}

/**
 * Git API 服务实例
 */
export const gitApi = GitApiService.getInstance();

/**
 * Git 克隆管理器
 * 提供高级的克隆操作接口
 */
export class GitCloneManager {
  private gitApi: GitApiService;
  private activeOperations: Map<string, boolean> = new Map();

  constructor() {
    this.gitApi = GitApiService.getInstance();
  }

  /**
   * 执行克隆操作并监听进度
   */
  async cloneWithProgress(
    options: CloneOptions,
    onProgress?: CloneProgressCallback,
    onComplete?: (result: CloneResult) => void,
    onError?: (error: Error) => void
  ): Promise<string> {
    const operationId = `clone-${Date.now()}-${Math.random().toString(36).substring(2, 11)}`;
    this.activeOperations.set(operationId, false);

    try {
      // 设置进度监听
      let progressUnlisten: UnlistenFn | null = null;
      if (onProgress) {
        progressUnlisten = await this.gitApi.listenCloneProgress((progress) => {
          if (progress.id === operationId) {
            onProgress(progress);

            // 检查是否完成或出错
            if (progress.stage === 'Completed' || progress.stage === 'Error') {
              if (progressUnlisten) {
                progressUnlisten();
              }
              this.activeOperations.delete(operationId);
            }
          }
        });
      }

      // 执行克隆
      const result = await this.gitApi.cloneRepository({
        ...options,
        // 可以在这里添加操作 ID 到选项中，如果后端支持的话
      });

      if (onComplete) {
        onComplete(result);
      }

      return operationId;
    } catch (error) {
      this.activeOperations.delete(operationId);
      const gitError = error instanceof Error ? error : new Error(String(error));
      if (onError) {
        onError(gitError);
      }
      throw gitError;
    }
  }

  /**
   * 取消克隆操作
   */
  async cancelClone(operationId: string): Promise<void> {
    if (this.activeOperations.has(operationId)) {
      await this.gitApi.cancelCloneOperation(operationId);
      this.activeOperations.set(operationId, true); // 标记为已取消
    }
  }

  /**
   * 检查操作是否被取消
   */
  isOperationCancelled(operationId: string): boolean {
    return this.activeOperations.get(operationId) === true;
  }

  /**
   * 清理操作
   */
  async cleanup(operationId: string): Promise<void> {
    await this.gitApi.cleanupCloneOperation(operationId);
    this.activeOperations.delete(operationId);
  }

  /**
   * 获取活动操作列表
   */
  getActiveOperations(): string[] {
    return Array.from(this.activeOperations.keys());
  }
}

/**
 * Git 克隆管理器实例
 */
export const gitCloneManager = new GitCloneManager();

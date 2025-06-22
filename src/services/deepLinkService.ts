import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import router from '@/router/index';
import { useToast } from '@/components/ui/toast';

export interface DeepLinkData {
  url: string;
  path?: string;
  params?: Record<string, string>;
  source?: string;
}

export interface AuthCallbackData {
  code: string;
  state: string;
  access_token?: string;
  token_type?: string;
  scope?: string;
}

class DeepLinkService {
  private initialized = false;
  private toast = useToast();

  /**
   * 初始化深度链接服务
   */
  async initialize() {
    if (this.initialized) return;

    try {
      // 监听深度链接事件（来自单实例插件或直接启动）
      await listen<DeepLinkData>('deep-link-received', (event) => {
        console.log('收到深度链接事件:', event.payload);
        this.handleDeepLink(event.payload);
      });

      // 注册协议处理器
      await invoke('register_protocol_handler');

      this.initialized = true;
      console.log('深度链接服务已初始化');

      // 检查启动时是否有深度链接参数
      this.checkStartupDeepLink();
    } catch (error) {
      console.error('初始化深度链接服务失败:', error);
    }
  }

  /**
   * 检查启动时的深度链接参数
   */
  private async checkStartupDeepLink() {
    try {
      // 检查是否有启动参数中的深度链接
      const args = await invoke<string[]>('get_startup_args');
      if (args && args.length > 0) {
        for (const arg of args) {
          if (arg.startsWith('atomic-heart://')) {
            console.log('检测到启动时的深度链接:', arg);
            this.handleDeepLink({ url: arg, source: 'startup' });
            break;
          }
        }
      }
    } catch (error) {
      // 如果命令不存在或失败，忽略错误
      console.debug('无法获取启动参数:', error);
    }
  }

  /**
   * 处理深度链接
   */
  private async handleDeepLink(data: DeepLinkData) {
    console.log('收到深度链接:', data);

    try {
      const url = new URL(data.url);
      const path = url.pathname;
      const params = new URLSearchParams(url.search);

      // 根据路径处理不同的深度链接
      switch (path) {
        case '/auth/callback':
          await this.handleAuthCallback(params);
          break;
        case '/oauth/callback':
          await this.handleOAuthCallback(params);
          break;
        case '/callback':
          await this.handleOAuthCallback(params);
          break;
        case '/repository/open':
          await this.handleRepositoryOpen(params);
          break;
        case '/clone':
          await this.handleCloneRepository(params);
          break;
        default:
          console.warn('未知的深度链接路径:', path);
          this.toast.warning('收到未知的应用链接');
      }
    } catch (error) {
      console.error('处理深度链接失败:', error);
      this.toast.error('处理应用链接时出错');
    }
  }

  /**
   * 处理认证回调
   */
  private async handleAuthCallback(params: URLSearchParams) {
    const code = params.get('code');
    const state = params.get('state');
    const error = params.get('error');
    const errorDescription = params.get('error_description');

    if (error) {
      console.error('认证错误:', error, errorDescription);
      this.toast.error(`认证失败: ${errorDescription || error}`);
      return;
    }

    if (code && state) {
      // 发送认证成功事件
      window.dispatchEvent(new CustomEvent('auth-callback-received', {
        detail: { code, state }
      }));

      this.toast.success('认证成功，正在处理...');

      // 导航到主页或指定页面
      const returnUrl = params.get('return_url') || '/';
      await router.push(returnUrl);
    }
  }

  /**
   * 处理OAuth回调
   */
  private async handleOAuthCallback(params: URLSearchParams) {
    const accessToken = params.get('access_token');
    const tokenType = params.get('token_type');
    const scope = params.get('scope');
    const state = params.get('state');

    if (accessToken) {
      // 发送OAuth成功事件
      window.dispatchEvent(new CustomEvent('oauth-callback-received', {
        detail: {
          access_token: accessToken,
          token_type: tokenType,
          scope,
          state
        }
      }));

      this.toast.success('OAuth认证成功');

      // 导航到主页
      await router.push('/');
    }
  }

  /**
   * 处理仓库打开
   */
  private async handleRepositoryOpen(params: URLSearchParams) {
    const repoId = params.get('id');
    const repoPath = params.get('path');
    const repoUrl = params.get('url');

    if (repoId) {
      // 通过ID打开本地仓库
      await router.push(`/local-repositories/${repoId}`);
      this.toast.success('正在打开仓库...');
    } else if (repoPath) {
      // 通过路径打开仓库
      // 这里可以添加导入仓库的逻辑
      this.toast.info('正在导入仓库...');
    } else if (repoUrl) {
      // 通过URL克隆仓库
      await router.push('/local-repositories');
      // 触发克隆对话框
      window.dispatchEvent(new CustomEvent('trigger-clone-dialog', {
        detail: { url: repoUrl }
      }));
      this.toast.info('正在准备克隆仓库...');
    }
  }

  /**
   * 处理克隆仓库
   */
  private async handleCloneRepository(params: URLSearchParams) {
    const repoUrl = params.get('url');
    const branch = params.get('branch');
    const directory = params.get('directory');

    if (repoUrl) {
      await router.push('/local-repositories');

      // 触发克隆对话框并预填信息
      window.dispatchEvent(new CustomEvent('trigger-clone-dialog', {
        detail: {
          url: repoUrl,
          branch,
          directory
        }
      }));

      this.toast.info('正在准备克隆仓库...');
    }
  }

  /**
   * 生成深度链接URL
   */
  generateDeepLink(path: string, params?: Record<string, string>): string {
    const url = new URL(`atomic-heart://${path}`);

    if (params) {
      Object.entries(params).forEach(([key, value]) => {
        url.searchParams.set(key, value);
      });
    }

    return url.toString();
  }

  /**
   * 生成认证回调链接
   */
  generateAuthCallbackLink(returnUrl?: string): string {
    const params: Record<string, string> = {};
    if (returnUrl) {
      params.return_url = returnUrl;
    }

    return this.generateDeepLink('/auth/callback', params);
  }

  /**
   * 生成OAuth回调链接
   */
  generateOAuthCallbackLink(): string {
    return this.generateDeepLink('/oauth/callback');
  }

  /**
   * 生成仓库打开链接
   */
  generateRepositoryOpenLink(repoId?: string, repoPath?: string, repoUrl?: string): string {
    const params: Record<string, string> = {};

    if (repoId) params.id = repoId;
    if (repoPath) params.path = repoPath;
    if (repoUrl) params.url = repoUrl;

    return this.generateDeepLink('/repository/open', params);
  }

  /**
   * 生成克隆仓库链接
   */
  generateCloneRepositoryLink(repoUrl: string, branch?: string, directory?: string): string {
    const params: Record<string, string> = { url: repoUrl };

    if (branch) params.branch = branch;
    if (directory) params.directory = directory;

    return this.generateDeepLink('/clone', params);
  }

  /**
   * 复制深度链接到剪贴板
   */
  async copyDeepLinkToClipboard(link: string): Promise<void> {
    try {
      await navigator.clipboard.writeText(link);
      this.toast.success('链接已复制到剪贴板');
    } catch (error) {
      console.error('复制链接失败:', error);
      this.toast.error('复制链接失败');
    }
  }
}

// 导出单例实例
export const deepLinkService = new DeepLinkService();

// 导出类型
export type { DeepLinkService };

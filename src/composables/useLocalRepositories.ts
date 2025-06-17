import { ref, computed, readonly } from 'vue';
import { Store } from '@tauri-apps/plugin-store';
import { gitInfoApi, type RepositoryInfo } from '@/api/git-info';
import type {
  LocalRepository,
  LocalRepositoryStore,
  AddRepositoryParams,
  UpdateRepositoryParams,
  RepositoryOperationResult
} from '@/types/local-repository';

// 全局状态
const repositories = ref<LocalRepository[]>([]);
const isLoading = ref(false);
const lastError = ref<string | null>(null);

// Store 实例
let store: Store | null = null;
const STORE_FILE = 'local-repositories.dat';
const STORE_KEY = 'repositories';
const STORE_VERSION = '1.0.0';

export const useLocalRepositories = () => {
  // 初始化 Store
  const initStore = async (): Promise<void> => {
    try {
      if (!store) {
        store = await Store.load(STORE_FILE);
      }
      await loadRepositories();
    } catch (error) {
      console.error('初始化本地仓库存储失败:', error);
      lastError.value = '初始化存储失败';
      throw error;
    }
  };

  // 加载仓库列表
  const loadRepositories = async (): Promise<void> => {
    try {
      isLoading.value = true;
      lastError.value = null;

      if (!store) {
        throw new Error('Store 未初始化');
      }

      const data = await store.get<LocalRepositoryStore>(STORE_KEY);

      if (data && data.repositories) {
        repositories.value = data.repositories;
        console.log(`加载了 ${data.repositories.length} 个本地仓库`);
      } else {
        // 首次使用，初始化空数据
        repositories.value = [];
        await saveRepositories();
      }
    } catch (error) {
      console.error('加载仓库列表失败:', error);
      lastError.value = '加载仓库列表失败';
      repositories.value = [];
    } finally {
      isLoading.value = false;
    }
  };

  // 保存仓库列表
  const saveRepositories = async (): Promise<void> => {
    try {
      if (!store) {
        throw new Error('Store 未初始化');
      }

      const data: LocalRepositoryStore = {
        repositories: repositories.value,
        lastUpdated: new Date().toISOString(),
        version: STORE_VERSION
      };

      await store.set(STORE_KEY, data);
      await store.save();
    } catch (error) {
      lastError.value = '保存仓库列表失败';
      throw error;
    }
  };

  // 生成唯一 ID
  const generateId = (): string => {
    return `repo_${Date.now()}_${Math.random().toString(36).substring(2, 11)}`;
  };

  // 验证并获取仓库信息
  const validateAndGetRepositoryInfo = async (path: string): Promise<{
    isValid: boolean;
    info: RepositoryInfo | null;
    status: 'valid' | 'invalid' | 'unknown';
  }> => {
    try {
      const result = await gitInfoApi.validateAndGetInfo(path);

      return {
        isValid: result.isValid,
        info: result.info,
        status: result.isValid ? 'valid' : 'invalid'
      };
    } catch (error) {
      console.error('验证仓库信息失败:', error);
      return {
        isValid: false,
        info: null,
        status: 'unknown'
      };
    }
  };

  // 验证仓库数据
  const validateRepository = (params: AddRepositoryParams): RepositoryOperationResult => {
    if (!params.name || params.name.trim().length === 0) {
      return { success: false, message: '仓库名称不能为空' };
    }

    if (!params.path || params.path.trim().length === 0) {
      return { success: false, message: '仓库路径不能为空' };
    }

    // 检查路径是否已存在
    const existingRepo = repositories.value.find(repo => repo.path === params.path);
    if (existingRepo) {
      return { success: false, message: '该路径的仓库已存在' };
    }

    return { success: true, message: '验证通过' };
  };

  // 添加仓库
  const addRepository = async (params: AddRepositoryParams): Promise<RepositoryOperationResult> => {
    try {
      // 验证数据
      const validation = validateRepository(params);
      if (!validation.success) {
        return validation;
      }

      console.log('开始验证仓库信息:', params.path);

      // 验证并获取 Git 仓库信息
      const repoValidation = await validateAndGetRepositoryInfo(params.path);

      // 使用 Git 信息或用户提供的信息
      const repoName = repoValidation.info?.name || params.name.trim();
      const remoteUrl = repoValidation.info?.remote_url || params.remoteUrl?.trim() || undefined;
      const currentBranch = repoValidation.info?.current_branch || params.currentBranch?.trim() || undefined;

      const newRepository: LocalRepository = {
        id: generateId(),
        name: repoName,
        path: params.path.trim(),
        remoteUrl,
        currentBranch,
        addedAt: new Date().toISOString(),
        lastChecked: new Date().toISOString(),
        status: repoValidation.status
      };

      repositories.value.unshift(newRepository); // 添加到列表开头
      await saveRepositories();

      return {
        success: true,
        message: `仓库添加成功 (状态: ${repoValidation.status})`,
        data: newRepository
      };
    } catch (error) {
      console.error('添加仓库失败:', error);
      return {
        success: false,
        message: '添加仓库失败: ' + (error as Error).message
      };
    }
  };

  // 删除仓库
  const removeRepository = async (id: string): Promise<RepositoryOperationResult> => {
    try {
      const index = repositories.value.findIndex(repo => repo.id === id);
      if (index === -1) {
        return { success: false, message: '仓库不存在' };
      }

      const removedRepo = repositories.value.splice(index, 1)[0];
      await saveRepositories();

      return {
        success: true,
        message: '仓库删除成功',
        data: removedRepo
      };
    } catch (error) {
      console.error('删除仓库失败:', error);
      return {
        success: false,
        message: '删除仓库失败: ' + (error as Error).message
      };
    }
  };

  // 更新仓库
  const updateRepository = async (params: UpdateRepositoryParams): Promise<RepositoryOperationResult> => {
    try {
      const index = repositories.value.findIndex(repo => repo.id === params.id);
      if (index === -1) {
        return { success: false, message: '仓库不存在' };
      }

      const updatedRepo = {
        ...repositories.value[index],
        ...params.updates,
        lastChecked: new Date().toISOString()
      };

      repositories.value[index] = updatedRepo;
      await saveRepositories();

      return {
        success: true,
        message: '仓库更新成功',
        data: updatedRepo
      };
    } catch (error) {
      console.error('更新仓库失败:', error);
      return {
        success: false,
        message: '更新仓库失败: ' + (error as Error).message
      };
    }
  };

  // 根据 ID 获取仓库
  const getRepository = (id: string): LocalRepository | undefined => {
    return repositories.value.find(repo => repo.id === id);
  };

  // 根据路径获取仓库
  const getRepositoryByPath = (path: string): LocalRepository | undefined => {
    return repositories.value.find(repo => repo.path === path);
  };

  // 刷新仓库状态
  const refreshRepository = async (id: string): Promise<RepositoryOperationResult> => {
    try {
      const index = repositories.value.findIndex(repo => repo.id === id);
      if (index === -1) {
        return { success: false, message: '仓库不存在' };
      }

      const repo = repositories.value[index];
      console.log('刷新仓库状态:', repo.path);

      // 重新验证仓库信息
      const repoValidation = await validateAndGetRepositoryInfo(repo.path);

      // 更新仓库信息
      const updatedRepo = {
        ...repo,
        name: repoValidation.info?.name || repo.name,
        remoteUrl: repoValidation.info?.remote_url || repo.remoteUrl,
        currentBranch: repoValidation.info?.current_branch || repo.currentBranch,
        status: repoValidation.status,
        lastChecked: new Date().toISOString()
      };

      repositories.value[index] = updatedRepo;
      await saveRepositories();

      return {
        success: true,
        message: `仓库状态已刷新 (${repoValidation.status})`,
        data: updatedRepo
      };
    } catch (error) {
      console.error('刷新仓库状态失败:', error);
      return {
        success: false,
        message: '刷新仓库状态失败: ' + (error as Error).message
      };
    }
  };

  // 批量刷新所有仓库状态
  const refreshAllRepositories = async (): Promise<RepositoryOperationResult> => {
    try {
      console.log('开始批量刷新仓库状态');

      const refreshPromises = repositories.value.map(async (repo) => {
        const repoValidation = await validateAndGetRepositoryInfo(repo.path);
        return {
          ...repo,
          name: repoValidation.info?.name || repo.name,
          remoteUrl: repoValidation.info?.remote_url || repo.remoteUrl,
          currentBranch: repoValidation.info?.current_branch || repo.currentBranch,
          status: repoValidation.status,
          lastChecked: new Date().toISOString()
        };
      });

      const updatedRepositories = await Promise.all(refreshPromises);
      repositories.value = updatedRepositories;
      await saveRepositories();

      return {
        success: true,
        message: `已刷新 ${updatedRepositories.length} 个仓库的状态`
      };
    } catch (error) {
      console.error('批量刷新仓库状态失败:', error);
      return {
        success: false,
        message: '批量刷新仓库状态失败: ' + (error as Error).message
      };
    }
  };

  // 打开仓库文件夹
  const openRepositoryFolder = async (id: string): Promise<RepositoryOperationResult> => {
    try {
      const repo = getRepository(id);
      if (!repo) {
        return { success: false, message: '仓库不存在' };
      }

      const success = await gitInfoApi.openFolder(repo.path);

      if (success) {
        return {
          success: true,
          message: '文件夹已打开'
        };
      } else {
        return {
          success: false,
          message: '打开文件夹失败'
        };
      }
    } catch (error) {
      console.error('打开仓库文件夹失败:', error);
      return {
        success: false,
        message: '打开仓库文件夹失败: ' + (error as Error).message
      };
    }
  };

  // 清空所有仓库
  const clearRepositories = async (): Promise<RepositoryOperationResult> => {
    try {
      repositories.value = [];
      await saveRepositories();

      return { success: true, message: '已清空所有仓库' };
    } catch (error) {
      console.error('清空仓库失败:', error);
      return {
        success: false,
        message: '清空仓库失败: ' + (error as Error).message
      };
    }
  };

  // 计算属性
  const repositoryCount = computed(() => repositories.value.length);
  const hasRepositories = computed(() => repositories.value.length > 0);
  const validRepositories = computed(() =>
    repositories.value.filter(repo => repo.status === 'valid')
  );
  const invalidRepositories = computed(() =>
    repositories.value.filter(repo => repo.status === 'invalid')
  );

  return {
    // 状态
    repositories: readonly(repositories),
    isLoading: readonly(isLoading),
    lastError: readonly(lastError),

    // 计算属性
    repositoryCount,
    hasRepositories,
    validRepositories,
    invalidRepositories,

    // 方法
    initStore,
    loadRepositories,
    addRepository,
    removeRepository,
    updateRepository,
    getRepository,
    getRepositoryByPath,
    refreshRepository,
    refreshAllRepositories,
    openRepositoryFolder,
    clearRepositories
  };
};

// 全局初始化函数
export const initLocalRepositories = async (): Promise<void> => {
  const { initStore } = useLocalRepositories();
  await initStore();
};

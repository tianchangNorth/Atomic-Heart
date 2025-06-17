import { ref, computed, readonly } from 'vue';
import { Store } from '@tauri-apps/plugin-store';
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
      
      console.log('仓库列表已保存');
    } catch (error) {
      console.error('保存仓库列表失败:', error);
      lastError.value = '保存仓库列表失败';
      throw error;
    }
  };

  // 生成唯一 ID
  const generateId = (): string => {
    return `repo_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
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

      const newRepository: LocalRepository = {
        id: generateId(),
        name: params.name.trim(),
        path: params.path.trim(),
        remoteUrl: params.remoteUrl?.trim() || undefined,
        currentBranch: params.currentBranch?.trim() || undefined,
        addedAt: new Date().toISOString(),
        status: 'unknown'
      };

      repositories.value.unshift(newRepository); // 添加到列表开头
      await saveRepositories();

      return {
        success: true,
        message: '仓库添加成功',
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
    clearRepositories
  };
};

// 全局初始化函数
export const initLocalRepositories = async (): Promise<void> => {
  const { initStore } = useLocalRepositories();
  await initStore();
};

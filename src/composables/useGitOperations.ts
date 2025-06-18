import { ref, computed } from 'vue';
import { gitOperationsApi, type RepositoryStatus, type CommitHistoryItem, type CommitOptions } from '@/api/git-operations';
import { useToast } from '@/components/ui/toast';

export interface GitOperationState {
  loading: boolean;
  error: string | null;
}

export const useGitOperations = (repositoryPath: string) => {
  const { success, error } = useToast();

  // 状态管理
  const repositoryStatus = ref<RepositoryStatus | null>(null);
  const commitHistory = ref<CommitHistoryItem[]>([]);
  const selectedFiles = ref<Set<string>>(new Set());

  // 操作状态
  const statusState = ref<GitOperationState>({ loading: false, error: null });
  const stageState = ref<GitOperationState>({ loading: false, error: null });
  const commitState = ref<GitOperationState>({ loading: false, error: null });
  const historyState = ref<GitOperationState>({ loading: false, error: null });

  // 计算属性
  const stagedFiles = computed(() =>
    repositoryStatus.value?.files.filter(f => f.staged) || []
  );

  const unstagedFiles = computed(() =>
    repositoryStatus.value?.files.filter(f => !f.staged) || []
  );

  const hasChanges = computed(() =>
    (repositoryStatus.value?.files.length ?? 0) > 0
  );

  const hasStagedChanges = computed(() =>
    stagedFiles.value.length > 0
  );

  const isClean = computed(() =>
    repositoryStatus.value?.is_clean ?? true
  );

  // 获取仓库状态
  const refreshStatus = async () => {
    if (!repositoryPath) return;

    statusState.value = { loading: true, error: null };

    try {
      repositoryStatus.value = await gitOperationsApi.getRepositoryStatus(repositoryPath);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '获取仓库状态失败';
      statusState.value.error = errorMessage;
      error(errorMessage, '获取仓库状态失败');
    } finally {
      statusState.value.loading = false;
    }
  };

  // 暂存文件
  const stageFiles = async (filePaths: string[]) => {
    if (!repositoryPath || filePaths.length === 0) return;

    stageState.value = { loading: true, error: null };

    try {
      await gitOperationsApi.stageFiles(repositoryPath, filePaths);
      await refreshStatus(); // 刷新状态

      success(`已暂存 ${filePaths.length} 个文件`);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '暂存文件失败';
      stageState.value.error = errorMessage;
      error(errorMessage, '暂存文件失败');
    } finally {
      stageState.value.loading = false;
    }
  };

  // 取消暂存文件
  const unstageFiles = async (filePaths: string[]) => {
    if (!repositoryPath || filePaths.length === 0) return;

    stageState.value = { loading: true, error: null };

    try {
      await gitOperationsApi.unstageFiles(repositoryPath, filePaths);
      await refreshStatus(); // 刷新状态

      success(`已取消暂存 ${filePaths.length} 个文件`);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '取消暂存文件失败';
      stageState.value.error = errorMessage;
      error(errorMessage, '取消暂存文件失败');
    } finally {
      stageState.value.loading = false;
    }
  };

  // 暂存单个文件
  const stageFile = async (filePath: string) => {
    await stageFiles([filePath]);
  };

  // 取消暂存单个文件
  const unstageFile = async (filePath: string) => {
    await unstageFiles([filePath]);
  };

  // 暂存所有文件
  const stageAllFiles = async () => {
    const unstagedPaths = unstagedFiles.value.map(f => f.path);
    if (unstagedPaths.length > 0) {
      await stageFiles(unstagedPaths);
    }
  };

  // 取消暂存所有文件
  const unstageAllFiles = async () => {
    const stagedPaths = stagedFiles.value.map(f => f.path);
    if (stagedPaths.length > 0) {
      await unstageFiles(stagedPaths);
    }
  };

  // 创建提交
  const createCommit = async (options: CommitOptions) => {
    if (!repositoryPath) return null;

    commitState.value = { loading: true, error: null };

    try {
      const commitSha = await gitOperationsApi.createCommit(repositoryPath, options);
      await refreshStatus(); // 刷新状态
      await loadCommitHistory(); // 刷新提交历史

      success(`提交已创建: ${commitSha.slice(0, 7)}`);

      return commitSha;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '创建提交失败';
      commitState.value.error = errorMessage;
      error(errorMessage, '创建提交失败');
      return null;
    } finally {
      commitState.value.loading = false;
    }
  };

  // 加载提交历史
  const loadCommitHistory = async (limit = 50, skip = 0) => {
    if (!repositoryPath) return;

    historyState.value = { loading: true, error: null };

    try {
      const commits = await gitOperationsApi.getCommitHistory(repositoryPath, limit, skip);

      if (skip === 0) {
        commitHistory.value = commits;
      } else {
        commitHistory.value.push(...commits);
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '获取提交历史失败';
      historyState.value.error = errorMessage;
      error(errorMessage, '获取提交历史失败');
    } finally {
      historyState.value.loading = false;
    }
  };

  // 获取文件差异
  const getFileDiff = async (filePath: string, staged = false) => {
    if (!repositoryPath) return '';

    try {
      return await gitOperationsApi.getFileDiff(repositoryPath, filePath, staged);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '获取文件差异失败';
      error(errorMessage, '获取文件差异失败');
      return '';
    }
  };

  // 文件选择管理
  const toggleFileSelection = (filePath: string) => {
    if (selectedFiles.value.has(filePath)) {
      selectedFiles.value.delete(filePath);
    } else {
      selectedFiles.value.add(filePath);
    }
  };

  const selectAllFiles = () => {
    repositoryStatus.value?.files.forEach(file => {
      selectedFiles.value.add(file.path);
    });
  };

  const clearSelection = () => {
    selectedFiles.value.clear();
  };

  const isFileSelected = (filePath: string) => {
    return selectedFiles.value.has(filePath);
  };

  // 批量操作选中的文件
  const stageSelectedFiles = async () => {
    const selectedPaths = Array.from(selectedFiles.value);
    const unstagedSelected = selectedPaths.filter(path =>
      unstagedFiles.value.some(f => f.path === path)
    );

    if (unstagedSelected.length > 0) {
      await stageFiles(unstagedSelected);
      clearSelection();
    }
  };

  const unstageSelectedFiles = async () => {
    const selectedPaths = Array.from(selectedFiles.value);
    const stagedSelected = selectedPaths.filter(path =>
      stagedFiles.value.some(f => f.path === path)
    );

    if (stagedSelected.length > 0) {
      await unstageFiles(stagedSelected);
      clearSelection();
    }
  };

  // 初始化
  const initialize = async () => {
    await refreshStatus();
    await loadCommitHistory();
  };

  return {
    // 状态
    repositoryStatus,
    commitHistory,
    selectedFiles,

    // 计算属性
    stagedFiles,
    unstagedFiles,
    hasChanges,
    hasStagedChanges,
    isClean,

    // 操作状态
    statusState,
    stageState,
    commitState,
    historyState,

    // 方法
    refreshStatus,
    stageFile,
    unstageFile,
    stageFiles,
    unstageFiles,
    stageAllFiles,
    unstageAllFiles,
    createCommit,
    loadCommitHistory,
    getFileDiff,

    // 文件选择
    toggleFileSelection,
    selectAllFiles,
    clearSelection,
    isFileSelected,
    stageSelectedFiles,
    unstageSelectedFiles,

    // 初始化
    initialize,
  };
};

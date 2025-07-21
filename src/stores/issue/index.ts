import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type {
  Issue,
} from '@/types/issue';
import { $fetch } from '@/utils/fetch';

export const useIssueStore = defineStore('issue', () => {
  // 状态
  const currentIssue = ref<Issue | null>(null);

  // 加载状态
  const loading = ref(false);
  const error = ref<string | null>(null);

  // 计算属性
  const isIssueLoaded = computed(() => currentIssue.value !== null);

  // Actions
  const resetState = () => {
    currentIssue.value = null;
  };

  // 获取issue详情
  const fetchIssue = async (owner: string, repo: string, number: string) => {
    loading.value = true;
    error.value = null;

    try {
      const { success, data } = await $fetch(`/repos/${owner}/${repo}/issues/${number}`, {
        method: 'get'
      });

      if (success && data) {
        currentIssue.value = data;
        return { success: true, data };
      } else {
        throw new Error('获取issue信息失败');
      }
    } catch (err) {
      console.error('获取issue详情失败:', err);
      error.value = err instanceof Error ? err.message : '获取issue详情失败';
      return { success: false, error: error.value };
    } finally {
      loading.value = false;
    }
  };

  const fetchIssueData = async (owner: string, repo: string, number: string) => {
    resetState();

    // 首先获取基本仓库信息
    const issueResult = await fetchIssue(owner, repo, number);
    console.log('issueResult', issueResult);

    if (!issueResult.success) {
      return issueResult;
    }

    // 并行获取其他数据
    await Promise.all([
      // fetchRepositoryStats(owner, repo),
      // fetchLanguages(owner, repo)
    ]);

    return { success: true };
  };

  return {
    // 状态
    currentIssue,

    // 加载状态
    loading,
    error,

    // 计算属性
    isIssueLoaded,

    // Actions
    resetState,
    fetchIssue,

    fetchIssueData
  };
});

export default useIssueStore;

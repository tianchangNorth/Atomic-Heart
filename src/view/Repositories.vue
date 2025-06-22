<script setup lang="ts">
import { onMounted, ref, computed } from 'vue';
import { useRouter } from 'vue-router';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Badge } from '@/components/ui/badge';
import { $fetch } from '@/utils/fetch';
import { useUserStore } from '@/stores/index';
import RepoClone from '@/components/git/RepoClone.vue';
import { getToken } from '@/utils/token';
import { useLocalRepositories } from '@/composables/useLocalRepositories';
import { extractRepositoryName } from '@/utils/utils'
import { invoke } from '@tauri-apps/api/core';
import { useToast } from '@/components/ui/toast';

// 定义仓库数据接口
interface Repository {
  private: boolean;        // 仓库是否为私有
  full_name: string;      // 完整仓库名（用户名/仓库名）
  name: string;           // 仓库名称
  description: string;    // 仓库描述
  default_branch: string; // 默认分支名
  git_url: string;        // Git 克隆地址
  html_url: string;       // 仓库网页地址
  created_at?: string;    // 创建时间
  updated_at?: string;    // 更新时间
  language?: string;      // 主要编程语言
  stargazersCount?: number;   // 星标数
  forks_count?: number;   // 分叉数
}

const router = useRouter();
const userStore = useUserStore();
const { user } = userStore;
const { success } = useToast();

// 状态管理
const repositories = ref<Repository[]>([]);
const loading = ref(false);
const error = ref<string | null>(null);

// 搜索和筛选状态
const searchQuery = ref('');
const searchInput = ref(''); // 输入框的值
const sortBy = ref('name');
const filterBy = ref('all');

// 克隆相关状态
const showCloneDialog = ref(false);
const selectedRepoForClone = ref<Repository | null>(null);

// 使用本地仓库管理
const { addRepository } = useLocalRepositories();

// 获取仓库数据
const fetchRepositories = async () => {
  loading.value = true;
  error.value = null;

  try {
    const { success, data } = await $fetch(`/users/${user.login}/repos`, { method: 'get' });
    if (success && Array.isArray(data)) {
      repositories.value = data;
    } else {
      // 如果 API 失败，使用模拟数据
    }
  } catch (err) {
    console.error('获取仓库失败:', err);
    error.value = '获取仓库数据失败';
    // 使用模拟数据作为降级方案
  } finally {
    loading.value = false;
  }
};

// 筛选和排序的仓库列表
const filteredRepositories = computed(() => {
  let filtered = repositories.value;
  // 搜索筛选
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase();
    filtered = filtered.filter(repo =>
      repo.name.toLowerCase().includes(query) ||
      repo.description?.toLowerCase().includes(query) ||
      repo.full_name.toLowerCase().includes(query)
    );
  }


  // 状态筛选
  if (filterBy.value === 'public') {
    filtered = filtered.filter(repo => !repo.private);
  } else if (filterBy.value === 'private') {
    filtered = filtered.filter(repo => repo.private);
  }

  // 排序
  filtered.sort((a, b) => {
    switch (sortBy.value) {
      case 'name':
        return a.name.localeCompare(b.name);
      case 'updated':
        return new Date(b.updated_at || 0).getTime() - new Date(a.updated_at || 0).getTime();
      case 'created':
        return new Date(b.created_at || 0).getTime() - new Date(a.created_at || 0).getTime();
      case 'stars':
        return (b.stargazersCount || 0) - (a.stargazersCount || 0);
      default:
        return 0;
    }
  });

  return filtered;
});

// 格式化时间显示
const formatDate = (dateString?: string): string => {
  if (!dateString) return '未知';

  const date = new Date(dateString);
  const now = new Date();
  const diffInSeconds = Math.floor((now.getTime() - date.getTime()) / 1000);

  if (diffInSeconds < 60) {
    return '刚刚';
  } else if (diffInSeconds < 3600) {
    const minutes = Math.floor(diffInSeconds / 60);
    return `${minutes}分钟前`;
  } else if (diffInSeconds < 86400) {
    const hours = Math.floor(diffInSeconds / 3600);
    return `${hours}小时前`;
  } else if (diffInSeconds < 2592000) {
    const days = Math.floor(diffInSeconds / 86400);
    return `${days}天前`;
  } else {
    return date.toLocaleDateString('zh-CN');
  }
};

// 获取编程语言的颜色
const getLanguageColor = (language?: string): string => {
  const colors: Record<string, string> = {
    TypeScript: '#3178c6',
    JavaScript: '#f1e05a',
    Vue: '#4fc08d',
    React: '#61dafb',
    Python: '#3572a5',
    Java: '#b07219',
    Rust: '#dea584',
    Go: '#00add8',
    PHP: '#4f5d95',
    Ruby: '#701516',
    Swift: '#fa7343',
    Kotlin: '#a97bff',
    Dart: '#00b4ab',
    HTML: '#e34c26',
    CSS: '#1572b6'
  };
  return colors[language || ''] || '#6b7280';
};

// 处理操作
const handleCloneRepo = (repo: Repository) => {
  selectedRepoForClone.value = repo;
  showCloneDialog.value = true;
};

// 获取克隆配置
const getCloneConfig = computed(() => {
  if (!selectedRepoForClone.value) return null;

  const repo = selectedRepoForClone.value;
  const token = getToken();

  return {
    url: repo.html_url, // 确保使用 HTTPS URL 进行 token 认证
    suggestedDirectory: '', // 不提供默认路径，让用户自己选择
    // 如果用户已登录且有 token，提供默认的 token 认证配置
    authConfig: token && user.login ? {
      auth_type: 'token' as const,
      token: token,
      username: user.login
    } : undefined
  };
});

// 关闭克隆对话框
const closeCloneDialog = () => {
  showCloneDialog.value = false;
  selectedRepoForClone.value = null;
};

const handleViewRepo = (repo: Repository) => {
  // 跳转到仓库详情页面
  const [owner, repoName] = repo.full_name.split('/');
  console.log(owner, repoName);

  router.push({
    name: 'RepositoryDetail',
    params: { owner, repo: repoName }
  });
};

const handleCreateRepo = () => {
  // 跳转到创建仓库页面
  invoke('open_url', { url: 'https://atomgit.com/project/new' });
};

const handleCloneSuccess = async (result: any) => {
  try {
    if (result.success && result.repository_path) {
      // 从 URL 中提取仓库名称
      const repoUrl = result.repository_url || '';
      const repoName = extractRepositoryName(repoUrl);

      // 添加到本地仓库列表
      const addResult = await addRepository({
        name: repoName,
        path: result.repository_path,
        remoteUrl: repoUrl || undefined,
        currentBranch: result.branch || undefined
      });

      if (addResult.success) {
        // 关闭克隆对话框
        showCloneDialog.value = false;
        success('仓库克隆成功');
        router.push('/local-repositories');
      } else {
        console.error('添加仓库到本地列表失败:', addResult.message);
      }
    } else {
      console.warn('克隆成功但缺少必要信息:', result);
    }
  } catch (error) {
    console.error('处理克隆成功事件时出错:', error);
  }
};

const clearSearch = () => {
  searchInput.value = '';
  searchQuery.value = '';
};

// 重试获取数据
const retryFetch = () => {
  fetchRepositories();
};

// 组件挂载时获取数据
onMounted(() => {
  fetchRepositories();
});
</script>

<style scoped>
/* 自定义下拉框箭头样式 */
.custom-select {
  background-image: url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='M6 8l4 4 4-4'/%3e%3c/svg%3e");
  background-position: right 0.5rem center;
  background-repeat: no-repeat;
  background-size: 1.5em 1.5em;
}

/* 深色模式下的箭头颜色 */
@media (prefers-color-scheme: dark) {
  .custom-select {
    background-image: url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%9ca3af' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='M6 8l4 4 4-4'/%3e%3c/svg%3e");
  }
}

/* 聚焦状态下的箭头颜色 */
.custom-select:focus {
  background-image: url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%23059669' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='M6 8l4 4 4-4'/%3e%3c/svg%3e");
}
</style>

<template>
  <div class="space-y-6">
    <!-- 页面标题 -->
    <div class="flex flex-col sm:flex-row sm:justify-between sm:items-center gap-4">
      <div>
        <h1 class="text-3xl font-bold text-foreground">仓库</h1>
        <p class="text-muted-foreground">管理您的代码仓库</p>
      </div>
      <div class="flex gap-2">
        <Button @click="handleCreateRepo" class="self-start sm:self-auto cursor-pointer">
          <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
          </svg>
          新建仓库
        </Button>
      </div>
    </div>

    <!-- 搜索、筛选和排序 -->
    <div class="flex flex-col sm:flex-row gap-4">
      <!-- 搜索框 -->
      <div class="flex-1">
        <div class="relative">
          <Input
            v-model="searchQuery"
            type="text"
            placeholder="搜索仓库名称、描述.."
            class="pl-10 pr-20"
          />
          <svg class="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
          </svg>

          <!-- 搜索和清除按钮 -->
          <div class="absolute right-2 top-1/2 transform -translate-y-1/2 flex items-center space-x-1">
            <Button
              v-if="searchQuery"
              variant="ghost"
              size="sm"
              @click="clearSearch"
              class="h-6 w-6 p-0 text-muted-foreground hover:text-foreground"
              title="清除搜索"
            >
              <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
              </svg>
            </Button>
          </div>
        </div>
      </div>

      <!-- 筛选器 -->
      <div class="flex gap-2">
        <select
          v-model="filterBy"
          class="custom-select px-3 py-2 pr-8 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-ring appearance-none"
        >
          <option value="all">全部仓库</option>
          <option value="public">公开仓库</option>
          <option value="private">私有仓库</option>
        </select>

        <select
          v-model="sortBy"
          class="custom-select px-3 py-2 pr-8 bg-background border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-ring appearance-none"
        >
          <option value="name">按名称排序</option>
          <option value="updated">按更新时间</option>
          <option value="created">按创建时间</option>
          <option value="stars">按星标数</option>
        </select>
      </div>
    </div>

    <!-- 统计信息 -->
    <div class="flex items-center justify-between text-sm text-muted-foreground">
      <span>
        找到 {{ filteredRepositories.length }} 个仓库
        <span v-if="searchQuery || filterBy !== 'all'">
          （共 {{ repositories.length }} 个）
        </span>
      </span>
      <Button
        v-if="error"
        variant="ghost"
        size="sm"
        @click="retryFetch"
        class="text-muted-foreground hover:text-foreground"
      >
        <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
        </svg>
        重试
      </Button>
    </div>

    <!-- 加载状态 -->
    <div v-if="loading" class="flex items-center justify-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
      <span class="ml-3 text-muted-foreground">加载仓库数据...</span>
    </div>

    <!-- 错误状态 -->
    <div v-else-if="error" class="text-center py-12">
      <div class="text-muted-foreground mb-4">
        <svg class="w-12 h-12 mx-auto mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.34 16.5c-.77.833.192 2.5 1.732 2.5z"/>
        </svg>
        <p class="text-sm">{{ error }}</p>
      </div>
      <Button variant="outline" @click="retryFetch">
        <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
        </svg>
        重试
      </Button>
    </div>

    <!-- 空状态 -->
    <div v-else-if="filteredRepositories.length === 0 && repositories.length === 0" class="text-center py-12">
      <svg class="w-16 h-16 mx-auto text-muted-foreground mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"/>
      </svg>
      <h3 class="text-lg font-semibold text-foreground mb-2">还没有仓库</h3>
      <p class="text-muted-foreground mb-4">创建您的第一个仓库来开始使用 AtomGit</p>
      <Button @click="handleCreateRepo">
        <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
        </svg>
        新建仓库
      </Button>
    </div>

    <!-- 搜索无结果 -->
    <div v-else-if="filteredRepositories.length === 0" class="text-center py-12">
      <svg class="w-16 h-16 mx-auto text-muted-foreground mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
      </svg>
      <h3 class="text-lg font-semibold text-foreground mb-2">未找到匹配的仓库</h3>
      <p class="text-muted-foreground mb-4">尝试调整搜索条件或筛选器</p>
      <Button variant="outline" @click="searchInput = ''; searchQuery = ''; filterBy = 'all'">
        清除筛选条件
      </Button>
    </div>

    <!-- 仓库列表 -->
    <div v-else class="space-y-4">
      <Card
        v-for="repo in filteredRepositories"
        :key="repo.full_name"
        class="hover:shadow-md transition-shadow cursor-pointer"
        @click="handleViewRepo(repo)"
      >
        <CardContent class="p-6">
          <div class="flex justify-between items-start">
            <div class="flex-1 min-w-0">
              <!-- 仓库标题和状态 -->
              <div class="flex items-center space-x-3 mb-3">
                <h3 class="text-lg font-semibold text-foreground hover:text-primary transition-colors truncate">
                  {{ repo.full_name }}
                </h3>
                <Badge :variant="repo.private ? 'secondary' : 'default'" class="flex-shrink-0">
                  <svg class="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path v-if="repo.private" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
                    <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                  </svg>
                  {{ repo.private ? '私有' : '公开' }}
                </Badge>
              </div>

              <!-- 仓库描述 -->
              <p class="text-muted-foreground mb-4 line-clamp-2">
                {{ repo.description || '暂无描述' }}
              </p>

              <!-- 仓库元信息 -->
              <div class="flex items-center flex-wrap gap-4 text-sm text-muted-foreground">
                <!-- 编程语言 -->
                <div v-if="repo.language" class="flex items-center space-x-1">
                  <div
                    class="w-3 h-3 rounded-full"
                    :style="{ backgroundColor: getLanguageColor(repo.language) }"
                  ></div>
                  <span>{{ repo.language }}</span>
                </div>

                <!-- 星标数 -->
                <div class="flex items-center space-x-1">
                  <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z"/>
                  </svg>
                  <span>{{ repo.stargazersCount || 0 }}</span>
                </div>

                <!-- 分叉数 -->
                <div class="flex items-center space-x-1">
                  <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M7.707 3.293a1 1 0 010 1.414L5.414 7H11a7 7 0 017 7v2a1 1 0 11-2 0v-2a5 5 0 00-5-5H5.414l2.293 2.293a1 1 0 11-1.414 1.414L2.586 7l3.707-3.707a1 1 0 011.414 0z" clip-rule="evenodd"/>
                  </svg>
                  <span>{{ repo.forks_count || 0 }}</span>
                </div>

                <!-- 默认分支 -->
                <div class="flex items-center space-x-1">
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                  </svg>
                  <span>{{ repo.default_branch }}</span>
                </div>

                <!-- 更新时间 -->
                <span>更新于 {{ formatDate(repo.updated_at) }}</span>
              </div>
            </div>

            <!-- 操作按钮 -->
            <div class="flex space-x-2 ml-4">
              <Button
                variant="outline"
                size="sm"
                @click.stop="handleCloneRepo(repo)"
                title="克隆仓库到本地"
              >
                <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"/>
                </svg>
                克隆
              </Button>
              <Button
                variant="outline"
                size="sm"
                @click.stop="handleViewRepo(repo)"
                title="查看仓库详情"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"/>
                </svg>
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>

    <!-- 克隆对话框 -->
    <div v-if="showCloneDialog" class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 overflow-hidden">
      <div class="w-full max-w-4xl overflow-hidden">
        <div class="relative scrollbar-hide overflow-y-auto">
          <Button
            variant="ghost"
            class="absolute top-4 right-32 z-10 cursor-pointer"
            @click="closeCloneDialog"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
            </svg>
          </Button>
          <RepoClone
            v-if="selectedRepoForClone && getCloneConfig"
            :initial-url="getCloneConfig.url"
            :initial-directory="getCloneConfig.suggestedDirectory"
            :initial-auth-config="getCloneConfig.authConfig"
            @cloneSuccess="handleCloneSuccess"
          />
        </div>
      </div>
    </div>

  </div>
</template>


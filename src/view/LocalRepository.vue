<script setup lang="ts">
import { ref, computed } from 'vue';
import { useRouter } from 'vue-router';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import RepoClone from '@/components/git/RepoClone.vue';
import type { LocalRepository } from '@/types/git';

const router = useRouter();

// 模拟本地仓库数据
const localRepositories = ref<LocalRepository[]>([
  {
    id: '1',
    name: 'AtomDesk',
    path: '/Users/developer/Projects/AtomDesk',
    remoteUrl: 'https://github.com/user/AtomDesk.git',
    currentBranch: 'main',
    status: 'dirty',
    ahead: 2,
    behind: 1,
    createdAt: '2024-12-10T10:00:00Z',
    updatedAt: '2024-12-15T14:30:00Z'
  },
  {
    id: '2',
    name: 'vue-components',
    path: '/Users/developer/Projects/vue-components',
    remoteUrl: 'https://github.com/user/vue-components.git',
    currentBranch: 'develop',
    status: 'clean',
    ahead: 0,
    behind: 0,
    createdAt: '2024-12-08T15:30:00Z',
    updatedAt: '2024-12-14T09:15:00Z'
  },
  {
    id: '3',
    name: 'api-server',
    path: '/Users/developer/Projects/api-server',
    currentBranch: 'feature/auth',
    status: 'conflict',
    ahead: 3,
    behind: 2,
    createdAt: '2024-12-05T11:20:00Z',
    updatedAt: '2024-12-15T16:45:00Z'
  }
]);

const showCloneDialog = ref(false);
const searchQuery = ref('');
const viewMode = ref<'grid' | 'list'>('grid');

// 计算属性
const repositoryStats = computed(() => {
  const total = localRepositories.value.length;
  const clean = localRepositories.value.filter(r => r.status === 'clean').length;
  const dirty = localRepositories.value.filter(r => r.status === 'dirty').length;
  const conflict = localRepositories.value.filter(r => r.status === 'conflict').length;

  return { total, clean, dirty, conflict };
});

const filteredRepositories = computed(() => {
  if (!searchQuery.value) return localRepositories.value;
  return localRepositories.value.filter(repo =>
    repo.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
    repo.path.toLowerCase().includes(searchQuery.value.toLowerCase())
  );
});

const getStatusText = (status: LocalRepository['status']) => {
  switch (status) {
    case 'clean': return '干净';
    case 'dirty': return '有变更';
    case 'conflict': return '有冲突';
    case 'syncing': return '同步中';
    default: return '未知';
  }
};

const getStatusBadgeVariant = (status: LocalRepository['status']) => {
  switch (status) {
    case 'clean': return 'default';
    case 'dirty': return 'secondary';
    case 'conflict': return 'destructive';
    case 'syncing': return 'outline';
    default: return 'outline';
  }
};

// 方法
const openRepositoryDetail = (repo: LocalRepository) => {
  // 跳转到本地仓库详情页面
  router.push(`/local-repositories/${repo.id}`);
};

const openRepository = (repo: LocalRepository) => {
  // 这里将来会调用 Tauri API 打开文件夹
  console.log('打开仓库:', repo.path);
};

const removeRepository = (repoId: string) => {
  const index = localRepositories.value.findIndex(r => r.id === repoId);
  if (index > -1) {
    localRepositories.value.splice(index, 1);
  }
};

const refreshRepository = (repo: LocalRepository) => {
  // 这里将来会调用 API 刷新仓库状态
  console.log('刷新仓库状态:', repo.name);
};

const formatDate = (dateString: string): string => {
  const date = new Date(dateString);
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  });
};
</script>

<template>
  <div class="min-h-screen bg-background">
    <div class="container mx-auto px-4 py-6 max-w-7xl">
      <!-- 页面标题 -->
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-8">
        <div>
          <h1 class="text-3xl font-bold text-foreground">本地仓库管理</h1>
          <p class="text-muted-foreground mt-1">管理您的本地 Git 仓库</p>
        </div>
        <div class="flex items-center space-x-3">
          <Button variant="outline" @click="showCloneDialog = true">
            <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
            </svg>
            克隆仓库
          </Button>
        </div>
      </div>

      <!-- 统计卡片 -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
        <Card>
          <CardContent class="p-6">
            <div class="flex items-center justify-between">
              <div>
                <p class="text-sm font-medium text-muted-foreground">总仓库数</p>
                <p class="text-2xl font-bold">{{ repositoryStats.total }}</p>
              </div>
              <svg class="w-8 h-8 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"/>
              </svg>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent class="p-6">
            <div class="flex items-center justify-between">
              <div>
                <p class="text-sm font-medium text-muted-foreground">干净状态</p>
                <p class="text-2xl font-bold text-green-600">{{ repositoryStats.clean }}</p>
              </div>
              <svg class="w-8 h-8 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
              </svg>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent class="p-6">
            <div class="flex items-center justify-between">
              <div>
                <p class="text-sm font-medium text-muted-foreground">有变更</p>
                <p class="text-2xl font-bold text-yellow-600">{{ repositoryStats.dirty }}</p>
              </div>
              <svg class="w-8 h-8 text-yellow-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/>
              </svg>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent class="p-6">
            <div class="flex items-center justify-between">
              <div>
                <p class="text-sm font-medium text-muted-foreground">有冲突</p>
                <p class="text-2xl font-bold text-red-600">{{ repositoryStats.conflict }}</p>
              </div>
              <svg class="w-8 h-8 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.34 16.5c-.77.833.192 2.5 1.732 2.5z"/>
              </svg>
            </div>
          </CardContent>
        </Card>
      </div>

      <!-- 搜索和视图控制 -->
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-6">
        <div class="flex-1 max-w-md">
          <div class="relative">
            <svg class="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
            </svg>
            <Input
              v-model="searchQuery"
              placeholder="搜索仓库名称或路径..."
              class="w-full pl-10"
            />
          </div>
        </div>
        <div class="flex items-center space-x-2">
          <Button
            :variant="viewMode === 'grid' ? 'default' : 'outline'"
            size="sm"
            @click="viewMode = 'grid'"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"/>
            </svg>
          </Button>
          <Button
            :variant="viewMode === 'list' ? 'default' : 'outline'"
            size="sm"
            @click="viewMode = 'list'"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h16M4 18h16"/>
            </svg>
          </Button>
        </div>
      </div>

      <!-- 仓库列表 -->
      <div v-if="filteredRepositories.length === 0 && searchQuery" class="text-center py-12">
        <svg class="w-16 h-16 mx-auto mb-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
        </svg>
        <h3 class="text-lg font-medium text-foreground mb-2">未找到匹配的仓库</h3>
        <p class="text-muted-foreground">尝试使用不同的关键词搜索</p>
      </div>

      <div v-else-if="filteredRepositories.length === 0" class="text-center py-12">
        <svg class="w-16 h-16 mx-auto mb-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"/>
        </svg>
        <h3 class="text-lg font-medium text-foreground mb-2">暂无本地仓库</h3>
        <p class="text-muted-foreground mb-4">开始克隆您的第一个仓库</p>
        <Button @click="showCloneDialog = true">
          <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
          </svg>
          克隆仓库
        </Button>
      </div>

      <!-- 网格视图 -->
      <div v-else-if="viewMode === 'grid'" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <Card
          v-for="repo in filteredRepositories"
          :key="repo.id"
          class="cursor-pointer transition-all hover:shadow-lg hover:-translate-y-1"
          @click="openRepositoryDetail(repo)"
        >
          <CardHeader class="pb-3">
            <div class="flex items-center justify-between">
              <CardTitle class="text-lg truncate pr-2">{{ repo.name }}</CardTitle>
              <Badge :variant="getStatusBadgeVariant(repo.status)" class="text-xs flex-shrink-0">
                {{ getStatusText(repo.status) }}
              </Badge>
            </div>
          </CardHeader>
          <CardContent class="space-y-3">
            <div class="flex items-center space-x-2 text-sm text-muted-foreground">
              <svg class="w-4 h-4 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16l2.879-2.879m0 0a3 3 0 104.243-4.242 3 3 0 00-4.243 4.242zM21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
              </svg>
              <span class="font-medium">{{ repo.currentBranch }}</span>
              <div v-if="repo.ahead > 0 || repo.behind > 0" class="flex items-center space-x-2 ml-auto">
                <span v-if="repo.ahead > 0" class="text-blue-600 text-xs">↑{{ repo.ahead }}</span>
                <span v-if="repo.behind > 0" class="text-orange-600 text-xs">↓{{ repo.behind }}</span>
              </div>
            </div>

            <div class="flex items-center space-x-2 text-sm text-muted-foreground">
              <svg class="w-4 h-4 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"/>
              </svg>
              <span class="truncate">{{ repo.path }}</span>
            </div>

            <div class="flex items-center justify-between pt-2 border-t border-border">
              <span class="text-xs text-muted-foreground">{{ formatDate(repo.updatedAt) }}</span>
              <div class="flex space-x-1">
                <Button
                  variant="ghost"
                  size="sm"
                  @click.stop="refreshRepository(repo)"
                  class="h-7 w-7 p-0"
                  title="刷新状态"
                >
                  <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
                  </svg>
                </Button>
                <Button
                  variant="ghost"
                  size="sm"
                  @click.stop="openRepository(repo)"
                  class="h-7 w-7 p-0"
                  title="在文件管理器中打开"
                >
                  <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"/>
                  </svg>
                </Button>
                <Button
                  variant="ghost"
                  size="sm"
                  @click.stop="removeRepository(repo.id)"
                  class="h-7 w-7 p-0 text-red-600 hover:text-red-700"
                  title="移除仓库"
                >
                  <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
                  </svg>
                </Button>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      <!-- 列表视图 -->
      <div v-else class="space-y-3">
        <Card
          v-for="repo in filteredRepositories"
          :key="repo.id"
          class="cursor-pointer transition-all hover:shadow-md"
          @click="openRepositoryDetail(repo)"
        >
          <CardContent class="p-4">
            <div class="flex items-center justify-between">
              <div class="flex items-center space-x-4 flex-1 min-w-0">
                <div class="flex-1 min-w-0">
                  <h3 class="font-semibold text-lg truncate">{{ repo.name }}</h3>
                  <p class="text-sm text-muted-foreground truncate">{{ repo.path }}</p>
                </div>
                <div class="flex items-center space-x-4">
                  <div class="flex items-center space-x-2 text-sm">
                    <svg class="w-4 h-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16l2.879-2.879m0 0a3 3 0 104.243-4.242 3 3 0 00-4.243 4.242zM21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                    </svg>
                    <span>{{ repo.currentBranch }}</span>
                  </div>
                  <div v-if="repo.ahead > 0 || repo.behind > 0" class="flex items-center space-x-2">
                    <span v-if="repo.ahead > 0" class="text-blue-600 text-sm">↑{{ repo.ahead }}</span>
                    <span v-if="repo.behind > 0" class="text-orange-600 text-sm">↓{{ repo.behind }}</span>
                  </div>
                  <Badge :variant="getStatusBadgeVariant(repo.status)">
                    {{ getStatusText(repo.status) }}
                  </Badge>
                  <span class="text-xs text-muted-foreground">{{ formatDate(repo.updatedAt) }}</span>
                </div>
              </div>
              <div class="flex space-x-1 ml-4">
                <Button
                  variant="ghost"
                  size="sm"
                  @click.stop="refreshRepository(repo)"
                  class="h-8 w-8 p-0"
                  title="刷新状态"
                >
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
                  </svg>
                </Button>
                <Button
                  variant="ghost"
                  size="sm"
                  @click.stop="openRepository(repo)"
                  class="h-8 w-8 p-0"
                  title="在文件管理器中打开"
                >
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"/>
                  </svg>
                </Button>
                <Button
                  variant="ghost"
                  size="sm"
                  @click.stop="removeRepository(repo.id)"
                  class="h-8 w-8 p-0 text-red-600 hover:text-red-700"
                  title="移除仓库"
                >
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
                  </svg>
                </Button>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

    </div>
    </div>

    <!-- 克隆仓库弹窗 -->
    <div v-if="showCloneDialog" class="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div class="w-full max-w-4xl max-h-[90vh] overflow-y-auto">
        <div class="relative">
          <Button
            variant="ghost"
            class="absolute top-4 right-32 z-10 cursor-pointer"
            @click="showCloneDialog = false"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
            </svg>
          </Button>
          <RepoClone />
        </div>
      </div>
    </div>
</template>

<style scoped>
/* 自定义滚动条样式 */
.scrollbar-thin {
  scrollbar-width: thin;
}

.scrollbar-thumb-muted {
  scrollbar-color: hsl(var(--muted-foreground) / 0.3) transparent;
}

.scrollbar-track-transparent {
  scrollbar-color: hsl(var(--muted-foreground) / 0.3) transparent;
}

/* Webkit 滚动条样式 */
.overflow-x-auto::-webkit-scrollbar {
  height: 8px;
}

.overflow-x-auto::-webkit-scrollbar-track {
  background: transparent;
}

.overflow-x-auto::-webkit-scrollbar-thumb {
  background: hsl(var(--muted-foreground) / 0.3);
  border-radius: 4px;
}

.overflow-x-auto::-webkit-scrollbar-thumb:hover {
  background: hsl(var(--muted-foreground) / 0.5);
}

/* 仓库卡片动画 */
.flex-shrink-0 {
  transition: all 0.2s ease-in-out;
}

.flex-shrink-0:hover {
  transform: translateY(-2px);
}

/* 响应式优化 */
@media (max-width: 768px) {
  .w-80 {
    width: 280px;
  }
}

@media (max-width: 640px) {
  .w-80 {
    width: 240px;
  }
}

/* 选项卡内容区域最小高度 */
.min-h-\[600px\] {
  min-height: 600px;
}

@media (max-width: 768px) {
  .min-h-\[600px\] {
    min-height: 400px;
  }
}

/* 确保代码查看区域有足够空间 */
:deep(.grid.grid-cols-1.lg\\:grid-cols-2) {
  min-height: 500px;
}

:deep(.diff-viewer) {
  min-height: 400px;
}

/* 优化移动端体验 */
@media (max-width: 1024px) {
  :deep(.grid.grid-cols-1.lg\\:grid-cols-2) {
    grid-template-columns: 1fr;
    gap: 1rem;
  }

  :deep(.diff-viewer) {
    min-height: 300px;
  }
}
</style>

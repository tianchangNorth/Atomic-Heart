<script setup lang="ts">
import { ref, computed } from 'vue';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import RepoClone from '@/components/git/RepoClone.vue';
import CommitManager from '@/components/git/CommitManager.vue';
import SyncManager from '@/components/git/SyncManager.vue';
import BranchManager from '@/components/git/BranchManager.vue';
import ConflictDialog from '@/components/git/ui/ConflictDialog.vue';
import type { LocalRepository, ConflictFile } from '@/types/git';

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

const selectedRepository = ref<LocalRepository | null>(null);
const activeTab = ref('overview');
const showCloneDialog = ref(false);
const showConflictDialog = ref(false);

// 模拟冲突文件数据
const conflictFiles = ref<ConflictFile[]>([
  {
    path: 'src/components/Header.vue',
    resolved: false,
    conflicts: [
      {
        id: 'conflict-1',
        startLine: 15,
        endLine: 25,
        currentContent: `<template>
  <header class="bg-blue-600 text-white">
    <h1>AtomDesk v2.0</h1>
  </header>
</template>`,
        incomingContent: `<template>
  <header class="bg-green-600 text-white">
    <h1>AtomDesk v2.1</h1>
  </header>
</template>`
      }
    ]
  }
]);

// 计算属性
const repositoryStats = computed(() => {
  const total = localRepositories.value.length;
  const clean = localRepositories.value.filter(r => r.status === 'clean').length;
  const dirty = localRepositories.value.filter(r => r.status === 'dirty').length;
  const conflict = localRepositories.value.filter(r => r.status === 'conflict').length;

  return { total, clean, dirty, conflict };
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
const selectRepository = (repo: LocalRepository) => {
  selectedRepository.value = repo;
  activeTab.value = 'overview';
};

const openRepository = (repo: LocalRepository) => {
  // 这里将来会调用 Tauri API 打开文件夹
  console.log('打开仓库:', repo.path);
};

const removeRepository = (repoId: string) => {
  const index = localRepositories.value.findIndex(r => r.id === repoId);
  if (index > -1) {
    localRepositories.value.splice(index, 1);
    if (selectedRepository.value?.id === repoId) {
      selectedRepository.value = null;
    }
  }
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

const handleConflictResolve = (files: ConflictFile[]) => {
  console.log('冲突已解决:', files);
  showConflictDialog.value = false;

  // 更新仓库状态
  if (selectedRepository.value) {
    selectedRepository.value.status = 'dirty';
  }
};

const handleConflictCancel = () => {
  console.log('取消解决冲突');
  showConflictDialog.value = false;
};
</script>

<template>
  <div class="min-h-screen bg-background">
    <div class="container mx-auto px-4 py-6 max-w-7xl">
      <!-- 页面标题 -->
      <div class="flex items-center justify-between mb-8">
        <div>
          <h1 class="text-3xl font-bold text-foreground">本地仓库管理</h1>
          <p class="text-muted-foreground mt-1">管理您的本地 Git 仓库</p>
        </div>
        <Button @click="showCloneDialog = true">
          <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
          </svg>
          克隆仓库
        </Button>
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

      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <!-- 左侧：仓库列表 -->
        <div class="lg:col-span-1">
          <Card>
            <CardHeader>
              <CardTitle class="flex items-center space-x-2">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"/>
                </svg>
                <span>本地仓库</span>
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div class="space-y-3">
                <div
                  v-for="repo in localRepositories"
                  :key="repo.id"
                  class="p-3 rounded-lg border cursor-pointer transition-colors"
                  :class="{
                    'bg-accent border-primary': selectedRepository?.id === repo.id,
                    'hover:bg-accent/50': selectedRepository?.id !== repo.id
                  }"
                  @click="selectRepository(repo)"
                >
                  <div class="flex items-center justify-between mb-2">
                    <h3 class="font-medium truncate">{{ repo.name }}</h3>
                    <Badge :variant="getStatusBadgeVariant(repo.status)" class="text-xs">
                      {{ getStatusText(repo.status) }}
                    </Badge>
                  </div>
                  
                  <div class="text-sm text-muted-foreground space-y-1">
                    <div class="flex items-center space-x-2">
                      <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16l2.879-2.879m0 0a3 3 0 104.243-4.242 3 3 0 00-4.243 4.242zM21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                      </svg>
                      <span>{{ repo.currentBranch }}</span>
                    </div>
                    
                    <div v-if="repo.ahead > 0 || repo.behind > 0" class="flex items-center space-x-3 text-xs">
                      <span v-if="repo.ahead > 0" class="text-blue-600">↑{{ repo.ahead }}</span>
                      <span v-if="repo.behind > 0" class="text-orange-600">↓{{ repo.behind }}</span>
                    </div>
                    
                    <div class="truncate">{{ repo.path }}</div>
                  </div>
                  
                  <div class="flex items-center justify-between mt-3">
                    <span class="text-xs text-muted-foreground">
                      {{ formatDate(repo.updatedAt) }}
                    </span>
                    <div class="flex space-x-1">
                      <Button
                        variant="ghost"
                        size="sm"
                        @click.stop="openRepository(repo)"
                        class="h-6 w-6 p-0"
                      >
                        <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"/>
                        </svg>
                      </Button>
                      <Button
                        variant="ghost"
                        size="sm"
                        @click.stop="removeRepository(repo.id)"
                        class="h-6 w-6 p-0 text-red-600 hover:text-red-700"
                      >
                        <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
                        </svg>
                      </Button>
                    </div>
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>

        <!-- 右侧：仓库详情 -->
        <div class="lg:col-span-2">
          <div v-if="!selectedRepository" class="flex items-center justify-center h-96">
            <div class="text-center text-muted-foreground">
              <svg class="w-16 h-16 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"/>
              </svg>
              <p class="text-lg font-medium">选择仓库</p>
              <p class="text-sm">从左侧列表中选择一个仓库来查看详情</p>
            </div>
          </div>

          <div v-else>
            <Tabs v-model="activeTab" class="w-full">
              <TabsList class="grid w-full grid-cols-4">
                <TabsTrigger value="overview">概览</TabsTrigger>
                <TabsTrigger value="commits">提交</TabsTrigger>
                <TabsTrigger value="sync">同步</TabsTrigger>
                <TabsTrigger value="branches">分支</TabsTrigger>
              </TabsList>

              <TabsContent value="overview" class="mt-6">
                <Card>
                  <CardHeader>
                    <CardTitle class="flex items-center justify-between">
                      <span>{{ selectedRepository.name }}</span>
                      <div class="flex items-center space-x-2">
                        <Badge :variant="getStatusBadgeVariant(selectedRepository.status)">
                          {{ getStatusText(selectedRepository.status) }}
                        </Badge>
                        <Button
                          v-if="selectedRepository.status === 'conflict'"
                          variant="destructive"
                          size="sm"
                          @click="showConflictDialog = true"
                        >
                          解决冲突
                        </Button>
                      </div>
                    </CardTitle>
                  </CardHeader>
                  <CardContent class="space-y-4">
                    <div class="grid grid-cols-2 gap-4 text-sm">
                      <div>
                        <span class="font-medium">路径：</span>
                        <span class="text-muted-foreground">{{ selectedRepository.path }}</span>
                      </div>
                      <div>
                        <span class="font-medium">当前分支：</span>
                        <span class="text-muted-foreground">{{ selectedRepository.currentBranch }}</span>
                      </div>
                      <div v-if="selectedRepository.remoteUrl">
                        <span class="font-medium">远程地址：</span>
                        <span class="text-muted-foreground">{{ selectedRepository.remoteUrl }}</span>
                      </div>
                      <div>
                        <span class="font-medium">最后更新：</span>
                        <span class="text-muted-foreground">{{ formatDate(selectedRepository.updatedAt) }}</span>
                      </div>
                    </div>
                    
                    <div v-if="selectedRepository.ahead > 0 || selectedRepository.behind > 0" class="flex items-center space-x-4 p-3 bg-muted rounded-lg">
                      <div v-if="selectedRepository.ahead > 0" class="flex items-center space-x-2 text-blue-600">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"/>
                        </svg>
                        <span>领先 {{ selectedRepository.ahead }} 个提交</span>
                      </div>
                      <div v-if="selectedRepository.behind > 0" class="flex items-center space-x-2 text-orange-600">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 5l-9-2 9 18 9-18-9 2zm0 0v8"/>
                        </svg>
                        <span>落后 {{ selectedRepository.behind }} 个提交</span>
                      </div>
                    </div>
                  </CardContent>
                </Card>
              </TabsContent>

              <TabsContent value="commits" class="mt-6">
                <CommitManager />
              </TabsContent>

              <TabsContent value="sync" class="mt-6">
                <SyncManager />
              </TabsContent>

              <TabsContent value="branches" class="mt-6">
                <BranchManager />
              </TabsContent>
            </Tabs>
          </div>
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

    <!-- 冲突解决弹窗 -->
    <ConflictDialog
      v-model:open="showConflictDialog"
      :files="conflictFiles"
      @resolve="handleConflictResolve"
      @cancel="handleConflictCancel"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import CommitManager from '@/components/git/CommitManager.vue';
import SyncManager from '@/components/git/SyncManager.vue';
import BranchManager from '@/components/git/BranchManager.vue';
import ConflictDialog from '@/components/git/ui/ConflictDialog.vue';
import type { LocalRepository, ConflictFile } from '@/types/git';

const route = useRoute();
const router = useRouter();

// 组件状态
const repository = ref<LocalRepository | null>(null);
const activeTab = ref('overview');
const showConflictDialog = ref(false);
const isLoading = ref(true);

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

// 模拟仓库数据（实际应该从 API 获取）
const mockRepositories: LocalRepository[] = [
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
];

// 计算属性
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
const loadRepository = async () => {
  const repoId = route.params.id as string;

  // 模拟 API 调用
  await new Promise(resolve => setTimeout(resolve, 500));

  const repo = mockRepositories.find(r => r.id === repoId);
  if (repo) {
    repository.value = repo;
  } else {
    // 仓库不存在，返回列表页
    router.push('/local-repositories');
  }

  isLoading.value = false;
};

const goBack = () => {
  router.push('/local-repositories');
};

const openRepository = () => {
  if (repository.value) {
    // 这里将来会调用 Tauri API 打开文件夹
    console.log('打开仓库:', repository.value.path);
  }
};

const refreshRepository = async () => {
  if (repository.value) {
    // 这里将来会调用 API 刷新仓库状态
    console.log('刷新仓库状态:', repository.value.name);
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
  if (repository.value) {
    repository.value.status = 'dirty';
  }
};

const handleConflictCancel = () => {
  console.log('取消解决冲突');
  showConflictDialog.value = false;
};

// 生命周期
onMounted(() => {
  loadRepository();
});
</script>

<template>
  <div class="min-h-screen bg-background">
    <div class="container mx-auto px-4 py-6 max-w-7xl">
      <!-- 加载状态 -->
      <div v-if="isLoading" class="flex items-center justify-center min-h-[400px]">
        <div class="text-center">
          <svg class="w-8 h-8 mx-auto mb-4 animate-spin" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
          </svg>
          <p class="text-muted-foreground">加载仓库信息...</p>
        </div>
      </div>

      <!-- 仓库详情 -->
      <div v-else-if="repository" class="space-y-6">
        <!-- 面包屑导航和标题 -->
        <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
          <div class="space-y-2">
            <!-- 面包屑导航 -->
            <nav class="flex items-center space-x-2 text-sm text-muted-foreground">
              <button @click="goBack" class="hover:text-foreground transition-colors">
                本地仓库
              </button>
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
              </svg>
              <span class="text-foreground font-medium">{{ repository.name }}</span>
            </nav>
            
            <!-- 页面标题 -->
            <div class="flex items-center space-x-3">
              <h1 class="text-3xl font-bold text-foreground">{{ repository.name }}</h1>
              <Badge :variant="getStatusBadgeVariant(repository.status)">
                {{ getStatusText(repository.status) }}
              </Badge>
            </div>
            <p class="text-muted-foreground">{{ repository.path }}</p>
          </div>
          
          <!-- 操作按钮 -->
          <div class="flex items-center space-x-3">
            <Button variant="outline" @click="refreshRepository">
              <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
              </svg>
              刷新状态
            </Button>
            <Button variant="outline" @click="openRepository">
              <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"/>
              </svg>
              打开文件夹
            </Button>
            <Button
              v-if="repository.status === 'conflict'"
              variant="destructive"
              @click="showConflictDialog = true"
            >
              <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.34 16.5c-.77.833.192 2.5 1.732 2.5z"/>
              </svg>
              解决冲突
            </Button>
          </div>
        </div>

        <!-- 快速信息卡片 -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <Card>
            <CardContent class="p-4">
              <div class="flex items-center space-x-3">
                <svg class="w-8 h-8 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16l2.879-2.879m0 0a3 3 0 104.243-4.242 3 3 0 00-4.243 4.242zM21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                </svg>
                <div>
                  <p class="text-sm text-muted-foreground">当前分支</p>
                  <p class="font-semibold">{{ repository.currentBranch }}</p>
                </div>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardContent class="p-4">
              <div class="flex items-center space-x-3">
                <svg class="w-8 h-8 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"/>
                </svg>
                <div>
                  <p class="text-sm text-muted-foreground">领先提交</p>
                  <p class="font-semibold text-blue-600">{{ repository.ahead }}</p>
                </div>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardContent class="p-4">
              <div class="flex items-center space-x-3">
                <svg class="w-8 h-8 text-orange-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 5l-9-2 9 18 9-18-9 2zm0 0v8"/>
                </svg>
                <div>
                  <p class="text-sm text-muted-foreground">落后提交</p>
                  <p class="font-semibold text-orange-600">{{ repository.behind }}</p>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>

        <!-- 功能选项卡 -->
        <Tabs v-model="activeTab" class="w-full" default-value="overview">
          <TabsList class="grid w-full grid-cols-4 mb-6">
            <TabsTrigger value="overview">概览</TabsTrigger>
            <TabsTrigger value="commits">提交管理</TabsTrigger>
            <TabsTrigger value="sync">同步操作</TabsTrigger>
            <TabsTrigger value="branches">分支管理</TabsTrigger>
          </TabsList>

          <TabsContent value="overview" class="space-y-6">
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
              <Card>
                <CardHeader>
                  <CardTitle>基本信息</CardTitle>
                </CardHeader>
                <CardContent class="space-y-4">
                  <div class="grid grid-cols-2 gap-4 text-sm">
                    <div>
                      <span class="text-muted-foreground">仓库名称：</span>
                      <p class="font-medium">{{ repository.name }}</p>
                    </div>
                    <div>
                      <span class="text-muted-foreground">当前分支：</span>
                      <p class="font-medium">{{ repository.currentBranch }}</p>
                    </div>
                    <div class="col-span-2">
                      <span class="text-muted-foreground">本地路径：</span>
                      <p class="font-medium break-all">{{ repository.path }}</p>
                    </div>
                    <div v-if="repository.remoteUrl" class="col-span-2">
                      <span class="text-muted-foreground">远程地址：</span>
                      <p class="font-medium break-all">{{ repository.remoteUrl }}</p>
                    </div>
                    <div>
                      <span class="text-muted-foreground">创建时间：</span>
                      <p class="font-medium">{{ formatDate(repository.createdAt) }}</p>
                    </div>
                    <div>
                      <span class="text-muted-foreground">最后更新：</span>
                      <p class="font-medium">{{ formatDate(repository.updatedAt) }}</p>
                    </div>
                  </div>
                </CardContent>
              </Card>

              <Card>
                <CardHeader>
                  <CardTitle>同步状态</CardTitle>
                </CardHeader>
                <CardContent class="space-y-4">
                  <div class="flex items-center justify-between p-3 bg-muted rounded-lg">
                    <span class="text-sm font-medium">仓库状态</span>
                    <Badge :variant="getStatusBadgeVariant(repository.status)">
                      {{ getStatusText(repository.status) }}
                    </Badge>
                  </div>
                  <div class="flex items-center justify-between p-3 bg-muted rounded-lg">
                    <span class="text-sm font-medium">领先提交</span>
                    <Badge variant="outline" class="text-blue-600">{{ repository.ahead }}</Badge>
                  </div>
                  <div class="flex items-center justify-between p-3 bg-muted rounded-lg">
                    <span class="text-sm font-medium">落后提交</span>
                    <Badge variant="outline" class="text-orange-600">{{ repository.behind }}</Badge>
                  </div>
                </CardContent>
              </Card>
            </div>
          </TabsContent>

          <TabsContent value="commits">
            <CommitManager />
          </TabsContent>

          <TabsContent value="sync">
            <SyncManager />
          </TabsContent>

          <TabsContent value="branches">
            <BranchManager />
          </TabsContent>
        </Tabs>
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

<style scoped>
/* 确保代码查看区域有足够空间 */
:deep(.min-h-\[600px\]) {
  min-height: 600px;
}

/* 优化移动端体验 */
@media (max-width: 768px) {
  :deep(.min-h-\[600px\]) {
    min-height: 400px;
  }
}
</style>

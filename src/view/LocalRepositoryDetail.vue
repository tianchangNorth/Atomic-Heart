<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { useLocalRepositories } from '@/composables/useLocalRepositories';
import type { LocalRepository } from '@/types/local-repository';
import CommitManager from '@/components/git/CommitManager.vue';
import SyncManager from '@/components/git/SyncManager.vue';
import BranchManager from '@/components/git/BranchManager.vue';

const route = useRoute();
const router = useRouter();

// 使用本地仓库管理
const { getRepository, openRepositoryFolder, refreshRepository: refreshRepoStatus } = useLocalRepositories();

// 组件状态
const repository = ref<LocalRepository | null>(null);
const activeTab = ref('overview');
const isLoading = ref(true);

// 计算属性
const getStatusText = (status: LocalRepository['status']) => {
  switch (status) {
    case 'valid': return '有效';
    case 'invalid': return '无效';
    case 'unknown': return '未知';
    default: return '未知';
  }
};

const getStatusBadgeVariant = (status: LocalRepository['status']) => {
  switch (status) {
    case 'valid': return 'default';
    case 'invalid': return 'destructive';
    case 'unknown': return 'secondary';
    default: return 'outline';
  }
};

// 方法
const loadRepository = async () => {
  const repoId = route.params.id as string;

  try {
    // 从本地存储获取仓库信息
    const repo = getRepository(repoId);
    if (repo) {
      repository.value = repo;
    } else {
      // 仓库不存在，返回列表页
      console.warn('仓库不存在:', repoId);
      router.push('/local-repositories');
    }
  } catch (error) {
    console.error('加载仓库失败:', error);
    router.push('/local-repositories');
  } finally {
    isLoading.value = false;
  }
};

const goBack = () => {
  router.push('/local-repositories');
};

const openRepository = async () => {
  if (repository.value) {
    try {
      const result = await openRepositoryFolder(repository.value.id);
      if (result.success) {
        console.log('文件夹已打开:', repository.value.path);
      } else {
        console.error('打开文件夹失败:', result.message);
      }
    } catch (error) {
      console.error('打开仓库文件夹时出错:', error);
    }
  }
};

const refreshRepository = async () => {
  if (repository.value) {
    try {
      console.log('开始刷新仓库状态:', repository.value.name);
      const result = await refreshRepoStatus(repository.value.id);
      if (result.success) {
        console.log('仓库状态刷新成功:', result.message);
        // 重新加载仓库信息以显示最新状态
        await loadRepository();
      } else {
        console.error('刷新仓库状态失败:', result.message);
      }
    } catch (error) {
      console.error('刷新仓库状态时出错:', error);
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

// 生命周期
onMounted(() => {
  loadRepository();
});
</script>

<template>
  <div class="bg-background">
    <div class="mx-auto">
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
            <!-- 冲突解决按钮已移除，因为新的数据结构不包含冲突状态 -->
          </div>
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
                      <span class="text-muted-foreground">添加时间：</span>
                      <p class="font-medium">{{ formatDate(repository.addedAt) }}</p>
                    </div>
                    <div v-if="repository.lastChecked">
                      <span class="text-muted-foreground">最后检查：</span>
                      <p class="font-medium">{{ formatDate(repository.lastChecked) }}</p>
                    </div>
                  </div>
                </CardContent>
              </Card>

              <Card>
                <CardHeader>
                  <CardTitle>仓库信息</CardTitle>
                </CardHeader>
                <CardContent class="space-y-4">
                  <div class="flex items-center justify-between p-3 bg-muted rounded-lg">
                    <span class="text-sm font-medium">仓库状态</span>
                    <Badge :variant="getStatusBadgeVariant(repository.status)">
                      {{ getStatusText(repository.status) }}
                    </Badge>
                  </div>
                  <div v-if="repository.remoteUrl" class="flex items-center justify-between p-3 bg-muted rounded-lg">
                    <span class="text-sm font-medium">远程仓库</span>
                    <Badge variant="outline" class="text-green-600">已配置</Badge>
                  </div>
                  <div v-else class="flex items-center justify-between p-3 bg-muted rounded-lg">
                    <span class="text-sm font-medium">远程仓库</span>
                    <Badge variant="outline" class="text-gray-600">未配置</Badge>
                  </div>
                  <div class="flex items-center justify-between p-3 bg-muted rounded-lg">
                    <span class="text-sm font-medium">当前分支</span>
                    <Badge variant="outline" class="text-blue-600">{{ repository.currentBranch || '未知' }}</Badge>
                  </div>
                </CardContent>
              </Card>
            </div>
          </TabsContent>

          <TabsContent value="commits">
            <CommitManager :repository-path="repository?.path || ''" />
          </TabsContent>

          <TabsContent value="sync">
            <SyncManager :repository-path="repository?.path || ''" />
          </TabsContent>

          <TabsContent value="branches">
            <BranchManager :repo-path="repository?.path || ''" />
          </TabsContent>
        </Tabs>
      </div>
    </div>
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

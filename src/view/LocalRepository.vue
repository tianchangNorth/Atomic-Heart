<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { useLocalRepositories } from '@/composables/useLocalRepositories';
import type { LocalRepository } from '@/types/local-repository';
import { extractRepositoryName } from '@/utils/utils'
import RepoClone from '@/components/git/RepoClone.vue';
import { open } from '@tauri-apps/plugin-dialog';
import { useToast } from '@/components/ui/toast';
import {
  RefreshCw,
  FolderOpen,
  Download,
  Folder,
  CheckCircle,
  AlertTriangle,
  HelpCircle,
  Search,
  Grid3X3,
  List,
  GitBranch,
  ExternalLink,
  Trash2,
  X
} from 'lucide-vue-next';

const router = useRouter();

// 使用本地仓库管理
const {
  repositories,
  removeRepository: removeRepo,
  addRepository,
  loadRepositories,
  refreshRepository: refreshRepoStatus,
  refreshAllRepositories,
  openRepositoryFolder
} = useLocalRepositories();

// 组件状态
const showCloneDialog = ref(false);
const searchQuery = ref('');
const viewMode = ref<'grid' | 'list'>('grid');
const { success } = useToast();

// 计算属性
const repositoryStats = computed(() => {
  const total = repositories.value.length;
  const valid = repositories.value.filter(r => r.status === 'valid').length;
  const invalid = repositories.value.filter(r => r.status === 'invalid').length;
  const unknown = repositories.value.filter(r => r.status === 'unknown').length;

  return { total, valid, invalid, unknown };
});

const filteredRepositories = computed(() => {
  if (!searchQuery.value) return repositories.value;
  return repositories.value.filter(repo =>
    repo.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
    repo.path.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
    (repo.remoteUrl && repo.remoteUrl.toLowerCase().includes(searchQuery.value.toLowerCase()))
  );
});

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
const openRepositoryDetail = (repo: LocalRepository) => {
  // 跳转到本地仓库详情页面
  router.push(`/local-repositories/${repo.id}`);
};

const openRepository = async (repo: LocalRepository) => {
  try {
    const result = await openRepositoryFolder(repo.id);
    if (result.success) {
      console.log('文件夹已打开:', repo.path);
    } else {
      console.error('打开文件夹失败:', result.message);
    }
  } catch (error) {
    console.error('打开仓库文件夹时出错:', error);
  }
};

const handleRemoveRepository = async (repoId: string) => {
  try {
    const result = await removeRepo(repoId);
    if (result.success) {
      console.log('仓库删除成功');
    } else {
      console.error('删除仓库失败:', result.message);
    }
  } catch (error) {
    console.error('删除仓库时出错:', error);
  }
};

const refreshRepository = async (repo: LocalRepository) => {
  try {
    console.log('开始刷新仓库状态:', repo.name);
    const result = await refreshRepoStatus(repo.id);
    if (result.success) {
      console.log('仓库状态刷新成功:', result.message);
    } else {
      console.error('刷新仓库状态失败:', result.message);
    }
  } catch (error) {
    console.error('刷新仓库状态时出错:', error);
  }
};

const handleRefreshAll = async () => {
  try {
    console.log('开始刷新所有仓库状态');
    const result = await refreshAllRepositories();
    if (result.success) {
      console.log('所有仓库状态刷新成功:', result.message);
    } else {
      console.error('刷新所有仓库状态失败:', result.message);
    }
  } catch (error) {
    console.error('刷新所有仓库状态时出错:', error);
  }
};

// 处理克隆成功事件
const handleCloneSuccess = async (result: any) => {
  try {
    if (result.success && result.repository_path) {
      // 从 URL 中提取仓库名称
      const repoUrl = result.repository_url || '';
      const repoName = extractRepositoryName(repoUrl);

      console.log('提取的仓库信息:', {
        name: repoName,
        path: result.repository_path,
        url: repoUrl,
        branch: result.branch
      });

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

// 从路径中提取文件夹名称
const extractFolderName = (path: string): string => {
  try {
    const parts = path.replace(/\\/g, '/').split('/');
    return parts[parts.length - 1] || 'Unknown Repository';
  } catch (error) {
    console.error('提取文件夹名称失败:', error);
    return 'Unknown Repository';
  }
};

// 处理导入本地仓库
const handleImportRepository = async () => {
  try {
    // 使用 Tauri dialog API 选择文件夹
    const selectedPath = await open({
      directory: true,
      multiple: false,
      title: '选择 Git 仓库文件夹'
    });

    if (selectedPath && typeof selectedPath === 'string') {
      // 验证选择的文件夹是否为 Git 仓库
      const repoName = extractFolderName(selectedPath);

      // 添加到本地仓库列表
      const addResult = await addRepository({
        name: repoName,
        path: selectedPath,
        // remoteUrl 和 currentBranch 暂时为空，后续可以通过 API 获取
      });

      if (addResult.success) {
        console.log('本地仓库导入成功:', repoName);
      } else {
        console.error('导入本地仓库失败:', addResult.message);
        // TODO: 显示错误提示给用户
      }
    }
  } catch (error) {
    console.error('导入本地仓库时出错:', error);
    // TODO: 显示错误提示给用户
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

// 组件挂载时加载数据
onMounted(async () => {
  try {
    await loadRepositories();
  } catch (error) {
    console.error('加载本地仓库失败:', error);
  }
});
</script>

<template>
  <div class="bg-background">
      <!-- 页面标题 -->
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-8">
        <div>
          <h1 class="text-3xl font-bold text-foreground">本地仓库管理</h1>
          <p class="text-muted-foreground mt-1">管理您的本地 Git 仓库</p>
        </div>
        <div class="flex items-center space-x-3">
          <Button variant="outline" @click="handleRefreshAll">
            <RefreshCw class="w-4 h-4 mr-2" />
            刷新所有
          </Button>
          <Button variant="outline" @click="handleImportRepository">
            <FolderOpen class="w-4 h-4 mr-2" />
            导入本地仓库
          </Button>
          <Button variant="outline" @click="showCloneDialog = true">
            <Download class="w-4 h-4 mr-2" />
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
              <Folder class="w-8 h-8 text-muted-foreground" />
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent class="p-6">
            <div class="flex items-center justify-between">
              <div>
                <p class="text-sm font-medium text-muted-foreground">有效仓库</p>
                <p class="text-2xl font-bold text-green-600">{{ repositoryStats.valid }}</p>
              </div>
              <CheckCircle class="w-8 h-8 text-green-600" />
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent class="p-6">
            <div class="flex items-center justify-between">
              <div>
                <p class="text-sm font-medium text-muted-foreground">无效仓库</p>
                <p class="text-2xl font-bold text-red-600">{{ repositoryStats.invalid }}</p>
              </div>
              <AlertTriangle class="w-8 h-8 text-red-600" />
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent class="p-6">
            <div class="flex items-center justify-between">
              <div>
                <p class="text-sm font-medium text-muted-foreground">未知状态</p>
                <p class="text-2xl font-bold text-yellow-600">{{ repositoryStats.unknown }}</p>
              </div>
              <HelpCircle class="w-8 h-8 text-yellow-600" />
            </div>
          </CardContent>
        </Card>
      </div>

      <!-- 搜索和视图控制 -->
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-6">
        <div class="flex-1 max-w-md">
          <div class="relative">
            <Search class="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-muted-foreground" />
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
            <Grid3X3 class="w-4 h-4" />
          </Button>
          <Button
            :variant="viewMode === 'list' ? 'default' : 'outline'"
            size="sm"
            @click="viewMode = 'list'"
          >
            <List class="w-4 h-4" />
          </Button>
        </div>
      </div>

      <!-- 仓库列表 -->
      <div v-if="filteredRepositories.length === 0 && searchQuery" class="text-center py-12">
        <Search class="w-16 h-16 mx-auto mb-4 text-muted-foreground" />
        <h3 class="text-lg font-medium text-foreground mb-2">未找到匹配的仓库</h3>
        <p class="text-muted-foreground">尝试使用不同的关键词搜索</p>
      </div>

      <div v-else-if="filteredRepositories.length === 0" class="text-center py-12">
        <Folder class="w-16 h-16 mx-auto mb-4 text-muted-foreground" />
        <h3 class="text-lg font-medium text-foreground mb-2">暂无本地仓库</h3>
        <p class="text-muted-foreground mb-4">开始克隆您的第一个仓库</p>
        <Button @click="showCloneDialog = true">
          <Download class="w-4 h-4 mr-2" />
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
              <GitBranch class="w-4 h-4 flex-shrink-0" />
              <span class="font-medium">{{ repo.currentBranch }}</span>
              <!-- Git 状态信息暂时不可用 -->
            </div>

            <div class="flex items-center space-x-2 text-sm text-muted-foreground">
              <Folder class="w-4 h-4 flex-shrink-0" />
              <span class="truncate">{{ repo.path }}</span>
            </div>

            <div class="flex items-center justify-between pt-2 border-t border-border">
              <span class="text-xs text-muted-foreground">{{ formatDate(repo.addedAt) }}</span>
              <div class="flex space-x-1">
                <Button
                  variant="ghost"
                  size="sm"
                  @click.stop="refreshRepository(repo)"
                  class="h-7 w-7 p-0"
                  title="刷新状态"
                >
                  <RefreshCw class="w-3 h-3" />
                </Button>
                <Button
                  variant="ghost"
                  size="sm"
                  @click.stop="openRepository(repo)"
                  class="h-7 w-7 p-0"
                  title="在文件管理器中打开"
                >
                  <ExternalLink class="w-3 h-3" />
                </Button>
                <Button
                  variant="ghost"
                  size="sm"
                  @click.stop="handleRemoveRepository(repo.id)"
                  class="h-7 w-7 p-0 text-red-600 hover:text-red-700"
                  title="移除仓库"
                >
                  <Trash2 class="w-3 h-3" />
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
                    <GitBranch class="w-4 h-4 text-muted-foreground" />
                    <span>{{ repo.currentBranch }}</span>
                  </div>
                  <!-- Git 状态信息暂时不可用 -->
                  <Badge :variant="getStatusBadgeVariant(repo.status)">
                    {{ getStatusText(repo.status) }}
                  </Badge>
                  <span class="text-xs text-muted-foreground">{{ formatDate(repo.addedAt) }}</span>
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
                  <RefreshCw class="w-4 h-4" />
                </Button>
                <Button
                  variant="ghost"
                  size="sm"
                  @click.stop="openRepository(repo)"
                  class="h-8 w-8 p-0"
                  title="在文件管理器中打开"
                >
                  <ExternalLink class="w-4 h-4" />
                </Button>
                <Button
                  variant="ghost"
                  size="sm"
                  @click.stop="handleRemoveRepository(repo.id)"
                  class="h-8 w-8 p-0 text-red-600 hover:text-red-700"
                  title="移除仓库"
                >
                  <Trash2 class="w-4 h-4" />
                </Button>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>

    <!-- 克隆仓库弹窗 -->
    <div v-if="showCloneDialog" class="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div class="w-full max-w-4xl max-h-[90vh] overflow-y-auto scrollbar-hide">
        <div class="relative">
          <Button
            variant="ghost"
            class="absolute top-4 right-32 z-10 cursor-pointer"
            @click="showCloneDialog = false"
          >
            <X class="w-4 h-4" />
          </Button>
          <RepoClone @cloneSuccess="handleCloneSuccess" />
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

/* 全局样式中添加（例如 Tailwind 的 utilities.css 或 index.css） */
.scrollbar-hide::-webkit-scrollbar {
  display: none;
}

.scrollbar-hide {
  -ms-overflow-style: none;
  /* IE/Edge */
  scrollbar-width: none;
  /* Firefox */
}
</style>

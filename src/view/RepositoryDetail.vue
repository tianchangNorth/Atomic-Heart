<script setup lang="ts">
import { onMounted, onUnmounted, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { storeToRefs } from 'pinia';
import { useRepositoryStore } from '@/stores/repository';
import { Button } from '@/components/ui/button';
import RepoInfo from '@/components/repository/RepoInfo.vue';
import BranchSelector from '@/components/repository/BranchSelector.vue';
import FileTree from '@/components/repository/FileTree.vue';
import ReadmeViewer from '@/components/repository/ReadmeViewer.vue';

const route = useRoute();
const router = useRouter();
const repositoryStore = useRepositoryStore();

// 使用 storeToRefs 来保持响应性
const {
  currentRepository,
  repositoryStats,
  branches,
  currentBranch,
  fileTree,
  currentPath,
  readme,
  languages,
  loading,
  error
} = storeToRefs(repositoryStore);

// 从路由参数获取仓库信息
const owner = route.params.owner as string;
const repo = route.params.repo as string;

// 处理分支切换
const handleBranchChange = async (event: { branch: string; commit: string }) => {
  await repositoryStore.switchBranch(event.branch, owner, repo);
};

// 处理文件选择
const handleFileSelect = (file: any) => {
  // 这里可以实现文件预览功能
  console.log('选择文件:', file);
};

// 处理目录切换
const handleDirectoryToggle = async (directory: any) => {
  await repositoryStore.toggleDirectory(directory, owner, repo);
};

// 处理错误重试
const handleRetry = () => {
  repositoryStore.fetchRepositoryData(owner, repo);
};

// 处理返回操作
const handleGoBack = () => {
  // 优先返回到仓库列表页面，如果没有历史记录则使用 push
  if (window.history.length > 1) {
    router.back();
  } else {
    router.push('/repositories');
  }
};

function getLanguageColor(language: string): string {
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
    CSS: '#1572b6',
    'C++': '#f34b7d',
    'C#': '#239120',
    C: '#555555'
  };
  return colors[language] || '#6b7280';
}

// 格式化文件大小的辅助函数
function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B';

  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
}

// 监听路由参数变化
watch(
  () => [route.params.owner, route.params.repo],
  ([newOwner, newRepo]) => {
    if (newOwner && newRepo) {
      repositoryStore.fetchRepositoryData(newOwner as string, newRepo as string);
    }
  }
);

// 组件挂载时获取数据
onMounted(() => {
  if (owner && repo) {
    repositoryStore.fetchRepositoryData(owner, repo);
  }
});

// 组件卸载时清理状态
onUnmounted(() => {
  repositoryStore.resetState();
});
</script>

<template>
  <div class="min-h-screen bg-background">
    <!-- 加载状态 -->
    <div v-if="loading" class="flex items-center justify-center py-20">
      <div class="text-center">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mx-auto mb-4"></div>
        <p class="text-muted-foreground">正在加载仓库信息...</p>
      </div>
    </div>

    <!-- 错误状态 -->
    <div v-else-if="error" class="flex items-center justify-center py-20">
      <div class="text-center max-w-md">
        <div class="text-muted-foreground mb-6">
          <svg class="w-16 h-16 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.34 16.5c-.77.833.192 2.5 1.732 2.5z"/>
          </svg>
          <h3 class="text-lg font-semibold text-foreground mb-2">加载失败</h3>
          <p class="text-sm">{{ error }}</p>
        </div>
        <div class="space-x-3">
          <button
            @click="handleRetry"
            class="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors"
          >
            重试
          </button>
          <button
            @click="handleGoBack"
            class="px-4 py-2 border border-border rounded-md hover:bg-accent transition-colors"
          >
            返回
          </button>
        </div>
      </div>
    </div>

    <!-- 仓库详情内容 -->
    <div v-else-if="currentRepository">
      <!-- 顶部导航栏 -->
      <div class="sticky top-0 z-10 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 border-b border-border">
        <div class="container mx-auto px-4 py-4 max-w-7xl">
          <!-- 面包屑导航 -->
          <nav class="breadcrumb-nav flex items-center space-x-3 text-sm py-2" aria-label="面包屑导航">
            <div class="flex items-center space-x-2">
              <svg class="w-4 h-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"/>
              </svg>
              <router-link
                to="/repositories"
                class="breadcrumb-link text-primary hover:text-primary/80 font-medium transition-colors"
              >
                仓库列表
              </router-link>
            </div>
            <svg class="w-4 h-4 text-muted-foreground/60 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
            </svg>
            <span class="text-foreground font-semibold truncate">{{ currentRepository.name }}</span>
          </nav>

          <!-- 返回按钮和页面标题 -->
          <div class="flex flex-col sm:flex-row sm:items-center gap-4">

            <!-- 页面标题 -->
            <div class="flex-1 min-w-0">
              <h1 class="text-xl sm:text-2xl lg:text-3xl font-bold text-foreground truncate">
                {{ currentRepository.name }}
              </h1>
            </div>
          </div>
        </div>
      </div>

      <!-- 主要内容区域 -->
      <div class="container mx-auto px-4 py-6 max-w-7xl">
        <!-- 仓库基本信息 -->
        <div class="mb-8">
          <RepoInfo
            :repository="currentRepository"
            :stats="repositoryStats"
            :languages="languages"
            :contributors="[]"
          />
        </div>

      <!-- 分支选择器 -->
      <div class="mb-6">
        <BranchSelector
          :branches="branches"
          :current-branch="currentBranch"
          @branch-change="handleBranchChange"
        />
      </div>

      <!-- 主要内容区域 -->
      <div class="">
        <!-- 文件树 -->
        <div class="lg:col-span-2">
          <div class="bg-card rounded-lg border shadow-sm">
            <div class="border-b border-border px-4 py-3">
              <h3 class="font-semibold text-foreground">文件浏览器</h3>
            </div>
            <div class="p-4">
              <FileTree
                :items="fileTree"
                :current-path="currentPath"
                :loading="repositoryStore.fileTreeLoading"
                @file-select="handleFileSelect"
                @directory-toggle="handleDirectoryToggle"
              />
            </div>
          </div>
        </div>

        <!-- README 和其他信息 -->
        <div class="mt-6">
          <!-- README 文档 -->
          <div class="bg-card rounded-lg border shadow-sm">
            <div class="border-b border-border px-4 py-3">
              <h3 class="font-semibold text-foreground">README</h3>
            </div>
            <div class="p-4">
              <ReadmeViewer
                :readme="readme"
                :loading="repositoryStore.readmeLoading"
              />
            </div>
          </div>

          <!-- 语言统计 -->
          <div v-if="repositoryStore.languageStats.length > 0" class="bg-card rounded-lg border shadow-sm">
            <div class="border-b border-border px-4 py-3">
              <h3 class="font-semibold text-foreground">语言统计</h3>
            </div>
            <div class="p-4">
              <div class="space-y-3">
                <div 
                  v-for="lang in repositoryStore.languageStats.slice(0, 5)" 
                  :key="lang.language"
                  class="flex items-center justify-between"
                >
                  <div class="flex items-center space-x-2">
                    <div 
                      class="w-3 h-3 rounded-full"
                      :style="{ backgroundColor: getLanguageColor(lang.language) }"
                    ></div>
                    <span class="text-sm font-medium">{{ lang.language }}</span>
                  </div>
                  <span class="text-sm text-muted-foreground">{{ lang.percentage }}%</span>
                </div>
              </div>
            </div>
          </div>

          <!-- 仓库统计 -->
          <div v-if="repositoryStats" class="bg-card rounded-lg border shadow-sm">
            <div class="border-b border-border px-4 py-3">
              <h3 class="font-semibold text-foreground">统计信息</h3>
            </div>
            <div class="p-4">
              <div class="grid grid-cols-2 gap-4 text-sm">
                <div class="text-center">
                  <div class="font-semibold text-lg">{{ repositoryStats.commits_count || 0 }}</div>
                  <div class="text-muted-foreground">提交</div>
                </div>
                <div class="text-center">
                  <div class="font-semibold text-lg">{{ repositoryStats.branches_count || 0 }}</div>
                  <div class="text-muted-foreground">分支</div>
                </div>
                <div class="text-center">
                  <div class="font-semibold text-lg">{{ repositoryStats.files_count || 0 }}</div>
                  <div class="text-muted-foreground">文件</div>
                </div>
                <div class="text-center">
                  <div class="font-semibold text-lg">{{ formatFileSize(repositoryStats.total_size || 0) }}</div>
                  <div class="text-muted-foreground">大小</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      </div>
    </div>

    <!-- 空状态 -->
    <div v-else class="flex items-center justify-center py-20">
      <div class="text-center max-w-md">
        <svg class="w-16 h-16 mx-auto mb-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"/>
        </svg>
        <h3 class="text-lg font-semibold text-foreground mb-2">仓库不存在</h3>
        <p class="text-sm text-muted-foreground mb-4">
          请检查仓库地址是否正确，或者您是否有访问权限。
        </p>
        <Button
          @click="handleGoBack"
          variant="outline"
          class="back-button"
        >
          <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
          </svg>
          返回
        </Button>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 容器样式 */
.container {
  width: 100%;
  margin-left: auto;
  margin-right: auto;
  padding-left: 1rem;
  padding-right: 1rem;
}

@media (min-width: 640px) {
  .container {
    max-width: 640px;
  }
}

@media (min-width: 768px) {
  .container {
    max-width: 768px;
  }
}

@media (min-width: 1024px) {
  .container {
    max-width: 1024px;
  }
}

@media (min-width: 1280px) {
  .container {
    max-width: 1280px;
  }
}

@media (min-width: 1536px) {
  .container {
    max-width: 1536px;
  }
}
</style>

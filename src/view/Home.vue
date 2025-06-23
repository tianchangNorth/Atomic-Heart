<script setup lang="ts">
import { onMounted, ref, computed } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { useUserStore } from '@/stores/index';
import { Button } from '@/components/ui/button';
import { delToken } from '@/utils/token';
import NotificationPanel from '@/components/ui/notification/NotificationPanel.vue';
import { initializeNotifications, unreadCount } from '@/services/notificationService';
import RepoClone from '@/components/git/RepoClone.vue';
import { useLocalRepositories } from '@/composables/useLocalRepositories';
import { extractRepositoryName } from '@/utils/utils';
import { invoke } from '@tauri-apps/api/core';
import { useToast } from '@/components/ui/toast';
import {
  Plus,
  Bell,
  User,
  LogOut,
  BarChart3,
  Globe,
  AlertCircle,
  ClipboardList,
  Folder,
  Settings,
  Download,
  X
} from 'lucide-vue-next';

const router = useRouter();
const route = useRoute();
const userStore = useUserStore();
const { success } = useToast();

// 响应式数据
const { user } = userStore;
const { addRepository } = useLocalRepositories();
const showNotifications = ref(false);
const showCloneDialog = ref(false);

// 导航项配置
const navigationItems = computed(() => [
  {
    path: '/overview',
    label: '概览',
    badge: null,
    icon: 'dashboard' // 仪表盘图标
  },
  {
    path: '/repositories',
    label: '仓库',
    badge: user.total_repos || 0,
    icon: 'repository' // 远程仓库图标
  },
  {
    path: '/issues',
    label: 'Issues',
    badge: null,
    icon: 'issues' // 问题/Bug图标
  },
  {
    path: '/local-repositories',
    label: '本地仓库',
    badge: null,
    icon: 'local-repository' // 本地文件夹图标
  },
  {
    path: '/settings',
    label: '设置',
    badge: null,
    icon: 'settings' // 设置齿轮图标
  }
]);

// 消息通知面板控制
const toggleNotificationPanel = () => {
  showNotifications.value = !showNotifications.value;
};

const hideNotificationPanel = () => {
  showNotifications.value = false;
};

// Tab导航相关方法
const navigateToTab = (path: string) => {
  router.push(path);
};

const getNavItemClass = (path: string) => {
  const isActive = route.path === path;
  return [
    isActive
      ? 'bg-primary text-primary-foreground'
      : 'text-muted-foreground hover:text-foreground hover:bg-muted'
  ];
};

const handleCreateRepo = () => {
  invoke('open_url', { url: 'https://atomgit.com/project/new' });
};

// 克隆仓库相关方法
const handleCloneRepo = () => {
  showCloneDialog.value = true;
};

const handleCloneSuccess = async (result: any) => {
  try {
    if (result.success && result.repository_path) {
      const repoUrl = result.repository_url || '';
      const repoName = extractRepositoryName(repoUrl);

      await addRepository({
        name: repoName,
        path: result.repository_path,
        remoteUrl: repoUrl || undefined,
        currentBranch: result.branch || undefined
      });
      showCloneDialog.value = false;
      success('仓库克隆成功');
      router.push('/local-repositories');
    }
  } catch (error) {
    console.error('处理克隆成功事件时出错:', error);
  }
};

// 退出登录
const logout = () => {
  delToken();
  userStore.resetInfo();
  router.push('/login');
};

// 组件挂载时获取用户信息
onMounted(async () => {
  if (!user.id) {
    await userStore.fetchInfo();
  }
  // 初始化通知服务
  initializeNotifications();
});
</script>

<template>
  <div class="h-screen bg-background flex flex-col overflow-hidden">
    <!-- 顶部导航栏 -->
    <header class="bg-card border-b border-border shadow-sm">
      <div class="px-6">
        <div class="flex justify-between items-center h-16">
          <!-- 左侧：品牌标识和搜索 -->
          <div class="flex items-center space-x-8">
            <div class="flex items-center space-x-3">
              <!-- AtomGit Logo -->
              <div class="w-8 h-8 bg-gray-50 dark:bg-gray-800 rounded-lg flex items-center justify-center">
                <img src="@/assets/icon.png" class="w-6 h-6">
              </div>
              <h1 class="text-xl font-bold text-foreground">Atomic Heart</h1>
            </div>
          </div>

          <!-- 右侧：用户操作区域 -->
          <div class="flex items-center space-x-4">
            <!-- 新建按钮 -->
            <Button variant="default" size="sm" class="hidden md:flex" @click="handleCreateRepo">
              <Plus class="w-4 h-4 mr-2" />
              新建
            </Button>

            <!-- 通知按钮 -->
            <button
              @click="toggleNotificationPanel"
              class="relative p-2 text-muted-foreground hover:text-foreground hover:bg-muted rounded-lg transition-colors"
              :title="showNotifications ? '隐藏通知' : '显示通知'"
            >
              <Bell class="w-5 h-5" />
              <span
                v-if="unreadCount > 0"
                class="absolute -top-1 -right-1 bg-yellow-500 text-destructive-foreground text-xs rounded-full h-5 w-5 flex items-center justify-center font-medium"
              >
                {{ unreadCount > 99 ? '99+' : unreadCount }}
              </span>
            </button>

            <!-- 用户菜单 -->
            <div class="flex items-center space-x-3">
              <img
                v-if="user.avatar_url"
                :src="user.avatar_url"
                :alt="user.name || '用户头像'"
                class="w-8 h-8 rounded-full border border-border"
              >
              <div v-else class="w-8 h-8 bg-muted rounded-full flex items-center justify-center">
                <User class="w-4 h-4 text-muted-foreground" />
              </div>
              <div class="hidden md:block">
                <p class="text-sm font-medium text-foreground">{{ user.name || '用户' }}</p>
                <p class="text-xs text-muted-foreground">{{ user.email || '' }}</p>
              </div>
              <button @click="logout" class="text-muted-foreground hover:text-destructive transition-colors">
                <LogOut class="w-4 h-4" />
              </button>
            </div>
          </div>
        </div>
      </div>
    </header>

    <!-- 主体布局 -->
    <div class="flex-1 flex overflow-hidden">
      <!-- 侧边栏导航 -->
      <aside class="w-64 bg-card border-r border-border flex-shrink-0">
        <div class="p-6">
          <!-- 用户信息简要展示 -->
          <div class="mb-8">
            <div class="flex items-center space-x-3 mb-4">
              <img
                v-if="user.avatar_url"
                :src="user.avatar_url"
                :alt="user.name || '用户头像'"
                class="w-12 h-12 rounded-full border border-border"
              >
              <div v-else class="w-12 h-12 bg-muted rounded-full flex items-center justify-center">
                <User class="w-6 h-6 text-muted-foreground" />
              </div>
              <div class="flex-1 min-w-0">
                <h3 class="font-semibold text-foreground truncate">{{ user.name || '用户' }}</h3>
                <p class="text-sm text-muted-foreground truncate">{{ user.bio || '暂无简介' }}</p>
              </div>
            </div>

            <!-- 用户统计 -->
            <div class="grid grid-cols-3 gap-4 text-center">
              <div>
                <div class="text-lg font-bold text-foreground">{{ user.total_repos || 0 }}</div>
                <div class="text-xs text-muted-foreground">仓库</div>
              </div>
              <div>
                <div class="text-lg font-bold text-foreground">{{ user.followers || 0 }}</div>
                <div class="text-xs text-muted-foreground">关注者</div>
              </div>
              <div>
                <div class="text-lg font-bold text-foreground">{{ user.following || 0 }}</div>
                <div class="text-xs text-muted-foreground">关注中</div>
              </div>
            </div>
          </div>

          <!-- 导航菜单 -->
          <nav class="space-y-2">
            <button
              v-for="item in navigationItems"
              :key="item.path"
              @click="navigateToTab(item.path)"
              :class="getNavItemClass(item.path)"
              class="w-full flex items-center space-x-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors"
            >
              <!-- 概览图标 -->
              <BarChart3 v-if="item.path === '/overview'" class="w-5 h-5" />
              <!-- 远程仓库图标 -->
              <Globe v-else-if="item.path === '/repositories'" class="w-5 h-5" />
              <!-- Issues图标 -->
              <AlertCircle v-else-if="item.path === '/issues'" class="w-5 h-5" />
              <!-- 项目管理图标 -->
              <ClipboardList v-else-if="item.path === '/projects'" class="w-5 h-5" />
              <!-- 本地仓库图标 -->
              <Folder v-else-if="item.path === '/local-repositories'" class="w-5 h-5" />
              <!-- 设置图标 -->
              <Settings v-else-if="item.path === '/settings'" class="w-5 h-5" />

              <span>{{ item.label }}</span>
              <span v-if="item.badge" class="ml-auto bg-primary text-primary-foreground text-xs px-2 py-1 rounded-full">
                {{ item.badge }}
              </span>
            </button>
          </nav>

          <!-- 快速操作 -->
          <div class="mt-8 space-y-3">
            <h4 class="text-sm font-semibold text-muted-foreground uppercase tracking-wider">快速操作</h4>
            <Button variant="outline" size="sm" class="w-full justify-start cursor-pointer" @click="handleCreateRepo()">
              <Plus class="w-4 h-4 mr-2" />
              新建仓库
            </Button>
            <Button variant="outline" size="sm" class="w-full justify-start cursor-pointer" @click="handleCloneRepo">
              <Download class="w-4 h-4 mr-2" />
              克隆仓库
            </Button>
          </div>
        </div>
      </aside>

      <!-- 主内容区域 -->
      <main class="flex-1 flex overflow-hidden">
        <!-- 内容容器 -->
        <div class="flex-1 bg-card rounded-lg border border-border overflow-hidden flex flex-col m-6">
          <!-- 路由视图容器 -->
          <div class="flex-1 overflow-y-auto">
            <div class="p-6">
              <router-view />
            </div>
          </div>
        </div>
      </main>

      <!-- 右侧：消息通知面板 -->
      <Transition
        enter-active-class="transition-all duration-300 ease-out"
        enter-from-class="transform translate-x-full opacity-0"
        enter-to-class="transform translate-x-0 opacity-100"
        leave-active-class="transition-all duration-300 ease-in"
        leave-from-class="transform translate-x-0 opacity-100"
        leave-to-class="transform translate-x-full opacity-0"
      >
        <div v-show="showNotifications" class="flex-shrink-0 w-80 mt-6 mr-6">
          <NotificationPanel
            @close="hideNotificationPanel"
          />
        </div>
      </Transition>
    </div>

    <!-- 克隆仓库对话框 -->
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
  </div>
</template>

<style scoped>
/* 自定义滚动条样式 */
.overflow-y-auto::-webkit-scrollbar {
  width: 6px;
}

.overflow-y-auto::-webkit-scrollbar-track {
  background: hsl(var(--muted));
  border-radius: 3px;
}

.overflow-y-auto::-webkit-scrollbar-thumb {
  background: hsl(var(--muted-foreground) / 0.3);
  border-radius: 3px;
}

.overflow-y-auto::-webkit-scrollbar-thumb:hover {
  background: hsl(var(--muted-foreground) / 0.5);
}
</style>
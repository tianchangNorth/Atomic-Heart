<script setup lang="ts">
import { onMounted, ref, computed } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { useUserStore } from '@/stores/index';
import { Button } from '@/components/ui/button';
import { delToken } from '@/utils/token';
import NotificationPanel from '@/components/ui/notification/NotificationPanel.vue';
import { initializeNotifications, unreadCount } from '@/services/notificationService';

const router = useRouter();
const route = useRoute();
const userStore = useUserStore();

// 响应式数据
const { user } = userStore;
const showNotifications = ref(false);

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
  // {
  //   path: '/projects',
  //   label: '项目',
  //   badge: null,
  //   icon: 'projects' // 项目管理图标
  // },
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
              <div class="w-8 h-8 bg-primary rounded-lg flex items-center justify-center">
                <svg class="w-5 h-5 text-primary-foreground" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
                </svg>
              </div>
              <h1 class="text-xl font-bold text-foreground">AtomGit</h1>
            </div>

            <!-- 快速搜索 -->
            <div class="hidden md:flex items-center">
              <div class="relative">
                <input
                  type="text"
                  placeholder="搜索仓库、用户..."
                  class="w-80 px-4 py-2 pl-10 bg-muted border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-ring focus:border-transparent"
                >
                <svg class="absolute left-3 top-2.5 w-4 h-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                </svg>
              </div>
            </div>
          </div>

          <!-- 右侧：用户操作区域 -->
          <div class="flex items-center space-x-4">
            <!-- 新建按钮 -->
            <Button variant="default" size="sm" class="hidden md:flex">
              <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
              </svg>
              新建
            </Button>

            <!-- 通知按钮 -->
            <button
              @click="toggleNotificationPanel"
              class="relative p-2 text-muted-foreground hover:text-foreground hover:bg-muted rounded-lg transition-colors"
              :title="showNotifications ? '隐藏通知' : '显示通知'"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 2a6 6 0 00-6 6v3.586l-.707.707A1 1 0 004 14h12a1 1 0 00.707-1.707L16 11.586V8a6 6 0 00-6-6zM10 18a3 3 0 01-3-3h6a3 3 0 01-3 3z"/>
              </svg>
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
                <svg class="w-4 h-4 text-muted-foreground" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd"/>
                </svg>
              </div>
              <div class="hidden md:block">
                <p class="text-sm font-medium text-foreground">{{ user.name || '用户' }}</p>
                <p class="text-xs text-muted-foreground">{{ user.email || '' }}</p>
              </div>
              <button @click="logout" class="text-muted-foreground hover:text-destructive transition-colors">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"/>
                </svg>
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
                <svg class="w-6 h-6 text-muted-foreground" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd"/>
                </svg>
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
              <svg v-if="item.path === '/overview'" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/>
              </svg>
              <!-- 远程仓库图标 -->
              <svg v-else-if="item.path === '/repositories'" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
              </svg>
              <!-- Issues图标 -->
              <svg v-else-if="item.path === '/issues'" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
              </svg>
              <!-- 项目管理图标 -->
              <svg v-else-if="item.path === '/projects'" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01"/>
              </svg>
              <!-- 本地仓库图标 -->
              <svg v-else-if="item.path === '/local-repositories'" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"/>
              </svg>
              <!-- 设置图标 -->
              <svg v-else-if="item.path === '/settings'" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
              </svg>

              <span>{{ item.label }}</span>
              <span v-if="item.badge" class="ml-auto bg-primary text-primary-foreground text-xs px-2 py-1 rounded-full">
                {{ item.badge }}
              </span>
            </button>
          </nav>

          <!-- 快速操作 -->
          <div class="mt-8 space-y-3">
            <h4 class="text-sm font-semibold text-muted-foreground uppercase tracking-wider">快速操作</h4>
            <Button variant="outline" size="sm" class="w-full justify-start">
              <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
              </svg>
              新建仓库
            </Button>
            <Button variant="outline" size="sm" class="w-full justify-start">
              <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/>
              </svg>
              加入组织
            </Button>
            <Button variant="outline" size="sm" class="w-full justify-start">
              <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10"/>
              </svg>
              导入仓库
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
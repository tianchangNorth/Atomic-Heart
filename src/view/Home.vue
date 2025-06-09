<template>
  <div class="h-screen bg-gradient-to-br from-blue-50 to-indigo-100 overflow-hidden">
    <!-- 顶部导航栏 -->
    <header class="bg-white/80 backdrop-blur-sm shadow-sm border-b border-gray-200">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between items-center h-16">
          <div class="flex items-center">
            <h1 class="text-xl font-semibold text-gray-900">开发者工作台</h1>
          </div>
          <div class="flex items-center space-x-4">
            <!-- 消息通知切换按钮 -->
            <button 
              @click="toggleNotificationPanel"
              class="relative p-2 text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded-lg transition-colors"
              :title="showNotifications ? '隐藏通知' : '显示通知'"
            >
              <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20">
                <path d="M10 2a6 6 0 00-6 6v3.586l-.707.707A1 1 0 004 14h12a1 1 0 00.707-1.707L16 11.586V8a6 6 0 00-6-6zM10 18a3 3 0 01-3-3h6a3 3 0 01-3 3z"/>
              </svg>
              <!-- 未读消息红点提示 -->
              <span 
                v-if="unreadNotificationCount > 0" 
                class="absolute -top-1 -right-1 bg-red-500 text-white text-xs rounded-full h-5 w-5 flex items-center justify-center font-medium"
              >
                {{ unreadNotificationCount > 99 ? '99+' : unreadNotificationCount }}
              </span>
            </button>
            
            <span class="text-sm text-gray-700">欢迎，{{ user.name || '用户' }}</span>
            <button @click="logout" class="text-sm text-red-600 hover:text-red-800 transition-colors">
              退出登录
            </button>
          </div>
        </div>
      </div>
    </header>

    <div class="flex-1 flex max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8 h-[calc(100vh-4rem)]" :class="showNotifications ? 'space-x-4' : 'space-x-8'">
      <!-- 左侧：用户资料和Tab内容 -->
      <div class="flex-1 flex flex-col min-w-0">
        <!-- 用户资料卡片 -->
        <div class="flex-shrink-0 bg-white rounded-2xl shadow-xl border border-gray-100 mb-8" :class="showNotifications ? 'p-6' : 'p-8'">
          <div class="flex flex-col" :class="showNotifications ? 'xl:flex-row xl:space-y-0 xl:space-x-6 space-y-6' : 'lg:flex-row lg:space-y-0 lg:space-x-12 space-y-8'">
            <!-- 头像和基本信息 -->
            <div class="flex items-center" :class="showNotifications ? 'space-x-4' : 'space-x-6'">
              <div class="relative">
                <img 
                  v-if="user.avatar_url" 
                  :src="user.avatar_url" 
                  :alt="user.name || '用户头像'"
                  :class="showNotifications ? 'w-16 h-16' : 'w-24 h-24'"
                  class="rounded-full object-cover ring-4 ring-blue-100 transition-all duration-300"
                />
                <div 
                  v-else
                  :class="showNotifications ? 'w-16 h-16' : 'w-24 h-24'"
                  class="bg-gradient-to-br from-blue-500 to-purple-600 rounded-full flex items-center justify-center ring-4 ring-blue-100 transition-all duration-300"
                >
                  <span class="text-white font-bold" :class="showNotifications ? 'text-lg' : 'text-2xl'">
                    {{ (user.name || 'U').charAt(0).toUpperCase() }}
                  </span>
                </div>
                <div class="absolute -bottom-1 -right-1 bg-green-500 rounded-full border-2 border-white transition-all duration-300" :class="showNotifications ? 'w-4 h-4' : 'w-6 h-6'"></div>
              </div>
              
              <div class="flex-1 min-w-0">
                <h2 class="font-bold text-gray-900 mb-2 truncate transition-all duration-300" :class="showNotifications ? 'text-xl' : 'text-3xl'">
                  {{ user.name || '未知用户' }}
                </h2>
                <p class="text-gray-600 mb-3 line-clamp-2 transition-all duration-300" :class="showNotifications ? 'text-sm' : 'text-lg'">
                  {{ user.bio || '这个人很懒，什么都没有留下...' }}
                </p>
                <div class="flex flex-wrap text-gray-500 transition-all duration-300" :class="showNotifications ? 'gap-2 text-xs' : 'gap-4 text-sm'">
                  <span v-if="user.followers !== undefined" class="flex items-center">
                    <svg :class="showNotifications ? 'w-3 h-3 mr-1' : 'w-4 h-4 mr-1'" fill="currentColor" viewBox="0 0 20 20">
                      <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
                    </svg>
                    {{ user.followers }} 关注者
                  </span>
                  <span v-if="user.following !== undefined" class="flex items-center">
                    <svg :class="showNotifications ? 'w-3 h-3 mr-1' : 'w-4 h-4 mr-1'" fill="currentColor" viewBox="0 0 20 20">
                      <path d="M8 9a3 3 0 100-6 3 3 0 000 6zM8 11a6 6 0 016 6H2a6 6 0 016-6z"/>
                    </svg>
                    {{ user.following }} 正在关注
                  </span>
                  <span v-if="user.total_repos !== undefined" class="flex items-center">
                    <svg :class="showNotifications ? 'w-3 h-3 mr-1' : 'w-4 h-4 mr-1'" fill="currentColor" viewBox="0 0 20 20">
                      <path d="M3 4a1 1 0 011-1h12a1 1 0 011 1v2a1 1 0 01-1 1H4a1 1 0 01-1-1V4zM3 10a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H4a1 1 0 01-1-1v-6zM14 9a1 1 0 00-1 1v6a1 1 0 001 1h2a1 1 0 001-1v-6a1 1 0 00-1-1h-2z"/>
                    </svg>
                    {{ user.total_repos }} 仓库
                  </span>
                </div>
              </div>
            </div>

            <!-- 联系信息 -->
            <div :class="showNotifications ? 'xl:ml-auto space-y-2' : 'lg:ml-auto space-y-3'">
              <div v-if="user.email" class="flex items-center space-x-3 text-gray-700">
                <svg class="w-4 h-4 text-gray-400 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                  <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z"/>
                  <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z"/>
                </svg>
                <a :href="`mailto:${user.email}`" class="hover:text-blue-600 transition-colors truncate" :class="showNotifications ? 'text-sm' : 'text-base'">
                  {{ user.email }}
                </a>
              </div>
              
              <div v-if="user.location" class="flex items-center space-x-3 text-gray-700">
                <svg class="w-4 h-4 text-gray-400 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M5.05 4.05a7 7 0 119.9 9.9L10 18.9l-4.95-4.95a7 7 0 010-9.9zM10 11a2 2 0 100-4 2 2 0 000 4z" clip-rule="evenodd"/>
                </svg>
                <span class="truncate" :class="showNotifications ? 'text-sm' : 'text-base'">
                  {{ user.location }}
                </span>
              </div>
              
              <div v-if="user.html_url" class="flex items-center space-x-3 text-gray-700">
                <svg class="w-4 h-4 text-gray-400 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M12.586 4.586a2 2 0 112.828 2.828l-3 3a2 2 0 01-2.828 0 1 1 0 00-1.414 1.414 4 4 0 005.656 0l3-3a4 4 0 00-5.656-5.656l-1.5 1.5a1 1 0 101.414 1.414l1.5-1.5zm-5 5a2 2 0 012.828 0 1 1 0 101.414-1.414 4 4 0 00-5.656 0l-3 3a4 4 0 105.656 5.656l1.5-1.5a1 1 0 10-1.414-1.414l-1.5 1.5a2 2 0 11-2.828-2.828l3-3z" clip-rule="evenodd"/>
                </svg>
                <a :href="user.html_url" target="_blank" class="hover:text-blue-600 transition-colors flex items-center truncate" :class="showNotifications ? 'text-sm' : 'text-base'">
                  查看主页
                  <svg class="w-3 h-3 ml-1 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M11 3a1 1 0 100 2h2.586l-6.293 6.293a1 1 0 101.414 1.414L15 6.414V9a1 1 0 102 0V4a1 1 0 00-1-1h-5z"/>
                    <path d="M5 5a2 2 0 00-2 2v8a2 2 0 002 2h8a2 2 0 002-2v-3a1 1 0 10-2 0v3H5V7h3a1 1 0 000-2H5z"/>
                  </svg>
                </a>
              </div>
              
              <div v-if="user.id" class="text-xs text-gray-400 pt-2">
                ID: {{ user.id }}
              </div>
            </div>
          </div>
        </div>

        <!-- Tab导航和内容 -->
        <div class="flex-1 bg-white rounded-2xl shadow-xl border border-gray-100 overflow-hidden flex flex-col">
          <MenubarRoot class="border-b border-gray-200 bg-gray-50/50 flex-shrink-0">
            <MenubarMenu>
              <MenubarTrigger 
                :class="getTabClass('/overview')"
                @click="navigateToTab('/overview')"
              >
                概览
              </MenubarTrigger>
            </MenubarMenu>
            
            <MenubarMenu>
              <MenubarTrigger 
                :class="getTabClass('/issues')"
                @click="navigateToTab('/issues')"
              >
                Issues
              </MenubarTrigger>
            </MenubarMenu>
            
            <MenubarMenu>
              <MenubarTrigger 
                :class="getTabClass('/repositories')"
                @click="navigateToTab('/repositories')"
              >
                仓库
              </MenubarTrigger>
            </MenubarMenu>
            
            <MenubarMenu>
              <MenubarTrigger 
                :class="getTabClass('/projects')"
                @click="navigateToTab('/projects')"
              >
                项目
              </MenubarTrigger>
            </MenubarMenu>
          </MenubarRoot>

          <!-- 路由视图容器 -->
          <div class="flex-1 overflow-y-auto">
            <div class="p-6">
              <router-view />
            </div>
          </div>
        </div>
      </div>

      <!-- 右侧：消息通知面板 -->
      <Transition
        enter-active-class="transition-all duration-300 ease-out"
        enter-from-class="transform translate-x-full opacity-0"
        enter-to-class="transform translate-x-0 opacity-100"
        leave-active-class="transition-all duration-300 ease-in"
        leave-from-class="transform translate-x-0 opacity-100"
        leave-to-class="transform translate-x-full opacity-0"
      >
        <div v-if="showNotifications" class="flex-shrink-0" :class="showNotifications ? 'w-80' : 'w-96'">
          <NotificationPanel 
            @close="hideNotificationPanel"
            @unread-count-change="updateUnreadCount"
          />
        </div>
      </Transition>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { useUserStore } from '@/stores/index';
import {
  MenubarRoot,
  MenubarMenu,
  MenubarTrigger
} from 'reka-ui';
import { delToken } from '@/utils/token';
import NotificationPanel from '@/components/ui/notification/NotificationPanel.vue';

const router = useRouter();
const route = useRoute();
const userStore = useUserStore();

// 响应式数据
const { user } = userStore;
const showNotifications = ref(true);
const unreadNotificationCount = ref(0);

// 消息通知面板控制
const toggleNotificationPanel = () => {
  showNotifications.value = !showNotifications.value;
};

const hideNotificationPanel = () => {
  showNotifications.value = false;
};

const updateUnreadCount = (count: number) => {
  unreadNotificationCount.value = count;
};

// Tab导航相关方法
const navigateToTab = (path: string) => {
  router.push(path);
};

const getTabClass = (path: string) => {
  const isActive = route.path === path;
  return [
    'px-4 py-2 text-sm font-medium rounded-md transition-colors cursor-pointer',
    isActive
      ? 'bg-blue-100 text-blue-700 border-blue-200'
      : 'text-gray-600 hover:text-gray-900 hover:bg-gray-100'
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
});
</script>

<style scoped>
/* 自定义滚动条样式 */
.overflow-y-auto::-webkit-scrollbar {
  width: 6px;
}

.overflow-y-auto::-webkit-scrollbar-track {
  background: #f1f5f9;
  border-radius: 3px;
}

.overflow-y-auto::-webkit-scrollbar-thumb {
  background: #cbd5e1;
  border-radius: 3px;
}

.overflow-y-auto::-webkit-scrollbar-thumb:hover {
  background: #94a3b8;
}
</style>
import { NotificationPanel } from '@/components/ui/notification';

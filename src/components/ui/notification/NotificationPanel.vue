<template>
  <div class="bg-white rounded-xl shadow-lg border border-gray-200 overflow-hidden">
    <!-- 通知头部 -->
    <div class="bg-gradient-to-r from-blue-500 to-purple-600 px-6 py-4">
      <div class="flex items-center justify-between">
        <h3 class="text-lg font-semibold text-white flex items-center">
          <svg class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 20 20">
            <path d="M10 2a6 6 0 00-6 6v3.586l-.707.707A1 1 0 004 14h12a1 1 0 00.707-1.707L16 11.586V8a6 6 0 00-6-6zM10 18a3 3 0 01-3-3h6a3 3 0 01-3 3z"/>
          </svg>
          消息通知
        </h3>
        <div class="flex items-center space-x-3">
          <span class="bg-white/20 text-white text-xs px-2 py-1 rounded-full">
            {{ unreadCount }} 条未读
          </span>
          <button 
            @click="markAllAsRead" 
            class="text-white/80 hover:text-white text-sm transition-colors"
            v-if="unreadCount > 0"
          >
            全部已读
          </button>
          <!-- 关闭按钮 -->
          <button 
            @click="$emit('close')"
            class="text-white/80 hover:text-white transition-colors p-1 rounded-md hover:bg-white/10"
            title="关闭通知面板"
          >
            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"/>
            </svg>
          </button>
        </div>
      </div>
    </div>

    <!-- 通知列表 -->
    <div class="max-h-96 overflow-y-auto">
      <div v-if="notifications.length === 0" class="p-8 text-center text-gray-500">
        <svg class="w-12 h-12 mx-auto mb-4 text-gray-300" fill="currentColor" viewBox="0 0 20 20">
          <path d="M10 2a6 6 0 00-6 6v3.586l-.707.707A1 1 0 004 14h12a1 1 0 00.707-1.707L16 11.586V8a6 6 0 00-6-6zM10 18a3 3 0 01-3-3h6a3 3 0 01-3 3z"/>
        </svg>
        <p>暂无消息通知</p>
      </div>
      
      <div v-else>
        <div 
          v-for="notification in notifications" 
          :key="notification.id"
          class="border-b border-gray-100 last:border-b-0 hover:bg-gray-50 transition-colors"
        >
          <div class="p-4 flex items-start space-x-3">
            <!-- 通知图标 -->
            <div class="flex-shrink-0">
              <div 
                :class="getNotificationIconClass(notification.type)"
                class="w-8 h-8 rounded-full flex items-center justify-center"
              >
                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                  <path v-if="notification.type === 'issue'" d="M10 12a2 2 0 100-4 2 2 0 000 4z"/>
                  <path v-else-if="notification.type === 'pr'" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
                  <path v-else-if="notification.type === 'mention'" d="M8 9a3 3 0 100-6 3 3 0 000 6zM8 11a6 6 0 016 6H2a6 6 0 016-6z"/>
                  <path v-else d="M10 2a6 6 0 00-6 6v3.586l-.707.707A1 1 0 004 14h12a1 1 0 00.707-1.707L16 11.586V8a6 6 0 00-6-6z"/>
                </svg>
              </div>
            </div>
            
            <!-- 通知内容 -->
            <div class="flex-1 min-w-0">
              <div class="flex items-center justify-between">
                <p class="text-sm font-medium text-gray-900 truncate">
                  {{ notification.title }}
                </p>
                <div class="flex items-center space-x-2">
                  <span v-if="!notification.read" class="w-2 h-2 bg-blue-500 rounded-full"></span>
                  <span class="text-xs text-gray-500">
                    {{ formatTime(notification.created_at) }}
                  </span>
                </div>
              </div>
              <p class="text-sm text-gray-600 mt-1 line-clamp-2">
                {{ notification.content }}
              </p>
              <div class="flex items-center mt-2 space-x-2">
                <span 
                  :class="getNotificationTypeClass(notification.type)"
                  class="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium"
                >
                  {{ getNotificationTypeText(notification.type) }}
                </span>
                <span v-if="notification.repository" class="text-xs text-gray-500">
                  来自 {{ notification.repository }}
                </span>
              </div>
            </div>
            
            <!-- 操作按钮 -->
            <div class="flex-shrink-0 flex flex-col space-y-1">
              <button 
                @click="markAsRead(notification.id)"
                v-if="!notification.read"
                class="text-gray-400 hover:text-green-600 transition-colors p-1 rounded-md hover:bg-green-50"
                title="标记为已读"
              >
                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                  <path d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"/>
                </svg>
              </button>
              <button 
                @click="dismissNotification(notification.id)"
                class="text-gray-400 hover:text-red-600 transition-colors p-1 rounded-md hover:bg-red-50"
                title="删除通知"
              >
                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"/>
                </svg>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- 底部操作 -->
    <div class="bg-gray-50 px-6 py-3 border-t border-gray-200">
      <div class="flex justify-between items-center">
        <button 
          @click="loadMore"
          v-if="hasMore"
          class="text-sm text-blue-600 hover:text-blue-800 transition-colors flex items-center"
        >
          <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd"/>
          </svg>
          加载更多
        </button>
        <div class="flex items-center space-x-2">
          <button 
            @click="refreshNotifications"
            class="text-sm text-gray-600 hover:text-gray-800 transition-colors flex items-center"
          >
            <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd"/>
            </svg>
            刷新
          </button>
          <button 
            @click="clearAllNotifications"
            v-if="notifications.length > 0"
            class="text-sm text-red-600 hover:text-red-800 transition-colors flex items-center"
          >
            <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M9 2a1 1 0 000 2h2a1 1 0 100-2H9z" clip-rule="evenodd"/>
              <path fill-rule="evenodd" d="M4 5a2 2 0 012-2h8a2 2 0 012 2v6a2 2 0 01-2 2H6a2 2 0 01-2-2V5zm3 3a1 1 0 000 2h6a1 1 0 100-2H7z" clip-rule="evenodd"/>
            </svg>
            清空全部
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { $fetch } from '@/utils/fetch';

interface Notification {
  id: string;
  type: 'issue' | 'pr' | 'mention' | 'system';
  title: string;
  content: string;
  repository?: string;
  read: boolean;
  created_at: string;
}

// 定义事件
const emit = defineEmits<{
  close: [];
  'unread-count-change': [count: number];
}>();

const notifications = ref<Notification[]>([]);
const hasMore = ref(false);
const loading = ref(false);

const unreadCount = computed(() => {
  return notifications.value.filter(n => !n.read).length;
});

// 监听未读数量变化，通知父组件
watch(unreadCount, (newCount) => {
  emit('unread-count-change', newCount);
}, { immediate: true });

const getNotificationIconClass = (type: string) => {
  const classes = {
    issue: 'bg-red-100 text-red-600',
    pr: 'bg-green-100 text-green-600',
    mention: 'bg-blue-100 text-blue-600',
    system: 'bg-gray-100 text-gray-600'
  };
  return classes[type as keyof typeof classes] || classes.system;
};

const getNotificationTypeClass = (type: string) => {
  const classes = {
    issue: 'bg-red-100 text-red-800',
    pr: 'bg-green-100 text-green-800',
    mention: 'bg-blue-100 text-blue-800',
    system: 'bg-gray-100 text-gray-800'
  };
  return classes[type as keyof typeof classes] || classes.system;
};

const getNotificationTypeText = (type: string) => {
  const texts = {
    issue: 'Issue',
    pr: 'Pull Request',
    mention: '提及',
    system: '系统通知'
  };
  return texts[type as keyof typeof texts] || '通知';
};

const formatTime = (dateString: string) => {
  const date = new Date(dateString);
  const now = new Date();
  const diff = now.getTime() - date.getTime();

  const minutes = Math.floor(diff / (1000 * 60));
  const hours = Math.floor(diff / (1000 * 60 * 60));
  const days = Math.floor(diff / (1000 * 60 * 60 * 24));

  if (minutes < 60) {
    return `${minutes}分钟前`;
  } else if (hours < 24) {
    return `${hours}小时前`;
  } else if (days < 7) {
    return `${days}天前`;
  } else {
    return date.toLocaleDateString('zh-CN');
  }
};

const fetchNotifications = async () => {
  try {
    loading.value = true;
    const { success, data } = await $fetch('/notifications', { method: 'get' });
    if (success) {
      notifications.value = data.notifications || [];
      hasMore.value = data.hasMore || false;
    }
  } catch (error) {
    console.error('获取通知失败:', error);
    // 模拟数据用于演示
    notifications.value = [
      {
        id: '1',
        type: 'issue',
        title: '新的Issue被分配给您',
        content: '在项目 "tauri-app" 中，Issue #123 "修复登录问题" 已被分配给您处理。',
        repository: 'tauri-app',
        read: false,
        created_at: new Date(Date.now() - 1000 * 60 * 30).toISOString()
      },
      {
        id: '2',
        type: 'pr',
        title: 'Pull Request 需要您的审核',
        content: 'PR #45 "添加用户认证功能" 正在等待您的代码审核。',
        repository: 'tauri-app',
        read: false,
        created_at: new Date(Date.now() - 1000 * 60 * 60 * 2).toISOString()
      },
      {
        id: '3',
        type: 'mention',
        title: '有人在评论中提及了您',
        content: '@您的用户名 请帮忙看一下这个问题的解决方案。',
        repository: 'tauri-app',
        read: true,
        created_at: new Date(Date.now() - 1000 * 60 * 60 * 24).toISOString()
      }
    ];
  } finally {
    loading.value = false;
  }
};

const markAsRead = async (notificationId: string) => {
  try {
    await $fetch(`/notifications/${notificationId}/read`, { method: 'post' });
    const notification = notifications.value.find(n => n.id === notificationId);
    if (notification) {
      notification.read = true;
    }
  } catch (error) {
    console.error('标记已读失败:', error);
    // 本地更新用于演示
    const notification = notifications.value.find(n => n.id === notificationId);
    if (notification) {
      notification.read = true;
    }
  }
};

const markAllAsRead = async () => {
  try {
    await $fetch('/notifications/read-all', { method: 'post' });
    notifications.value.forEach(n => n.read = true);
  } catch (error) {
    console.error('全部标记已读失败:', error);
    // 本地更新用于演示
    notifications.value.forEach(n => n.read = true);
  }
};

const dismissNotification = async (notificationId: string) => {
  try {
    await $fetch(`/notifications/${notificationId}`, { method: 'post' });
    const index = notifications.value.findIndex(n => n.id === notificationId);
    if (index > -1) {
      notifications.value.splice(index, 1);
    }
  } catch (error) {
    console.error('删除通知失败:', error);
    // 本地删除用于演示
    const index = notifications.value.findIndex(n => n.id === notificationId);
    if (index > -1) {
      notifications.value.splice(index, 1);
    }
  }
};

const clearAllNotifications = async () => {
  try {
    await $fetch('/notifications/clear-all', { method: 'post' });
    notifications.value = [];
  } catch (error) {
    console.error('清空通知失败:', error);
    // 本地清空用于演示
    notifications.value = [];
  }
};

const loadMore = async () => {
  // 实现加载更多逻辑
  console.log('加载更多通知');
};

const refreshNotifications = () => {
  fetchNotifications();
};

onMounted(() => {
  fetchNotifications();
});
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
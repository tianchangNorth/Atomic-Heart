<template>
  <Card class="w-80 h-[600px] flex flex-col overflow-hidden shadow-lg">
    <!-- 通知头部 -->
    <CardHeader class="pb-3">
      <div class="flex items-center justify-between">
        <div class="flex items-center space-x-2">
          <div class="w-8 h-8 bg-primary/10 rounded-lg flex items-center justify-center">
            <svg class="w-4 h-4 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-5 5v-5zM11 3.055A9.001 9.001 0 1020.945 13H11V3.055z"/>
            </svg>
          </div>
          <div>
            <CardTitle class="text-lg">通知</CardTitle>
            <CardDescription class="text-xs">
              {{ unreadCount > 0 ? `${unreadCount} 条未读消息` : '暂无未读消息' }}
            </CardDescription>
          </div>
        </div>

        <div class="flex items-center space-x-1">
          <Button
            v-if="unreadCount > 0"
            variant="ghost"
            size="sm"
            @click="markAllAsRead"
            class="text-xs h-8 px-2"
          >
            全部已读
          </Button>
          <Button
            variant="ghost"
            size="sm"
            @click="$emit('close')"
            class="h-8 w-8 p-0"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
            </svg>
          </Button>
        </div>
      </div>
    </CardHeader>

    <!-- 通知列表 -->
    <CardContent class="flex-1 overflow-hidden p-0">
      <div class="h-full overflow-y-auto">
        <!-- 空状态 -->
        <div v-if="notifications.length === 0" class="flex flex-col items-center justify-center h-full p-6 text-center">
          <div class="w-16 h-16 bg-muted rounded-full flex items-center justify-center mb-4">
            <svg class="w-8 h-8 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-5 5v-5zM11 3.055A9.001 9.001 0 1020.945 13H11V3.055z"/>
            </svg>
          </div>
          <h3 class="text-sm font-medium text-foreground mb-1">暂无通知</h3>
          <p class="text-xs text-muted-foreground">当有新的活动时，通知会显示在这里</p>
        </div>

        <!-- 通知列表 -->
        <div v-else class="divide-y divide-border">
          <div
            v-for="notification in notifications"
            :key="notification.id"
            class="p-4 hover:bg-muted/50 transition-colors cursor-pointer group"
            @click="handleNotificationClick(notification)"
          >
            <div class="flex items-start space-x-3">
              <!-- 通知图标 -->
              <div class="flex-shrink-0 mt-0.5">
                <div
                  :class="getNotificationIconClass(notification.type)"
                  class="w-8 h-8 rounded-full flex items-center justify-center"
                >
                  <!-- Issue 图标 -->
                  <svg v-if="notification.type === 'issue'" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.34 16.5c-.77.833.192 2.5 1.732 2.5z"/>
                  </svg>
                  <!-- PR 图标 -->
                  <svg v-else-if="notification.type === 'pr'" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
                  </svg>
                  <!-- 提及图标 -->
                  <svg v-else-if="notification.type === 'mention'" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
                  </svg>
                  <!-- 系统通知图标 -->
                  <svg v-else class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                  </svg>
                </div>
              </div>

              <!-- 通知内容 -->
              <div class="flex-1 min-w-0">
                <div class="flex items-start justify-between mb-1">
                  <h4 class="text-sm font-medium text-foreground line-clamp-1 pr-2">
                    {{ notification.title }}
                  </h4>
                  <div class="flex items-center space-x-2 flex-shrink-0">
                    <div v-if="!notification.read" class="w-2 h-2 bg-primary rounded-full"></div>
                    <span class="text-xs text-muted-foreground">
                      {{ formatTime(notification.created_at) }}
                    </span>
                  </div>
                </div>

                <p class="text-xs text-muted-foreground line-clamp-2 mb-2">
                  {{ notification.content }}
                </p>

                <div class="flex items-center justify-between">
                  <div class="flex items-center space-x-2">
                    <Badge
                      :variant="getNotificationBadgeVariant(notification.type)"
                      class="text-xs"
                    >
                      {{ getNotificationTypeText(notification.type) }}
                    </Badge>
                    <span v-if="notification.repository" class="text-xs text-muted-foreground">
                      {{ notification.repository }}
                    </span>
                  </div>

                  <!-- 操作按钮 -->
                  <div class="flex items-center space-x-1 opacity-0 group-hover:opacity-100 transition-opacity">
                    <Button
                      v-if="!notification.read"
                      variant="ghost"
                      size="sm"
                      @click.stop="markAsRead(notification.id)"
                      class="h-6 w-6 p-0"
                    >
                      <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                      </svg>
                    </Button>
                    <Button
                      variant="ghost"
                      size="sm"
                      @click.stop="dismissNotification(notification.id)"
                      class="h-6 w-6 p-0 text-muted-foreground hover:text-destructive"
                    >
                      <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                      </svg>
                    </Button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </CardContent>
    
    <!-- 底部操作 -->
    <CardFooter class="p-3 border-t border-border bg-muted/30">
      <div class="flex justify-between items-center w-full">
        <Button
          v-if="hasMore"
          variant="ghost"
          size="sm"
          @click="loadMore"
          :disabled="loading"
          class="text-xs h-8"
        >
          <svg v-if="loading" class="w-3 h-3 mr-1 animate-spin" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          <svg v-else class="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
          </svg>
          加载更多
        </Button>

        <div class="flex items-center space-x-1">
          <Button
            variant="ghost"
            size="sm"
            @click="refreshNotifications"
            :disabled="loading"
            class="h-8 w-8 p-0"
          >
            <svg class="w-3 h-3" :class="{ 'animate-spin': loading }" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
            </svg>
          </Button>

          <Button
            v-if="notifications.length > 0"
            variant="ghost"
            size="sm"
            @click="clearAllNotifications"
            class="h-8 w-8 p-0 text-muted-foreground hover:text-destructive"
          >
            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
            </svg>
          </Button>
        </div>
      </div>
    </CardFooter>
  </Card>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { $fetch } from '@/utils/fetch';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';

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
    issue: 'bg-destructive/10 text-destructive',
    pr: 'bg-green-500/10 text-green-600',
    mention: 'bg-blue-500/10 text-blue-600',
    system: 'bg-muted text-muted-foreground'
  };
  return classes[type as keyof typeof classes] || classes.system;
};

const getNotificationBadgeVariant = (type: string): 'default' | 'secondary' | 'destructive' | 'outline' => {
  const variants = {
    issue: 'destructive' as const,
    pr: 'default' as const,
    mention: 'secondary' as const,
    system: 'outline' as const
  };
  return variants[type as keyof typeof variants] || variants.system;
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

const handleNotificationClick = (notification: Notification) => {
  // 点击通知时自动标记为已读
  if (!notification.read) {
    markAsRead(notification.id);
  }

  // 这里可以添加跳转到相关页面的逻辑
  console.log('Clicked notification:', notification);
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
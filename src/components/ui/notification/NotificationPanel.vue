<script setup lang="ts">
// computed 不再需要，因为使用服务中的
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { invoke } from '@tauri-apps/api/core'
import {
  notifications,
  loading,
  hasMore,
  unreadCount,
  markAsRead,
  markAllAsRead,
  dismissNotification,
  clearAllNotifications,
  refreshNotifications,
  type Notification
} from '@/services/notificationService';

// 从通知内容中提取通知类型
const getNotificationType = (content: string): string => {
  const lowerContent = content.toLowerCase();
  if (lowerContent.includes('issue') || lowerContent.includes('问题')) {
    return 'issue';
  } else if (lowerContent.includes('pull request') || lowerContent.includes('pr') || lowerContent.includes('合并请求')) {
    return 'pr';
  } else if (lowerContent.includes('mention') || lowerContent.includes('提及') || lowerContent.includes('@')) {
    return 'mention';
  } else if (lowerContent.includes('star') || lowerContent.includes('fork') || lowerContent.includes('watch')) {
    return 'activity';
  }
  return 'system';
};

const getNotificationBadgeVariant = (content: string): 'default' | 'secondary' | 'destructive' | 'outline' => {
  const type = getNotificationType(content);
  const variants = {
    issue: 'destructive' as const,
    pr: 'default' as const,
    mention: 'secondary' as const,
    activity: 'outline' as const,
    system: 'outline' as const
  };
  return variants[type as keyof typeof variants] || variants.system;
};

const getNotificationTypeText = (content: string) => {
  const type = getNotificationType(content);
  const texts = {
    issue: 'Issue',
    pr: 'Pull Request',
    mention: '提及',
    activity: '活动',
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

  if (minutes < 1) {
    return '刚刚';
  } else if (minutes < 60) {
    return `${minutes}分钟前`;
  } else if (hours < 24) {
    return `${hours}小时前`;
  } else if (days < 7) {
    return `${days}天前`;
  } else {
    return date.toLocaleDateString('zh-CN');
  }
};

// 安全渲染HTML内容
const sanitizeHtml = (html: string): string => {
  // 简单的HTML清理，移除潜在危险的标签和属性
  return html
    .replace(/<script[^>]*>.*?<\/script>/gi, '')
    .replace(/<iframe[^>]*>.*?<\/iframe>/gi, '')
    .replace(/on\w+="[^"]*"/gi, '')
    .replace(/javascript:/gi, '');
};

// fetchNotifications 现在在服务中

// 所有函数现在都在服务中，直接使用导入的函数

const loadMore = async () => {
  // 实现加载更多逻辑
  console.log('加载更多通知');
};

const handleNotificationClick = (notification: Notification) => {
  // 点击通知时自动标记为已读
  if (notification.unread) {
    markAsRead(notification.id);
  }

  // 跳转到通知对应的页面
  if (notification.html_url) {
    console.log('跳转到:', notification.html_url);
    // 这里可以添加路由跳转逻辑
    // router.push(notification.html_url);
  }
};

const openUrl = async (url: string) => {
  try {
    await invoke('open_url', { url });
  } catch (error) {
    console.error('打开链接失败:', error);
  }
};

</script>

<template>
  <Card class="w-80 h-[886px] flex flex-col overflow-hidden shadow-lg">
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
              <!-- 发送者头像 -->
              <Avatar size="sm" class="flex-shrink-0">
                <AvatarImage
                  :src="notification.sender.avatar_url"
                  :alt="notification.sender.login"
                />
                <AvatarFallback class="text-xs">
                  {{ notification.sender.login.charAt(0).toUpperCase() }}
                </AvatarFallback>
              </Avatar>

              <!-- 通知类型图标 -->

              <!-- 通知内容 -->
              <div class="flex-1 min-w-0">
                <div class="flex items-start justify-between mb-1">
                  <div class="flex items-center space-x-2 flex-1">
                    <span class="text-sm font-medium text-foreground">
                      {{ notification.sender.login }}
                    </span>
                    <div v-if="notification.unread" class="w-2 h-2 bg-primary rounded-full"></div>
                  </div>
                  <span class="text-xs text-muted-foreground flex-shrink-0">
                    {{ formatTime(notification.updated_at) }}
                  </span>
                </div>

                <!-- 通知内容（安全渲染HTML） -->
                <div
                  class="text-sm text-foreground line-clamp-3 mb-2"
                  v-html="sanitizeHtml(notification.content)"
                ></div>

                <div class="flex items-center justify-between">
                  <div class="flex items-center space-x-2">
                    <Badge
                      :variant="getNotificationBadgeVariant(notification.content)"
                      class="text-xs"
                    >
                      {{ getNotificationTypeText(notification.content) }}
                    </Badge>
                    <div
                      v-if="notification.html_url"
                      class="text-xs text-muted-foreground hover:text-primary transition-colors"
                      @click.stop="openUrl(notification.html_url)"
                    >
                      查看详情
                  </div>
                  </div>

                  <!-- 操作按钮 -->
                  <div class="flex items-center space-x-1 opacity-0 group-hover:opacity-100 transition-opacity">
                    <Button
                      v-if="notification.unread"
                      variant="ghost"
                      size="sm"
                      @click.stop="markAsRead(notification.id)"
                      class="h-6 w-6 p-0"
                      title="标记为已读"
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
                      title="删除通知"
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

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.line-clamp-3 {
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

/* 通知内容中的HTML样式 */
.line-clamp-3 :deep(a) {
  color: hsl(var(--primary));
  text-decoration: none;
}

.line-clamp-3 :deep(a:hover) {
  text-decoration: underline;
}

.line-clamp-3 :deep(strong) {
  font-weight: 600;
}

.line-clamp-3 :deep(em) {
  font-style: italic;
}

.line-clamp-3 :deep(code) {
  background-color: hsl(var(--muted));
  padding: 0.125rem 0.25rem;
  border-radius: 0.25rem;
  font-size: 0.875em;
  font-family: ui-monospace, SFMono-Regular, "SF Mono", Consolas, "Liberation Mono", Menlo, monospace;
}
</style>
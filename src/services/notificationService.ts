import { ref, computed } from 'vue';
import { $fetch } from '@/utils/fetch';

// 通知数据结构
export interface Notification {
  id: string;                    // 通知唯一标识
  content: string;               // 通知内容（包含HTML）
  unread: boolean;               // 是否未读
  updated_at: string;            // 更新时间
  url: string;                   // API地址
  html_url: string;              // 网页地址
  sender: {                      // 发送者信息
    login: string;               // 用户名
    id: string;                  // 用户ID
    avatar_url: string;          // 头像URL
    html_url: string;            // 用户主页
  };
}

// 全局通知状态
const notifications = ref<Notification[]>([]);
const loading = ref(false);
const hasMore = ref(false);
const initialized = ref(false);

// 计算未读数量
export const unreadCount = computed(() => {
  return notifications.value.filter(n => n.unread).length;
});

// 获取通知数据
export const fetchNotifications = async () => {
  if (loading.value) return;

  try {
    loading.value = true;
    const { success, data } = await $fetch('/notifications/messages', {
      method: 'get',
      data: {
        per_page: 10
      }
    });

    if (success && data) {
      notifications.value = data.list || [];
      hasMore.value = data.hasMore || false;
      console.log('获取通知成功:', data);
    } else {
      hasMore.value = false;
    }
  } catch (error) {
    console.error('获取通知失败:', error);
    hasMore.value = false;
  } finally {
    loading.value = false;
    initialized.value = true;
  }
};

// 标记单个通知为已读
// 由于openapi不支持信息操作，所以这里使用本地更新
export const markAsRead = async (notificationId: string) => {
  try {
    await $fetch(`/notifications/${notificationId}/read`, { method: 'post' });
    const notification = notifications.value.find(n => n.id === notificationId);
    if (notification) {
      notification.unread = false;
    }
  } catch (error) {
    console.error('标记已读失败:', error);
    // 本地更新用于演示
    const notification = notifications.value.find(n => n.id === notificationId);
    if (notification) {
      notification.unread = false;
    }
  }
};

// 标记所有通知为已读
export const markAllAsRead = async () => {
  try {
    await $fetch('/notifications/read-all', { method: 'post' });
    notifications.value.forEach(n => n.unread = false);
  } catch (error) {
    console.error('全部标记已读失败:', error);
    // 本地更新用于演示
    notifications.value.forEach(n => n.unread = false);
  }
};

// 删除通知
export const dismissNotification = async (notificationId: string) => {
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

// 清空所有通知
export const clearAllNotifications = async () => {
  try {
    await $fetch('/notifications/clear-all', { method: 'post' });
    notifications.value = [];
  } catch (error) {
    console.error('清空通知失败:', error);
    // 本地清空用于演示
    notifications.value = [];
  }
};

// 刷新通知
export const refreshNotifications = () => {
  fetchNotifications();
};

// 初始化通知服务
export const initializeNotifications = () => {
  if (!initialized.value) {
    fetchNotifications();
  }
};

// 导出状态
export {
  notifications,
  loading,
  hasMore,
  initialized
};

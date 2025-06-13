<script setup lang="ts">
import { useUserStore } from '@/stores/index';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { $fetch } from '@/utils/fetch';
import { onMounted, ref } from 'vue';
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'

// 定义活动数据接口
interface Activity {
  type: string;
  actor: {
    login: string;
    id: string;
    avatar_url: string;
    html_url: string;
  };
  repo: {
    id: number;
    name: string;
    path: string;
    web_url: string;
  };
  created_at: string;
  payload: string;
}

const userStore = useUserStore();
const { user } = userStore;
const router = useRouter();

// 活动数据状态
const recentActivity = ref<Activity[]>([]);
const loading = ref(false);
const error = ref<string | null>(null);

// 获取活动数据
const fetchRecentActivity = async () => {
  loading.value = true;
  error.value = null;

  try {
    const { success, data } = await $fetch(`/users/${user.login}/events`, { method: 'get' });
    if (success && Array.isArray(data)) {
      recentActivity.value = data;
    }
  } catch (error) {
    console.error('获取最近活动失败:', error);
  } finally {
    loading.value = false;
  }
};

// 获取活动类型的图标和颜色
const getActivityIcon = (type: string) => {
  const iconMap: Record<string, { icon: string; color: string }> = {
    push: {
      icon: 'M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10',
      color: 'text-green-500'
    },
    create: {
      icon: 'M12 4v16m8-8H4',
      color: 'text-blue-500'
    },
    delete: {
      icon: 'M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16',
      color: 'text-red-500'
    },
    fork: {
      icon: 'M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l3-3m0 0l-3-3m3 3H9',
      color: 'text-purple-500'
    },
    issues: {
      icon: 'M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.34 16.5c-.77.833.192 2.5 1.732 2.5z',
      color: 'text-orange-500'
    },
    pull_request: {
      icon: 'M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 12l2 2 4-4',
      color: 'text-indigo-500'
    },
    release: {
      icon: 'M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z',
      color: 'text-yellow-500'
    },
    star: {
      icon: 'M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z',
      color: 'text-amber-500'
    }
  };

  return iconMap[type] || iconMap.push;
};

// 格式化时间显示
const formatTimeAgo = (dateString: string): string => {
  const date = new Date(dateString);
  const now = new Date();
  const diffInSeconds = Math.floor((now.getTime() - date.getTime()) / 1000);

  if (diffInSeconds < 60) {
    return '刚刚';
  } else if (diffInSeconds < 3600) {
    const minutes = Math.floor(diffInSeconds / 60);
    return `${minutes}分钟前`;
  } else if (diffInSeconds < 86400) {
    const hours = Math.floor(diffInSeconds / 3600);
    return `${hours}小时前`;
  } else if (diffInSeconds < 2592000) {
    const days = Math.floor(diffInSeconds / 86400);
    return `${days}天前`;
  } else {
    return date.toLocaleDateString('zh-CN');
  }
};

// 获取活动描述文本
const getActivityDescription = (activity: Activity): string => {
  const { type, repo, payload } = activity;

  switch (type) {
    case 'push':
      return `推送了代码到 ${repo.name}`;
    case 'mirror_sync':
      return `同步更新了 ${repo.name}`;
    case 'create_repo':
      return `新建了代码库 ${repo.name} `;
    case 'delete':
      return `在 ${repo.name} 中删除了内容`;
    case 'fork':
      return `Fork 了 ${repo.name}`;
    case 'create_issue':
      return `在 ${repo.name} 中创建了 Issue`;
    case 'pull_request':
      return `在 ${repo.name} 中提交了 Pull Request`;
    case 'release':
      return `为 ${repo.name} 发布了新版本`;
    case 'star':
      return `给 ${repo.name} 点了星标`;
    default:
      return payload || `在 ${repo.name} 中进行了操作`;
  }
};

// 处理点击跳转
const handleUserClick = (userUrl: string) => {
  invoke('open_url', { url: userUrl });
};

const handleRepoClick = (repoUrl: string) => {
  invoke('open_url', { url: repoUrl });
};

// 组件挂载时获取数据
onMounted(() => {
  fetchRecentActivity();
});
</script>

<template>
  <div class="space-y-6">
    <!-- 欢迎标题 -->
    <div class="mb-8">
      <h1 class="text-3xl font-bold text-foreground mb-2">欢迎回来，{{ user.name || '用户' }}！</h1>
      <p class="text-muted-foreground">AtomGit 工作台</p>
    </div>

    <!-- 统计卡片 -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
      <Card>
        <CardContent class="p-6 cursor-pointer" @click="() => { router.push('/repositories') }">
          <div class="flex items-center justify-between" >
            <div>
              <p class="text-sm font-medium text-muted-foreground">总仓库数</p>
              <p class="text-2xl font-bold text-foreground">{{ user.total_repos || 0 }}</p>
            </div>
            <div class="w-12 h-12 bg-primary/10 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"/>
              </svg>
            </div>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardContent class="p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-muted-foreground">关注者</p>
              <p class="text-2xl font-bold text-foreground">{{ user.followers || 0 }}</p>
            </div>
            <div class="w-12 h-12 bg-green-500/10 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/>
              </svg>
            </div>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardContent class="p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-muted-foreground">正在关注</p>
              <p class="text-2xl font-bold text-foreground">{{ user.following || 0 }}</p>
            </div>
            <div class="w-12 h-12 bg-blue-500/10 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-blue-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/>
              </svg>
            </div>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardContent class="p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-muted-foreground">未处理任务</p>
              <p class="text-2xl font-bold text-foreground">12</p>
            </div>
            <div class="w-12 h-12 bg-orange-500/10 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-orange-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.34 16.5c-.77.833.192 2.5 1.732 2.5z"/>
              </svg>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>

    <!-- 最近活动和快速操作 -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- 最近活动 -->
      <Card>
        <CardHeader>
          <CardTitle>最近活动</CardTitle>
          <CardDescription>您最近的代码活动记录</CardDescription>
        </CardHeader>
        <CardContent>
          <!-- 加载状态 -->
          <div v-if="loading" class="flex items-center justify-center py-8">
            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
            <span class="ml-2 text-sm text-muted-foreground">加载活动数据...</span>
          </div>

          <!-- 错误状态 -->
          <div v-else-if="error" class="text-center py-8">
            <div class="text-muted-foreground mb-2">
              <svg class="w-12 h-12 mx-auto mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.34 16.5c-.77.833.192 2.5 1.732 2.5z"/>
              </svg>
              <p class="text-sm">加载活动数据失败</p>
            </div>
            <Button variant="outline" size="sm" @click="fetchRecentActivity">
              重试
            </Button>
          </div>

          <!-- 空状态 -->
          <div v-else-if="recentActivity.length === 0" class="text-center py-8">
            <div class="text-muted-foreground">
              <svg class="w-12 h-12 mx-auto mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"/>
              </svg>
              <p class="text-sm">暂无活动记录</p>
            </div>
          </div>

          <!-- 活动列表 -->
          <div v-else class="space-y-4">
            <div
              v-for="activity in recentActivity.slice(0, 5)"
              :key="`${activity.type}-${activity.created_at}-${activity.repo.id}`"
              class="flex items-start space-x-3 group hover:bg-muted/50 rounded-lg p-2 -m-2 transition-colors cursor-pointer"
              @click="handleRepoClick(activity.repo.web_url)"
            >
              <!-- 用户头像 -->
              <Avatar size="sm" class="flex-shrink-0">
                <AvatarImage
                  :src="activity.actor.avatar_url"
                  :alt="activity.actor.login"
                />
                <AvatarFallback class="text-xs">
                  {{ activity.actor.login?.charAt(0)?.toUpperCase() || 'U' }}
                </AvatarFallback>
              </Avatar>

              <!-- 活动图标 -->
              <div class="flex-shrink-0 mt-1">
                <div :class="['w-6 h-6 rounded-full flex items-center justify-center', getActivityIcon(activity.type).color.replace('text-', 'bg-').replace('-500', '-100')]">
                  <svg
                    :class="['w-3 h-3', getActivityIcon(activity.type).color]"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      :d="getActivityIcon(activity.type).icon"
                    />
                  </svg>
                </div>
              </div>

              <!-- 活动内容 -->
              <div class="flex-1 min-w-0">
                <div class="flex items-start justify-between">
                  <div class="flex-1 min-w-0">
                    <p class="text-sm font-medium text-foreground group-hover:text-primary transition-colors">
                      <button
                        @click.stop="handleUserClick(activity.actor.html_url)"
                        class="hover:underline font-semibold"
                      >
                        {{ activity.actor.login }}
                      </button>
                      {{ getActivityDescription(activity) }}
                    </p>
                    <div class="flex items-center space-x-2 mt-1">
                      <button
                        @click.stop="handleRepoClick(activity.repo.web_url)"
                        class="text-xs text-muted-foreground hover:text-primary transition-colors hover:underline"
                      >
                        {{ activity.repo.path }}
                      </button>
                      <span class="text-xs text-muted-foreground">•</span>
                      <span class="text-xs text-muted-foreground">
                        {{ formatTimeAgo(activity.created_at) }}
                      </span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      <!-- 快速操作 -->
      <Card>
        <CardHeader>
          <CardTitle>快速操作</CardTitle>
          <CardDescription>常用功能的快速入口</CardDescription>
        </CardHeader>
        <CardContent>
          <div class="grid grid-cols-2 gap-4">
            <Button variant="outline" class="h-20 flex-col space-y-2">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
              </svg>
              <span class="text-sm">新建仓库</span>
            </Button>
            <Button variant="outline" class="h-20 flex-col space-y-2">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10"/>
              </svg>
              <span class="text-sm">导入仓库</span>
            </Button>
            <Button variant="outline" class="h-20 flex-col space-y-2">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/>
              </svg>
              <span class="text-sm">加入组织</span>
            </Button>
            <Button variant="outline" class="h-20 flex-col space-y-2">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
              </svg>
              <span class="text-sm">设置</span>
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  </div>
</template>

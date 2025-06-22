<script setup lang="ts">
import { useUserStore } from '@/stores/index';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { $fetch } from '@/utils/fetch';
import { onMounted, ref } from 'vue';
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { useToast } from '@/components/ui/toast';
import RepoClone from '@/components/git/RepoClone.vue';
import { useLocalRepositories } from '@/composables/useLocalRepositories';
import { extractRepositoryName } from '@/utils/utils';

import {
  Upload,
  Plus,
  Trash2,
  GitFork,
  Users,
  Settings,
  Globe,
  Heart,
  AlertTriangle,
  FileText,
  RefreshCw,
  UserPlus,
  Download,
  CircleDot,
  Folder,
  X
} from 'lucide-vue-next'

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
  payload: {
    title?: string;
    id?: string;
    iid?: string;
  };
}

const { success, warning } = useToast();

const userStore = useUserStore();
const { user } = userStore;
const { addRepository, repositoryCount } = useLocalRepositories();
const router = useRouter();

// 活动数据状态
const recentActivity = ref<Activity[]>([]);
const loading = ref(false);
const error = ref<string | null>(null);
const showCloneDialog = ref(false);

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
  const iconMap: Record<string, { component: any; color: string }> = {
    push: {
      component: Upload,
      color: 'text-green-500'
    },
    create: {
      component: Plus,
      color: 'text-blue-500'
    },
    delete: {
      component: Trash2,
      color: 'text-red-500'
    },
    fork: {
      component: GitFork,
      color: 'text-purple-500'
    },
    issues: {
      component: AlertTriangle,
      color: 'text-orange-500'
    },
    pull_request: {
      component: GitFork,
      color: 'text-indigo-500'
    },
    release: {
      component: FileText,
      color: 'text-yellow-500'
    },
    star: {
      component: Heart,
      color: 'text-amber-500'
    },
    mirror_sync: {
      component: RefreshCw,
      color: 'text-blue-500'
    },
    create_repo: {
      component: Plus,
      color: 'text-green-500'
    },
    create_issue: {
      component: CircleDot,
      color: 'text-orange-500'
    },
    merge_created: {
      component: GitFork,
      color: 'text-purple-500'
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
    case 'merge_created':
      return `合并了 ${payload.title} 到 ${repo.name}`;
    default:
      return payload.title || `在 ${repo.name} 中进行了操作`;
  }
};

// 处理点击跳转
const handleUserClick = (userUrl: string) => {
  invoke('open_url', { url: userUrl });
};

const handleRepoClick = (repoUrl: string) => {
  invoke('open_url', { url: repoUrl });
};

const createRepo = () => {
  invoke('open_url', { url: 'https://atomgit.com/project/new' });
};

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

const importRepos = () => {
  warning('功能开发中...');
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
              <p class="text-sm font-medium text-muted-foreground">远程仓库数</p>
              <p class="text-2xl font-bold text-foreground">{{ user.total_repos || 0 }}</p>
            </div>
            <div class="w-12 h-12 bg-primary/10 rounded-lg flex items-center justify-center">
              <Globe  class="w-6 h-6 text-primary" />
            </div>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardContent class="p-6" @click="() => { router.push('/local-repositories') }">
          <div class="flex items-center justify-between cursor-pointer">
            <div>
              <p class="text-sm font-medium text-muted-foreground">本地仓库数</p>
              <p class="text-2xl font-bold text-foreground">{{ repositoryCount }}</p>
            </div>
            <div class="w-12 h-12 bg-orange-500/10 rounded-lg flex items-center justify-center">
              <Folder class="w-6 h-6 text-orange-500" />
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
              <Users class="w-6 h-6 text-green-500" />
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
              <Heart class="w-6 h-6 text-blue-500" />
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
              <AlertTriangle class="w-12 h-12 mx-auto mb-2" />
              <p class="text-sm">加载活动数据失败</p>
            </div>
            <Button variant="outline" size="sm" @click="fetchRecentActivity">
              重试
            </Button>
          </div>

          <!-- 空状态 -->
          <div v-else-if="recentActivity.length === 0" class="text-center py-8">
            <div class="text-muted-foreground">
              <FileText class="w-12 h-12 mx-auto mb-2" />
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
                  <component
                    :is="getActivityIcon(activity.type).component"
                    :class="['w-3 h-3', getActivityIcon(activity.type).color]"
                  />
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
            <Button variant="outline" class="h-20 flex-col space-y-2 cursor-pointer" @click="createRepo()">
              <Plus class="w-6 h-6" />
              <span class="text-sm">新建仓库</span>
            </Button>
            <Button variant="outline" class="h-20 flex-col space-y-2 cursor-pointer" @click="handleCloneRepo">
            <Download class="w-6 h-6" />
              <span class="text-sm">克隆仓库</span>
            </Button>
            <Button variant="outline" class="h-20 flex-col space-y-2 cursor-pointer" @click="importRepos()">
              <UserPlus class="w-6 h-6" />
              <span class="text-sm">加入组织</span>
            </Button>
            <Button variant="outline" class="h-20 flex-col space-y-2 cursor-pointer" @click="() => { router.push('/settings') }">
              <Settings class="w-6 h-6" />
              <span class="text-sm">设置</span>
            </Button>
          </div>
        </CardContent>
      </Card>
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

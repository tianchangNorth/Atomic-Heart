<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { storeToRefs } from 'pinia';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { useIssueStore } from '@/stores/issue';
import { Separator } from '@/components/ui/separator';
import { useToast } from '@/components/ui/toast';
import type { Issue, Comment } from '@/types/issue'
import {
  ArrowLeft,
  Info,
  CheckCircle,
  Lock,
  Unlock,
  Edit,
  MessageSquare,
  Calendar,
  User,
  Tag,
  GitBranch,
  ExternalLink,
  Send,
  Trash2,
} from 'lucide-vue-next';

const issueStore = useIssueStore();
const { currentIssue } = storeToRefs(issueStore);
// 路由和状态
const route = useRoute();
const router = useRouter();
const { success, warning } = useToast();

// 状态管理
const loading = ref(true);
const issue = ref<Issue | null>(null);
const comments = ref<Comment[]>([]);
const newComment = ref('');
const isEditing = ref(false);
const editTitle = ref('');
const editBody = ref('');

// 从路由参数获取仓库信息
const owner = route.params.owner as string;
const repo = route.params.repo as string;
const number = route.params.number as string;

// 模拟数据
const mockIssue: Issue = {
  id: '1',
  number: 123,
  title: 'Bug: 应用在暗色主题下按钮颜色显示异常',
  body: `## 问题描述

在暗色主题模式下，主要按钮的颜色显示不正确，导致用户体验不佳。

## 重现步骤

1. 切换到暗色主题
2. 导航到设置页面
3. 观察主要按钮的颜色

## 预期行为

按钮应该在暗色主题下显示正确的颜色对比度。

## 实际行为

按钮颜色过于暗淡，难以识别。

## 环境信息

- 操作系统: Windows 11
- 浏览器: Chrome 120.0
- 应用版本: 1.0.0

## 截图

![image](https://via.placeholder.com/600x300/333/fff?text=Screenshot)

## 附加信息

这个问题可能与 CSS 变量的定义有关。`,
  state: 'open',
  locked: false,
  user: {
    id: '1',
    login: 'developer',
    avatar_url: 'https://github.com/github.png',
    html_url: 'https://github.com/developer'
  },
  assignee: {
    id: '2',
    login: 'maintainer',
    avatar_url: 'https://github.com/github.png',
    html_url: 'https://github.com/maintainer'
  },
  labels: [
    {
      id: '1',
      name: 'bug',
      color: 'd73a49',
      description: 'Something isn\'t working'
    },
    {
      id: '2',
      name: 'ui',
      color: '0075ca',
      description: 'User interface related'
    },
    {
      id: '3',
      name: 'high priority',
      color: 'e99695',
      description: 'High priority issue'
    }
  ],
  milestone: {
    id: '1',
    title: 'v1.1.0',
    description: 'Next minor release',
    state: 'open',
    due_on: '2024-02-15T00:00:00Z'
  },
  created_at: '2024-01-15T10:30:00Z',
  updated_at: '2024-01-16T14:20:00Z',
  closed_at: null,
  repository: {
    name: 'AtomDesk',
    full_name: 'user/AtomDesk',
    html_url: 'https://github.com/user/AtomDesk'
  },
  comments_count: 3
};

const mockComments: Comment[] = [
  {
    id: '1',
    body: '我也遇到了同样的问题。这确实影响了用户体验。',
    user: {
      id: '3',
      login: 'user1',
      avatar_url: 'https://github.com/github.png',
      html_url: 'https://github.com/user1'
    },
    created_at: '2024-01-15T11:00:00Z',
    updated_at: '2024-01-15T11:00:00Z'
  },
  {
    id: '2',
    body: '我正在调查这个问题。看起来是 CSS 变量 `--primary` 在暗色主题下的定义有问题。\n\n```css\n.dark {\n  --primary: oklch(0.984 0.003 247.858);\n}\n```\n\n我会尽快修复这个问题。',
    user: {
      id: '2',
      login: 'maintainer',
      avatar_url: 'https://github.com/github.png',
      html_url: 'https://github.com/maintainer'
    },
    created_at: '2024-01-15T15:30:00Z',
    updated_at: '2024-01-15T15:30:00Z'
  },
  {
    id: '3',
    body: '感谢快速响应！期待修复。',
    user: {
      id: '1',
      login: 'developer',
      avatar_url: 'https://github.com/github.png',
      html_url: 'https://github.com/developer'
    },
    created_at: '2024-01-16T09:15:00Z',
    updated_at: '2024-01-16T09:15:00Z'
  }
];


// 时间格式化
const formatDate = (dateString: string): string => {
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
    return date.toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  }
};

// 获取标签颜色
const getLabelStyle = (color: string) => {
  return {
    backgroundColor: `#${color}`,
    color: getContrastColor(color)
  };
};

// 计算对比色
const getContrastColor = (hexColor: string): string => {
  const r = parseInt(hexColor.substr(0, 2), 16);
  const g = parseInt(hexColor.substr(2, 2), 16);
  const b = parseInt(hexColor.substr(4, 2), 16);
  const brightness = (r * 299 + g * 587 + b * 114) / 1000;
  return brightness > 128 ? '#000000' : '#ffffff';
};

// 处理函数
const goBack = () => {
  router.back();
};

const toggleIssueState = () => {
  if (!issue.value) return;

  const newState = issue.value.state === 'open' ? 'closed' : 'open';
  issue.value.state = newState;

  if (newState === 'closed') {
    issue.value.closed_at = new Date().toISOString();
    success('Issue 已关闭');
  } else {
    issue.value.closed_at = null;
    success('Issue 已重新打开');
  }
};

const toggleLock = () => {
  if (!issue.value) return;

  issue.value.locked = !issue.value.locked;
  success(issue.value.locked ? 'Issue 已锁定' : 'Issue 已解锁');
};

const startEdit = () => {
  if (!issue.value) return;

  isEditing.value = true;
  editTitle.value = issue.value.title;
  editBody.value = issue.value.body;
};

const cancelEdit = () => {
  isEditing.value = false;
  editTitle.value = '';
  editBody.value = '';
};

const saveEdit = () => {
  if (!issue.value) return;

  issue.value.title = editTitle.value;
  issue.value.body = editBody.value;
  issue.value.updated_at = new Date().toISOString();

  isEditing.value = false;
  success('Issue 已更新');
};

const addComment = () => {
  if (!newComment.value.trim()) {
    warning('请输入评论内容');
    return;
  }

  const comment: Comment = {
    id: Date.now().toString(),
    body: newComment.value,
    user: {
      id: 'current-user',
      login: 'current-user',
      avatar_url: 'https://github.com/github.png',
      html_url: 'https://github.com/current-user'
    },
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString()
  };

  comments.value.push(comment);
  newComment.value = '';

  if (issue.value) {
    issue.value.comments_count++;
  }

  success('评论已添加');
};

const deleteComment = (commentId: string) => {
  const index = comments.value.findIndex(c => c.id === commentId);
  if (index > -1) {
    comments.value.splice(index, 1);
    if (issue.value) {
      issue.value.comments_count--;
    }
    success('评论已删除');
  }
};

const openExternalLink = (url: string) => {
  window.open(url, '_blank');
};

// 组件挂载
onMounted(() => {
  // 模拟加载数据
  setTimeout(() => {
    issue.value = mockIssue;
    comments.value = mockComments;
    loading.value = false;
  }, 1000);
});

// 组件挂载时获取数据
onMounted(() => {
  if (owner && repo) {
    issueStore.fetchIssueData(owner, repo, number);
  }
});

onUnmounted(() => {
  issueStore.resetState();
});
</script>

<template>
  <div class="space-y-6">
    <div>{{ currentIssue }}</div>
    <!-- 加载状态 -->
    <div v-if="loading" class="flex items-center justify-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
      <span class="ml-3 text-muted-foreground">加载 Issue 详情...</span>
    </div>

    <!-- Issue 详情 -->
    <div v-else-if="issue" class="space-y-6">
      <!-- 头部导航 -->
      <div class="flex items-center justify-between">
        <Button variant="outline" @click="goBack" class="cursor-pointer">
          <ArrowLeft class="w-4 h-4 mr-2" />
          返回
        </Button>
        
        <div class="flex items-center space-x-2">
          <Button 
            variant="outline" 
            @click="toggleLock" 
            :title="issue.locked ? '解锁 Issue' : '锁定 Issue'"
            class="cursor-pointer"
          >
            <Lock v-if="issue.locked" class="w-4 h-4 mr-2" />
            <Unlock v-else class="w-4 h-4 mr-2" />
            {{ issue.locked ? '解锁' : '锁定' }}
          </Button>
          
          <Button 
            variant="outline" 
            @click="startEdit" 
            v-if="!isEditing"
            class="cursor-pointer"
          >
            <Edit class="w-4 h-4 mr-2" />
            编辑
          </Button>
          
          <Button 
            :variant="issue.state === 'open' ? 'destructive' : 'default'" 
            @click="toggleIssueState"
            class="cursor-pointer"
          >
            <CheckCircle v-if="issue.state === 'open'" class="w-4 h-4 mr-2" />
            <Info v-else class="w-4 h-4 mr-2" />
            {{ issue.state === 'open' ? '关闭 Issue' : '重新打开' }}
          </Button>
          
          <Button 
            variant="outline" 
            @click="openExternalLink(issue.repository.html_url + '/issues/' + issue.number)"
            title="在 GitHub 中查看"
            class="cursor-pointer"
          >
            <ExternalLink class="w-4 h-4" />
          </Button>
        </div>
      </div>

      <!-- Issue 主要内容 -->
      <div class="grid grid-cols-1 lg:grid-cols-4 gap-6">
        <!-- 左侧主要内容 -->
        <div class="lg:col-span-3 space-y-6">
          <!-- Issue 标题和状态 -->
          <Card>
            <CardHeader>
              <div class="flex items-start justify-between">
                <div class="flex-1">
                  <div class="flex items-center space-x-3 mb-2">
                    <div
                      :class="[
                        'w-8 h-8 rounded-full flex items-center justify-center',
                        issue.state === 'open'
                          ? 'bg-green-100 text-green-600'
                          : 'bg-purple-100 text-purple-600'
                      ]"
                    >
                      <Info v-if="issue.state === 'open'" class="w-4 h-4" />
                      <CheckCircle v-else class="w-4 h-4" />
                    </div>
                    
                    <div>
                      <h1 v-if="!isEditing" class="text-2xl font-bold text-foreground">
                        {{ issue.title }}
                      </h1>
                      <Input 
                        v-else 
                        v-model="editTitle" 
                        class="text-2xl font-bold" 
                        placeholder="Issue 标题"
                      />
                      
                      <div class="flex items-center space-x-2 mt-1">
                        <span class="text-muted-foreground">#{{ issue.number }}</span>
                        <Badge :variant="issue.state === 'open' ? 'default' : 'secondary'">
                          {{ issue.state === 'open' ? '开放' : '已关闭' }}
                        </Badge>
                        <Badge v-if="issue.locked" variant="outline">
                          <Lock class="w-3 h-3 mr-1" />
                          锁定
                        </Badge>
                      </div>
                    </div>
                  </div>
                  
                  <!-- 元信息 -->
                  <div class="flex items-center flex-wrap gap-4 text-sm text-muted-foreground">
                    <div class="flex items-center space-x-1">
                      <Avatar class="w-5 h-5">
                        <AvatarImage :src="issue.user.avatar_url" :alt="issue.user.login" />
                        <AvatarFallback class="text-xs">
                          {{ issue.user.login.charAt(0).toUpperCase() }}
                        </AvatarFallback>
                      </Avatar>
                      <span>{{ issue.user.login }} 创建于 {{ formatDate(issue.created_at) }}</span>
                    </div>
                    
                    <span v-if="issue.updated_at !== issue.created_at">
                      更新于 {{ formatDate(issue.updated_at) }}
                    </span>
                    
                    <span v-if="issue.closed_at">
                      关闭于 {{ formatDate(issue.closed_at) }}
                    </span>
                    
                    <div class="flex items-center space-x-1">
                      <MessageSquare class="w-4 h-4" />
                      <span>{{ issue.comments_count }} 条评论</span>
                    </div>
                  </div>
                </div>
                
                <!-- 编辑操作按钮 -->
                <div v-if="isEditing" class="flex space-x-2">
                  <Button variant="outline" @click="cancelEdit" class="cursor-pointer">
                    取消
                  </Button>
                  <Button @click="saveEdit" class="cursor-pointer">
                    保存
                  </Button>
                </div>
              </div>
            </CardHeader>
            
            <CardContent>
              <!-- Issue 内容 -->
              <div v-if="!isEditing" class="prose prose-sm max-w-none">
                <div v-html="issue.body.replace(/\n/g, '<br>')" class="whitespace-pre-wrap"></div>
              </div>
              <Textarea 
                v-else 
                v-model="editBody" 
                placeholder="Issue 描述" 
                rows="10"
                class="min-h-[200px]"
              />
            </CardContent>
          </Card>

          <!-- 评论区域 -->
          <Card>
            <CardHeader>
              <CardTitle class="flex items-center space-x-2">
                <MessageSquare class="w-5 h-5" />
                <span>评论 ({{ comments.length }})</span>
              </CardTitle>
            </CardHeader>
            
            <CardContent class="space-y-6">
              <!-- 评论列表 -->
              <div v-for="comment in comments" :key="comment.id" class="space-y-4">
                <div class="flex items-start space-x-3">
                  <Avatar class="w-8 h-8">
                    <AvatarImage :src="comment.user.avatar_url" :alt="comment.user.login" />
                    <AvatarFallback>
                      {{ comment.user.login.charAt(0).toUpperCase() }}
                    </AvatarFallback>
                  </Avatar>
                  
                  <div class="flex-1 min-w-0">
                    <div class="bg-muted rounded-lg p-4">
                      <div class="flex items-center justify-between mb-2">
                        <div class="flex items-center space-x-2">
                          <span class="font-medium text-sm">{{ comment.user.login }}</span>
                          <span class="text-xs text-muted-foreground">
                            {{ formatDate(comment.created_at) }}
                          </span>
                        </div>
                        
                        <Button 
                          variant="ghost" 
                          size="sm" 
                          @click="deleteComment(comment.id)"
                          class="cursor-pointer"
                        >
                          <Trash2 class="w-3 h-3" />
                        </Button>
                      </div>
                      
                      <div class="prose prose-sm max-w-none">
                        <div v-html="comment.body.replace(/\n/g, '<br>')" class="whitespace-pre-wrap"></div>
                      </div>
                    </div>
                  </div>
                </div>
                
                <Separator v-if="comment.id !== comments[comments.length - 1]?.id" />
              </div>
              
              <!-- 添加评论 -->
              <div class="space-y-4">
                <Separator />
                
                <div class="flex items-start space-x-3">
                  <Avatar class="w-8 h-8">
                    <AvatarImage src="https://github.com/github.png" alt="current-user" />
                    <AvatarFallback>U</AvatarFallback>
                  </Avatar>
                  
                  <div class="flex-1 space-y-3">
                    <Textarea 
                      v-model="newComment" 
                      placeholder="添加评论..." 
                      rows="3"
                    />
                    
                    <div class="flex justify-end">
                      <Button @click="addComment" :disabled="!newComment.trim()" class="cursor-pointer">
                        <Send class="w-4 h-4 mr-2" />
                        发表评论
                      </Button>
                    </div>
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>

        <!-- 右侧边栏 -->
        <div class="space-y-6">
          <!-- 分配者 -->
          <Card>
            <CardHeader>
              <CardTitle class="text-sm flex items-center space-x-2">
                <User class="w-4 h-4" />
                <span>分配者</span>
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div v-if="issue.assignee" class="flex items-center space-x-2">
                <Avatar class="w-6 h-6">
                  <AvatarImage :src="issue.assignee.avatar_url" :alt="issue.assignee.login" />
                  <AvatarFallback class="text-xs">
                    {{ issue.assignee.login.charAt(0).toUpperCase() }}
                  </AvatarFallback>
                </Avatar>
                <span class="text-sm">{{ issue.assignee.login }}</span>
              </div>
              <span v-else class="text-sm text-muted-foreground">未分配</span>
            </CardContent>
          </Card>

          <!-- 标签 -->
          <Card>
            <CardHeader>
              <CardTitle class="text-sm flex items-center space-x-2">
                <Tag class="w-4 h-4" />
                <span>标签</span>
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div v-if="issue.labels.length > 0" class="flex flex-wrap gap-2">
                <Badge 
                  v-for="label in issue.labels" 
                  :key="label.id"
                  variant="outline"
                  :style="getLabelStyle(label.color)"
                  class="text-xs"
                >
                  {{ label.name }}
                </Badge>
              </div>
              <span v-else class="text-sm text-muted-foreground">无标签</span>
            </CardContent>
          </Card>

          <!-- 里程碑 -->
          <Card>
            <CardHeader>
              <CardTitle class="text-sm flex items-center space-x-2">
                <GitBranch class="w-4 h-4" />
                <span>里程碑</span>
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div v-if="issue.milestone">
                <div class="text-sm font-medium">{{ issue.milestone.title }}</div>
                <div class="text-xs text-muted-foreground mt-1">
                  {{ issue.milestone.description }}
                </div>
                <div v-if="issue.milestone.due_on" class="text-xs text-muted-foreground mt-1 flex items-center space-x-1">
                  <Calendar class="w-3 h-3" />
                  <span>截止: {{ formatDate(issue.milestone.due_on) }}</span>
                </div>
              </div>
              <span v-else class="text-sm text-muted-foreground">无里程碑</span>
            </CardContent>
          </Card>

          <!-- 仓库信息 -->
          <Card>
            <CardHeader>
              <CardTitle class="text-sm">仓库</CardTitle>
            </CardHeader>
            <CardContent>
              <div class="flex items-center justify-between">
                <span class="text-sm font-medium">{{ issue.repository.full_name }}</span>
                <Button 
                  variant="ghost" 
                  size="sm" 
                  @click="openExternalLink(issue.repository.html_url)"
                  class="cursor-pointer"
                >
                  <ExternalLink class="w-3 h-3" />
                </Button>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>

    <!-- 错误状态 -->
    <div v-else class="text-center py-12">
      <div class="text-muted-foreground mb-4">
        <Info class="w-12 h-12 mx-auto mb-2" />
        <p class="text-sm">Issue 不存在或已被删除</p>
      </div>
      <Button variant="outline" @click="goBack">
        <ArrowLeft class="w-4 h-4 mr-2" />
        返回
      </Button>
    </div>
  </div>
</template>

<style scoped>
.prose {
  color: inherit;
}

.prose h1,
.prose h2,
.prose h3,
.prose h4,
.prose h5,
.prose h6 {
  color: inherit;
  margin-top: 1.5em;
  margin-bottom: 0.5em;
}

.prose p {
  margin-top: 0.75em;
  margin-bottom: 0.75em;
}

.prose code {
  background-color: var(--color-muted);
  padding: 0.125rem 0.25rem;
  border-radius: 0.25rem;
  font-size: 0.875em;
}

.prose pre {
  background-color: var(--color-muted);
  padding: 1rem;
  border-radius: 0.5rem;
  overflow-x: auto;
}

.prose blockquote {
  border-left: 4px solid var(--color-border);
  padding-left: 1rem;
  margin: 1rem 0;
  font-style: italic;
}

.prose ul,
.prose ol {
  margin: 0.75em 0;
  padding-left: 1.5rem;
}

.prose li {
  margin: 0.25em 0;
}
</style>
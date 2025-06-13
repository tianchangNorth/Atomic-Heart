<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Input } from '@/components/ui/input';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Tabs, TabsList, TabsTrigger, TabsContent } from '@/components/ui/tabs';
import { $fetch } from '@/utils/fetch';

// Issue 数据结构定义
interface Issue {
  id: string;                    // Issue 唯一标识
  url: string;                   // API 地址
  number: number;                // Issue 编号
  state: "open" | "closed";      // Issue 状态
  title: string;                 // Issue 标题
  body: string;                  // Issue 内容描述
  user: {                        // 创建者信息
    id: string;
    login: string;
    url: string;
    avatar_url: string;
    html_url: string;
    type: "User";
  };
  assignee: {                    // 分配者信息
    id: string;
    login: string;
    url: string;
    avatar_url: string;
    html_url: string;
    type: "User";
  } | null;
  locked: boolean;               // 是否锁定
  repository_url: string;        // 仓库 API 地址
  html_url: string;              // Issue 网页地址
  closed_at: string | null;      // 关闭时间
  created_at: string;            // 创建时间
  updated_at: string;            // 更新时间
}

// 状态管理
const activeTab = ref('assigned'); // 'assigned' | 'created'
const activeFilter = ref('all'); // 'all' | 'open' | 'closed'
const searchQuery = ref('');
const loading = ref(false);
const error = ref<string | null>(null);

// Issues 数据
const assignedIssues = ref<Issue[]>([]);
const createdIssues = ref<Issue[]>([]);

// 当前显示的 Issues
const currentIssues = computed(() => {
  return activeTab.value === 'assigned' ? assignedIssues.value : createdIssues.value;
});

// 筛选后的 Issues
const filteredIssues = computed(() => {
  let filtered = currentIssues.value;

  // 按状态筛选
  if (activeFilter.value === 'open') {
    filtered = filtered.filter(issue => issue.state === 'open');
  } else if (activeFilter.value === 'closed') {
    filtered = filtered.filter(issue => issue.state === 'closed');
  }

  // 按搜索关键词筛选
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase();
    filtered = filtered.filter(issue =>
      issue.title.toLowerCase().includes(query) ||
      issue.body.toLowerCase().includes(query) ||
      issue.number.toString().includes(query)
    );
  }

  return filtered;
});

// 统计数据
const openCount = computed(() => currentIssues.value.filter(issue => issue.state === 'open').length);
const closedCount = computed(() => currentIssues.value.filter(issue => issue.state === 'closed').length);
const totalCount = computed(() => currentIssues.value.length);

// 获取分配给我的 Issues
const getAssignedIssues = async () => {
  loading.value = true;
  error.value = null;

  try {
    const { success, data } = await $fetch('/issues', {
      method: 'get',
      data: {
        filter: "assigned",
        page: 1,
        per_page: 100,
        state: "all"
      }
    });

    if (success && Array.isArray(data)) {
      assignedIssues.value = data;
    } else {
      // 没有数据时设置为空数组
      assignedIssues.value = [];
    }
  } catch (err) {
    console.error('获取分配的 Issues 失败:', err);
    error.value = '获取分配的 Issues 失败';
    // 错误时设置为空数组
    assignedIssues.value = [];
  } finally {
    loading.value = false;
  }
};

// 获取我创建的 Issues
const getCreatedIssues = async () => {
  loading.value = true;
  error.value = null;

  try {
    const { success, data } = await $fetch('/issues', {
      method: 'get',
      data: {
        filter: "created",
        page: 1,
        per_page: 100,
        state: "all"
      }
    });

    if (success && Array.isArray(data)) {
      createdIssues.value = data;
    } else {
      // 没有数据时设置为空数组
      createdIssues.value = [];
    }
  } catch (err) {
    console.error('获取创建的 Issues 失败:', err);
    error.value = '获取创建的 Issues 失败';
    // 错误时设置为空数组
    createdIssues.value = [];
  } finally {
    loading.value = false;
  }
};

// 时间格式化函数
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
    return date.toLocaleDateString('zh-CN');
  }
};

// 获取仓库名称
const getRepositoryName = (repositoryUrl: string): string => {
  const parts = repositoryUrl.split('/');
  return parts[parts.length - 1] || 'unknown';
};

// 处理操作
const handleIssueClick = (issue: Issue) => {
  console.log('查看 Issue:', issue.html_url);
  // 这里可以添加路由跳转逻辑
  // router.push(issue.html_url);
};

const handleCreateIssue = () => {
  console.log('创建新 Issue');
  // 这里可以添加创建 Issue 的逻辑
};

const handleRefresh = () => {
  if (activeTab.value === 'assigned') {
    getAssignedIssues();
  } else {
    getCreatedIssues();
  }
};

// 监听标签页切换
watch(activeTab, async (newTab) => {
  // 清除之前的错误状态
  error.value = null;

  if (newTab === 'assigned' && assignedIssues.value.length === 0) {
    await getAssignedIssues();
  } else if (newTab === 'created' && createdIssues.value.length === 0) {
    await getCreatedIssues();
  }
}, { immediate: false });

function extractPlainTextFromHtml(html: string): string {
  const div = document.createElement('div');
  div.innerHTML = html;
  return div.textContent?.trim() || '';
}

// 组件挂载时获取数据
onMounted(() => {
  getAssignedIssues();
});
</script>

<template>
  <div class="space-y-6">
    <!-- 页面标题 -->
    <div class="flex flex-col sm:flex-row sm:justify-between sm:items-center gap-4">
      <div>
        <h1 class="text-3xl font-bold text-foreground">Issues</h1>
        <p class="text-muted-foreground">跟踪和管理项目问题</p>
      </div>
      <div class="flex items-center space-x-2">
        <Button variant="outline" @click="handleRefresh" :disabled="loading">
          <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
          </svg>
          刷新
        </Button>
        <Button @click="handleCreateIssue">
          <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
          </svg>
          新建 Issue
        </Button>
      </div>
    </div>

    <!-- 分类标签页 -->
    <Tabs v-model:value="activeTab" default-value="assigned">
      <div class="flex flex-col sm:flex-row sm:justify-between sm:items-center gap-4">
        <TabsList>
          <TabsTrigger value="assigned">
            分配给我的
            <Badge variant="secondary" class="ml-2">
              {{ activeTab === 'assigned' ? totalCount : assignedIssues.length }}
            </Badge>
          </TabsTrigger>
          <TabsTrigger value="created">
            我创建的
            <Badge variant="secondary" class="ml-2">
              {{ activeTab === 'created' ? totalCount : createdIssues.length }}
            </Badge>
          </TabsTrigger>
        </TabsList>

        <!-- 搜索和筛选 -->
        <div class="flex items-center space-x-2">
          <div class="relative">
            <Input
              v-model="searchQuery"
              placeholder="搜索 Issues..."
              class="pl-8 w-64"
            />
            <svg class="absolute left-2 top-1/2 transform -translate-y-1/2 w-4 h-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
            </svg>
          </div>

          <div class="flex space-x-1">
            <Button
              variant="outline"
              size="sm"
              :class="activeFilter === 'all' ? 'bg-muted' : ''"
              @click="activeFilter = 'all'"
            >
              全部 ({{ totalCount }})
            </Button>
            <Button
              variant="outline"
              size="sm"
              :class="activeFilter === 'open' ? 'bg-muted' : ''"
              @click="activeFilter = 'open'"
            >
              开放 ({{ openCount }})
            </Button>
            <Button
              variant="outline"
              size="sm"
              :class="activeFilter === 'closed' ? 'bg-muted' : ''"
              @click="activeFilter = 'closed'"
            >
              已关闭 ({{ closedCount }})
            </Button>
          </div>
        </div>
      </div>

      <!-- 标签页内容 -->
      <TabsContent value="assigned" class="space-y-4">
        <!-- 加载状态 -->
        <div v-if="loading" class="flex items-center justify-center py-12">
          <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
          <span class="ml-3 text-muted-foreground">加载 Issues...</span>
        </div>

        <!-- 错误状态 -->
        <div v-else-if="error" class="text-center py-12">
          <div class="text-muted-foreground mb-4">
            <svg class="w-12 h-12 mx-auto mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.34 16.5c-.77.833.192 2.5 1.732 2.5z"/>
            </svg>
            <p class="text-sm">{{ error }}</p>
          </div>
          <Button variant="outline" @click="getAssignedIssues">
            <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
            </svg>
            重试
          </Button>
        </div>

        <!-- 空状态 -->
        <div v-else-if="filteredIssues.length === 0 && currentIssues.length === 0" class="text-center py-12">
          <svg class="w-16 h-16 mx-auto text-muted-foreground mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"/>
          </svg>
          <h3 class="text-lg font-semibold text-foreground mb-2">还没有 Issues</h3>
          <p class="text-muted-foreground mb-4">暂无分配给您的 Issues</p>
          <Button @click="handleCreateIssue">
            <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
            </svg>
            新建 Issue
          </Button>
        </div>

        <!-- 搜索无结果 -->
        <div v-else-if="filteredIssues.length === 0" class="text-center py-12">
          <svg class="w-16 h-16 mx-auto text-muted-foreground mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
          </svg>
          <h3 class="text-lg font-semibold text-foreground mb-2">未找到匹配的 Issues</h3>
          <p class="text-muted-foreground mb-4">尝试调整搜索条件或筛选器</p>
          <Button variant="outline" @click="searchQuery = ''; activeFilter = 'all'">
            清除筛选条件
          </Button>
        </div>

    <!-- Issues 列表 -->
    <div v-else class="space-y-4">
      <Card
        v-for="issue in filteredIssues"
        :key="issue.id"
        class="hover:shadow-md transition-shadow cursor-pointer"
        @click="handleIssueClick(issue)"
      >
        <CardContent class="p-6">
          <div class="flex items-start space-x-4">
            <!-- 状态图标 -->
            <div class="flex-shrink-0 mt-1">
              <div
                :class="[
                  'w-8 h-8 rounded-full flex items-center justify-center',
                  issue.state === 'open'
                    ? 'bg-green-100 text-green-600'
                    : 'bg-purple-100 text-purple-600'
                ]"
              >
                <svg v-if="issue.state === 'open'" class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"/>
                </svg>
                <svg v-else class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"/>
                </svg>
              </div>
            </div>

            <!-- Issue 内容 -->
            <div class="flex-1 min-w-0">
              <div class="flex items-start justify-between">
                <div class="flex-1 min-w-0">
                  <!-- 标题和状态 -->
                  <div class="flex items-center space-x-2 mb-2">
                    <h3 class="text-lg font-semibold text-foreground hover:text-primary transition-colors truncate">
                      {{ issue.title }}
                    </h3>
                    <Badge :variant="issue.state === 'open' ? 'default' : 'secondary'">
                      {{ issue.state === 'open' ? '开放' : '已关闭' }}
                    </Badge>
                    <Badge v-if="issue.locked" variant="outline">
                      <svg class="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
                      </svg>
                      锁定
                    </Badge>
                  </div>

                  <!-- 描述 -->         
                  <p class="text-muted-foreground mb-4 line-clamp-2">
                    {{ extractPlainTextFromHtml(issue.body) || '暂无描述' }}
                  </p>
                  
                  <!-- 元信息 -->
                  <div class="flex items-center flex-wrap gap-4 text-sm text-muted-foreground">
                    <span class="font-medium">#{{ issue.number }}</span>

                    <!-- 创建者信息 -->
                    <div class="flex items-center space-x-1">
                      <span>由</span>
                      <Avatar class="w-4 h-4">
                        <AvatarImage :src="issue.user.avatar_url" :alt="issue.user.login" />
                        <AvatarFallback class="text-xs">
                            {{ issue?.user?.login?.charAt(0)?.toUpperCase() || '?' }}
                        </AvatarFallback>
                      </Avatar>
                      <button
                        class="hover:text-primary transition-colors hover:underline"
                        @click.stop="console.log('查看用户:', issue.user.html_url)"
                      >
                        {{ issue.user.login }}
                      </button>
                      <span>创建</span>
                    </div>

                    <!-- 分配者信息 -->
                    <div v-if="issue.assignee" class="flex items-center space-x-1">
                      <span>分配给</span>
                      <Avatar class="w-4 h-4">
                        <AvatarImage :src="issue.assignee.avatar_url" :alt="issue.assignee.login" />
                        <AvatarFallback class="text-xs">
                          {{ issue.assignee?.login?.charAt(0)?.toUpperCase() || 'A' }}
                        </AvatarFallback>
                      </Avatar>
                      <button
                        class="hover:text-primary transition-colors hover:underline"
                        @click.stop="console.log('查看用户:', issue.assignee.html_url)"
                      >
                        {{ issue.assignee.login }}
                      </button>
                    </div>

                    <!-- 仓库信息 -->
                    <div class="flex items-center space-x-1">
                      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"/>
                      </svg>
                      <span>{{ getRepositoryName(issue.repository_url) }}</span>
                    </div>

                    <!-- 时间信息 -->
                    <span>创建于 {{ formatDate(issue.created_at) }}</span>
                    <span v-if="issue.updated_at !== issue.created_at">
                      更新于 {{ formatDate(issue.updated_at) }}
                    </span>
                    <span v-if="issue.closed_at">
                      关闭于 {{ formatDate(issue.closed_at) }}
                    </span>
                  </div>
                </div>

                <!-- 操作按钮 -->
                <div class="flex space-x-2 ml-4">
                  <Button
                    variant="outline"
                    size="sm"
                    @click.stop="handleIssueClick(issue)"
                    title="查看详情"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"/>
                    </svg>
                  </Button>
                </div>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
      </TabsContent>

      <TabsContent value="created" class="space-y-4">
        <!-- 加载状态 -->
        <div v-if="loading" class="flex items-center justify-center py-12">
          <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
          <span class="ml-3 text-muted-foreground">加载 Issues...</span>
        </div>

        <!-- 错误状态 -->
        <div v-else-if="error" class="text-center py-12">
          <div class="text-muted-foreground mb-4">
            <svg class="w-12 h-12 mx-auto mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.34 16.5c-.77.833.192 2.5 1.732 2.5z"/>
            </svg>
            <p class="text-sm">{{ error }}</p>
          </div>
          <Button variant="outline" @click="getCreatedIssues">
            <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
            </svg>
            重试
          </Button>
        </div>

        <!-- 空状态 -->
        <div v-else-if="filteredIssues.length === 0 && currentIssues.length === 0" class="text-center py-12">
          <svg class="w-16 h-16 mx-auto text-muted-foreground mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"/>
          </svg>
          <h3 class="text-lg font-semibold text-foreground mb-2">还没有 Issues</h3>
          <p class="text-muted-foreground mb-4">您还没有创建任何 Issues</p>
          <Button @click="handleCreateIssue">
            <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
            </svg>
            新建 Issue
          </Button>
        </div>

        <!-- 搜索无结果 -->
        <div v-else-if="filteredIssues.length === 0" class="text-center py-12">
          <svg class="w-16 h-16 mx-auto text-muted-foreground mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
          </svg>
          <h3 class="text-lg font-semibold text-foreground mb-2">未找到匹配的 Issues</h3>
          <p class="text-muted-foreground mb-4">尝试调整搜索条件或筛选器</p>
          <Button variant="outline" @click="searchQuery = ''; activeFilter = 'all'">
            清除筛选条件
          </Button>
        </div>

        <!-- Issues 列表 -->
        <div v-else class="space-y-4">
          <Card
            v-for="issue in filteredIssues"
            :key="issue.id"
            class="hover:shadow-md transition-shadow cursor-pointer"
            @click="handleIssueClick(issue)"
          >
            <CardContent class="p-6">
              <div class="flex items-start space-x-4">
                <!-- 状态图标 -->
                <div class="flex-shrink-0 mt-1">
                  <div
                    :class="[
                      'w-8 h-8 rounded-full flex items-center justify-center',
                      issue.state === 'open'
                        ? 'bg-green-100 text-green-600'
                        : 'bg-purple-100 text-purple-600'
                    ]"
                  >
                    <svg v-if="issue.state === 'open'" class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"/>
                    </svg>
                    <svg v-else class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"/>
                    </svg>
                  </div>
                </div>

                <!-- Issue 内容 -->
                <div class="flex-1 min-w-0">
                  <div class="flex items-start justify-between">
                    <div class="flex-1 min-w-0">
                      <!-- 标题和状态 -->
                      <div class="flex items-center space-x-2 mb-2">
                        <h3 class="text-lg font-semibold text-foreground hover:text-primary transition-colors truncate">
                          {{ issue.title }}
                        </h3>
                        <Badge :variant="issue.state === 'open' ? 'default' : 'secondary'">
                          {{ issue.state === 'open' ? '开放' : '已关闭' }}
                        </Badge>
                        <Badge v-if="issue.locked" variant="outline">
                          <svg class="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
                          </svg>
                          锁定
                        </Badge>
                      </div>

                      <!-- 描述 -->
                      <p class="text-muted-foreground mb-4 line-clamp-2">
                        {{ issue.body || '暂无描述' }}
                      </p>

                      <!-- 元信息 -->
                      <div class="flex items-center flex-wrap gap-4 text-sm text-muted-foreground">
                        <span class="font-medium">#{{ issue.number }}</span>

                        <!-- 创建者信息 -->
                        <div class="flex items-center space-x-1">
                          <span>由</span>
                          <Avatar class="w-4 h-4">
                            <AvatarImage :src="issue.user.avatar_url" :alt="issue.user.login" />
                            <AvatarFallback class="text-xs">
                              {{ issue.user?.login?.charAt(0)?.toUpperCase() || 'U' }}
                            </AvatarFallback>
                          </Avatar>
                          <button
                            class="hover:text-primary transition-colors hover:underline"
                            @click.stop="console.log('查看用户:', issue.user.html_url)"
                          >
                            {{ issue.user.login }}
                          </button>
                          <span>创建</span>
                        </div>

                        <!-- 分配者信息 -->
                        <div v-if="issue.assignee" class="flex items-center space-x-1">
                          <span>分配给</span>
                          <Avatar class="w-4 h-4">
                            <AvatarImage :src="issue.assignee.avatar_url" :alt="issue.assignee.login" />
                            <AvatarFallback class="text-xs">
                             {{ issue?.user?.login?.charAt(0)?.toUpperCase() || '?' }}
                            </AvatarFallback>
                          </Avatar>
                          <button
                            class="hover:text-primary transition-colors hover:underline"
                            @click.stop="console.log('查看用户:', issue.assignee.html_url)"
                          >
                            {{ issue.assignee.login }}
                          </button>
                        </div>

                        <!-- 仓库信息 -->
                        <div class="flex items-center space-x-1">
                          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"/>
                          </svg>
                          <span>{{ getRepositoryName(issue.repository_url) }}</span>
                        </div>

                        <!-- 时间信息 -->
                        <span>创建于 {{ formatDate(issue.created_at) }}</span>
                        <span v-if="issue.updated_at !== issue.created_at">
                          更新于 {{ formatDate(issue.updated_at) }}
                        </span>
                        <span v-if="issue.closed_at">
                          关闭于 {{ formatDate(issue.closed_at) }}
                        </span>
                      </div>
                    </div>

                    <!-- 操作按钮 -->
                    <div class="flex space-x-2 ml-4">
                      <Button
                        variant="outline"
                        size="sm"
                        @click.stop="handleIssueClick(issue)"
                        title="查看详情"
                      >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"/>
                        </svg>
                      </Button>
                    </div>
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>
      </TabsContent>
    </Tabs>
  </div>
</template>

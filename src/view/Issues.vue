<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Input } from '@/components/ui/input';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Tabs, TabsList, TabsTrigger, TabsContent } from '@/components/ui/tabs';
import { $fetch } from '@/utils/fetch';
import { invoke } from '@tauri-apps/api/core'
import { useToast } from '@/components/ui/toast';
import {
  RefreshCw,
  Plus,
  Search,
  AlertTriangle,
  ClipboardList,
  Info,
  CheckCircle,
  Lock,
  Database,
  ExternalLink
} from 'lucide-vue-next';

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
const { warning } = useToast();

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
  // 这里可以添加路由跳转逻辑
  invoke('open_url', { url: issue.html_url });
};

const handleCreateIssue = () => {
  warning('功能开发中...');
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

const extractPlainTextFromHtml = (html: string): string => {
  const div = document.createElement('div');
  div.innerHTML = html;
  return div.textContent?.trim() || '';
}

// 组件挂载时获取数据
onMounted(() => {
  getAssignedIssues();
  getCreatedIssues();
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
        <Button variant="outline" @click="handleRefresh" :disabled="loading" class="cursor-pointer">
          <RefreshCw class="w-4 h-4 mr-2" />
          刷新
        </Button>
        <Button @click="handleCreateIssue" class="cursor-pointer">
          <Plus class="w-4 h-4 mr-2" />
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
            <Search class="absolute left-2 top-1/2 transform -translate-y-1/2 w-4 h-4 text-muted-foreground" />
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
            <AlertTriangle class="w-12 h-12 mx-auto mb-2" />
            <p class="text-sm">{{ error }}</p>
          </div>
          <Button variant="outline" @click="getAssignedIssues">
            <RefreshCw class="w-4 h-4 mr-2" />
            重试
          </Button>
        </div>

        <!-- 空状态 -->
        <div v-else-if="filteredIssues.length === 0 && currentIssues.length === 0" class="text-center py-12">
          <ClipboardList class="w-16 h-16 mx-auto text-muted-foreground mb-4" />
          <h3 class="text-lg font-semibold text-foreground mb-2">还没有 Issues</h3>
          <p class="text-muted-foreground mb-4">暂无分配给您的 Issues</p>
          <Button @click="handleCreateIssue">
            <Plus class="w-4 h-4 mr-2" />
            新建 Issue
          </Button>
        </div>

        <!-- 搜索无结果 -->
        <div v-else-if="filteredIssues.length === 0" class="text-center py-12">
          <Search class="w-16 h-16 mx-auto text-muted-foreground mb-4" />
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
                <Info v-if="issue.state === 'open'" class="w-4 h-4" />
                <CheckCircle v-else class="w-4 h-4" />
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
                      <Lock class="w-3 h-3 mr-1" />
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
                      <Database class="w-4 h-4" />
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
                    <ExternalLink class="w-4 h-4" />
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
            <AlertTriangle class="w-12 h-12 mx-auto mb-2" />
            <p class="text-sm">{{ error }}</p>
          </div>
          <Button variant="outline" @click="getCreatedIssues">
            <RefreshCw class="w-4 h-4 mr-2" />
            重试
          </Button>
        </div>

        <!-- 空状态 -->
        <div v-else-if="filteredIssues.length === 0 && currentIssues.length === 0" class="text-center py-12">
          <ClipboardList class="w-16 h-16 mx-auto text-muted-foreground mb-4" />
          <h3 class="text-lg font-semibold text-foreground mb-2">还没有 Issues</h3>
          <p class="text-muted-foreground mb-4">您还没有创建任何 Issues</p>
          <Button @click="handleCreateIssue">
            <Plus class="w-4 h-4 mr-2" />
            新建 Issue
          </Button>
        </div>

        <!-- 搜索无结果 -->
        <div v-else-if="filteredIssues.length === 0" class="text-center py-12">
          <Search class="w-16 h-16 mx-auto text-muted-foreground mb-4" />
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
                    <Info v-if="issue.state === 'open'" class="w-4 h-4" />
                    <CheckCircle v-else class="w-4 h-4" />
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
                          <Lock class="w-3 h-3 mr-1" />
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
                          <Database class="w-4 h-4" />
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
                        <ExternalLink class="w-4 h-4" />
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

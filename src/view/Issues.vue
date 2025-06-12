<template>
  <div class="space-y-6">
    <!-- 页面标题 -->
    <div class="flex justify-between items-center">
      <div>
        <h1 class="text-3xl font-bold text-foreground">Issues</h1>
        <p class="text-muted-foreground">跟踪和管理项目问题</p>
      </div>
      <Button>
        <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
        </svg>
        新建 Issue
      </Button>
    </div>

    <!-- 筛选标签 -->
    <div class="flex space-x-4">
      <Button variant="outline" size="sm" :class="activeFilter === 'all' ? 'bg-muted' : ''" @click="activeFilter = 'all'">
        全部 ({{ issues.length }})
      </Button>
      <Button variant="outline" size="sm" :class="activeFilter === 'open' ? 'bg-muted' : ''" @click="activeFilter = 'open'">
        开放 ({{ openIssues.length }})
      </Button>
      <Button variant="outline" size="sm" :class="activeFilter === 'closed' ? 'bg-muted' : ''" @click="activeFilter = 'closed'">
        已关闭 ({{ closedIssues.length }})
      </Button>
    </div>

    <!-- Issues 列表 -->
    <div class="space-y-4">
      <Card v-for="issue in filteredIssues" :key="issue.id">
        <CardContent class="p-6">
          <div class="flex items-start space-x-4">
            <!-- 状态图标 -->
            <div class="flex-shrink-0 mt-1">
              <div v-if="issue.status === 'open'" class="w-6 h-6 bg-green-100 text-green-600 rounded-full flex items-center justify-center">
                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"/>
                </svg>
              </div>
              <div v-else class="w-6 h-6 bg-purple-100 text-purple-600 rounded-full flex items-center justify-center">
                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"/>
                </svg>
              </div>
            </div>

            <!-- Issue 内容 -->
            <div class="flex-1">
              <div class="flex items-start justify-between">
                <div class="flex-1">
                  <h3 class="text-lg font-semibold text-foreground mb-2">{{ issue.title }}</h3>
                  <p class="text-muted-foreground mb-3">{{ issue.description }}</p>
                  
                  <!-- 标签和元信息 -->
                  <div class="flex items-center space-x-4 text-sm text-muted-foreground">
                    <span>#{{ issue.number }}</span>
                    <span>由 {{ issue.author }} 创建</span>
                    <span>{{ formatDate(issue.created_at) }}</span>
                    <div class="flex space-x-2">
                      <span v-for="label in issue.labels" :key="label.name" 
                            class="px-2 py-1 rounded-full text-xs"
                            :style="{ backgroundColor: label.color + '20', color: label.color }">
                        {{ label.name }}
                      </span>
                    </div>
                  </div>
                </div>

                <!-- 操作按钮 -->
                <div class="flex space-x-2">
                  <Button variant="outline" size="sm">
                    <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"/>
                    </svg>
                    评论 ({{ issue.comments }})
                  </Button>
                  <Button variant="outline" size="sm">
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"/>
                    </svg>
                  </Button>
                </div>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>

    <!-- 空状态 -->
    <div v-if="filteredIssues.length === 0" class="text-center py-12">
      <svg class="w-16 h-16 mx-auto text-muted-foreground mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.34 16.5c-.77.833.192 2.5 1.732 2.5z"/>
      </svg>
      <h3 class="text-lg font-semibold text-foreground mb-2">没有找到 Issues</h3>
      <p class="text-muted-foreground mb-4">创建您的第一个 Issue 来开始跟踪项目问题</p>
      <Button>
        <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
        </svg>
        新建 Issue
      </Button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';

const activeFilter = ref('all');

// 模拟 Issues 数据
const issues = ref([
  {
    id: 1,
    number: 123,
    title: '修复登录页面的样式问题',
    description: '在移动设备上登录表单显示不正确，需要调整响应式布局',
    status: 'open',
    author: 'tianchangNorth',
    created_at: '2024-01-15T10:30:00Z',
    comments: 3,
    labels: [
      { name: 'bug', color: '#d73a49' },
      { name: 'frontend', color: '#0366d6' }
    ]
  },
  {
    id: 2,
    number: 124,
    title: '添加暗色主题支持',
    description: '用户请求添加暗色主题选项，提升夜间使用体验',
    status: 'open',
    author: 'developer',
    created_at: '2024-01-14T15:45:00Z',
    comments: 7,
    labels: [
      { name: 'enhancement', color: '#a2eeef' },
      { name: 'ui/ux', color: '#7057ff' }
    ]
  },
  {
    id: 3,
    number: 122,
    title: '优化数据库查询性能',
    description: '某些查询操作响应时间过长，需要优化索引和查询语句',
    status: 'closed',
    author: 'backend-dev',
    created_at: '2024-01-10T09:20:00Z',
    comments: 12,
    labels: [
      { name: 'performance', color: '#fbca04' },
      { name: 'backend', color: '#0e8a16' }
    ]
  }
]);

const openIssues = computed(() => issues.value.filter(issue => issue.status === 'open'));
const closedIssues = computed(() => issues.value.filter(issue => issue.status === 'closed'));

const filteredIssues = computed(() => {
  switch (activeFilter.value) {
    case 'open':
      return openIssues.value;
    case 'closed':
      return closedIssues.value;
    default:
      return issues.value;
  }
});

const formatDate = (dateString: string) => {
  const date = new Date(dateString);
  const now = new Date();
  const diffTime = Math.abs(now.getTime() - date.getTime());
  const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));
  
  if (diffDays === 1) return '1天前';
  if (diffDays < 7) return `${diffDays}天前`;
  if (diffDays < 30) return `${Math.ceil(diffDays / 7)}周前`;
  return date.toLocaleDateString('zh-CN');
};
</script>

<template>
  <div class="space-y-6">
    <!-- 页面标题 -->
    <div class="flex justify-between items-center">
      <div>
        <h1 class="text-3xl font-bold text-foreground">项目</h1>
        <p class="text-muted-foreground">管理您的项目和团队协作</p>
      </div>
      <Button>
        <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
        </svg>
        新建项目
      </Button>
    </div>

    <!-- 项目统计 -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <Card>
        <CardContent class="p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-muted-foreground">活跃项目</p>
              <p class="text-2xl font-bold text-foreground">{{ activeProjects.length }}</p>
            </div>
            <div class="w-12 h-12 bg-green-500/10 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
              </svg>
            </div>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardContent class="p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-muted-foreground">已完成项目</p>
              <p class="text-2xl font-bold text-foreground">{{ completedProjects.length }}</p>
            </div>
            <div class="w-12 h-12 bg-blue-500/10 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-blue-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z"/>
              </svg>
            </div>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardContent class="p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-muted-foreground">团队成员</p>
              <p class="text-2xl font-bold text-foreground">12</p>
            </div>
            <div class="w-12 h-12 bg-purple-500/10 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-purple-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/>
              </svg>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>

    <!-- 项目列表 -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <Card v-for="project in projects" :key="project.id">
        <CardContent class="p-6">
          <div class="flex justify-between items-start mb-4">
            <div>
              <h3 class="text-lg font-semibold text-foreground mb-2">{{ project.name }}</h3>
              <p class="text-muted-foreground text-sm">{{ project.description }}</p>
            </div>
            <span :class="getStatusClass(project.status)" class="px-2 py-1 rounded-full text-xs font-medium">
              {{ getStatusText(project.status) }}
            </span>
          </div>

          <!-- 进度条 -->
          <div class="mb-4">
            <div class="flex justify-between text-sm mb-2">
              <span class="text-muted-foreground">进度</span>
              <span class="font-medium">{{ project.progress }}%</span>
            </div>
            <div class="w-full bg-muted rounded-full h-2">
              <div 
                class="bg-primary h-2 rounded-full transition-all duration-300" 
                :style="{ width: project.progress + '%' }"
              ></div>
            </div>
          </div>

          <!-- 项目信息 -->
          <div class="grid grid-cols-2 gap-4 text-sm">
            <div>
              <p class="text-muted-foreground">开始日期</p>
              <p class="font-medium">{{ formatDate(project.start_date) }}</p>
            </div>
            <div>
              <p class="text-muted-foreground">截止日期</p>
              <p class="font-medium" :class="isOverdue(project.end_date) ? 'text-destructive' : ''">
                {{ formatDate(project.end_date) }}
              </p>
            </div>
          </div>

          <!-- 团队成员头像 -->
          <div class="flex items-center justify-between mt-4">
            <div class="flex -space-x-2">
              <img 
                v-for="member in project.members.slice(0, 4)" 
                :key="member.id"
                :src="member.avatar" 
                :alt="member.name"
                class="w-8 h-8 rounded-full border-2 border-background"
              >
              <div 
                v-if="project.members.length > 4"
                class="w-8 h-8 bg-muted rounded-full border-2 border-background flex items-center justify-center text-xs font-medium text-muted-foreground"
              >
                +{{ project.members.length - 4 }}
              </div>
            </div>
            <div class="flex space-x-2">
              <Button variant="outline" size="sm">
                <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"/>
                </svg>
                查看
              </Button>
              <Button variant="outline" size="sm">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                </svg>
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>

    <!-- 空状态 -->
    <div v-if="projects.length === 0" class="text-center py-12">
      <svg class="w-16 h-16 mx-auto text-muted-foreground mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"/>
      </svg>
      <h3 class="text-lg font-semibold text-foreground mb-2">还没有项目</h3>
      <p class="text-muted-foreground mb-4">创建您的第一个项目来开始团队协作</p>
      <Button>
        <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
        </svg>
        新建项目
      </Button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';

// 模拟项目数据
const projects = ref([
  {
    id: 1,
    name: 'AtomGit 桌面应用',
    description: '基于 Tauri 的现代化代码托管平台桌面客户端',
    status: 'active',
    progress: 75,
    start_date: '2024-01-01',
    end_date: '2024-03-01',
    members: [
      { id: 1, name: 'Alice', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Alice' },
      { id: 2, name: 'Bob', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Bob' },
      { id: 3, name: 'Charlie', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Charlie' }
    ]
  },
  {
    id: 2,
    name: 'API 重构项目',
    description: '重构后端 API 架构，提升性能和可维护性',
    status: 'completed',
    progress: 100,
    start_date: '2023-11-01',
    end_date: '2023-12-31',
    members: [
      { id: 4, name: 'David', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=David' },
      { id: 5, name: 'Eve', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Eve' }
    ]
  }
]);

const activeProjects = computed(() => projects.value.filter(p => p.status === 'active'));
const completedProjects = computed(() => projects.value.filter(p => p.status === 'completed'));

const getStatusClass = (status: string) => {
  switch (status) {
    case 'active':
      return 'bg-green-100 text-green-700';
    case 'completed':
      return 'bg-blue-100 text-blue-700';
    case 'paused':
      return 'bg-yellow-100 text-yellow-700';
    default:
      return 'bg-gray-100 text-gray-700';
  }
};

const getStatusText = (status: string) => {
  switch (status) {
    case 'active':
      return '进行中';
    case 'completed':
      return '已完成';
    case 'paused':
      return '已暂停';
    default:
      return '未知';
  }
};

const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleDateString('zh-CN');
};

const isOverdue = (endDate: string) => {
  return new Date(endDate) < new Date();
};
</script>

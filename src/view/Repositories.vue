<template>
  <div class="space-y-6">
    <!-- 页面标题 -->
    <div class="flex justify-between items-center">
      <div>
        <h1 class="text-3xl font-bold text-foreground">仓库</h1>
        <p class="text-muted-foreground">管理您的代码仓库</p>
      </div>
      <Button>
        <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
        </svg>
        新建仓库
      </Button>
    </div>

    <!-- 搜索和筛选 -->
    <div class="flex space-x-4">
      <div class="flex-1">
        <input 
          type="text" 
          placeholder="搜索仓库..." 
          class="w-full px-4 py-2 bg-background border border-border rounded-lg focus:outline-none focus:ring-2 focus:ring-ring"
        >
      </div>
      <Button variant="outline">
        <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.207A1 1 0 013 6.5V4z"/>
        </svg>
        筛选
      </Button>
    </div>

    <!-- 仓库列表 -->
    <div class="space-y-4">
      <Card v-for="repo in repositories" :key="repo.id">
        <CardContent class="p-6">
          <div class="flex justify-between items-start">
            <div class="flex-1">
              <div class="flex items-center space-x-2 mb-2">
                <h3 class="text-lg font-semibold text-foreground">{{ repo.name }}</h3>
                <span v-if="repo.private" class="px-2 py-1 bg-muted text-muted-foreground text-xs rounded-full">
                  私有
                </span>
                <span v-else class="px-2 py-1 bg-green-100 text-green-700 text-xs rounded-full">
                  公开
                </span>
              </div>
              <p class="text-muted-foreground mb-4">{{ repo.description || '暂无描述' }}</p>
              <div class="flex items-center space-x-6 text-sm text-muted-foreground">
                <div class="flex items-center space-x-1">
                  <div class="w-3 h-3 rounded-full" :style="{ backgroundColor: repo.language_color }"></div>
                  <span>{{ repo.language || 'Unknown' }}</span>
                </div>
                <div class="flex items-center space-x-1">
                  <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"/>
                  </svg>
                  <span>{{ repo.stars || 0 }}</span>
                </div>
                <div class="flex items-center space-x-1">
                  <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M7.707 3.293a1 1 0 010 1.414L5.414 7H11a7 7 0 017 7v2a1 1 0 11-2 0v-2a5 5 0 00-5-5H5.414l2.293 2.293a1 1 0 11-1.414 1.414L2.586 7l3.707-3.707a1 1 0 011.414 0z" clip-rule="evenodd"/>
                  </svg>
                  <span>{{ repo.forks || 0 }}</span>
                </div>
                <span>更新于 {{ formatDate(repo.updated_at) }}</span>
              </div>
            </div>
            <div class="flex space-x-2">
              <Button variant="outline" size="sm">
                <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"/>
                </svg>
                克隆
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
    <div v-if="repositories.length === 0" class="text-center py-12">
      <svg class="w-16 h-16 mx-auto text-muted-foreground mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"/>
      </svg>
      <h3 class="text-lg font-semibold text-foreground mb-2">还没有仓库</h3>
      <p class="text-muted-foreground mb-4">创建您的第一个仓库来开始使用 AtomGit</p>
      <Button>
        <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
        </svg>
        新建仓库
      </Button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';

// 模拟仓库数据
const repositories = ref([
  {
    id: 1,
    name: 'awesome-project',
    description: '一个很棒的开源项目，用于演示现代化的代码托管平台功能',
    private: false,
    language: 'TypeScript',
    language_color: '#3178c6',
    stars: 42,
    forks: 8,
    updated_at: '2024-01-15T10:30:00Z'
  },
  {
    id: 2,
    name: 'my-private-repo',
    description: '私有仓库，包含一些个人项目代码',
    private: true,
    language: 'Vue',
    language_color: '#4fc08d',
    stars: 0,
    forks: 0,
    updated_at: '2024-01-14T15:45:00Z'
  }
]);

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

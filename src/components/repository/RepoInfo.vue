<script setup lang="ts">
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import type { RepoInfoProps } from '@/types/repository';
import { useUserStore } from '@/stores';

defineProps<RepoInfoProps>();
const userStore = useUserStore();
const { user } = userStore;

// 格式化时间显示
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

// 格式化数字显示
const formatNumber = (num: number): string => {
  if (num >= 1000000) {
    return (num / 1000000).toFixed(1) + 'M';
  } else if (num >= 1000) {
    return (num / 1000).toFixed(1) + 'K';
  }
  return num.toString();
};
</script>

<template>
  <div class="bg-card rounded-lg border shadow-sm p-6">
    <!-- 仓库标题和基本信息 -->
    <div class="flex flex-col lg:flex-row lg:items-start lg:justify-between gap-6">
      <!-- 左侧：仓库信息 -->
      <div class="flex-1 min-w-0">
        <!-- 仓库名称和状态 -->
        <div class="flex items-center flex-wrap gap-3 mb-4">
          <h1 class="text-2xl font-bold text-foreground truncate">
            {{ repository.full_name }}
          </h1>
          
          <Badge :variant="repository.private ? 'secondary' : 'default'">
            <svg class="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path v-if="repository.private" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                    d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
              <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                    d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
            </svg>
            {{ repository.private ? '私有' : '公开' }}
          </Badge>

          <Badge v-if="repository.archived" variant="outline">
            <svg class="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                    d="M5 8l6 6 6-6"/>
            </svg>
            已归档
          </Badge>
        </div>

        <!-- 仓库描述 -->
        <p v-if="repository.description" class="text-muted-foreground mb-4 text-base leading-relaxed">
          {{ repository.description }}
        </p>
        <p v-else class="text-muted-foreground mb-4 italic">
          暂无描述
        </p>

        <!-- 仓库所有者信息 -->
        <div class="flex items-center space-x-3 mb-4">
          <Avatar class="h-8 w-8">
            <AvatarImage :src="user.avatar_url" :alt="user.login" />
            <AvatarFallback>{{ user.login }}</AvatarFallback>
          </Avatar>
          <div>
            <div class="font-medium text-foreground">{{ user.name ?? 'error' }}</div>
            <div class="text-sm text-muted-foreground">{{ repository?.owner?.type ?? 'Onwer' }}</div>
          </div>
        </div>

        <!-- 仓库统计信息 -->
        <div class="flex items-center flex-wrap gap-6 text-sm text-muted-foreground mb-4">
          <!-- 星标数 -->
          <div class="flex items-center space-x-1">
            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
              <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z"/>
            </svg>
            <span>{{ formatNumber(0) }} 星标</span>
          </div>

          <!-- 分叉数 -->
          <div class="flex items-center space-x-1">
            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M7.707 3.293a1 1 0 010 1.414L5.414 7H11a7 7 0 017 7v2a1 1 0 11-2 0v-2a5 5 0 00-5-5H5.414l2.293 2.293a1 1 0 11-1.414 1.414L2.586 7l3.707-3.707a1 1 0 011.414 0z" clip-rule="evenodd"/>
            </svg>
            <span>{{ formatNumber(repository.forksCount) }} 分叉</span>
          </div>

          <!-- 观察者数 -->
          <div class="flex items-center space-x-1">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"/>
            </svg>
            <span>{{ formatNumber(0) }} 观察</span>
          </div>

          <!-- 问题数 -->
          <div class="flex items-center space-x-1">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
            </svg>
            <span>{{ repository.open_issues_count }} 问题</span>
          </div>

          <!-- 主要语言 -->
          <div v-if="repository.language" class="flex items-center space-x-1">
            <div class="w-3 h-3 rounded-full bg-blue-500"></div>
            <span>{{ repository.language }}</span>
          </div>
        </div>

        <!-- 时间信息 -->
        <div class="flex items-center flex-wrap gap-4 text-sm text-muted-foreground">
          <span>创建于 {{ formatDate(repository.createdAt) }}</span>
          <span>更新于 {{ formatDate(repository.updatedAt) }}</span>
          <span v-if="repository.pushed_at">推送于 {{ formatDate(repository.pushed_at) }}</span>
        </div>

        <!-- 主题标签 -->
        <div v-if="repository.topics && repository.topics.length > 0" class="mt-4">
          <div class="flex flex-wrap gap-2">
            <Badge 
              v-for="topic in repository.topics" 
              :key="topic" 
              variant="outline"
              class="text-xs"
            >
              {{ topic }}
            </Badge>
          </div>
        </div>

        <!-- 许可证信息 -->
        <div v-if="repository.license" class="mt-4">
          <div class="flex items-center space-x-2 text-sm text-muted-foreground">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
            </svg>
            <span>许可证: {{ repository.license.name }}</span>
          </div>
        </div>
      </div>

      <!-- 右侧：操作按钮 -->
      <div class="flex flex-col space-y-3 lg:w-64">
        <!-- 克隆按钮组 -->
        <div class="space-y-2">
          <Button 
            class="w-full justify-start"
            variant="default"
          >
            <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"/>
            </svg>
            克隆 (HTTPS)
          </Button>
          
          <Button 
            class="w-full justify-start"
            variant="outline"
          >
            <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z"/>
            </svg>
            克隆 (SSH)
          </Button>
        </div>

        <!-- 其他操作 -->
        <div class="space-y-2">
          <Button 
            class="w-full justify-start"
            variant="outline"
          >
            <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
            </svg>
            下载 ZIP
          </Button>

          <Button 
            class="w-full justify-start"
            variant="outline"
          >
            <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"/>
            </svg>
            在线查看
          </Button>
        </div>
      </div>
    </div>
  </div>
</template>

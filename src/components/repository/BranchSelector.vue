<script setup lang="ts">
import { ref, computed } from 'vue';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Badge } from '@/components/ui/badge';
import type { BranchSelectorProps, BranchChangeEvent } from '@/types/repository';

const props = withDefaults(defineProps<BranchSelectorProps>(), {
  loading: false
});

const emit = defineEmits<{
  'branch-change': [event: BranchChangeEvent]
}>();

// 组件状态
const isOpen = ref(false);
const searchQuery = ref('');

// 计算属性
const currentBranchInfo = computed(() => {
  return props.branches.find(branch => branch.name === props.currentBranch);
});

const filteredBranches = computed(() => {
  if (!searchQuery.value) {
    return props.branches;
  }

  const query = searchQuery.value.toLowerCase();
  return props.branches.filter(branch =>
    branch.name.toLowerCase().includes(query)
  );
});

const defaultBranches = computed(() => {
  return filteredBranches.value.filter(branch =>
    ['main', 'master', 'develop', 'dev'].includes(branch.name.toLowerCase())
  );
});

const otherBranches = computed(() => {
  return filteredBranches.value.filter(branch =>
    !['main', 'master', 'develop', 'dev'].includes(branch.name.toLowerCase())
  );
});

// 方法
const toggleDropdown = () => {
  if (props.loading) return;
  isOpen.value = !isOpen.value;
  if (isOpen.value) {
    searchQuery.value = '';
  }
};

const selectBranch = (branchName: string) => {
  if (branchName === props.currentBranch) {
    isOpen.value = false;
    return;
  }

  const branch = props.branches.find(b => b.name === branchName);
  if (branch) {
    emit('branch-change', {
      branch: branchName,
      commit: branch.commit.sha
    });
  }

  isOpen.value = false;
};

// 安全获取提交消息
const getCommitMessage = (branch: any): string => {
  if (branch?.commit?.commit?.message) {
    return branch.commit.commit.message;
  }
  return '无提交消息';
};

// 安全获取提交作者信息
const getCommitAuthor = (branch: any): { name: string; date: string } | null => {
  if (branch?.commit?.commit?.author) {
    return branch.commit.commit.author;
  }
  if (branch?.commit?.author) {
    return branch.commit.author;
  }
  return null;
};

const formatCommitMessage = (message: string): string => {
  if (!message || message === '无提交消息') {
    return message;
  }
  return message.length > 50 ? message.substring(0, 50) + '...' : message;
};

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

// 点击外部关闭下拉框
const handleClickOutside = (event: Event) => {
  const target = event.target as HTMLElement;
  const dropdown = document.getElementById('branch-dropdown');
  if (dropdown && !dropdown.contains(target)) {
    isOpen.value = false;
  }
};

// 监听点击外部事件
if (typeof window !== 'undefined') {
  document.addEventListener('click', handleClickOutside);
}
</script>

<template>
  <div class="relative" id="branch-dropdown">
    <!-- 分支选择器按钮 -->
    <Button
      @click="toggleDropdown"
      :disabled="loading || branches.length === 0"
      variant="outline"
      class="min-w-[200px] justify-between"
    >
      <div class="flex items-center space-x-2">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
        </svg>
        <span v-if="loading">加载中...</span>
        <span v-else-if="branches.length === 0">无分支</span>
        <span v-else class="truncate">{{ currentBranch || '选择分支' }}</span>
      </div>
      
      <svg 
        class="w-4 h-4 transition-transform duration-200"
        :class="{ 'rotate-180': isOpen }"
        fill="none" 
        stroke="currentColor" 
        viewBox="0 0 24 24"
      >
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
      </svg>
    </Button>

    <!-- 下拉菜单 -->
    <div 
      v-if="isOpen && !loading && branches.length > 0"
      class="absolute top-full left-0 right-0 mt-1 bg-popover border border-border rounded-md shadow-lg z-50 max-h-96 overflow-hidden"
    >
      <!-- 搜索框 -->
      <div class="p-3 border-b border-border">
        <div class="relative">
          <Input
            v-model="searchQuery"
            placeholder="搜索分支..."
            class="pl-8"
          />
          <svg class="absolute left-2 top-1/2 transform -translate-y-1/2 w-4 h-4 text-muted-foreground" 
               fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                  d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
          </svg>
        </div>
      </div>

      <!-- 分支列表 -->
      <div class="max-h-80 overflow-y-auto">
        <!-- 当前分支信息 -->
        <div v-if="currentBranchInfo" class="p-3 bg-accent/50 border-b border-border">
          <div class="text-sm font-medium text-foreground mb-1">当前分支</div>
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-2">
              <svg class="w-4 h-4 text-green-600" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"/>
              </svg>
              <span class="font-medium">{{ currentBranchInfo.name }}</span>
              <Badge v-if="currentBranchInfo.protected" variant="secondary" class="text-xs">
                受保护
              </Badge>
            </div>
          </div>
          <div class="text-xs text-muted-foreground mt-1">
            {{ formatCommitMessage(getCommitMessage(currentBranchInfo)) }}
          </div>
          <div class="text-xs text-muted-foreground">
            <template v-if="getCommitAuthor(currentBranchInfo)">
              {{ formatDate(getCommitAuthor(currentBranchInfo)!.date) }} · {{ currentBranchInfo.commit.sha.substring(0, 7) }}
            </template>
            <template v-else>
              {{ currentBranchInfo.commit.sha.substring(0, 7) }}
            </template>
          </div>
        </div>

        <!-- 默认分支 -->
        <div v-if="defaultBranches.length > 0">
          <div class="px-3 py-2 text-xs font-medium text-muted-foreground bg-muted/50">
            默认分支
          </div>
          <div
            v-for="branch in defaultBranches"
            :key="branch.name"
            @click="selectBranch(branch.name)"
            class="px-3 py-3 hover:bg-accent cursor-pointer border-b border-border/50 last:border-b-0"
            :class="{ 'bg-accent': branch.name === currentBranch }"
          >
            <div class="flex items-center justify-between">
              <div class="flex items-center space-x-2 min-w-0 flex-1">
                <svg class="w-4 h-4 text-muted-foreground flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                        d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                </svg>
                <span class="font-medium truncate">{{ branch.name }}</span>
                <Badge v-if="branch.protected" variant="secondary" class="text-xs flex-shrink-0">
                  受保护
                </Badge>
              </div>
              <svg v-if="branch.name === currentBranch" class="w-4 h-4 text-green-600 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"/>
              </svg>
            </div>
            <div class="text-xs text-muted-foreground mt-1 ml-6">
              {{ formatCommitMessage(getCommitMessage(branch)) }}
            </div>
            <div class="text-xs text-muted-foreground ml-6">
              <template v-if="getCommitAuthor(branch)">
                {{ formatDate(getCommitAuthor(branch)!.date) }} · {{ branch.commit.sha.substring(0, 7) }}
              </template>
              <template v-else>
                {{ branch.commit.sha.substring(0, 7) }}
              </template>
            </div>
          </div>
        </div>

        <!-- 其他分支 -->
        <div v-if="otherBranches.length > 0">
          <div class="px-3 py-2 text-xs font-medium text-muted-foreground bg-muted/50">
            其他分支 ({{ otherBranches.length }})
          </div>
          <div
            v-for="branch in otherBranches"
            :key="branch.name"
            @click="selectBranch(branch.name)"
            class="px-3 py-3 hover:bg-accent cursor-pointer border-b border-border/50 last:border-b-0"
            :class="{ 'bg-accent': branch.name === currentBranch }"
          >
            <div class="flex items-center justify-between">
              <div class="flex items-center space-x-2 min-w-0 flex-1">
                <svg class="w-4 h-4 text-muted-foreground flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                        d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                </svg>
                <span class="font-medium truncate">{{ branch.name }}</span>
                <Badge v-if="branch.protected" variant="secondary" class="text-xs flex-shrink-0">
                  受保护
                </Badge>
              </div>
              <svg v-if="branch.name === currentBranch" class="w-4 h-4 text-green-600 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"/>
              </svg>
            </div>
            <div class="text-xs text-muted-foreground mt-1 ml-6">
              {{ formatCommitMessage(getCommitMessage(branch)) }}
            </div>
            <div class="text-xs text-muted-foreground ml-6">
              <template v-if="getCommitAuthor(branch)">
                {{ formatDate(getCommitAuthor(branch)!.date) }} · {{ branch.commit.sha.substring(0, 7) }}
              </template>
              <template v-else>
                {{ branch.commit.sha.substring(0, 7) }}
              </template>
            </div>
          </div>
        </div>

        <!-- 无搜索结果 -->
        <div v-if="filteredBranches.length === 0" class="p-6 text-center text-muted-foreground">
          <svg class="w-8 h-8 mx-auto mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                  d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
          </svg>
          <p class="text-sm">未找到匹配的分支</p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 确保下拉菜单在最上层 */
.z-50 {
  z-index: 50;
}
</style>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { Input } from '@/components/ui/input';
import type { FileTreeProps, FileTreeItem, FileSelectEvent, DirectoryToggleEvent } from '@/types/repository';

const props = withDefaults(defineProps<FileTreeProps>(), {
  currentPath: '',
  loading: false
});

const emit = defineEmits<{
  'file-select': [event: FileSelectEvent]
  'directory-toggle': [event: DirectoryToggleEvent]
}>();

// 组件状态
const searchQuery = ref('');

// 计算属性
const filteredItems = computed(() => {
  // 安全检查：确保 props.items 存在且为数组
  if (!props.items || !Array.isArray(props.items)) {
    return [];
  }

  if (!searchQuery.value) {
    return props.items;
  }

  const query = searchQuery.value.toLowerCase();
  return props.items.filter(item =>
    item && item.name && item.path &&
    (item.name.toLowerCase().includes(query) ||
      item.path.toLowerCase().includes(query))
  );
});

// 格式化文件大小
const formatFileSize = (bytes?: number): string => {
  if (!bytes) return '';

  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
};

// 格式化时间
const formatDate = (dateString?: string): string => {
  if (!dateString) return '';

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

// 处理文件/目录点击
const handleItemClick = (item: FileTreeItem) => {
  if (item.type === 'dir') {
    // 目录：切换展开状态
    emit('directory-toggle', {
      directory: item,
      expanded: !item.expanded
    });
  } else {
    // 文件：选择文件
    emit('file-select', {
      file: item,
      path: item.path
    });
  }
};

// 渲染文件树项目
const renderTreeItem = (item: FileTreeItem, level = 0) => {
  // 安全检查：确保 item 存在
  if (!item) {
    return {
      item: { name: '', path: '', type: 'file', sha: '', url: '', html_url: '' },
      level,
      isDirectory: false,
      isExpanded: false,
      isLoading: false,
      hasChildren: false
    };
  }

  return {
    item,
    level,
    isDirectory: item.type === 'dir',
    isExpanded: Boolean(item.expanded),
    isLoading: Boolean(item.loading),
    hasChildren: Boolean(item.children && Array.isArray(item.children) && item.children.length > 0)
  };
};

// 递归渲染文件树
const renderTree = (items: FileTreeItem[], level = 0): any[] => {
  // 安全检查：确保 items 存在且为数组
  if (!items || !Array.isArray(items)) {
    return [];
  }

  const result: any[] = [];

  for (const item of items) {
    if (!item) continue; // 跳过空项

    result.push(renderTreeItem(item, level));

    if (item.type === 'dir' && item.expanded && item.children && Array.isArray(item.children)) {
      result.push(...renderTree(item.children, level + 1));
    }
  }

  return result;
};

const treeItems = computed(() => renderTree(filteredItems.value));
</script>

<template>
  <div class="space-y-4">
    <!-- 搜索框 -->
    <div class="relative">
      <Input
        v-model="searchQuery"
        placeholder="搜索文件和文件夹..."
        class="pl-8"
      />
      <svg class="absolute left-2 top-1/2 transform -translate-y-1/2 w-4 h-4 text-muted-foreground" 
           fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
      </svg>
    </div>

    <!-- 文件树 -->
    <div class="space-y-1">
      <!-- 加载状态 -->
      <div v-if="loading" class="flex items-center justify-center py-8">
        <div class="animate-spin rounded-full h-6 w-6 border-b-2 border-primary"></div>
        <span class="ml-2 text-sm text-muted-foreground">加载文件列表...</span>
      </div>

      <!-- 空状态 -->
      <div v-else-if="filteredItems.length === 0" class="text-center py-8">
        <svg class="w-12 h-12 mx-auto text-muted-foreground mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"/>
        </svg>
        <p class="text-sm text-muted-foreground">
          {{ searchQuery ? '未找到匹配的文件' : '此目录为空' }}
        </p>
      </div>

      <!-- 文件树列表 -->
      <div v-else class="space-y-1">
        <div
          v-for="treeItem in treeItems"
          :key="treeItem.item.path"
          @click="handleItemClick(treeItem.item)"
          class="flex items-center space-x-2 px-2 py-1.5 rounded hover:bg-accent cursor-pointer group"
          :style="{ paddingLeft: `${treeItem.level * 20 + 8}px` }"
        >
          <!-- 展开/收起图标 -->
          <div class="w-4 h-4 flex items-center justify-center">
            <svg 
              v-if="treeItem.isDirectory"
              class="w-3 h-3 text-muted-foreground transition-transform"
              :class="{ 'rotate-90': treeItem.isExpanded }"
              fill="none" 
              stroke="currentColor" 
              viewBox="0 0 24 24"
            >
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
            </svg>
          </div>

          <!-- 文件/文件夹图标 -->
          <div class="w-4 h-4 flex-shrink-0">
            <!-- 文件夹图标 -->
            <svg v-if="treeItem.isDirectory" class="w-4 h-4 text-blue-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path v-if="treeItem.isExpanded" stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M5 19a2 2 0 01-2-2V7a2 2 0 012-2h4l2 2h4a2 2 0 012 2v1M5 19h14a2 2 0 002-2v-5a2 2 0 00-2-2H9a2 2 0 00-2 2v5a2 2 0 01-2 2z"/>
              <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"/>
            </svg>
            <!-- 文件图标 -->
            <svg v-else class="w-4 h-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
            </svg>
          </div>

          <!-- 加载图标 -->
          <div v-if="treeItem.isLoading" class="w-3 h-3 animate-spin border border-primary border-t-transparent rounded-full"></div>

          <!-- 文件/文件夹名称 -->
          <span 
            class="flex-1 text-sm truncate"
            :class="{
              'font-medium': treeItem.isDirectory,
              'text-foreground': true
            }"
          >
            {{ treeItem.item.path }}
          </span>

          <!-- 文件信息 -->
          <div v-if="!treeItem.isDirectory" class="flex items-center space-x-2 text-xs text-muted-foreground opacity-0 group-hover:opacity-100 transition-opacity">
            <span v-if="treeItem.item.size">{{ formatFileSize(treeItem.item.size) }}</span>
            <span v-if="treeItem.item.last_commit">{{ formatDate(treeItem.item.last_commit.author.date) }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- 路径导航 -->
    <div v-if="currentPath" class="text-xs text-muted-foreground border-t border-border pt-2">
      <span>当前路径: {{ currentPath || '/' }}</span>
    </div>
  </div>
</template>

<style scoped>
/* 确保图标正确显示 */
svg {
  flex-shrink: 0;
}

/* 悬停效果 */
.group:hover .opacity-0 {
  opacity: 1;
}
</style>

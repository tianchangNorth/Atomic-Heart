<script setup lang="ts">
import { computed } from 'vue';

interface Props {
  progress: number; // 0-100
  status: 'pending' | 'running' | 'success' | 'error';
  message?: string;
  showPercentage?: boolean;
  size?: 'sm' | 'md' | 'lg';
}

const props = withDefaults(defineProps<Props>(), {
  showPercentage: true,
  size: 'md'
});

const progressClasses = computed(() => {
  const baseClasses = 'transition-all duration-300 ease-in-out rounded-full';
  const statusClasses = {
    pending: 'bg-muted-foreground/30',
    running: 'bg-primary',
    success: 'bg-green-500',
    error: 'bg-red-500'
  };
  return `${baseClasses} ${statusClasses[props.status]}`;
});

const containerClasses = computed(() => {
  const sizeClasses = {
    sm: 'h-1',
    md: 'h-2',
    lg: 'h-3'
  };
  return `w-full bg-muted rounded-full overflow-hidden ${sizeClasses[props.size]}`;
});

const statusIcon = computed(() => {
  switch (props.status) {
    case 'running':
      return 'loading';
    case 'success':
      return 'check';
    case 'error':
      return 'x';
    default:
      return null;
  }
});

const statusColor = computed(() => {
  switch (props.status) {
    case 'running':
      return 'text-primary';
    case 'success':
      return 'text-green-600';
    case 'error':
      return 'text-red-600';
    default:
      return 'text-muted-foreground';
  }
});
</script>

<template>
  <div class="space-y-2">
    <!-- 状态信息 -->
    <div v-if="message || showPercentage" class="flex items-center justify-between text-sm">
      <div class="flex items-center space-x-2">
        <!-- 状态图标 -->
        <div v-if="statusIcon" :class="statusColor">
          <svg v-if="statusIcon === 'loading'" class="w-4 h-4 animate-spin" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
          </svg>
          <svg v-else-if="statusIcon === 'check'" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
          </svg>
          <svg v-else-if="statusIcon === 'x'" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
          </svg>
        </div>
        
        <!-- 状态消息 -->
        <span v-if="message" :class="statusColor">{{ message }}</span>
      </div>
      
      <!-- 进度百分比 -->
      <span v-if="showPercentage" class="text-muted-foreground font-mono text-xs">
        {{ Math.round(progress) }}%
      </span>
    </div>

    <!-- 进度条 -->
    <div :class="containerClasses">
      <div 
        :class="progressClasses"
        :style="{ width: `${Math.min(100, Math.max(0, progress))}%` }"
      >
        <!-- 动画效果（仅在运行状态下显示） -->
        <div 
          v-if="status === 'running'" 
          class="h-full w-full bg-gradient-to-r from-transparent via-white/20 to-transparent animate-pulse"
        ></div>
      </div>
    </div>
  </div>
</template>

<style scoped>
@keyframes shimmer {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(100%);
  }
}

.animate-shimmer {
  animation: shimmer 2s infinite;
}
</style>

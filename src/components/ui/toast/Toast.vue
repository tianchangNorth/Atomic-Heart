<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { CheckCircle, AlertCircle, Info, X } from 'lucide-vue-next';

export interface ToastProps {
  id: string;
  title?: string;
  description: string;
  variant?: 'default' | 'success' | 'error' | 'warning' | 'info';
  duration?: number;
  closable?: boolean;
}

const props = withDefaults(defineProps<ToastProps>(), {
  variant: 'default',
  duration: 5000,
  closable: true,
});

const emit = defineEmits<{
  close: [id: string];
}>();

const visible = ref(false);
const timer = ref<NodeJS.Timeout | null>(null);

// 计算样式
const toastClasses = computed(() => {
  const baseClasses = 'flex items-start gap-3 p-4 rounded-lg border shadow-lg transition-all duration-300 transform';
  const variantClasses = {
    default: 'bg-background border-border text-foreground',
    success: 'bg-green-50 border-green-200 text-green-800 dark:bg-green-900/20 dark:border-green-800 dark:text-green-200',
    error: 'bg-red-50 border-red-200 text-red-800 dark:bg-red-900/20 dark:border-red-800 dark:text-red-200',
    warning: 'bg-yellow-50 border-yellow-200 text-yellow-800 dark:bg-yellow-900/20 dark:border-yellow-800 dark:text-yellow-200',
    info: 'bg-blue-50 border-blue-200 text-blue-800 dark:bg-blue-900/20 dark:border-blue-800 dark:text-blue-200',
  };
  
  return `${baseClasses} ${variantClasses[props.variant]}`;
});

// 获取图标
const getIcon = () => {
  switch (props.variant) {
    case 'success':
      return CheckCircle;
    case 'error':
      return AlertCircle;
    case 'warning':
      return AlertCircle;
    case 'info':
      return Info;
    default:
      return Info;
  }
};

const IconComponent = getIcon();

// 关闭Toast
const close = () => {
  visible.value = false;
  setTimeout(() => {
    emit('close', props.id);
  }, 300);
};

// 自动关闭
const startTimer = () => {
  if (props.duration > 0) {
    timer.value = setTimeout(() => {
      close();
    }, props.duration);
  }
};

const stopTimer = () => {
  if (timer.value) {
    clearTimeout(timer.value);
    timer.value = null;
  }
};

onMounted(() => {
  // 延迟显示以触发动画
  setTimeout(() => {
    visible.value = true;
  }, 50);
  
  startTimer();
});
</script>

<template>
  <div
    :class="[
      toastClasses,
      visible ? 'translate-x-0 opacity-100' : 'translate-x-full opacity-0'
    ]"
    @mouseenter="stopTimer"
    @mouseleave="startTimer"
  >
    <!-- 图标 -->
    <div class="flex-shrink-0 mt-0.5">
      <IconComponent class="w-5 h-5" />
    </div>
    
    <!-- 内容 -->
    <div class="flex-1 min-w-0">
      <div v-if="title" class="font-medium text-sm mb-1">
        {{ title }}
      </div>
      <div class="text-sm">
        {{ description }}
      </div>
    </div>
    
    <!-- 关闭按钮 -->
    <button
      v-if="closable"
      @click="close"
      class="flex-shrink-0 p-1 rounded-md hover:bg-black/10 dark:hover:bg-white/10 transition-colors"
    >
      <X class="w-4 h-4" />
    </button>
  </div>
</template>

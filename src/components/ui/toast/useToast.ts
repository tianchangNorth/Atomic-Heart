import { ref } from 'vue';

export interface ToastOptions {
  title?: string;
  description: string;
  variant?: 'default' | 'success' | 'error' | 'warning' | 'info';
  duration?: number;
  closable?: boolean;
}

export interface Toast extends ToastOptions {
  id: string;
}

// 全局Toast状态
const toasts = ref<Toast[]>([]);

// 生成唯一ID
const generateId = () => {
  return Date.now().toString(36) + Math.random().toString(36).substr(2);
};

// Toast Store
export const useToastStore = () => {
  const addToast = (options: ToastOptions): string => {
    const id = generateId();
    const toast: Toast = {
      id,
      variant: 'default',
      duration: 5000,
      closable: true,
      ...options,
    };

    toasts.value.push(toast);

    // 限制最大显示数量
    if (toasts.value.length > 5) {
      toasts.value.shift();
    }

    return id;
  };

  const removeToast = (id: string) => {
    const index = toasts.value.findIndex(toast => toast.id === id);
    if (index > -1) {
      toasts.value.splice(index, 1);
    }
  };

  const clearAllToasts = () => {
    toasts.value = [];
  };

  return {
    toasts,
    addToast,
    removeToast,
    clearAllToasts,
  };
};

// 便捷的Toast函数
export const useToast = () => {
  const store = useToastStore();

  const toast = (options: ToastOptions) => {
    return store.addToast(options);
  };

  const success = (description: string, title?: string) => {
    return toast({
      title,
      description,
      variant: 'success',
    });
  };

  const error = (description: string, title?: string) => {
    return toast({
      title,
      description,
      variant: 'error',
    });
  };

  const warning = (description: string, title?: string) => {
    return toast({
      title,
      description,
      variant: 'warning',
    });
  };

  const info = (description: string, title?: string) => {
    return toast({
      title,
      description,
      variant: 'info',
    });
  };

  return {
    toast,
    success,
    error,
    warning,
    info,
    ...store,
  };
};

import { ref, watch, readonly } from 'vue';
import { Store } from '@tauri-apps/plugin-store';

export type ThemeMode = 'light' | 'dark';

interface ThemeConfig {
  mode: ThemeMode;
  autoDetect: boolean;
}

// 全局主题状态
const theme = ref<ThemeMode>('light');
const autoDetect = ref(false);
const isLoading = ref(true);

// Tauri Store 实例
let store: Store | null = null;

export const useTheme = () => {
  // 初始化 Store
  const initStore = async () => {
    try {
      store = await Store.load('settings.dat');
      await loadThemeConfig();
    } catch (error) {
      console.error('初始化主题存储失败:', error);
      // 使用默认主题
      await applyTheme('light');
      isLoading.value = false;
    }
  };

  // 加载主题配置
  const loadThemeConfig = async () => {
    try {
      if (!store) return;

      const savedConfig = await store.get<ThemeConfig>('theme');

      if (savedConfig) {
        theme.value = savedConfig.mode;
        autoDetect.value = savedConfig.autoDetect;

        // 如果启用自动检测，检查系统主题
        if (savedConfig.autoDetect) {
          await detectSystemTheme();
        } else {
          await applyTheme(savedConfig.mode);
        }
      } else {
        // 首次使用，检测系统主题
        await detectSystemTheme();
        autoDetect.value = true;
      }
    } catch (error) {
      console.error('加载主题配置失败:', error);
      await applyTheme('light');
    } finally {
      isLoading.value = false;
    }
  };

  // 保存主题配置
  const saveThemeConfig = async () => {
    try {
      if (!store) return;

      const config: ThemeConfig = {
        mode: theme.value,
        autoDetect: autoDetect.value
      };

      await store.set('theme', config);
      await store.save();
    } catch (error) {
      console.error('保存主题配置失败:', error);
    }
  };

  // 检测系统主题
  const detectSystemTheme = async () => {
    try {
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      const systemTheme: ThemeMode = prefersDark ? 'dark' : 'light';

      theme.value = systemTheme;
      await applyTheme(systemTheme);
    } catch (error) {
      console.error('检测系统主题失败:', error);
      await applyTheme('light');
    }
  };

  // 应用主题
  const applyTheme = async (mode: ThemeMode) => {
    try {
      const root = document.documentElement;

      // 移除现有主题类
      root.classList.remove('light', 'dark');

      // 添加新主题类
      root.classList.add(mode);

      // 设置 data 属性（用于 CSS 变量）
      root.setAttribute('data-theme', mode);

      // 更新状态
      theme.value = mode;

      console.log(`主题已切换到: ${mode}`);
    } catch (error) {
      console.error('应用主题失败:', error);
    }
  };

  // 切换主题
  const toggleTheme = async () => {
    const newTheme: ThemeMode = theme.value === 'light' ? 'dark' : 'light';
    await setTheme(newTheme);
  };

  // 设置主题
  const setTheme = async (mode: ThemeMode) => {
    await applyTheme(mode);
    autoDetect.value = false; // 手动设置时禁用自动检测
    await saveThemeConfig();
  };

  // 设置自动检测
  const setAutoDetect = async (enabled: boolean) => {
    autoDetect.value = enabled;

    if (enabled) {
      await detectSystemTheme();

      // 监听系统主题变化
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
      const handleChange = (e: MediaQueryListEvent) => {
        if (autoDetect.value) {
          const newTheme: ThemeMode = e.matches ? 'dark' : 'light';
          applyTheme(newTheme);
        }
      };

      mediaQuery.addEventListener('change', handleChange);
    }

    await saveThemeConfig();
  };

  // 获取主题预览样式
  const getThemePreview = (mode: ThemeMode) => {
    const previews = {
      light: {
        background: '#ffffff',
        foreground: '#0f172a',
        primary: '#3b82f6',
        secondary: '#f1f5f9',
        border: '#e2e8f0'
      },
      dark: {
        background: '#0f172a',
        foreground: '#f8fafc',
        primary: '#60a5fa',
        secondary: '#1e293b',
        border: '#334155'
      }
    };

    return previews[mode];
  };

  // 监听主题变化
  watch(theme, (newTheme) => {
    console.log('主题状态变化:', newTheme);
  });

  return {
    // 状态
    theme: readonly(theme),
    autoDetect: readonly(autoDetect),
    isLoading: readonly(isLoading),

    // 方法
    initStore,
    toggleTheme,
    setTheme,
    setAutoDetect,
    detectSystemTheme,
    getThemePreview
  };
};

// 全局初始化
export const initTheme = async () => {
  const { initStore } = useTheme();
  await initStore();
};

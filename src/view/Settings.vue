<script setup lang="ts">
import { computed } from 'vue';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { useTheme, type ThemeMode } from '@/composables/useTheme';
import {
  Captions
} from 'lucide-vue-next';
const {
  theme,
  autoDetect,
  isLoading,
  setTheme,
  setAutoDetect,
  getThemePreview
} = useTheme();

// 主题选项
const themeOptions = [
  {
    value: 'light' as ThemeMode,
    label: 'light',
    description: '适合白天使用的明亮界面',
    icon: '☀️'
  },
  {
    value: 'dark' as ThemeMode,
    label: 'dark',
    description: '适合夜间使用的深色界面',
    icon: '🌙'
  }
];

// 处理主题选择
const handleThemeChange = async (mode: ThemeMode) => {
  await setTheme(mode);
};

// 处理自动检测切换
const handleAutoDetectChange = async () => {
  await setAutoDetect(!autoDetect.value);
};

// 获取当前主题的预览样式
const currentThemePreview = computed(() => {
  return getThemePreview(theme.value);
});
</script>

<template>
  <div class="space-y-6">
    <!-- 页面标题 -->
    <div class="flex items-center space-x-3">
      <div class="w-8 h-8 bg-primary rounded-lg flex items-center justify-center">
        <svg class="w-5 h-5 text-primary-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
        </svg>
      </div>
      <div>
        <h1 class="text-3xl font-bold text-foreground">设置</h1>
        <p class="text-muted-foreground">个性化您的 AtomDesk 体验</p>
      </div>
    </div>

    <!-- 加载状态 -->
    <div v-if="isLoading" class="flex items-center justify-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
      <span class="ml-3 text-muted-foreground">加载设置...</span>
    </div>

    <!-- 设置内容 -->
    <div v-else class="space-y-6">
      <!-- 外观设置 -->
      <Card>
        <CardHeader>
          <CardTitle class="flex items-center space-x-2">
            <Captions class="size-6" />
            <span>外观设置</span>
          </CardTitle>
        </CardHeader>
        <CardContent class="space-y-6">
          <!-- 当前主题状态 -->
          <div class="flex items-center justify-between p-4 bg-muted rounded-lg">
            <div class="flex items-center space-x-3">
              <div 
                class="w-4 h-4 rounded-full border-2 border-border"
                :style="{ backgroundColor: currentThemePreview.primary }"
              ></div>
              <div>
                <p class="font-medium">当前主题</p>
                <p class="text-sm text-muted-foreground">
                  {{ theme === 'light' ? '亮色主题' : '暗色主题' }}
                  <Badge v-if="autoDetect" variant="secondary" class="ml-2">自动</Badge>
                </p>
              </div>
            </div>
            <div class="text-2xl">
              {{ theme === 'light' ? '☀️' : '🌙' }}
            </div>
          </div>

          <!-- 自动检测系统主题 -->
          <div class="flex items-center justify-between p-4 border border-border rounded-lg">
            <div class="flex-1">
              <h3 class="font-medium">跟随系统主题</h3>
              <p class="text-sm text-muted-foreground mt-1">
                自动根据系统设置切换亮色和暗色主题
              </p>
            </div>
            <Button
              :variant="autoDetect ? 'default' : 'outline'"
              size="sm"
              @click="handleAutoDetectChange"
              class="ml-4"
            >
              <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path v-if="autoDetect" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
              </svg>
              {{ autoDetect ? '已启用' : '已禁用' }}
            </Button>
          </div>

          <!-- 主题选择 -->
          <div class="space-y-3">
            <h3 class="font-medium">选择主题</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div
                v-for="option in themeOptions"
                :key="option.value"
                class="relative cursor-pointer transition-all duration-200 hover:scale-105"
                @click="handleThemeChange(option.value)"
              >
                <div 
                  class="p-4 border-2 rounded-lg transition-all duration-200"
                  :class="{
                    'border-primary bg-primary/5': theme === option.value && !autoDetect,
                    'border-border hover:border-primary/50': theme !== option.value || autoDetect
                  }"
                >
                  <!-- 主题预览 -->
                  <div class="mb-3">
                    <div 
                      class="w-full h-16 rounded-md border border-border overflow-hidden"
                      :style="{ backgroundColor: getThemePreview(option.value).background }"
                    >
                      <div class="h-4 flex">
                        <div 
                          class="flex-1"
                          :style="{ backgroundColor: getThemePreview(option.value).primary }"
                        ></div>
                        <div 
                          class="flex-1"
                          :style="{ backgroundColor: getThemePreview(option.value).secondary }"
                        ></div>
                      </div>
                      <div 
                        class="h-12 p-2"
                        :style="{ color: getThemePreview(option.value).foreground }"
                      >
                        <div class="text-xs font-medium">{{ option.label }}</div>
                        <div class="text-xs opacity-60 mt-1">示例文本</div>
                      </div>
                    </div>
                  </div>

                  <!-- 主题信息 -->
                  <div class="flex items-center justify-between">
                    <div>
                      <div class="flex items-center space-x-2">
                        <span class="text-lg">{{ option.icon }}</span>
                        <h4 class="font-medium">{{ option.label }}</h4>
                      </div>
                      <p class="text-sm text-muted-foreground mt-1">{{ option.description }}</p>
                    </div>
                    
                    <!-- 选中状态 -->
                    <div 
                      v-if="theme === option.value && !autoDetect"
                      class="w-5 h-5 bg-primary rounded-full flex items-center justify-center"
                    >
                      <svg class="w-3 h-3 text-primary-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                      </svg>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- 主题说明 -->
          <div class="p-4 bg-muted/50 rounded-lg">
            <div class="flex items-start space-x-3">
              <svg class="w-5 h-5 text-blue-500 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
              </svg>
              <div>
                <h4 class="font-medium text-sm">主题设置说明</h4>
                <p class="text-sm text-muted-foreground mt-1">
                  主题更改会立即应用到整个应用。启用"跟随系统主题"后，应用会自动根据您的操作系统设置切换主题。
                </p>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      <!-- 其他设置预留区域 -->
      <Card>
        <CardHeader>
          <CardTitle class="flex items-center space-x-2">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4"/>
            </svg>
            <span>其他设置</span>
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div class="text-center py-8 text-muted-foreground">
            <svg class="w-12 h-12 mx-auto mb-3 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"/>
            </svg>
            <p>更多设置功能即将推出</p>
          </div>
        </CardContent>
      </Card>
    </div>
  </div>
</template>

<style scoped>
/* 主题切换动画 */
.theme-transition {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

/* 主题预览悬停效果 */
.theme-option:hover .theme-preview {
  transform: scale(1.02);
}
</style>

<script setup lang="ts">
import { ref, computed, reactive } from 'vue';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import ProgressBar from './ui/ProgressBar.vue';
import type { CloneOptions, OperationProgress } from '@/types/git';

// 组件状态
const cloneForm = reactive<CloneOptions>({
  url: '',
  directory: '',
  branch: '',
  depth: '',
  recursive: false,
  auth: {
    type: 'password',
    username: '',
    password: ''
  }
});

const showAdvanced = ref(false);
const isCloning = ref(false);
const urlError = ref('');
const directoryError = ref('');

// 模拟克隆进度
const cloneProgress = ref<OperationProgress>({
  id: 'clone-1',
  type: 'clone',
  status: 'pending',
  progress: 0,
  message: '准备克隆...',
  startTime: new Date().toISOString()
});

// 计算属性
const isValidUrl = computed(() => {
  const urlPattern = /^(https?:\/\/|git@|ssh:\/\/)/;
  return urlPattern.test(cloneForm.url);
});

const canClone = computed(() => {
  return cloneForm.url && cloneForm.directory && isValidUrl.value && !isCloning.value;
});

const authTypeOptions = [
  { value: 'password', label: 'HTTP(S) 用户名/密码' },
  { value: 'token', label: 'Personal Access Token' },
  { value: 'ssh', label: 'SSH 密钥' }
];

// 方法
const validateUrl = () => {
  if (!cloneForm.url) {
    urlError.value = '';
    return;
  }

  if (!isValidUrl.value) {
    urlError.value = '请输入有效的 Git 仓库地址';
  } else {
    urlError.value = '';
  }
};

const selectDirectory = () => {
  // 这里将来会调用 Tauri API 选择目录
  console.log('选择目录');
};

const startClone = async () => {
  if (!canClone.value) return;

  isCloning.value = true;
  cloneProgress.value.status = 'running';
  cloneProgress.value.message = '正在克隆仓库...';

  // 模拟克隆进度
  const progressSteps = [
    { progress: 10, message: '连接到远程仓库...' },
    { progress: 30, message: '下载对象...' },
    { progress: 60, message: '解压对象...' },
    { progress: 80, message: '检出文件...' },
    { progress: 100, message: '克隆完成！' }
  ];

  for (const step of progressSteps) {
    await new Promise(resolve => setTimeout(resolve, 1000));
    cloneProgress.value.progress = step.progress;
    cloneProgress.value.message = step.message;
  }

  cloneProgress.value.status = 'success';
  cloneProgress.value.endTime = new Date().toISOString();

  setTimeout(() => {
    isCloning.value = false;
    resetForm();
  }, 2000);
};

const resetForm = () => {
  Object.assign(cloneForm, {
    url: '',
    directory: '',
    branch: '',
    depth: '',
    recursive: false,
    auth: {
      type: 'password',
      username: '',
      password: ''
    }
  });

  cloneProgress.value = {
    id: 'clone-1',
    type: 'clone',
    status: 'pending',
    progress: 0,
    message: '准备克隆...',
    startTime: new Date().toISOString()
  };

  urlError.value = '';
  directoryError.value = '';
  showAdvanced.value = false;
};
</script>

<template>
  <Card class="w-full max-w-2xl mx-auto">
    <CardHeader>
      <CardTitle class="flex items-center space-x-2">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
        </svg>
        <span>克隆仓库</span>
      </CardTitle>
    </CardHeader>
    
    <CardContent class="space-y-6">
      <!-- 基本信息 -->
      <div class="space-y-4">
        <!-- 仓库地址 -->
        <div class="space-y-2">
          <label class="text-sm font-medium">仓库地址 *</label>
          <Input
            v-model="cloneForm.url"
            placeholder="https://github.com/user/repo.git 或 git@github.com:user/repo.git"
            :class="urlError ? 'border-red-500' : ''"
            @blur="validateUrl"
          />
          <p v-if="urlError" class="text-sm text-red-600">{{ urlError }}</p>
          <div v-else-if="isValidUrl" class="flex items-center space-x-1 text-sm text-green-600">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
            </svg>
            <span>有效的仓库地址</span>
          </div>
        </div>

        <!-- 目标目录 -->
        <div class="space-y-2">
          <label class="text-sm font-medium">目标目录 *</label>
          <div class="flex space-x-2">
            <Input
              v-model="cloneForm.directory"
              placeholder="选择或输入目标目录路径"
              class="flex-1"
            />
            <Button variant="outline" @click="selectDirectory">
              <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"/>
              </svg>
              浏览
            </Button>
          </div>
        </div>
      </div>

      <!-- 认证信息 -->
      <div class="space-y-4 border-t pt-4">
        <h3 class="text-sm font-medium">认证信息</h3>
        
        <!-- 认证类型选择 -->
        <div class="space-y-2">
          <label class="text-sm font-medium">认证方式</label>
          <div class="flex flex-wrap gap-2">
            <Badge
              v-for="option in authTypeOptions"
              :key="option.value"
              :variant="cloneForm.auth?.type === option.value ? 'default' : 'outline'"
              class="cursor-pointer"
              @click="cloneForm.auth!.type = option.value as any"
            >
              {{ option.label }}
            </Badge>
          </div>
        </div>

        <!-- 用户名/密码 -->
        <div v-if="cloneForm.auth?.type === 'password'" class="grid grid-cols-2 gap-4">
          <div class="space-y-2">
            <label class="text-sm font-medium">用户名</label>
            <Input v-model="cloneForm.auth.username" placeholder="Git 用户名" />
          </div>
          <div class="space-y-2">
            <label class="text-sm font-medium">密码</label>
            <Input v-model="cloneForm.auth.password" type="password" placeholder="密码" />
          </div>
        </div>

        <!-- Token -->
        <div v-else-if="cloneForm.auth?.type === 'token'" class="space-y-2">
          <label class="text-sm font-medium">Personal Access Token</label>
          <Input v-model="cloneForm.auth.token" type="password" placeholder="ghp_xxxxxxxxxxxx" />
        </div>

        <!-- SSH 密钥 -->
        <div v-else-if="cloneForm.auth?.type === 'ssh'" class="space-y-2">
          <label class="text-sm font-medium">SSH 密钥路径</label>
          <div class="flex space-x-2">
            <Input v-model="cloneForm.auth.sshKeyPath" placeholder="~/.ssh/id_rsa" class="flex-1" />
            <Button variant="outline">选择</Button>
          </div>
        </div>
      </div>

      <!-- 高级选项 -->
      <div class="border-t pt-4">
        <Button
          variant="ghost"
          size="sm"
          @click="showAdvanced = !showAdvanced"
          class="mb-4"
        >
          <svg 
            class="w-4 h-4 mr-2 transition-transform"
            :class="{ 'rotate-90': showAdvanced }"
            fill="none" 
            stroke="currentColor" 
            viewBox="0 0 24 24"
          >
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
          </svg>
          高级选项
        </Button>

        <div v-if="showAdvanced" class="space-y-4 pl-6 border-l-2 border-border">
          <!-- 指定分支 -->
          <div class="space-y-2">
            <label class="text-sm font-medium">指定分支</label>
            <Input v-model="cloneForm.branch" placeholder="留空则使用默认分支" />
          </div>

          <!-- 克隆深度 -->
          <div class="space-y-2">
            <label class="text-sm font-medium">克隆深度</label>
            <Input
              v-model="cloneForm.depth"
              type="number"
              placeholder="留空则克隆完整历史"
              min="1"
            />
          </div>

          <!-- 递归克隆 -->
          <div class="flex items-center space-x-2">
            <input 
              id="recursive"
              v-model="cloneForm.recursive"
              type="checkbox"
              class="rounded border-border"
            />
            <label for="recursive" class="text-sm font-medium">递归克隆子模块</label>
          </div>
        </div>
      </div>

      <!-- 克隆进度 -->
      <div v-if="isCloning" class="border-t pt-4">
        <ProgressBar
          :progress="cloneProgress.progress"
          :status="cloneProgress.status"
          :message="cloneProgress.message"
        />
      </div>

      <!-- 操作按钮 -->
      <div class="flex justify-end space-x-3 border-t pt-4">
        <Button variant="outline" @click="resetForm" :disabled="isCloning">
          重置
        </Button>
        <Button @click="startClone" :disabled="!canClone">
          <svg v-if="isCloning" class="w-4 h-4 mr-2 animate-spin" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
          </svg>
          <svg v-else class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
          </svg>
          {{ isCloning ? '克隆中...' : '开始克隆' }}
        </Button>
      </div>
    </CardContent>
  </Card>
</template>

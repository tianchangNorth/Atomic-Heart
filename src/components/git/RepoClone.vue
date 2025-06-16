<script setup lang="ts">
import { ref, computed, reactive, onMounted, onUnmounted } from 'vue';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { open } from '@tauri-apps/plugin-dialog';
import { gitApi, gitCloneManager } from '@/services/git-api';
import { CloneOptionsBuilder, AuthConfigBuilder, formatBytes, formatDuration, extractRepoNameFromUrl } from '@/types/git-backend';
import type { CloneProgress, AuthType, CloneResult, AuthConfig } from '@/types/git-backend';

// 组件状态
const cloneForm = reactive({
  url: '',
  directory: '',
  branch: '',
  depth: undefined as number | undefined,
  recursive: false,
  authType: 'none' as AuthType,
  username: '',
  password: '',
  token: '',
  sshKeyPath: '',
  sshKeyPassphrase: ''
});

const showAdvanced = ref(false);
const isCloning = ref(false);
const urlError = ref('');
const directoryError = ref('');
const currentOperationId = ref<string | null>(null);
const defaultSshKeys = ref<string[]>([]);

// 克隆进度状态
const cloneProgress = ref<CloneProgress | null>(null);
const cloneResult = ref<CloneResult | null>(null);
const networkSpeed = ref<number>(0); // 网络速度 (bytes/s)
const lastProgressTime = ref<number>(0);
const lastReceivedBytes = ref<number>(0);

// 计算属性
const isValidUrl = computed(() => {
  if (!cloneForm.url) return false;
  const urlPattern = /^(https?:\/\/|git@|ssh:\/\/)/;
  return urlPattern.test(cloneForm.url);
});

const canClone = computed(() => {
  return cloneForm.url && cloneForm.directory && isValidUrl.value && !isCloning.value;
});

const authTypeOptions = [
  { value: 'none', label: '无认证（公开仓库）' },
  { value: 'password', label: 'HTTP(S) 用户名/密码' },
  { value: 'token', label: 'Personal Access Token' },
  { value: 'ssh', label: 'SSH 密钥' }
];

const suggestedDirectory = computed(() => {
  if (!cloneForm.url) return '';
  const repoName = extractRepoNameFromUrl(cloneForm.url);
  return `./projects/${repoName}`;
});

// 克隆阶段图标和描述
const getStageIcon = (stage: string) => {
  switch (stage) {
    case 'Initializing':
      return '🔄';
    case 'Connecting':
      return '🔗';
    case 'Downloading':
      return '⬇️';
    case 'Unpacking':
      return '📦';
    case 'CheckingOut':
      return '✅';
    case 'Completed':
      return '🎉';
    case 'Error':
      return '❌';
    default:
      return '⏳';
  }
};

const getStageDescription = (stage: string) => {
  switch (stage) {
    case 'Initializing':
      return '初始化仓库';
    case 'Connecting':
      return '连接远程仓库';
    case 'Downloading':
      return '下载对象';
    case 'Unpacking':
      return '解压对象';
    case 'CheckingOut':
      return '检出文件';
    case 'Completed':
      return '克隆完成';
    case 'Error':
      return '发生错误';
    default:
      return '处理中';
  }
};

const formatNetworkSpeed = (bytesPerSecond: number): string => {
  if (bytesPerSecond < 1024) return `${bytesPerSecond.toFixed(0)} B/s`;
  if (bytesPerSecond < 1024 * 1024) return `${(bytesPerSecond / 1024).toFixed(1)} KB/s`;
  return `${(bytesPerSecond / (1024 * 1024)).toFixed(1)} MB/s`;
};

// 生命周期
onMounted(async () => {
  // 加载默认 SSH 密钥
  try {
    defaultSshKeys.value = await gitApi.getDefaultSshKeys();
  } catch (error) {
    console.error('加载默认 SSH 密钥失败:', error);
  }

  // 自动检测认证类型
  if (cloneForm.url) {
    detectAuthType();
  }
});

onUnmounted(() => {
  // 清理资源
  if (currentOperationId.value) {
    gitCloneManager.cleanup(currentOperationId.value);
  }
});

// 方法
const validateUrl = async () => {
  if (!cloneForm.url) {
    urlError.value = '';
    return;
  }

  try {
    const isValid = await gitApi.validateRepositoryUrl(cloneForm.url);
    if (!isValid) {
      urlError.value = '请输入有效的 Git 仓库地址';
    } else {
      urlError.value = '';
      // 自动填充目录名
      if (!cloneForm.directory) {
        cloneForm.directory = suggestedDirectory.value;
      }
      // 检测认证类型
      await detectAuthType();
    }
  } catch (error) {
    urlError.value = '验证仓库地址时出错';
  }
};

const detectAuthType = async () => {
  if (!cloneForm.url) return;

  try {
    const authType = await gitApi.detectAuthType(cloneForm.url);
    cloneForm.authType = authType as AuthType;

    // 尝试加载已保存的凭据
    const savedAuth = await gitApi.loadCredentials(cloneForm.url);
    if (savedAuth) {
      cloneForm.authType = savedAuth.auth_type;
      cloneForm.username = savedAuth.username || '';
      cloneForm.password = savedAuth.password || '';
      cloneForm.token = savedAuth.token || '';
      cloneForm.sshKeyPath = savedAuth.ssh_key_path || '';
      cloneForm.sshKeyPassphrase = savedAuth.ssh_key_passphrase || '';
    }
  } catch (error) {
    console.error('检测认证类型失败:', error);
  }
};

const selectDirectory = async () => {
  try {
    const selectedPath = await open({
      directory: true,
      multiple: false,
      title: '选择克隆目标目录',
    });

    if (selectedPath) {
      cloneForm.directory = selectedPath;
      directoryError.value = '';

      // 验证目录
      await validateDirectory();
    }
  } catch (error) {
    console.error('选择目录失败:', error);
    directoryError.value = '选择目录失败';
  }
};

const validateDirectory = async () => {
  if (!cloneForm.directory) {
    directoryError.value = '';
    return;
  }

  try {
    const validation = await gitApi.validateCloneDirectory(cloneForm.directory);
    if (validation) {
      if (!validation.is_valid) {
        directoryError.value = validation.message;
      } else {
        directoryError.value = '';
        if (!validation.is_empty) {
          directoryError.value = '警告: ' + validation.message;
        }
      }
    }
  } catch (error) {
    console.error('验证目录失败:', error);
    directoryError.value = '验证目录时出错';
  }
};

const selectSshKeyFile = async () => {
  try {
    const selectedPath = await open({
      directory: false,
      multiple: false,
      title: '选择 SSH 私钥文件',
      filters: [
        {
          name: 'SSH 密钥文件',
          extensions: ['pem', 'key', 'rsa', 'ed25519', 'ecdsa', 'dsa']
        },
        {
          name: '所有文件',
          extensions: ['*']
        }
      ]
    });

    if (selectedPath) {
      cloneForm.sshKeyPath = selectedPath;

      // 验证 SSH 密钥
      await validateSshKey();
    }
  } catch (error) {
    console.error('选择 SSH 密钥文件失败:', error);
  }
};

const validateSshKey = async () => {
  if (!cloneForm.sshKeyPath) return;

  try {
    const isValid = await gitApi.validateSshKey(
      cloneForm.sshKeyPath,
      cloneForm.sshKeyPassphrase || undefined
    );

    if (!isValid) {
      console.warn('SSH 密钥验证失败');
    }
  } catch (error) {
    console.error('验证 SSH 密钥失败:', error);
  }
};

const startClone = async () => {
  if (!canClone.value) return;

  isCloning.value = true;
  cloneProgress.value = null;
  cloneResult.value = null;

  try {
    // 构建认证配置
    let authConfig: AuthConfig | undefined;
    if (cloneForm.authType !== 'none') {
      const authBuilder = new AuthConfigBuilder(cloneForm.authType);

      if (cloneForm.authType === 'password') {
        authBuilder.username(cloneForm.username).password(cloneForm.password);
      } else if (cloneForm.authType === 'token') {
        authBuilder.token(cloneForm.token);
        if (cloneForm.username) {
          authBuilder.username(cloneForm.username);
        }
      } else if (cloneForm.authType === 'ssh') {
        authBuilder.sshKey(cloneForm.sshKeyPath, cloneForm.sshKeyPassphrase || undefined);
        if (cloneForm.username) {
          authBuilder.username(cloneForm.username);
        }
      }

      authConfig = authBuilder.build();
    }

    // 构建克隆选项
    const optionsBuilder = new CloneOptionsBuilder(cloneForm.url, cloneForm.directory)
      .recursive(cloneForm.recursive);

    if (cloneForm.branch) {
      optionsBuilder.branch(cloneForm.branch);
    }

    if (cloneForm.depth) {
      optionsBuilder.depth(cloneForm.depth);
    }

    if (authConfig) {
      optionsBuilder.auth(authConfig);
    }

    const options = optionsBuilder.build();

    // 执行克隆
    currentOperationId.value = await gitCloneManager.cloneWithProgress(
      options,
      (progress) => {
        cloneProgress.value = progress;

        // 计算网络速度
        if (progress.network_progress) {
          const currentTime = Date.now();
          const currentBytes = progress.network_progress.received_bytes;

          if (lastProgressTime.value > 0 && lastReceivedBytes.value > 0) {
            const timeDiff = (currentTime - lastProgressTime.value) / 1000; // 秒
            const bytesDiff = currentBytes - lastReceivedBytes.value;

            if (timeDiff > 0) {
              networkSpeed.value = bytesDiff / timeDiff;
            }
          }

          lastProgressTime.value = currentTime;
          lastReceivedBytes.value = currentBytes;
        }
      },
      (result) => {
        cloneResult.value = result;
        isCloning.value = false;

        // 保存凭据（如果用户选择）
        if (authConfig && result.success) {
          gitApi.storeCredentials(cloneForm.url, authConfig).catch(console.error);
        }
      },
      (error) => {
        console.error('克隆失败:', error);
        isCloning.value = false;
        cloneProgress.value = {
          id: currentOperationId.value || 'error',
          stage: 'Error',
          progress: 0,
          message: `克隆失败: ${error.message}`,
        };
      }
    );
  } catch (error) {
    console.error('启动克隆失败:', error);
    isCloning.value = false;
    cloneProgress.value = {
      id: 'error',
      stage: 'Error',
      progress: 0,
      message: `启动克隆失败: ${error}`,
    };
  }
};

const cancelClone = async () => {
  if (currentOperationId.value) {
    try {
      await gitCloneManager.cancelClone(currentOperationId.value);
      isCloning.value = false;
      cloneProgress.value = {
        id: currentOperationId.value,
        stage: 'Error',
        progress: 0,
        message: '克隆已取消',
      };
    } catch (error) {
      console.error('取消克隆失败:', error);
    }
  }
};

const resetForm = () => {
  Object.assign(cloneForm, {
    url: '',
    directory: '',
    branch: '',
    depth: undefined,
    recursive: false,
    authType: 'none' as AuthType,
    username: '',
    password: '',
    token: '',
    sshKeyPath: '',
    sshKeyPassphrase: ''
  });

  cloneProgress.value = null;
  cloneResult.value = null;
  urlError.value = '';
  directoryError.value = '';
  showAdvanced.value = false;
  currentOperationId.value = null;
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
              :class="directoryError ? 'border-red-500' : ''"
              class="flex-1"
              @blur="validateDirectory"
            />
            <Button variant="outline" @click="selectDirectory">
              <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"/>
              </svg>
              浏览
            </Button>
          </div>
          <p v-if="directoryError" class="text-sm" :class="directoryError.startsWith('警告') ? 'text-yellow-600' : 'text-red-600'">
            {{ directoryError }}
          </p>
          <div v-else-if="cloneForm.directory && !directoryError" class="flex items-center space-x-1 text-sm text-green-600">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
            </svg>
            <span>目录可用</span>
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
              :variant="cloneForm.authType === option.value ? 'default' : 'outline'"
              class="cursor-pointer"
              @click="cloneForm.authType = option.value as AuthType"
            >
              {{ option.label }}
            </Badge>
          </div>
        </div>

        <!-- 用户名/密码 -->
        <div v-if="cloneForm.authType === 'password'" class="grid grid-cols-2 gap-4">
          <div class="space-y-2">
            <label class="text-sm font-medium">用户名</label>
            <Input v-model="cloneForm.username" placeholder="Git 用户名" />
          </div>
          <div class="space-y-2">
            <label class="text-sm font-medium">密码</label>
            <Input v-model="cloneForm.password" type="password" placeholder="密码" />
          </div>
        </div>

        <!-- Token -->
        <div v-else-if="cloneForm.authType === 'token'" class="space-y-2">
          <label class="text-sm font-medium">Personal Access Token</label>
          <Input v-model="cloneForm.token" type="password" placeholder="ghp_xxxxxxxxxxxx" />
          <div class="space-y-2">
            <label class="text-sm font-medium">用户名（可选）</label>
            <Input v-model="cloneForm.username" placeholder="Git 用户名" />
          </div>
        </div>

        <!-- SSH 密钥 -->
        <div v-else-if="cloneForm.authType === 'ssh'" class="space-y-4">
          <div class="space-y-2">
            <label class="text-sm font-medium">SSH 密钥路径</label>
            <div class="flex space-x-2">
              <Input
                v-model="cloneForm.sshKeyPath"
                placeholder="~/.ssh/id_rsa"
                class="flex-1"
                @blur="validateSshKey"
              />
              <Button variant="outline" @click="selectSshKeyFile">
                <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
                </svg>
                选择
              </Button>
            </div>
            <!-- 显示默认 SSH 密钥选项 -->
            <div v-if="defaultSshKeys.length > 0" class="space-y-1">
              <p class="text-xs text-muted-foreground">常用密钥:</p>
              <div class="flex flex-wrap gap-1">
                <Button
                  v-for="keyPath in defaultSshKeys"
                  :key="keyPath"
                  variant="ghost"
                  size="sm"
                  class="text-xs h-6 px-2"
                  @click="cloneForm.sshKeyPath = keyPath"
                >
                  {{ keyPath.split('/').pop() }}
                </Button>
              </div>
            </div>
          </div>
          <div class="space-y-2">
            <label class="text-sm font-medium">密钥密码（可选）</label>
            <Input
              v-model="cloneForm.sshKeyPassphrase"
              type="password"
              placeholder="SSH 密钥密码"
              @blur="validateSshKey"
            />
          </div>
          <div class="space-y-2">
            <label class="text-sm font-medium">用户名（可选）</label>
            <Input v-model="cloneForm.username" placeholder="git" />
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
        <div class="space-y-4">
          <!-- 阶段指示器 -->
          <div class="flex items-center justify-center space-x-4 py-2">
            <div
              v-for="stage in ['Initializing', 'Connecting', 'Downloading', 'CheckingOut', 'Completed']"
              :key="stage"
              class="flex flex-col items-center space-y-1"
            >
              <div
                class="w-8 h-8 rounded-full flex items-center justify-center text-sm transition-all duration-300"
                :class="{
                  'bg-primary text-primary-foreground': cloneProgress?.stage === stage,
                  'bg-green-500 text-white': cloneProgress && ['Initializing', 'Connecting', 'Downloading', 'CheckingOut'].indexOf(stage) < ['Initializing', 'Connecting', 'Downloading', 'CheckingOut'].indexOf(cloneProgress.stage),
                  'bg-muted text-muted-foreground': !cloneProgress || ['Initializing', 'Connecting', 'Downloading', 'CheckingOut'].indexOf(stage) > ['Initializing', 'Connecting', 'Downloading', 'CheckingOut'].indexOf(cloneProgress.stage)
                }"
              >
                <span v-if="cloneProgress?.stage === stage" class="animate-pulse">
                  {{ getStageIcon(stage) }}
                </span>
                <span v-else>
                  {{ getStageIcon(stage) }}
                </span>
              </div>
              <span class="text-xs text-center">{{ getStageDescription(stage) }}</span>
            </div>
          </div>

          <!-- 进度条和详细信息 -->
          <div v-if="cloneProgress" class="space-y-3">
            <div class="flex items-center justify-between">
              <span class="text-sm font-medium flex items-center space-x-2">
                <span class="animate-pulse">{{ getStageIcon(cloneProgress.stage) }}</span>
                <span>{{ cloneProgress.message }}</span>
              </span>
              <span class="text-sm text-muted-foreground">{{ cloneProgress.progress }}%</span>
            </div>

            <!-- 进度条 -->
            <div class="w-full bg-secondary rounded-full h-3 overflow-hidden">
              <div
                class="h-full rounded-full transition-all duration-500 ease-out"
                :class="{
                  'bg-gradient-to-r from-blue-500 to-blue-600': cloneProgress.stage === 'Downloading',
                  'bg-gradient-to-r from-green-500 to-green-600': cloneProgress.stage === 'CheckingOut',
                  'bg-gradient-to-r from-purple-500 to-purple-600': cloneProgress.stage === 'Connecting',
                  'bg-primary': !['Downloading', 'CheckingOut', 'Connecting'].includes(cloneProgress.stage)
                }"
                :style="{ width: `${cloneProgress.progress}%` }"
              >
                <!-- 动画效果 -->
                <div
                  v-if="cloneProgress.progress < 100"
                  class="h-full w-full bg-gradient-to-r from-transparent via-white/20 to-transparent animate-pulse"
                ></div>
              </div>
            </div>

            <!-- 网络进度详情 -->
            <div v-if="cloneProgress.network_progress" class="grid grid-cols-2 gap-4 text-xs text-muted-foreground">
              <div class="space-y-1">
                <div class="flex justify-between">
                  <span>已下载:</span>
                  <span class="font-mono">{{ formatBytes(cloneProgress.network_progress.received_bytes) }}</span>
                </div>
                <div class="flex justify-between">
                  <span>对象:</span>
                  <span class="font-mono">{{ cloneProgress.network_progress.received_objects }}/{{ cloneProgress.network_progress.total_objects }}</span>
                </div>
              </div>
              <div class="space-y-1">
                <div class="flex justify-between">
                  <span>已索引:</span>
                  <span class="font-mono">{{ cloneProgress.network_progress.indexed_objects }}</span>
                </div>
                <div v-if="networkSpeed > 0" class="flex justify-between">
                  <span>速度:</span>
                  <span class="font-mono text-blue-600">{{ formatNetworkSpeed(networkSpeed) }}</span>
                </div>
              </div>
            </div>
          </div>

          <!-- 无进度信息时的脉冲动画 -->
          <div v-else class="space-y-3">
            <div class="flex items-center justify-between">
              <span class="text-sm font-medium">正在准备克隆...</span>
              <div class="flex space-x-1">
                <div class="w-2 h-2 bg-primary rounded-full animate-pulse"></div>
                <div class="w-2 h-2 bg-primary rounded-full animate-pulse" style="animation-delay: 0.2s"></div>
                <div class="w-2 h-2 bg-primary rounded-full animate-pulse" style="animation-delay: 0.4s"></div>
              </div>
            </div>
            <div class="w-full bg-secondary rounded-full h-3 overflow-hidden">
              <div class="h-full bg-gradient-to-r from-primary/50 to-primary animate-pulse"></div>
            </div>
          </div>
        </div>
      </div>

      <!-- 克隆结果 -->
      <div v-if="cloneResult" class="border-t pt-4">
        <div v-if="cloneResult.success" class="p-4 bg-green-50 border border-green-200 rounded-lg">
          <div class="flex items-center space-x-2 text-green-800">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
            </svg>
            <span class="font-medium">克隆成功！</span>
          </div>
          <div class="mt-2 text-sm text-green-700">
            <p>仓库已克隆到: {{ cloneResult.repository_path }}</p>
            <p v-if="cloneResult.stats">
              耗时: {{ formatDuration(cloneResult.stats.duration_ms) }}
              | 文件数: {{ cloneResult.stats.file_count }}
            </p>
          </div>
        </div>
        <div v-else class="p-4 bg-red-50 border border-red-200 rounded-lg">
          <div class="flex items-center space-x-2 text-red-800">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
            </svg>
            <span class="font-medium">克隆失败</span>
          </div>
          <div class="mt-2 text-sm text-red-700">
            {{ cloneResult.error }}
          </div>
        </div>
      </div>

      <!-- 操作按钮 -->
      <div class="flex justify-end space-x-3 border-t pt-4">
        <Button variant="outline" @click="resetForm" :disabled="isCloning">
          重置
        </Button>
        <Button
          v-if="isCloning"
          variant="destructive"
          @click="cancelClone"
        >
          <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
          </svg>
          取消克隆
        </Button>
        <Button
          v-else
          @click="startClone"
          :disabled="!canClone"
        >
          <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
          </svg>
          开始克隆
        </Button>
      </div>
    </CardContent>
  </Card>
</template>

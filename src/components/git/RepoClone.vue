<script setup lang="ts">
import { ref, computed, reactive, onMounted, onUnmounted, watch } from 'vue';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { open } from '@tauri-apps/plugin-dialog';
import { homeDir, join } from '@tauri-apps/api/path';
import { gitApi, gitCloneManager } from '@/services/git-api';
import { CloneOptionsBuilder, AuthConfigBuilder, formatBytes, formatDuration } from '@/types/git-backend';
import type { CloneProgress, AuthType, CloneResult, AuthConfig } from '@/types/git-backend';
import { useToast } from '@/components/ui/toast';
import {
  Download,
  Check,
  Folder,
  FileText,
  CheckCircle,
  X
} from 'lucide-vue-next';

// Props 定义
interface Props {
  initialUrl?: string;
  initialDirectory?: string;
  initialAuthConfig?: AuthConfig;
}

const props = withDefaults(defineProps<Props>(), {
  initialUrl: '',
  initialDirectory: '',
  initialAuthConfig: undefined
});

const { error: errorToast } = useToast();

// Emits 定义
const emit = defineEmits<{
  cloneSuccess: [result: CloneResult];
  cloneError: [error: Error];
}>();

// 组件状态
const cloneForm = reactive({
  url: props.initialUrl || '',
  directory: props.initialDirectory || '',
  branch: '',
  depth: undefined as number | undefined,
  recursive: false,
  authType: (props.initialAuthConfig?.auth_type || 'none') as AuthType,
  username: props.initialAuthConfig?.username || '',
  password: props.initialAuthConfig?.password || '',
  token: props.initialAuthConfig?.token || '',
  sshKeyPath: props.initialAuthConfig?.ssh_key_path || '',
  sshKeyPassphrase: props.initialAuthConfig?.ssh_key_passphrase || ''
});

const showAdvanced = ref(false);
const isCloning = ref(false);
const urlError = ref('');
const directoryError = ref('');
const currentOperationId = ref<string | null>(null);
const defaultSshKeys = ref<string[]>([]);
const defaultCloneDirectory = ref<string>('');
const suggestedDirectories = ref<string[]>([]);

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

// 从Git URL中提取仓库名称
const extractRepositoryName = (url: string): string => {
  try {
    // 处理不同格式的Git URL
    let repoName = '';

    if (url.includes('github.com') || url.includes('gitlab.com') || url.includes('atomgit.com') || url.includes('gitee.com')) {
      // HTTPS URL: https://github.com/user/repo.git
      // SSH URL: git@github.com:user/repo.git
      const match = url.match(/[\/:]([^\/]+)\/([^\/]+?)(?:\.git)?(?:\/)?$/);
      if (match) {
        repoName = match[2];
      }
    } else {
      // 通用处理：提取最后一个路径段
      const parts = url.replace(/\.git$/, '').split('/');
      repoName = parts[parts.length - 1] || 'repository';
    }

    return repoName || 'repository';
  } catch (error) {
    console.error('提取仓库名称失败:', error);
    return 'repository';
  }
};

// 生成默认克隆路径
const generateDefaultClonePath = async (repoUrl: string): Promise<string> => {
  try {
    const repoName = extractRepositoryName(repoUrl);
    const home = await homeDir();

    // 生成多个可能的路径选项
    const possiblePaths = [
      await join(home, 'Projects', repoName),
      await join(home, 'Code', repoName),
      await join(home, 'git', repoName),
      await join(home, 'repositories', repoName),
      await join(home, 'Desktop', repoName),
      await join(home, repoName)
    ];

    // 更新建议路径列表
    suggestedDirectories.value = possiblePaths;

    // 返回第一个作为默认路径
    return possiblePaths[0];
  } catch (error) {
    console.error('生成默认克隆路径失败:', error);
    return '';
  }
};

// 监听 props 变化
watch(() => props.initialUrl, async (newUrl) => {
  if (newUrl && newUrl !== cloneForm.url) {
    cloneForm.url = newUrl;

    // 如果没有通过props设置初始目录，则生成默认路径
    if (!props.initialDirectory) {
      const defaultPath = await generateDefaultClonePath(newUrl);
      if (defaultPath) {
        cloneForm.directory = defaultPath;
        defaultCloneDirectory.value = defaultPath;
      }
    }

    detectAuthType();
  }
}, { immediate: true });

watch(() => props.initialDirectory, (newDirectory) => {
  if (newDirectory && newDirectory !== cloneForm.directory) {
    cloneForm.directory = newDirectory;
  }
}, { immediate: true });

watch(() => props.initialAuthConfig, (newAuthConfig) => {
  console.log('initialAuthConfig 变化:', newAuthConfig);
  if (newAuthConfig) {
    cloneForm.authType = newAuthConfig.auth_type;
    cloneForm.username = newAuthConfig.username || '';
    cloneForm.password = newAuthConfig.password || '';
    cloneForm.token = newAuthConfig.token || '';
    cloneForm.sshKeyPath = newAuthConfig.ssh_key_path || '';
    cloneForm.sshKeyPassphrase = newAuthConfig.ssh_key_passphrase || '';
  }
}, { immediate: true });

// 监听URL变化，自动生成默认路径
watch(() => cloneForm.url, async (newUrl, oldUrl) => {
  // 只有当URL真正改变且不是通过props设置时才生成默认路径
  if (newUrl && newUrl !== oldUrl && newUrl !== props.initialUrl) {
    // 检查URL是否有效
    if (isValidUrl.value && !cloneForm.directory) {
      const defaultPath = await generateDefaultClonePath(newUrl);
      if (defaultPath) {
        cloneForm.directory = defaultPath;
        defaultCloneDirectory.value = defaultPath;
        console.log('自动生成默认克隆路径:', defaultPath);
      }
    }
  }
});

// 监听认证类型变化，自动设置SSH密钥默认值
watch(() => cloneForm.authType, (newAuthType, oldAuthType) => {
  // 当切换到SSH认证且没有设置SSH密钥路径时，自动选择第一个可用密钥
  if (newAuthType === 'ssh' && !cloneForm.sshKeyPath && defaultSshKeys.value.length > 0) {
    cloneForm.sshKeyPath = defaultSshKeys.value[0];
    console.log('切换到SSH认证，自动选择默认SSH密钥:', defaultSshKeys.value[0]);
  }
  // 当从SSH切换到其他认证方式时，可以选择清空SSH相关字段（可选）
  else if (oldAuthType === 'ssh' && newAuthType !== 'ssh') {
    // 这里可以选择是否清空SSH字段，目前保留用户之前的选择
    console.log('从SSH认证切换到其他认证方式');
  }
});

// 生命周期
onMounted(async () => {
  // 加载默认 SSH 密钥
  try {
    defaultSshKeys.value = await gitApi.getDefaultSshKeys();

    // 如果当前认证类型是SSH且没有设置SSH密钥路径，自动设置第一个可用密钥
    if (cloneForm.authType === 'ssh' && !cloneForm.sshKeyPath && defaultSshKeys.value.length > 0) {
      cloneForm.sshKeyPath = defaultSshKeys.value[0];
      console.log('自动选择默认SSH密钥:', defaultSshKeys.value[0]);
    }
  } catch (error) {
    console.error('加载默认 SSH 密钥失败:', error);
  }

  // 如果有URL但没有目录，生成默认路径
  if (cloneForm.url && !cloneForm.directory) {
    try {
      const defaultPath = await generateDefaultClonePath(cloneForm.url);
      if (defaultPath) {
        cloneForm.directory = defaultPath;
        defaultCloneDirectory.value = defaultPath;
        console.log('初始化默认克隆路径:', defaultPath);
      }
    } catch (error) {
      console.error('初始化默认路径失败:', error);
    }
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
    // 如果已经通过 props 设置了认证配置，不要覆盖它
    if (props.initialAuthConfig) {
      console.log('跳过自动检测，使用 props 中的认证配置');
      return;
    }

    const authType = await gitApi.detectAuthType(cloneForm.url);
    cloneForm.authType = authType as AuthType;

    // 如果检测到SSH认证且没有设置SSH密钥路径，自动选择第一个可用密钥
    if (authType === 'ssh' && !cloneForm.sshKeyPath && defaultSshKeys.value.length > 0) {
      cloneForm.sshKeyPath = defaultSshKeys.value[0];
      console.log('检测到SSH认证，自动选择默认SSH密钥:', defaultSshKeys.value[0]);
    }

    // SSH URL 自动转换提示
    if (authType === 'ssh' && cloneForm.url.startsWith('git@')) {
      const httpsUrl = convertSshToHttpsUrl(cloneForm.url);

      // 可以在这里添加用户提示
      showSshToHttpsHint.value = {
        sshUrl: cloneForm.url,
        httpsUrl: httpsUrl
      };
    }

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

// SSH到HTTPS URL转换
const convertSshToHttpsUrl = (sshUrl: string): string => {
  const match = sshUrl.match(/git@([^:]+):(.+)/);
  if (match) {
    const [, host, path] = match;
    return `https://${host}/${path}`;
  }
  return sshUrl;
};

// SSH转换提示状态
const showSshToHttpsHint = ref<{ sshUrl: string, httpsUrl: string } | null>(null);

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
          extensions: ['*'] // SSH 密钥文件通常没有扩展名，如 id_ed25519, id_rsa
        },
        {
          name: 'PEM 格式密钥',
          extensions: ['pem', 'key']
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

        // 发出克隆成功事件，添加原始 URL 信息
        if (result.success) {
          const enrichedResult = {
            ...result,
            repository_url: cloneForm.url, // 添加原始仓库 URL
            target_directory: cloneForm.directory, // 添加目标目录
            branch: cloneForm.branch || 'main' // 添加分支信息
          };
          emit('cloneSuccess', enrichedResult);
        }
      },
      (error) => {
        // 设置错误进度状态，但保持isCloning为true避免闪烁
        cloneProgress.value = {
          id: currentOperationId.value || 'error',
          stage: 'Error',
          progress: 0,
          message: `克隆失败: ${error.message}`,
        };

        // 发出克隆错误事件
        emit('cloneError', error);

        // 延迟2秒后隐藏进度条，避免闪烁
        setTimeout(() => {
          isCloning.value = false;
        }, 2000);
      }
    );
  } catch (error) {
    errorToast('克隆失败: ' + (error as Error).message);

    // 设置错误进度状态，但保持isCloning为true避免闪烁
    cloneProgress.value = {
      id: 'error',
      stage: 'Error',
      progress: 0,
      message: `启动克隆失败: ${error}`,
    };

    // 延迟2秒后隐藏进度条，避免闪烁
    setTimeout(() => {
      isCloning.value = false;
    }, 2000);
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
  <Card class="w-full max-w-2xl mx-auto overflow-y-auto max-h-[84vh] scrollbar-hidden pb-3 py-6">
    <CardHeader>
      <CardTitle class="flex items-center space-x-2">
        <Download class="w-5 h-5" />
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
            <Check class="w-4 h-4" />
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
              <Folder class="w-4 h-4 mr-2" />
              浏览
            </Button>
          </div>

          <!-- 显示建议的目录路径 -->
          <div v-if="suggestedDirectories.length > 0 && !cloneForm.directory" class="space-y-1">
            <p class="text-xs text-muted-foreground">建议路径:</p>
            <div class="flex flex-wrap gap-1">
              <Button
                v-for="(path, index) in suggestedDirectories.slice(0, 4)"
                :key="path"
                variant="ghost"
                size="sm"
                class="text-xs h-6 px-2 max-w-[200px] truncate"
                :title="path"
                @click="cloneForm.directory = path"
              >
                {{ index === 0 ? '📁 ' : '📂 ' }}{{ path.split(/[/\\]/).slice(-2).join('/') }}
              </Button>
            </div>
          </div>

          <p v-if="directoryError" class="text-sm" :class="directoryError.startsWith('警告') ? 'text-yellow-600' : 'text-red-600'">
            {{ directoryError }}
          </p>
          <div v-else-if="cloneForm.directory && !directoryError" class="flex items-center space-x-1 text-sm text-green-600">
            <Check class="w-4 h-4" />
            <span>目录可用</span>
          </div>
        </div>
      </div>

      <!-- 认证信息 -->
      <div class="space-y-4">        
        <!-- 认证类型选择 -->
        <div class="space-y-2">
          <label class="text-sm font-medium">认证方式</label>
          <div class="flex flex-wrap gap-2 mt-2">
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
                placeholder="~/.ssh/id_ed25519 或 ~/.ssh/id_rsa"
                class="flex-1"
                @blur="validateSshKey"
              />
              <Button variant="outline" @click="selectSshKeyFile">
                <FileText class="w-4 h-4 mr-2" />
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
                  :variant="cloneForm.sshKeyPath === keyPath ? 'default' : 'ghost'"
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

      <!-- 克隆进度 -->
      <div v-if="isCloning">
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
            <CheckCircle class="w-5 h-5" />
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
            <X class="w-5 h-5" />
            <span class="font-medium">克隆失败</span>
          </div>
          <div class="mt-2 text-sm text-red-700">
            {{ cloneResult.error }}
          </div>
        </div>
      </div>

      <!-- 操作按钮 -->
      <div class="flex justify-end space-x-3 pt-4">
        <Button variant="outline" @click="resetForm" :disabled="isCloning">
          重置
        </Button>
        <Button
          v-if="isCloning"
          variant="destructive"
          @click="cancelClone"
        >
          <X class="w-4 h-4 mr-2" />
          取消克隆
        </Button>
        <Button
          v-else
          @click="startClone"
          :disabled="!canClone"
        >
          <Download class="w-4 h-4 mr-2" />
          开始克隆
        </Button>
      </div>
    </CardContent>
  </Card>
</template>

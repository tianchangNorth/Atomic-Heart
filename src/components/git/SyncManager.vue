<script setup lang="ts">
import { ref, computed, reactive, onMounted } from 'vue';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { useToast } from '@/components/ui/toast';
import { gitOperationsApi, type SyncResult, type PullStrategy, type ProtocolType, type TokenConfig } from '@/api/git-operations';
import { gitApi } from '@/services/git-api';
import TokenConfigDialog from './TokenConfigDialog.vue';
import {
  Download,
  Upload,
  RefreshCw,
  CheckCircle2,
  AlertCircle,
  Loader2,
  GitBranch,
  Key,
  Shield,
  Wifi,
  X,
  GitMerge,
  GitPullRequest,
  Zap,
  Tag,
  Settings,
  ChevronDown
} from 'lucide-vue-next';

// Props
interface Props {
  repositoryPath: string;
}

const props = defineProps<Props>();

// Toast
const { success, error } = useToast();

// 组件状态
const syncStatus = reactive({
  ahead: 0,
  behind: 0,
  conflicts: [] as string[],
  lastSync: null as string | null,
  remoteStatus: 'connected' as 'connected' | 'disconnected' | 'unknown'
});

// 认证状态
const authStatus = reactive({
  protocol: 'unknown' as ProtocolType,
  domain: '',
  hasToken: false,
  tokenConfigured: false,
  lastChecked: null as string | null
});

// Token配置弹窗状态
const tokenDialog = reactive({
  open: false,
  domain: ''
});

const currentOperation = ref<{
  type: 'fetch' | 'pull' | 'push';
  progress: number;
  message: string;
  loading: boolean;
} | null>(null);

const operationLogs = ref<string[]>([]);

const syncOptions = reactive({
  pullMode: 'merge' as PullStrategy,
  pushForce: false,
  pushTags: false,
  remoteName: 'origin',
  remoteBranch: 'main'
});

// 高级选项展开状态
const showAdvancedOptions = ref(false);

// SSH密钥管理
const showSshKeySelector = ref(false);
const availableSshKeys = ref<string[]>([]);
const selectedSshKey = ref<string>('');

// 操作状态
const fetchState = ref({ loading: false, error: null as string | null });
const pullState = ref({ loading: false, error: null as string | null });
const pushState = ref({ loading: false, error: null as string | null });

// 计算属性
const needsPull = computed(() => syncStatus.behind > 0);
const needsPush = computed(() => syncStatus.ahead > 0);
const hasConflicts = computed(() => syncStatus.conflicts.length > 0);
const isConnected = computed(() => syncStatus.remoteStatus === 'connected');
const isOperating = computed(() =>
  fetchState.value.loading || pullState.value.loading || pushState.value.loading
);

const statusColor = computed(() => {
  if (hasConflicts.value) return 'text-red-600';
  if (needsPull.value || needsPush.value) return 'text-yellow-600';
  return 'text-green-600';
});

const statusText = computed(() => {
  if (hasConflicts.value) return '存在冲突';
  if (needsPull.value && needsPush.value) return '需要同步';
  if (needsPull.value) return '需要拉取';
  if (needsPush.value) return '需要推送';
  return '已同步';
});

// 方法
const addLog = (message: string) => {
  const timestamp = new Date().toLocaleTimeString();
  operationLogs.value.unshift(`[${timestamp}] ${message}`);
  if (operationLogs.value.length > 50) {
    operationLogs.value = operationLogs.value.slice(0, 50);
  }
};

// 检测协议和认证状态
const detectProtocolAndAuth = async () => {
  try {
    addLog('检测仓库协议类型...');

    // 检测协议类型
    const protocol = await gitOperationsApi.detectRepositoryProtocol(props.repositoryPath);
    authStatus.protocol = protocol;

    addLog(`检测到协议类型: ${protocol.toUpperCase()}`);

    if (protocol === 'https') {
      // 获取远程URL并提取域名
      try {
        const remoteUrl = await gitOperationsApi.getRemoteUrl(props.repositoryPath);
        if (remoteUrl) {
          addLog(`检测到远程URL: ${remoteUrl}`);

          // 从URL提取域名
          const domain = await gitOperationsApi.extractDomainFromUrl(remoteUrl);
          authStatus.domain = domain;
          addLog(`提取到域名: ${domain}`);

          // 检查是否有存储的token
          const token = await gitOperationsApi.getAccessToken(domain);
          authStatus.hasToken = !!token;
          authStatus.tokenConfigured = !!token;

          if (token) {
            addLog(`找到${domain}的访问令牌`);
            await gitOperationsApi.updateTokenLastUsed(domain);
          } else {
            addLog(`未找到${domain}的访问令牌`);
          }
        } else {
          addLog('未找到远程URL');
          authStatus.domain = '';
        }
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : '获取远程信息失败';
        addLog(`获取远程信息失败: ${errorMessage}`);
        authStatus.domain = '';
      }
    } else if (protocol === 'ssh') {
      addLog('SSH协议将使用系统Git命令');
      authStatus.tokenConfigured = true; // SSH不需要token配置

      // 检测可用的SSH密钥
      try {
        const sshKeys = await gitApi.getDefaultSshKeys();
        availableSshKeys.value = sshKeys;
        if (sshKeys.length > 0) {
          selectedSshKey.value = sshKeys[0]; // 默认选择第一个
          addLog(`检测到 ${sshKeys.length} 个SSH密钥，默认使用: ${sshKeys[0].split('/').pop()}`);
        } else {
          addLog('未检测到SSH密钥');
        }
      } catch (error) {
        addLog('检测SSH密钥失败');
        console.error('检测SSH密钥失败:', error);
      }
    }

    authStatus.lastChecked = new Date().toISOString();
  } catch (err) {
    const errorMessage = err instanceof Error ? err.message : '协议检测失败';
    addLog(`协议检测失败: ${errorMessage}`);
    console.error('协议检测失败:', err);
  }
};

// 确保HTTPS认证
const ensureHttpsAuth = async (): Promise<boolean> => {
  if (authStatus.protocol === 'https' && !authStatus.tokenConfigured) {
    addLog('需要配置Personal Access Token');

    // 打开token配置弹窗
    tokenDialog.domain = authStatus.domain;
    tokenDialog.open = true;

    return false;
  }
  return true;
};

// 获取远程变更（智能fetch操作）
const fetchRemote = async () => {
  fetchState.value.loading = true;
  fetchState.value.error = null;

  currentOperation.value = {
    type: 'fetch',
    progress: 0,
    message: '连接到远程仓库...',
    loading: true
  };

  addLog('开始获取远程变更...');

  try {
    // 检查认证状态
    if (authStatus.protocol === 'https' && !(await ensureHttpsAuth())) {
      addLog('等待Token配置...');
      return;
    }

    currentOperation.value.progress = 30;
    currentOperation.value.message = '获取远程变更...';

    // 使用智能fetch操作，传递SSH密钥
    const sshKeyPath = authStatus.protocol === 'ssh' ? selectedSshKey.value : undefined;
    if (sshKeyPath) {
      addLog(`使用SSH密钥: ${sshKeyPath.split('/').pop()}`);
    }
    const result: SyncResult = await gitOperationsApi.smartFetchRemote(props.repositoryPath, undefined, sshKeyPath);

    currentOperation.value.progress = 100;
    currentOperation.value.message = '获取完成';

    if (result.success) {
      syncStatus.ahead = result.ahead;
      syncStatus.behind = result.behind;
      syncStatus.lastSync = new Date().toISOString();
      syncStatus.remoteStatus = 'connected';

      addLog(`获取成功 - 领先 ${result.ahead} 个提交，落后 ${result.behind} 个提交`);
    } else {
      addLog(`获取失败: ${result.message}`);
      error(result.message);
    }
  } catch (err) {
    const errorMessage = err instanceof Error ? err.message : '获取远程变更失败';
    fetchState.value.error = errorMessage;
    syncStatus.remoteStatus = 'disconnected';
    addLog(`获取失败: ${errorMessage}`);

    // 如果是认证错误，可能需要重新配置token
    if (errorMessage.includes('authentication') || errorMessage.includes('401') || errorMessage.includes('403')) {
      addLog('可能是认证问题，请检查Token配置');
      if (authStatus.protocol === 'https') {
        authStatus.tokenConfigured = false;
      }
    }

    error(errorMessage);
  } finally {
    fetchState.value.loading = false;
    setTimeout(() => {
      currentOperation.value = null;
    }, 1000);
  }
};

// 拉取远程变更（智能pull操作）
const pull = async () => {
  pullState.value.loading = true;
  pullState.value.error = null;

  currentOperation.value = {
    type: 'pull',
    progress: 0,
    message: '准备拉取...',
    loading: true
  };

  addLog(`开始拉取远程变更 (${syncOptions.pullMode})...`);

  try {
    // 检查认证状态
    if (authStatus.protocol === 'https' && !(await ensureHttpsAuth())) {
      addLog('等待Token配置...');
      return;
    }

    currentOperation.value.progress = 20;
    currentOperation.value.message = '连接到远程仓库...';

    await new Promise(resolve => setTimeout(resolve, 500));

    currentOperation.value.progress = 60;
    currentOperation.value.message = '拉取远程变更...';

    // 使用智能pull操作，传递SSH密钥
    const sshKeyPath = authStatus.protocol === 'ssh' ? selectedSshKey.value : undefined;
    if (sshKeyPath) {
      addLog(`使用SSH密钥: ${sshKeyPath.split('/').pop()}`);
    }
    const result: SyncResult = await gitOperationsApi.smartPullRemote(props.repositoryPath, syncOptions.pullMode, sshKeyPath);

    currentOperation.value.progress = 100;
    currentOperation.value.message = '拉取完成';

    if (result.success) {
      if (result.has_conflicts) {
        syncStatus.conflicts = result.conflict_files;
        addLog(`拉取完成但存在冲突: ${result.conflict_files.join(', ')}`);
        error('拉取时发现冲突，请手动解决后重试');
      } else {
        syncStatus.behind = result.behind;
        syncStatus.ahead = result.ahead;
        syncStatus.lastSync = new Date().toISOString();
        addLog('拉取成功');
        success(result.message);
      }
    } else {
      addLog(`拉取失败: ${result.message}`);
      error(result.message);
    }
  } catch (err) {
    const errorMessage = err instanceof Error ? err.message : '拉取远程变更失败';
    pullState.value.error = errorMessage;
    addLog(`拉取失败: ${errorMessage}`);

    // 如果是认证错误，可能需要重新配置token
    if (errorMessage.includes('authentication') || errorMessage.includes('401') || errorMessage.includes('403')) {
      addLog('可能是认证问题，请检查Token配置');
      if (authStatus.protocol === 'https') {
        authStatus.tokenConfigured = false;
      }
    }

    error(errorMessage);
  } finally {
    pullState.value.loading = false;
    setTimeout(() => {
      currentOperation.value = null;
    }, 1000);
  }
};

// 推送本地变更（智能push操作）
const push = async () => {
  pushState.value.loading = true;
  pushState.value.error = null;

  currentOperation.value = {
    type: 'push',
    progress: 0,
    message: '准备推送...',
    loading: true
  };

  addLog('开始推送本地变更...');

  try {
    // 检查认证状态
    if (authStatus.protocol === 'https' && !(await ensureHttpsAuth())) {
      addLog('等待Token配置...');
      return;
    }

    currentOperation.value.progress = 20;
    currentOperation.value.message = '连接到远程仓库...';

    await new Promise(resolve => setTimeout(resolve, 500));

    currentOperation.value.progress = 60;
    currentOperation.value.message = '推送本地变更...';

    // 使用智能push操作，传递SSH密钥
    const sshKeyPath = authStatus.protocol === 'ssh' ? selectedSshKey.value : undefined;
    if (sshKeyPath) {
      addLog(`使用SSH密钥: ${sshKeyPath.split('/').pop()}`);
    }
    const result: SyncResult = await gitOperationsApi.smartPushRemote(
      props.repositoryPath,
      syncOptions.remoteName,
      syncOptions.pushForce,
      sshKeyPath
    );

    currentOperation.value.progress = 100;
    currentOperation.value.message = '推送完成';

    if (result.success) {
      syncStatus.ahead = result.ahead;
      syncStatus.behind = result.behind;
      syncStatus.lastSync = new Date().toISOString();
      addLog('推送成功');
      success(result.message);
    } else {
      addLog(`推送失败: ${result.message}`);
      error(result.message);
    }
  } catch (err) {
    const errorMessage = err instanceof Error ? err.message : '推送本地变更失败';
    pushState.value.error = errorMessage;
    addLog(`推送失败: ${errorMessage}`);

    // 如果是认证错误，可能需要重新配置token
    if (errorMessage.includes('authentication') || errorMessage.includes('401') || errorMessage.includes('403')) {
      addLog('可能是认证问题，请检查Token配置');
      if (authStatus.protocol === 'https') {
        authStatus.tokenConfigured = false;
      }
    }

    error(errorMessage);
  } finally {
    pushState.value.loading = false;
    setTimeout(() => {
      currentOperation.value = null;
    }, 1000);
  }
};

// 同步操作（先拉取后推送）
const sync = async () => {
  if (needsPull.value) {
    await pull();
    await new Promise(resolve => setTimeout(resolve, 1000));
  }
  if (needsPush.value && !hasConflicts.value) {
    await push();
  }
};

const formatLastSync = (dateString: string) => {
  const date = new Date(dateString);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMins = Math.floor(diffMs / (1000 * 60));

  if (diffMins < 1) return '刚刚';
  if (diffMins < 60) return `${diffMins} 分钟前`;
  if (diffMins < 1440) return `${Math.floor(diffMins / 60)} 小时前`;
  return `${Math.floor(diffMins / 1440)} 天前`;
};

// Token配置成功处理
const handleTokenConfigSuccess = (token: TokenConfig) => {
  addLog(`Token配置成功: ${token.domain}`);
  authStatus.hasToken = true;
  authStatus.tokenConfigured = true;
  authStatus.domain = token.domain;

  // 关闭弹窗
  tokenDialog.open = false;

  // 可以继续之前被中断的操作
  success('Token配置成功，可以继续Git操作');
};

// 手动配置Token
const configureToken = () => {
  tokenDialog.domain = authStatus.domain || '';
  tokenDialog.open = true;
};

// SSH密钥选择
const selectSshKey = (keyPath: string) => {
  selectedSshKey.value = keyPath;
  showSshKeySelector.value = false;
  addLog(`切换到SSH密钥: ${keyPath.split('/').pop()}`);
};

// 初始化时获取远程信息和状态
onMounted(async () => {
  try {
    // 首先检测协议和认证状态
    await detectProtocolAndAuth();

    // 然后获取远程信息以确定正确的远程名称
    const remoteInfo = await gitOperationsApi.getRemoteInfo(props.repositoryPath);
    syncOptions.remoteName = remoteInfo.remote_name;
    syncOptions.remoteBranch = remoteInfo.branch_name;

    // 最后获取远程变更
    await fetchRemote();
  } catch (error) {
    console.error('初始化远程信息失败:', error);
    // 如果获取远程信息失败，仍然尝试fetch
    await fetchRemote();
  }
});
</script>

<template>
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
    <!-- 左侧：同步状态和操作 -->
    <div class="lg:col-span-2 space-y-4">
      <!-- 合并的状态与操作卡片 -->
      <Card>
        <CardHeader class="pb-3">
          <CardTitle class="flex items-center justify-between">
            <div class="flex items-center space-x-2">
              <GitBranch class="w-5 h-5" />
              <span>Git 同步</span>
            </div>
            <div class="flex items-center space-x-2">
              <Badge
                :variant="isConnected ? 'default' : 'destructive'"
                class="text-xs"
              >
                {{ isConnected ? '已连接' : '连接失败' }}
              </Badge>
              <Button variant="ghost" size="sm" @click="fetchRemote" :disabled="isOperating">
                <Loader2 v-if="fetchState.loading" class="w-4 h-4 animate-spin" />
                <RefreshCw v-else class="w-4 h-4" />
              </Button>
            </div>
          </CardTitle>
        </CardHeader>
        <CardContent class="space-y-4">
          <!-- 紧凑的协议和认证状态 -->
          <div class="flex items-center justify-between p-2 bg-muted/50 rounded text-sm">
            <div class="flex items-center space-x-2">
              <Shield v-if="authStatus.protocol === 'ssh'" class="w-3 h-3 text-green-600" />
              <Wifi v-else-if="authStatus.protocol === 'https'" class="w-3 h-3 text-blue-600" />
              <AlertCircle v-else class="w-3 h-3 text-gray-400" />
              <span class="font-medium">
                {{ authStatus.protocol === 'ssh' ? 'SSH' : authStatus.protocol === 'https' ? 'HTTPS' : '未知' }}
              </span>
              <span v-if="authStatus.domain" class="text-muted-foreground">{{ authStatus.domain }}</span>
            </div>
            <div class="flex items-center space-x-2">
              <div v-if="authStatus.protocol === 'https'" class="flex items-center space-x-1">
                <CheckCircle2 v-if="authStatus.tokenConfigured" class="w-3 h-3 text-green-600" />
                <AlertCircle v-else class="w-3 h-3 text-orange-600" />
                <span class="text-xs">
                  {{ authStatus.tokenConfigured ? 'Token已配置' : 'Token未配置' }}
                </span>
                <Button
                  v-if="!authStatus.tokenConfigured"
                  variant="outline"
                  size="sm"
                  class="h-6 px-2 text-xs"
                  @click="configureToken"
                >
                  <Key class="w-3 h-3 mr-1" />
                  配置
                </Button>
              </div>
              <div v-else-if="authStatus.protocol === 'ssh'" class="flex items-center space-x-2">
                <span class="text-xs text-green-600">
                  {{ selectedSshKey ? selectedSshKey.split('/').pop() : 'SSH密钥' }}
                </span>
                <Button
                  v-if="availableSshKeys.length > 1"
                  variant="ghost"
                  size="sm"
                  class="h-5 px-2 text-xs text-muted-foreground hover:text-foreground"
                  @click="showSshKeySelector = true"
                >
                  <Key class="w-3 h-3 mr-1" />
                  选择
                </Button>
              </div>
            </div>
          </div>

          <!-- 紧凑的同步状态 -->
          <div class="grid grid-cols-4 gap-3 text-center">
            <div class="space-y-1">
              <div class="text-lg font-bold text-blue-600">{{ syncStatus.ahead }}</div>
              <div class="text-xs text-muted-foreground">领先</div>
            </div>
            <div class="space-y-1">
              <div class="text-lg font-bold text-orange-600">{{ syncStatus.behind }}</div>
              <div class="text-xs text-muted-foreground">落后</div>
            </div>
            <div class="space-y-1">
              <div class="text-lg font-bold" :class="statusColor">
                <AlertCircle v-if="hasConflicts" class="w-5 h-5 mx-auto" />
                <RefreshCw v-else-if="needsPull || needsPush" class="w-5 h-5 mx-auto" />
                <CheckCircle2 v-else class="w-5 h-5 mx-auto" />
              </div>
              <div class="text-xs text-muted-foreground">{{ statusText }}</div>
            </div>
            <div class="space-y-1">
              <div class="text-xs text-muted-foreground">
                {{ syncStatus.lastSync ? formatLastSync(syncStatus.lastSync) : '未同步' }}
              </div>
              <div class="text-xs text-muted-foreground">最后同步</div>
            </div>
          </div>

          <!-- 同步操作按钮 -->
          <div class="grid grid-cols-1 sm:grid-cols-4 gap-2">
            <Button
              @click="fetchRemote"
              :disabled="isOperating"
              variant="outline"
              size="sm"
              class="flex flex-col items-center py-3 h-auto"
            >
              <Loader2 v-if="fetchState.loading" class="w-4 h-4 mb-1 animate-spin" />
              <RefreshCw v-else class="w-4 h-4 mb-1" />
              <span class="text-xs">获取</span>
            </Button>

            <Button
              @click="pull"
              :disabled="!needsPull || isOperating"
              variant="outline"
              size="sm"
              class="flex flex-col items-center py-3 h-auto"
            >
              <Loader2 v-if="pullState.loading" class="w-4 h-4 mb-1 animate-spin" />
              <Download v-else class="w-4 h-4 mb-1" />
              <span class="text-xs">拉取</span>
              <span class="text-xs text-muted-foreground">{{ syncStatus.behind }}</span>
            </Button>

            <Button
              @click="push"
              :disabled="!needsPush || isOperating"
              variant="outline"
              size="sm"
              class="flex flex-col items-center py-3 h-auto"
            >
              <Loader2 v-if="pushState.loading" class="w-4 h-4 mb-1 animate-spin" />
              <Upload v-else class="w-4 h-4 mb-1" />
              <span class="text-xs">推送</span>
              <span class="text-xs text-muted-foreground">{{ syncStatus.ahead }}</span>
            </Button>

            <Button
              @click="sync"
              :disabled="(!needsPull && !needsPush) || isOperating"
              size="sm"
              class="flex flex-col items-center py-3 h-auto"
            >
              <Loader2 v-if="isOperating" class="w-4 h-4 mb-1 animate-spin" />
              <RefreshCw v-else class="w-4 h-4 mb-1" />
              <span class="text-xs">同步</span>
            </Button>
          </div>

          <!-- 操作进度 -->
          <div v-if="currentOperation" class="space-y-2">
            <div class="flex items-center justify-between text-sm">
              <span>{{ currentOperation.message }}</span>
              <span>{{ currentOperation.progress }}%</span>
            </div>
            <div class="w-full bg-muted rounded-full h-1.5">
              <div
                class="bg-primary h-1.5 rounded-full transition-all duration-300"
                :style="{ width: `${currentOperation.progress}%` }"
              ></div>
            </div>
          </div>
        </CardContent>
      </Card>

      <!-- 可折叠的高级选项 -->
      <Card v-if="showAdvancedOptions" class="shadow-md">
        <CardHeader class="pb-4">
          <CardTitle class="flex items-center justify-between">
            <div class="flex items-center space-x-3">
              <div class="p-2 rounded-lg bg-muted">
                <Settings class="w-5 h-5 text-muted-foreground" />
              </div>
              <div>
                <span class="text-base font-semibold text-foreground">高级选项</span>
                <p class="text-xs text-muted-foreground mt-0.5">自定义Git同步策略和行为配置</p>
              </div>
            </div>
            <Button
              variant="ghost"
              size="sm"
              @click="showAdvancedOptions = false"
              class="hover:bg-destructive/10 hover:text-destructive transition-colors duration-200"
            >
              <X class="w-4 h-4" />
            </Button>
          </CardTitle>
        </CardHeader>
        <CardContent class="space-y-6">
          <!-- 拉取模式 -->
          <div class="space-y-4">
            <div class="flex items-center space-x-3">
              <div class="p-2 rounded-lg bg-muted">
                <GitMerge class="w-4 h-4 text-muted-foreground" />
              </div>
              <div>
                <label class="text-sm font-medium text-foreground">拉取模式</label>
                <p class="text-xs text-muted-foreground">选择如何将远程更改集成到本地分支</p>
              </div>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
              <!-- Merge 选项 -->
              <Card
                class="cursor-pointer group transition-all duration-200 hover:shadow-md"
                :class="syncOptions.pullMode === 'merge' ? 'ring-2 ring-primary ring-offset-2' : ''"
                @click="syncOptions.pullMode = 'merge'"
              >
                <CardContent class="p-4">
                  <div class="flex items-start space-x-3">
                    <div class="p-2 rounded-lg bg-muted group-hover:bg-primary/10 transition-colors">
                      <GitMerge class="w-4 h-4 text-muted-foreground group-hover:text-primary transition-colors" />
                    </div>
                    <div class="flex-1">
                      <div class="flex items-center space-x-2 mb-2">
                        <span class="font-medium text-sm">Merge</span>
                        <code class="px-1.5 py-0.5 text-xs bg-muted rounded font-mono">--no-ff</code>
                      </div>
                      <p class="text-xs text-muted-foreground leading-relaxed">
                        创建合并提交，保留分支历史
                      </p>
                    </div>
                    <CheckCircle2 v-if="syncOptions.pullMode === 'merge'" class="w-4 h-4 text-primary flex-shrink-0" />
                  </div>
                </CardContent>
              </Card>

              <!-- Rebase 选项 -->
              <Card
                class="cursor-pointer group transition-all duration-200 hover:shadow-md"
                :class="syncOptions.pullMode === 'rebase' ? 'ring-2 ring-primary ring-offset-2' : ''"
                @click="syncOptions.pullMode = 'rebase'"
              >
                <CardContent class="p-4">
                  <div class="flex items-start space-x-3">
                    <div class="p-2 rounded-lg bg-muted group-hover:bg-primary/10 transition-colors">
                      <GitPullRequest class="w-4 h-4 text-muted-foreground group-hover:text-primary transition-colors" />
                    </div>
                    <div class="flex-1">
                      <div class="flex items-center space-x-2 mb-2">
                        <span class="font-medium text-sm">Rebase</span>
                        <code class="px-1.5 py-0.5 text-xs bg-muted rounded font-mono">--rebase</code>
                      </div>
                      <p class="text-xs text-muted-foreground leading-relaxed">
                        重写提交历史，保持线性历史
                      </p>
                    </div>
                    <CheckCircle2 v-if="syncOptions.pullMode === 'rebase'" class="w-4 h-4 text-primary flex-shrink-0" />
                  </div>
                </CardContent>
              </Card>
            </div>
          </div>

          <!-- 推送选项 -->
          <div class="space-y-4">
            <div class="flex items-center space-x-3">
              <div class="p-2 rounded-lg bg-muted">
                <Upload class="w-4 h-4 text-muted-foreground" />
              </div>
              <div>
                <label class="text-sm font-medium text-foreground">推送选项</label>
                <p class="text-xs text-muted-foreground">控制推送到远程仓库的行为</p>
              </div>
            </div>

            <div class="space-y-3">
              <!-- 强制推送选项 -->
              <Card class="cursor-pointer group transition-all duration-200 hover:shadow-md" @click="syncOptions.pushForce = !syncOptions.pushForce">
                <CardContent class="p-4">
                  <div class="flex items-start space-x-3">
                    <div class="flex items-center mt-0.5">
                      <div class="w-4 h-4 rounded border-2 transition-all duration-200 flex items-center justify-center"
                           :class="syncOptions.pushForce
                            ? 'border-primary bg-primary'
                            : 'border-muted-foreground'">
                        <CheckCircle2 v-if="syncOptions.pushForce" class="w-2.5 h-2.5 text-white" />
                      </div>
                    </div>
                    <div class="p-2 rounded-lg bg-muted group-hover:bg-destructive/10 transition-colors">
                      <Zap class="w-4 h-4 text-muted-foreground group-hover:text-destructive transition-colors" />
                    </div>
                    <div class="flex-1">
                      <div class="flex items-center space-x-2 mb-2">
                        <span class="font-medium text-sm">强制推送</span>
                        <code class="px-1.5 py-0.5 text-xs bg-muted rounded font-mono">--force</code>
                        <span class="px-1.5 py-0.5 text-xs bg-destructive/10 text-destructive rounded-md font-medium">危险</span>
                      </div>
                      <p class="text-xs text-muted-foreground leading-relaxed">
                        强制覆盖远程分支，可能导致提交历史丢失
                      </p>
                    </div>
                  </div>
                </CardContent>
              </Card>

              <!-- 推送标签选项 -->
              <Card class="cursor-pointer group transition-all duration-200 hover:shadow-md" @click="syncOptions.pushTags = !syncOptions.pushTags">
                <CardContent class="p-4">
                  <div class="flex items-start space-x-3">
                    <div class="flex items-center mt-0.5">
                      <div class="w-4 h-4 rounded border-2 transition-all duration-200 flex items-center justify-center"
                           :class="syncOptions.pushTags
                            ? 'border-primary bg-primary'
                            : 'border-muted-foreground'">
                        <CheckCircle2 v-if="syncOptions.pushTags" class="w-2.5 h-2.5 text-white" />
                      </div>
                    </div>
                    <div class="p-2 rounded-lg bg-muted group-hover:bg-primary/10 transition-colors">
                      <Tag class="w-4 h-4 text-muted-foreground group-hover:text-primary transition-colors" />
                    </div>
                    <div class="flex-1">
                      <div class="flex items-center space-x-2 mb-2">
                        <span class="font-medium text-sm">推送标签</span>
                        <code class="px-1.5 py-0.5 text-xs bg-muted rounded font-mono">--tags</code>
                      </div>
                      <p class="text-xs text-muted-foreground leading-relaxed">
                        同时推送所有本地标签到远程仓库
                      </p>
                    </div>
                  </div>
                </CardContent>
              </Card>
            </div>
          </div>
        </CardContent>
      </Card>

      <!-- 高级选项切换按钮 -->
      <Card v-if="!showAdvancedOptions" class="cursor-pointer group hover:shadow-md transition-all duration-200" @click="showAdvancedOptions = true">
        <CardContent class="">
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-3">
              <div class="p-2 rounded-lg bg-muted group-hover:bg-primary/10 transition-colors duration-200">
                <Settings class="w-5 h-5 text-muted-foreground group-hover:text-primary transition-colors duration-200" />
              </div>
              <div>
                <div class="text-sm font-medium text-foreground group-hover:text-primary transition-colors duration-200">
                  高级选项
                </div>
                <div class="text-xs text-muted-foreground">
                  自定义Git同步策略和行为配置
                </div>
              </div>
            </div>
            <ChevronDown class="w-4 h-4 text-muted-foreground group-hover:text-primary group-hover:translate-y-0.5 transition-all duration-200" />
          </div>
        </CardContent>
      </Card>
    </div>

    <!-- 右侧：操作日志 -->
    <div>
      <Card class="h-fit">
        <CardHeader class="pb-3">
          <CardTitle class="flex items-center justify-between">
            <div class="flex items-center space-x-2">
              <RefreshCw class="w-4 h-4" />
              <span class="text-sm">操作日志</span>
            </div>
            <Button
              variant="ghost"
              size="sm"
              @click="operationLogs = []"
              class="h-6 px-2 text-xs"
            >
              清空
            </Button>
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div class="space-y-1 max-h-80 overflow-y-auto">
            <div v-if="operationLogs.length === 0" class="text-center py-6 text-muted-foreground">
              <RefreshCw class="w-8 h-8 mx-auto mb-2 opacity-50" />
              <p class="text-xs">暂无操作日志</p>
            </div>

            <div
              v-for="(log, index) in operationLogs"
              :key="index"
              class="text-xs font-mono p-2 rounded bg-muted/30 text-muted-foreground leading-relaxed"
            >
              {{ log }}
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  </div>

  <!-- SSH密钥选择对话框 -->
  <div v-if="showSshKeySelector" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click="showSshKeySelector = false">
    <Card class="w-96 max-w-[90vw]" @click.stop>
      <CardHeader>
        <CardTitle class="flex items-center space-x-2">
          <Key class="w-5 h-5" />
          <span>选择SSH密钥</span>
        </CardTitle>
      </CardHeader>
      <CardContent class="space-y-3">
        <div v-for="keyPath in availableSshKeys" :key="keyPath" class="space-y-2">
          <Button
            variant="outline"
            class="w-full justify-start text-left h-auto p-3"
            :class="selectedSshKey === keyPath ? 'ring-2 ring-primary' : ''"
            @click="selectSshKey(keyPath)"
          >
            <div class="flex items-center space-x-3">
              <Key class="w-4 h-4 flex-shrink-0" />
              <div class="flex-1 min-w-0">
                <div class="font-medium text-sm">{{ keyPath.split('/').pop() }}</div>
                <div class="text-xs text-muted-foreground truncate">{{ keyPath }}</div>
              </div>
              <CheckCircle2 v-if="selectedSshKey === keyPath" class="w-4 h-4 text-primary flex-shrink-0" />
            </div>
          </Button>
        </div>
        <div class="flex justify-end space-x-2 pt-2">
          <Button variant="outline" size="sm" @click="showSshKeySelector = false">
            取消
          </Button>
        </div>
      </CardContent>
    </Card>
  </div>

  <!-- Token配置弹窗 -->
  <TokenConfigDialog
    :open="tokenDialog.open"
    :domain="tokenDialog.domain"
    @update:open="(value) => tokenDialog.open = value"
    @close="() => tokenDialog.open = false"
    @success="handleTokenConfigSuccess"
  />
</template>

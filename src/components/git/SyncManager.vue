<script setup lang="ts">
import { ref, computed, reactive, onMounted } from 'vue';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { useToast } from '@/components/ui/toast';
import { gitOperationsApi, type SyncResult, type PullStrategy, type ProtocolType, type TokenConfig } from '@/api/git-operations';
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
  Wifi
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
  remoteStatus: 'unknown' as 'connected' | 'disconnected' | 'unknown'
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

    // 使用智能fetch操作
    const result: SyncResult = await gitOperationsApi.smartFetchRemote(props.repositoryPath);

    currentOperation.value.progress = 100;
    currentOperation.value.message = '获取完成';

    if (result.success) {
      syncStatus.ahead = result.ahead;
      syncStatus.behind = result.behind;
      syncStatus.lastSync = new Date().toISOString();
      syncStatus.remoteStatus = 'connected';

      addLog(`获取成功 - 领先 ${result.ahead} 个提交，落后 ${result.behind} 个提交`);
      success(result.message);
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

    // 使用智能pull操作
    const result: SyncResult = await gitOperationsApi.smartPullRemote(props.repositoryPath, syncOptions.pullMode);

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

    // 使用智能push操作
    const result: SyncResult = await gitOperationsApi.smartPushRemote(
      props.repositoryPath,
      syncOptions.remoteName,
      syncOptions.pushForce
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
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
    <!-- 左侧：同步状态和操作 -->
    <div class="lg:col-span-2 space-y-6">
      <!-- 远程仓库状态 -->
      <Card>
        <CardHeader>
          <CardTitle class="flex items-center justify-between">
            <div class="flex items-center space-x-2">
              <GitBranch class="w-5 h-5" />
              <span>远程仓库状态</span>
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
        <CardContent>
          <!-- 协议和认证状态 -->
          <div class="flex items-center justify-between p-3 bg-muted rounded-lg mb-4">
            <div class="flex items-center space-x-3">
              <div class="flex items-center space-x-2">
                <Shield v-if="authStatus.protocol === 'ssh'" class="w-4 h-4 text-green-600" />
                <Wifi v-else-if="authStatus.protocol === 'https'" class="w-4 h-4 text-blue-600" />
                <AlertCircle v-else class="w-4 h-4 text-gray-400" />
                <span class="text-sm font-medium">
                  {{ authStatus.protocol === 'ssh' ? 'SSH' : authStatus.protocol === 'https' ? 'HTTPS' : '未知' }}
                </span>
              </div>
              <div v-if="authStatus.domain" class="text-sm text-muted-foreground">
                {{ authStatus.domain }}
              </div>
            </div>
            <div class="flex items-center space-x-2">
              <div v-if="authStatus.protocol === 'https'" class="flex items-center space-x-2">
                <CheckCircle2 v-if="authStatus.tokenConfigured" class="w-4 h-4 text-green-600" />
                <AlertCircle v-else class="w-4 h-4 text-orange-600" />
                <span class="text-sm">
                  {{ authStatus.tokenConfigured ? 'Token已配置' : 'Token未配置' }}
                </span>
                <Button
                  v-if="!authStatus.tokenConfigured"
                  variant="outline"
                  size="sm"
                  @click="configureToken"
                >
                  <Key class="w-3 h-3 mr-1" />
                  配置Token
                </Button>
              </div>
              <div v-else-if="authStatus.protocol === 'ssh'" class="text-sm text-green-600">
                使用系统SSH
              </div>
            </div>
          </div>

          <div class="grid grid-cols-3 gap-4 text-center">
            <div class="space-y-2">
              <div class="text-2xl font-bold text-blue-600">{{ syncStatus.ahead }}</div>
              <div class="text-sm text-muted-foreground">领先提交</div>
            </div>
            <div class="space-y-2">
              <div class="text-2xl font-bold text-orange-600">{{ syncStatus.behind }}</div>
              <div class="text-sm text-muted-foreground">落后提交</div>
            </div>
            <div class="space-y-2">
              <div class="text-2xl font-bold" :class="statusColor">
                <AlertCircle v-if="hasConflicts" class="w-8 h-8 mx-auto" />
                <RefreshCw v-else-if="needsPull || needsPush" class="w-8 h-8 mx-auto" />
                <CheckCircle2 v-else class="w-8 h-8 mx-auto" />
              </div>
              <div class="text-sm text-muted-foreground">{{ statusText }}</div>
            </div>
          </div>
          
          <div v-if="syncStatus.lastSync" class="mt-4 text-center text-sm text-muted-foreground">
            最后同步：{{ formatLastSync(syncStatus.lastSync) }}
          </div>
        </CardContent>
      </Card>

      <!-- 同步选项 -->
      <Card>
        <CardHeader>
          <CardTitle class="flex items-center space-x-2">
            <RefreshCw class="w-5 h-5" />
            <span>同步选项</span>
          </CardTitle>
        </CardHeader>
        <CardContent class="space-y-4">
          <!-- 拉取模式 -->
          <div class="space-y-2">
            <label class="text-sm font-medium">拉取模式</label>
            <div class="flex space-x-2">
              <Button
                :variant="syncOptions.pullMode === 'merge' ? 'default' : 'outline'"
                size="sm"
                @click="syncOptions.pullMode = 'merge'"
              >
                Merge
              </Button>
              <Button
                :variant="syncOptions.pullMode === 'rebase' ? 'default' : 'outline'"
                size="sm"
                @click="syncOptions.pullMode = 'rebase'"
              >
                Rebase
              </Button>
            </div>
          </div>

          <!-- 推送选项 -->
          <div class="space-y-3">
            <label class="text-sm font-medium">推送选项</label>
            <div class="space-y-2">
              <label class="flex items-center space-x-2">
                <input v-model="syncOptions.pushForce" type="checkbox" class="rounded border-border">
                <span class="text-sm">强制推送 (--force)</span>
              </label>
              <label class="flex items-center space-x-2">
                <input v-model="syncOptions.pushTags" type="checkbox" class="rounded border-border">
                <span class="text-sm">推送标签 (--tags)</span>
              </label>
            </div>
          </div>
        </CardContent>
      </Card>

      <!-- 操作按钮 -->
      <Card>
        <CardHeader>
          <CardTitle class="flex items-center space-x-2">
            <GitBranch class="w-5 h-5" />
            <span>同步操作</span>
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
            <Button
              @click="pull"
              :disabled="!needsPull || isOperating"
              variant="outline"
              class="flex flex-col items-center p-4 h-auto"
            >
              <Loader2 v-if="pullState.loading" class="w-6 h-6 mb-2 animate-spin" />
              <Download v-else class="w-6 h-6 mb-2" />
              <span>拉取</span>
              <span class="text-xs text-muted-foreground">{{ syncStatus.behind }} 个提交</span>
            </Button>

            <Button
              @click="push"
              :disabled="!needsPush || isOperating"
              variant="outline"
              class="flex flex-col items-center p-4 h-auto"
            >
              <Loader2 v-if="pushState.loading" class="w-6 h-6 mb-2 animate-spin" />
              <Upload v-else class="w-6 h-6 mb-2" />
              <span>推送</span>
              <span class="text-xs text-muted-foreground">{{ syncStatus.ahead }} 个提交</span>
            </Button>

            <Button
              @click="sync"
              :disabled="(!needsPull && !needsPush) || isOperating"
              class="flex flex-col items-center p-4 h-auto"
            >
              <Loader2 v-if="isOperating" class="w-6 h-6 mb-2 animate-spin" />
              <RefreshCw v-else class="w-6 h-6 mb-2" />
              <span>同步</span>
              <span class="text-xs text-muted-foreground">拉取 + 推送</span>
            </Button>
          </div>

          <!-- 操作进度 -->
          <div v-if="currentOperation" class="mt-6">
            <div class="space-y-2">
              <div class="flex items-center justify-between text-sm">
                <span>{{ currentOperation.message }}</span>
                <span>{{ currentOperation.progress }}%</span>
              </div>
              <div class="w-full bg-muted rounded-full h-2">
                <div
                  class="bg-primary h-2 rounded-full transition-all duration-300"
                  :style="{ width: `${currentOperation.progress}%` }"
                ></div>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>

    <!-- 右侧：操作日志 -->
    <div class="space-y-6">
      <Card class="h-full">
        <CardHeader>
          <CardTitle class="flex items-center justify-between">
            <div class="flex items-center space-x-2">
              <RefreshCw class="w-5 h-5" />
              <span>操作日志</span>
            </div>
            <Button
              variant="ghost"
              size="sm"
              @click="operationLogs = []"
            >
              清空
            </Button>
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div class="space-y-1 max-h-96 overflow-y-auto">
            <div v-if="operationLogs.length === 0" class="text-center py-8 text-muted-foreground">
              <RefreshCw class="w-12 h-12 mx-auto mb-2" />
              <p>暂无操作日志</p>
            </div>

            <div
              v-for="(log, index) in operationLogs"
              :key="index"
              class="text-sm font-mono p-2 rounded bg-muted/50 text-muted-foreground"
            >
              {{ log }}
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
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

<script setup lang="ts">
import { ref, computed, reactive } from 'vue';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import ProgressBar from './ui/ProgressBar.vue';
import type { SyncStatus, OperationProgress } from '@/types/git';

// 组件状态
const syncStatus = ref<SyncStatus>({
  ahead: 3,
  behind: 2,
  conflicts: [],
  lastSync: '2024-12-15T10:30:00Z',
  remoteStatus: 'connected'
});

const currentOperation = ref<OperationProgress | null>(null);
const operationLogs = ref<string[]>([
  '[10:30:15] 成功连接到远程仓库',
  '[10:29:45] 检查远程分支状态',
  '[10:29:30] 本地分支领先 3 个提交',
  '[10:29:30] 远程分支领先 2 个提交'
]);

const syncOptions = reactive({
  pullMode: 'merge' as 'merge' | 'rebase',
  pushForce: false,
  pushTags: false,
  remoteName: 'origin',
  remoteBranch: 'main'
});

const isOperating = ref(false);

// 计算属性
const needsPull = computed(() => syncStatus.value.behind > 0);
const needsPush = computed(() => syncStatus.value.ahead > 0);
const hasConflicts = computed(() => syncStatus.value.conflicts.length > 0);
const isConnected = computed(() => syncStatus.value.remoteStatus === 'connected');

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

const simulateOperation = async (type: 'push' | 'pull', steps: Array<{progress: number, message: string}>) => {
  isOperating.value = true;
  currentOperation.value = {
    id: `${type}-${Date.now()}`,
    type,
    status: 'running',
    progress: 0,
    message: '准备中...',
    startTime: new Date().toISOString()
  };

  for (const step of steps) {
    await new Promise(resolve => setTimeout(resolve, 1000));
    currentOperation.value!.progress = step.progress;
    currentOperation.value!.message = step.message;
    addLog(step.message);
  }

  currentOperation.value!.status = 'success';
  currentOperation.value!.endTime = new Date().toISOString();
  
  // 更新同步状态
  if (type === 'pull') {
    syncStatus.value.behind = 0;
    syncStatus.value.lastSync = new Date().toISOString();
  } else if (type === 'push') {
    syncStatus.value.ahead = 0;
    syncStatus.value.lastSync = new Date().toISOString();
  }

  setTimeout(() => {
    currentOperation.value = null;
    isOperating.value = false;
  }, 2000);
};

const pull = async () => {
  const steps = [
    { progress: 20, message: '连接到远程仓库...' },
    { progress: 40, message: '获取远程变更...' },
    { progress: 60, message: '合并远程变更...' },
    { progress: 80, message: '更新工作目录...' },
    { progress: 100, message: '拉取完成！' }
  ];

  if (syncOptions.pullMode === 'rebase') {
    steps[2] = { progress: 60, message: '变基远程变更...' };
  }

  await simulateOperation('pull', steps);
};

const push = async () => {
  const steps = [
    { progress: 20, message: '连接到远程仓库...' },
    { progress: 40, message: '检查推送权限...' },
    { progress: 60, message: '上传本地变更...' },
    { progress: 80, message: '更新远程分支...' },
    { progress: 100, message: '推送完成！' }
  ];

  await simulateOperation('push', steps);
};

const sync = async () => {
  if (needsPull.value) {
    await pull();
    await new Promise(resolve => setTimeout(resolve, 1000));
  }
  if (needsPush.value) {
    await push();
  }
};

const fetchStatus = async () => {
  addLog('检查远程仓库状态...');
  
  // 模拟状态检查
  await new Promise(resolve => setTimeout(resolve, 1500));
  
  // 随机更新状态
  syncStatus.value.ahead = Math.floor(Math.random() * 5);
  syncStatus.value.behind = Math.floor(Math.random() * 3);
  syncStatus.value.lastSync = new Date().toISOString();
  
  addLog(`检查完成 - 领先 ${syncStatus.value.ahead} 个提交，落后 ${syncStatus.value.behind} 个提交`);
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
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9v-9m0-9v9"/>
              </svg>
              <span>远程仓库状态</span>
            </div>
            <div class="flex items-center space-x-2">
              <Badge 
                :variant="isConnected ? 'default' : 'destructive'"
                class="text-xs"
              >
                {{ isConnected ? '已连接' : '连接失败' }}
              </Badge>
              <Button variant="ghost" size="sm" @click="fetchStatus" :disabled="isOperating">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
                </svg>
              </Button>
            </div>
          </CardTitle>
        </CardHeader>
        <CardContent>
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
                <svg class="w-8 h-8 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path v-if="hasConflicts" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.34 16.5c-.77.833.192 2.5 1.732 2.5z"/>
                  <path v-else-if="needsPull || needsPush" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/>
                  <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
                </svg>
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
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
            </svg>
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
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l4-4 4 4m0 6l-4 4-4-4"/>
            </svg>
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
              <svg class="w-6 h-6 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4l-8 8h6v8h4v-8h6l-8-8z"/>
              </svg>
              <span>拉取</span>
              <span class="text-xs text-muted-foreground">{{ syncStatus.behind }} 个提交</span>
            </Button>

            <Button 
              @click="push" 
              :disabled="!needsPush || isOperating"
              variant="outline"
              class="flex flex-col items-center p-4 h-auto"
            >
              <svg class="w-6 h-6 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 20l8-8h-6V4h-4v8H4l8 8z"/>
              </svg>
              <span>推送</span>
              <span class="text-xs text-muted-foreground">{{ syncStatus.ahead }} 个提交</span>
            </Button>

            <Button 
              @click="sync" 
              :disabled="(!needsPull && !needsPush) || isOperating"
              class="flex flex-col items-center p-4 h-auto"
            >
              <svg class="w-6 h-6 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
              </svg>
              <span>同步</span>
              <span class="text-xs text-muted-foreground">拉取 + 推送</span>
            </Button>
          </div>

          <!-- 操作进度 -->
          <div v-if="currentOperation" class="mt-6">
            <ProgressBar
              :progress="currentOperation.progress"
              :status="currentOperation.status"
              :message="currentOperation.message"
            />
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
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
              </svg>
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
              <svg class="w-12 h-12 mx-auto mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
              </svg>
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
</template>

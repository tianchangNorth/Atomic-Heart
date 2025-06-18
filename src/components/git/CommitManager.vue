<script setup lang="ts">
import { ref, computed, reactive, watch } from 'vue';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import Textarea from '@/components/ui/textarea/Textarea.vue';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { useToast } from '@/components/ui/toast';
import { useGitOperations } from '@/composables/useGitOperations';
import DiffViewer from './ui/DiffViewer.vue';
import { RefreshCw, Loader2 } from 'lucide-vue-next';

// Props
interface Props {
  repositoryPath: string;
}

const props = defineProps<Props>();

// Composables
const { success } = useToast();
const {
  repositoryStatus,
  stagedFiles,
  unstagedFiles,
  // hasChanges,
  hasStagedChanges,
  // isClean,
  statusState,
  stageState,
  commitState,
  refreshStatus,
  stageFile,
  unstageFile,
  stageAllFiles,
  unstageAllFiles,
  createCommit,
  getFileDiff,
  initialize
} = useGitOperations(props.repositoryPath);

// 响应式数据
const selectedFile = ref<string | null>(null);
const selectedFileDiff = ref<string>('');
const loadingDiff = ref(false);

const commitForm = reactive({
  message: '',
  description: '',
  amend: false,
  signoff: false
});

// 监听仓库路径变化
watch(() => props.repositoryPath, async (newPath) => {
  if (newPath) {
    await initialize();
  }
}, { immediate: true });

// 监听选中文件变化，加载差异
watch(selectedFile, async (newFile) => {
  if (newFile && repositoryStatus.value) {
    loadingDiff.value = true;
    try {
      const file = repositoryStatus.value.files.find(f => f.path === newFile);
      if (file) {
        selectedFileDiff.value = await getFileDiff(newFile, file.staged);
      }
    } catch (error) {
      console.error('加载文件差异失败:', error);
      selectedFileDiff.value = '';
    } finally {
      loadingDiff.value = false;
    }
  } else {
    selectedFileDiff.value = '';
  }
});

// 计算属性
const canCommit = computed(() => {
  return !commitState.value.loading && hasStagedChanges.value && commitForm.message.trim();
});

const totalAdditions = computed(() =>
  stagedFiles.value.reduce((sum, file) => sum + file.additions, 0)
);

const totalDeletions = computed(() =>
  stagedFiles.value.reduce((sum, file) => sum + file.deletions, 0)
);

const selectedFileData = computed(() => {
  if (!selectedFile.value || !repositoryStatus.value) return null;
  return repositoryStatus.value.files.find(f => f.path === selectedFile.value);
});

// 获取文件状态样式
const getStatusBadge = (status: string) => {
  const statusConfig = {
    added: { variant: 'default' as const, color: 'text-green-600', label: '新增' },
    modified: { variant: 'secondary' as const, color: 'text-blue-600', label: '修改' },
    deleted: { variant: 'destructive' as const, color: 'text-red-600', label: '删除' },
    renamed: { variant: 'outline' as const, color: 'text-purple-600', label: '重命名' },
    untracked: { variant: 'outline' as const, color: 'text-gray-600', label: '未跟踪' }
  };

  return statusConfig[status as keyof typeof statusConfig] || statusConfig.modified;
};

// 获取文件图标
const getFileIcon = (path: string) => {
  const ext = path.split('.').pop()?.toLowerCase();
  const iconMap: Record<string, string> = {
    vue: 'text-green-500',
    ts: 'text-blue-500',
    js: 'text-yellow-500',
    json: 'text-orange-500',
    md: 'text-gray-600',
    css: 'text-pink-500',
    html: 'text-red-500'
  };

  return iconMap[ext || ''] || 'text-gray-500';
};

// 方法
const toggleStaged = async (file: { path: string; staged: boolean }) => {
  if (file.staged) {
    await unstageFile(file.path);
  } else {
    await stageFile(file.path);
  }
};

const selectFile = (filePath: string) => {
  selectedFile.value = selectedFile.value === filePath ? null : filePath;
};

const handleStageAll = async () => {
  await stageAllFiles();
};

const handleUnstageAll = async () => {
  await unstageAllFiles();
};

const handleRefresh = async () => {
  await refreshStatus();
  selectedFile.value = null;
};

const commit = async () => {
  if (!canCommit.value) return;

  try {
    const commitSha = await createCommit({
      message: commitForm.message,
      description: commitForm.description || undefined,
      amend: commitForm.amend,
      signoff: commitForm.signoff
    });

    if (commitSha) {
      // 重置表单
      commitForm.message = '';
      commitForm.description = '';
      commitForm.amend = false;
      commitForm.signoff = false;
      selectedFile.value = null;

      success(`提交 ${commitSha.slice(0, 7)} 已创建`, '提交成功');
    }
  } catch (error) {
    console.error('提交失败:', error);
  }
};
</script>

<template>
  <div class="grid grid-cols-1 xl:grid-cols-2 gap-6 min-h-[600px]">
    <!-- 左侧：文件变更列表和提交信息 -->
    <div class="space-y-6">
      <!-- 暂存区 -->
      <Card>
        <CardHeader>
          <CardTitle class="flex items-center justify-between">
            <div class="flex items-center space-x-2">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
              </svg>
              <span>暂存区</span>
              <Badge variant="secondary">{{ stagedFiles.length }}</Badge>
              <Button
                v-if="statusState.loading"
                variant="ghost"
                size="sm"
                disabled
              >
                <Loader2 class="w-4 h-4 animate-spin" />
              </Button>
              <Button
                v-else
                variant="ghost"
                size="sm"
                @click="handleRefresh"
              >
                <RefreshCw class="w-4 h-4" />
              </Button>
            </div>
            <Button
              v-if="stagedFiles.length > 0"
              variant="ghost"
              size="sm"
              @click="handleUnstageAll"
              :disabled="stageState.loading"
            >
              <Loader2 v-if="stageState.loading" class="w-4 h-4 animate-spin mr-2" />
              全部取消暂存
            </Button>
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div v-if="stagedFiles.length === 0" class="text-center py-8 text-muted-foreground">
            <svg class="w-12 h-12 mx-auto mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2 2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-2.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 009.586 13H7"/>
            </svg>
            <p>暂存区为空</p>
            <p class="text-sm">选择下方文件进行暂存</p>
          </div>
          
          <div v-else class="space-y-2">
            <div
              v-for="file in stagedFiles"
              :key="file.path"
              class="flex items-center justify-between p-3 rounded-lg border hover:bg-accent cursor-pointer"
              :class="{ 'bg-accent': selectedFile === file.path }"
              @click="selectFile(file.path)"
            >
              <div class="flex items-center space-x-3 flex-1 min-w-0">
                <svg class="w-4 h-4 flex-shrink-0" :class="getFileIcon(file.path)" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                  <polyline points="14,2 14,8 20,8"/>
                </svg>
                <div class="flex-1 min-w-0">
                  <p class="font-medium truncate">{{ file.path }}</p>
                  <div class="flex items-center space-x-2 mt-1">
                    <Badge :variant="getStatusBadge(file.status).variant" class="text-xs">
                      {{ getStatusBadge(file.status).label }}
                    </Badge>
                    <span class="text-xs text-green-600">+{{ file.additions }}</span>
                    <span class="text-xs text-red-600">-{{ file.deletions }}</span>
                  </div>
                </div>
              </div>
              <Button variant="ghost" size="sm" @click.stop="toggleStaged(file)">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4"/>
                </svg>
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>

      <!-- 工作区 -->
      <Card>
        <CardHeader>
          <CardTitle class="flex items-center justify-between">
            <div class="flex items-center space-x-2">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
              </svg>
              <span>工作区变更</span>
              <Badge variant="outline">{{ unstagedFiles.length }}</Badge>
            </div>
            <Button
              v-if="unstagedFiles.length > 0"
              variant="ghost"
              size="sm"
              @click="handleStageAll"
              :disabled="stageState.loading"
            >
              <Loader2 v-if="stageState.loading" class="w-4 h-4 animate-spin mr-2" />
              全部暂存
            </Button>
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div v-if="unstagedFiles.length === 0" class="text-center py-8 text-muted-foreground">
            <svg class="w-12 h-12 mx-auto mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
            </svg>
            <p>工作区干净</p>
            <p class="text-sm">没有未暂存的变更</p>
          </div>
          
          <div v-else class="space-y-2">
            <div
              v-for="file in unstagedFiles"
              :key="file.path"
              class="flex items-center justify-between p-3 rounded-lg border hover:bg-accent cursor-pointer"
              :class="{ 'bg-accent': selectedFile === file.path }"
              @click="selectFile(file.path)"
            >
              <div class="flex items-center space-x-3 flex-1 min-w-0">
                <svg class="w-4 h-4 flex-shrink-0" :class="getFileIcon(file.path)" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                  <polyline points="14,2 14,8 20,8"/>
                </svg>
                <div class="flex-1 min-w-0">
                  <p class="font-medium truncate">{{ file.path }}</p>
                  <div class="flex items-center space-x-2 mt-1">
                    <Badge :variant="getStatusBadge(file.status).variant" class="text-xs">
                      {{ getStatusBadge(file.status).label }}
                    </Badge>
                    <span class="text-xs text-green-600">+{{ file.additions }}</span>
                    <span class="text-xs text-red-600">-{{ file.deletions }}</span>
                  </div>
                </div>
              </div>
              <Button variant="ghost" size="sm" @click.stop="toggleStaged(file)">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"/>
                </svg>
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>

      <!-- 提交信息 -->
      <Card>
        <CardHeader>
          <CardTitle class="flex items-center space-x-2">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3a2 2 0 012-2h4a2 2 0 012 2v4m-6 0V6a2 2 0 012-2h4a2 2 0 012 2v1m-6 0h6m-6 0l-1 1v4a2 2 0 002 2h2m2-6h2a2 2 0 012 2v4a2 2 0 01-2 2h-2m-2-6v6m-2-6v6"/>
            </svg>
            <span>提交信息</span>
          </CardTitle>
        </CardHeader>
        <CardContent class="space-y-4">
          <div class="space-y-2">
            <label class="text-sm font-medium">提交标题 *</label>
            <Input
              v-model="commitForm.message"
              placeholder="简要描述本次提交的内容"
              maxlength="72"
            />
            <p class="text-xs text-muted-foreground">
              {{ commitForm.message.length }}/72 字符
            </p>
          </div>

          <div class="space-y-2">
            <label class="text-sm font-medium">详细描述</label>
            <Textarea
              v-model="commitForm.description"
              placeholder="详细描述本次提交的变更内容（可选）"
              rows="3"
            />
          </div>

          <div class="flex items-center space-x-4">
            <label class="flex items-center space-x-2">
              <input v-model="commitForm.amend" type="checkbox" class="rounded border-border">
              <span class="text-sm">修正上次提交</span>
            </label>
            <label class="flex items-center space-x-2">
              <input v-model="commitForm.signoff" type="checkbox" class="rounded border-border">
              <span class="text-sm">添加签名</span>
            </label>
          </div>

          <div v-if="stagedFiles.length > 0" class="flex items-center justify-between p-3 bg-muted rounded-lg">
            <div class="text-sm">
              <span class="font-medium">{{ stagedFiles.length }}</span> 个文件将被提交
            </div>
            <div class="flex items-center space-x-4 text-sm">
              <span class="text-green-600">+{{ totalAdditions }}</span>
              <span class="text-red-600">-{{ totalDeletions }}</span>
            </div>
          </div>

          <Button
            @click="commit"
            :disabled="!canCommit"
            class="w-full"
          >
            <Loader2 v-if="commitState.loading" class="w-4 h-4 mr-2 animate-spin" />
            <svg v-else class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3a2 2 0 012-2h4a2 2 0 012 2v4m-6 0V6a2 2 0 012-2h4a2 2 0 012 2v1m-6 0h6m-6 0l-1 1v4a2 2 0 002 2h2m2-6h2a2 2 0 012 2v4a2 2 0 01-2 2h-2m-2-6v6m-2-6v6"/>
            </svg>
            {{ commitState.loading ? '提交中...' : '提交变更' }}
          </Button>
        </CardContent>
      </Card>
    </div>

    <!-- 右侧：差异预览 -->
    <div class="flex flex-col min-h-[600px]">
      <div v-if="loadingDiff" class="flex-1 flex items-center justify-center">
        <div class="text-center">
          <Loader2 class="w-8 h-8 animate-spin mx-auto mb-2" />
          <p class="text-sm text-muted-foreground">加载文件差异...</p>
        </div>
      </div>

      <div v-else-if="selectedFileData && selectedFileDiff" class="flex-1">
        <DiffViewer
          :file-name="selectedFileData.path"
          :diff="selectedFileDiff"
          :additions="selectedFileData.additions"
          :deletions="selectedFileData.deletions"
          max-height="600px"
          class="diff-viewer"
        />
      </div>

      <div v-else class="flex-1 flex items-center justify-center">
        <div class="text-center text-muted-foreground">
          <svg class="w-16 h-16 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
          </svg>
          <p class="text-lg font-medium">选择文件查看差异</p>
          <p class="text-sm">点击左侧文件列表中的文件来查看详细变更</p>
          <p class="text-xs text-muted-foreground mt-2">💡 新布局为代码查看提供了更宽的显示空间</p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 优化代码查看体验 */
.diff-viewer {
  min-height: 400px;
}

/* 响应式布局优化 */
@media (max-width: 1280px) {
  .grid.grid-cols-1.xl\\:grid-cols-2 {
    grid-template-columns: 1fr;
    gap: 1.5rem;
  }

  .diff-viewer {
    min-height: 300px;
  }
}

/* 文件列表项悬停效果 */
.cursor-pointer:hover {
  transform: translateY(-1px);
  transition: all 0.2s ease-in-out;
}

/* 确保在小屏幕上有足够的空间 */
@media (max-width: 768px) {
  .min-h-\[600px\] {
    min-height: 400px;
  }
}
</style>

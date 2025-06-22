<script setup lang="ts">
import { ref, computed, reactive, watch } from 'vue';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import Textarea from '@/components/ui/textarea/Textarea.vue';
import { Card, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter } from '@/components/ui/dialog';
import { useToast } from '@/components/ui/toast';
import { useGitOperations } from '@/composables/useGitOperations';
import DiffViewer from './ui/DiffViewer.vue';
import {
  RefreshCw,
  Loader2,
  CheckCircle2,
  FileText,
  GitCommit,
  Plus,
  Minus,
  Archive,
  FolderOpen
} from 'lucide-vue-next';

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
  // statusState,
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

// 提交对话框状态
const showCommitDialog = ref(false);

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

// 打开提交对话框
const openCommitDialog = () => {
  if (!hasStagedChanges.value) return;
  showCommitDialog.value = true;
};

// 关闭提交对话框
const closeCommitDialog = () => {
  showCommitDialog.value = false;
  // 重置表单
  commitForm.message = '';
  commitForm.description = '';
  commitForm.amend = false;
  commitForm.signoff = false;
};

// 执行提交
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
      // 关闭对话框并重置表单
      closeCommitDialog();
      selectedFile.value = null;

      success(`提交 ${commitSha.slice(0, 7)} 已创建`, '提交成功');
    }
  } catch (error) {
    console.error('提交失败:', error);
  }
};

// 处理键盘快捷键
const handleCommitKeydown = (event: KeyboardEvent) => {
  if ((event.ctrlKey || event.metaKey) && event.key === 'Enter') {
    event.preventDefault();
    commit();
  }
};
</script>

<template>
  <div class="flex h-full gap-6">
    <div class="flex flex-col w-88 flex-shrink-0 space-y-4 left-panel">
      <!-- 文件变更 -->
      <Card class="gap-0">
         <CardContent class="space-y-4">
          <!-- 暂存区部分 -->
          <div class="flex items-center justify-between mb-2">
            <div class="flex items-center space-x-2">
              <CheckCircle2 class="w-4 h-4" />
              <span class="text-sm font-medium">暂存区</span>
              <Badge variant="secondary" class="text-xs">{{ stagedFiles.length }}</Badge>
              <Button
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
              <Loader2 v-if="stageState.loading" class="w-3 h-3 animate-spin mr-1" />
              <span class="text-xs">全部取消暂存</span>
            </Button>
          </div>

          <div v-if="stagedFiles.length === 0" class="text-center py-4 text-muted-foreground">
            <Archive class="w-8 h-8 mx-auto mb-1" />
            <p class="text-xs">暂存区为空</p>
          </div>

          <div v-else class="space-y-1">
            <div
              v-for="file in stagedFiles"
              :key="'staged-' + file.path"
              class="flex items-center justify-between p-2 rounded border hover:bg-accent cursor-pointer"
              :class="{ 'bg-accent': selectedFile === file.path }"
              @click="selectFile(file.path)"
            >
              <div class="flex items-center space-x-2 flex-1 min-w-0">
                <FileText class="w-3 h-3 flex-shrink-0" />
                <div class="flex-1 min-w-0">
                  <p class="text-xs font-medium truncate">{{ file.path }}</p>
                  <div class="flex items-center space-x-1 mt-0.5">
                    <Badge :variant="getStatusBadge(file.status).variant" class="text-xs px-1 py-0">
                      {{ getStatusBadge(file.status).label }}
                    </Badge>
                    <span class="text-xs text-green-600">+{{ file.additions }}</span>
                    <span class="text-xs text-red-600">-{{ file.deletions }}</span>
                  </div>
                </div>
              </div>
              <Button variant="ghost" size="sm" @click.stop="toggleStaged(file)" class="h-6 w-6 p-0">
                <Minus class="w-3 h-3" />
              </Button>
            </div>
          </div>

          <!-- 工作区部分 -->
          <div>
            <div class="flex items-center justify-between mb-2">
              <div class="flex items-center space-x-2">
                <FolderOpen class="w-4 h-4" />
                <span class="text-sm font-medium">工作区变更</span>
                <Badge variant="outline" class="text-xs">{{ unstagedFiles.length }}</Badge>
              </div>
              <Button
                v-if="unstagedFiles.length > 0"
                variant="ghost"
                size="sm"
                @click="handleStageAll"
                :disabled="stageState.loading"
              >
                <Loader2 v-if="stageState.loading" class="w-3 h-3 animate-spin mr-1" />
                <span class="text-xs">全部暂存</span>
              </Button>
            </div>

            <div v-if="unstagedFiles.length === 0" class="text-center py-4 text-muted-foreground">
              <CheckCircle2 class="w-8 h-8 mx-auto mb-1" />
              <p class="text-xs">工作区干净</p>
            </div>

            <div v-else class="space-y-1">
              <div
                v-for="file in unstagedFiles"
                :key="'unstaged-' + file.path"
                class="flex items-center justify-between p-2 rounded border hover:bg-accent cursor-pointer"
                :class="{ 'bg-accent': selectedFile === file.path }"
                @click="selectFile(file.path)"
              >
                <div class="flex items-center space-x-2 flex-1 min-w-0">
                  <FileText class="w-3 h-3 flex-shrink-0" />
                  <div class="flex-1 min-w-0">
                    <p class="text-xs font-medium truncate">{{ file.path }}</p>
                    <div class="flex items-center space-x-1 mt-0.5">
                      <Badge :variant="getStatusBadge(file.status).variant" class="text-xs px-1 py-0">
                        {{ getStatusBadge(file.status).label }}
                      </Badge>
                      <span class="text-xs text-green-600">+{{ file.additions }}</span>
                      <span class="text-xs text-red-600">-{{ file.deletions }}</span>
                    </div>
                  </div>
                </div>
                <Button variant="ghost" size="sm" @click.stop="toggleStaged(file)" class="h-6 w-6 p-0">
                  <Plus class="w-3 h-3" />
                </Button>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      <!-- 提交操作 -->
      <div class="mt-4">
        <Button
          @click="openCommitDialog"
          :disabled="!hasStagedChanges"
          class="w-full"
          variant="default"
        >
          <GitCommit class="w-4 h-4 mr-2" />
          提交变更
          <Badge v-if="stagedFiles.length > 0" variant="secondary" class="ml-2">
            {{ stagedFiles.length }}
          </Badge>
        </Button>
      </div>
    </div>

    <div class="flex-1">
        <div class="bg-background">
          <div v-if="loadingDiff" class="flex items-center justify-center flex-1 loading-state">
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
              :max-height="'calc(100vh - 420px)'"
            />
          </div>

          <div v-else class="flex items-center justify-center flex-1 mt-20">
            <div class="text-center text-muted-foreground">
              <FileText class="w-16 h-16 mx-auto mb-4 empty-state-icon" />
              <p class="text-lg font-medium">选择文件查看差异</p>
              <p class="text-sm">点击左侧文件列表中的文件来查看详细变更</p>
            </div>
          </div>
      </div>
    </div>

    <!-- 提交对话框 -->
    <Dialog :open="showCommitDialog" @update:open="showCommitDialog = $event">
      <DialogContent class="sm:max-w-md commit-dialog-content" @keydown="handleCommitKeydown">
        <DialogHeader>
          <DialogTitle class="flex items-center space-x-2">
            <GitCommit class="w-5 h-5" />
            <span>提交变更</span>
          </DialogTitle>
        </DialogHeader>

        <div class="space-y-4 py-4">
          <!-- 提交统计信息 -->
          <div class="flex items-center justify-between p-3 commit-dialog-stats">
            <div class="text-sm">
              <span class="font-medium">{{ stagedFiles.length }}</span> 个文件将被提交
            </div>
            <div class="flex items-center space-x-4 text-sm diff-stats">
              <span class="text-green-600">+{{ totalAdditions }}</span>
              <span class="text-red-600">-{{ totalDeletions }}</span>
            </div>
          </div>

          <!-- 提交标题 -->
          <div class="space-y-2">
            <label class="text-sm font-medium">提交标题 *</label>
            <Input
              v-model="commitForm.message"
              placeholder="简要描述本次提交的内容"
              maxlength="72"
              class="text-sm"
              @keydown.enter.prevent
            />
            <p class="text-xs text-muted-foreground">
              {{ commitForm.message.length }}/72 字符
            </p>
          </div>

          <!-- 详细描述 -->
          <div class="space-y-2">
            <label class="text-sm font-medium">详细描述</label>
            <Textarea
              v-model="commitForm.description"
              placeholder="详细描述本次提交的变更内容（可选）"
              rows="3"
              class="text-sm resize-none"
            />
          </div>

          <!-- 选项 -->
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
        </div>

        <DialogFooter class="flex-col space-y-2">
          <div class="flex justify-end space-x-2">
            <Button variant="outline" @click="closeCommitDialog">
              取消
            </Button>
            <Button
              @click="commit"
              :disabled="!canCommit"
              class="min-w-[100px]"
            >
              <Loader2 v-if="commitState.loading" class="w-4 h-4 mr-2 animate-spin" />
              <GitCommit v-else class="w-4 h-4 mr-2" />
              {{ commitState.loading ? '提交中...' : '提交' }}
            </Button>
          </div>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  </div>
</template>

<style scoped>
/* 文件列表项悬停效果 */
.cursor-pointer:hover {
  transform: translateY(-1px);
  transition: all 0.2s ease-in-out;
}

/* 左侧面板的滚动优化 */
.left-panel {
  overflow-y: auto;
  scrollbar-width: thin;
  scrollbar-color: hsl(var(--border)) transparent;
}

/* 空状态图标优化 */
.empty-state-icon {
  opacity: 0.5;
  transition: opacity 0.3s ease-in-out;
}

/* 加载状态优化 */
.loading-state {
  background: hsl(var(--muted) / 0.1);
  border-radius: 0.5rem;
  padding: 2rem;
}

/* 确保差异查看器内容不会产生视觉重叠 */
.diff-viewer-wrapper {
  position: relative;
  z-index: 1;
}

/* 统计信息的视觉优化 */
.diff-stats {
  font-family: ui-monospace, SFMono-Regular, "SF Mono", Consolas, "Liberation Mono", Menlo, monospace;
  font-weight: 600;
}

/* 优化文件列表的紧凑性 */
.file-list-item {
  padding: 0.5rem 0.75rem;
  transition: all 0.2s ease-in-out;
}

.file-list-item:hover {
  background: hsl(var(--muted) / 0.5);
}

/* 优化表单元素的紧凑性 */
.commit-form-container input,
.commit-form-container textarea {
  font-size: 0.875rem;
  /* 14px */
  line-height: 1.25rem;
  /* 20px */
}

.commit-form-container label {
  font-size: 0.875rem;
  /* 14px */
  font-weight: 500;
  margin-bottom: 0.25rem;
}

/* 优化复选框区域 */
.commit-form-container input[type="checkbox"] {
  width: 1rem;
  height: 1rem;
}

/* 提交对话框样式优化 */
.commit-dialog-content {
  max-height: 80vh;
  overflow-y: auto;
}

.commit-dialog-stats {
  background: hsl(var(--muted));
  border: 1px solid hsl(var(--border));
  border-radius: 0.5rem;
}

/* 对话框表单元素优化 */
.commit-dialog-content input,
.commit-dialog-content textarea {
  transition: border-color 0.2s ease-in-out;
}

.commit-dialog-content input:focus,
.commit-dialog-content textarea:focus {
  border-color: hsl(var(--primary));
  box-shadow: 0 0 0 2px hsl(var(--primary) / 0.2);
}
</style>

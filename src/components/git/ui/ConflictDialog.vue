<script setup lang="ts">
import { ref, computed } from 'vue';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';

interface ConflictFile {
  path: string;
  conflicts: ConflictSection[];
  resolved: boolean;
}

interface ConflictSection {
  id: string;
  startLine: number;
  endLine: number;
  currentContent: string;
  incomingContent: string;
  baseContent?: string;
  resolution?: 'current' | 'incoming' | 'both' | 'manual';
  manualContent?: string;
}

interface Props {
  open: boolean;
  files: ConflictFile[];
}

interface Emits {
  (e: 'update:open', value: boolean): void;
  (e: 'resolve', files: ConflictFile[]): void;
  (e: 'cancel'): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const selectedFile = ref<string | null>(null);
const selectedConflict = ref<string | null>(null);

// 计算属性
const currentFile = computed(() => {
  if (!selectedFile.value) return null;
  return props.files.find(f => f.path === selectedFile.value) || null;
});

const currentConflict = computed(() => {
  if (!currentFile.value || !selectedConflict.value) return null;
  return currentFile.value.conflicts.find(c => c.id === selectedConflict.value) || null;
});

const totalConflicts = computed(() => {
  return props.files.reduce((sum, file) => sum + file.conflicts.length, 0);
});

const resolvedConflicts = computed(() => {
  return props.files.reduce((sum, file) => 
    sum + file.conflicts.filter(c => c.resolution).length, 0
  );
});

const canResolve = computed(() => {
  return props.files.every(file => 
    file.conflicts.every(conflict => conflict.resolution)
  );
});

// 方法
const selectFile = (filePath: string) => {
  selectedFile.value = filePath;
  const file = props.files.find(f => f.path === filePath);
  if (file && file.conflicts.length > 0) {
    selectedConflict.value = file.conflicts[0].id;
  }
};

const selectConflict = (conflictId: string) => {
  selectedConflict.value = conflictId;
};

const resolveConflict = (conflictId: string, resolution: ConflictSection['resolution'], content?: string) => {
  const file = props.files.find(f => f.path === selectedFile.value);
  if (!file) return;
  
  const conflict = file.conflicts.find(c => c.id === conflictId);
  if (!conflict) return;
  
  conflict.resolution = resolution;
  if (resolution === 'manual' && content !== undefined) {
    conflict.manualContent = content;
  }
  
  // 检查文件是否完全解决
  file.resolved = file.conflicts.every(c => c.resolution);
};

const getResolutionContent = (conflict: ConflictSection): string => {
  switch (conflict.resolution) {
    case 'current':
      return conflict.currentContent;
    case 'incoming':
      return conflict.incomingContent;
    case 'both':
      return `${conflict.currentContent}\n${conflict.incomingContent}`;
    case 'manual':
      return conflict.manualContent || '';
    default:
      return '';
  }
};

const handleResolve = () => {
  emit('resolve', props.files);
  emit('update:open', false);
};

const handleCancel = () => {
  emit('cancel');
  emit('update:open', false);
};

// 初始化选择第一个文件
if (props.files.length > 0 && !selectedFile.value) {
  selectFile(props.files[0].path);
}
</script>

<template>
  <div v-if="open" class="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
    <div class="w-full max-w-6xl h-[90vh] bg-background rounded-lg shadow-lg flex flex-col">
      <!-- 头部 -->
      <div class="flex items-center justify-between p-6 border-b">
        <div class="flex items-center space-x-3">
          <svg class="w-6 h-6 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.34 16.5c-.77.833.192 2.5 1.732 2.5z"/>
          </svg>
          <div>
            <h2 class="text-xl font-semibold">解决合并冲突</h2>
            <p class="text-sm text-muted-foreground">
              {{ resolvedConflicts }}/{{ totalConflicts }} 个冲突已解决
            </p>
          </div>
        </div>
        
        <div class="flex items-center space-x-3">
          <div class="w-32 bg-muted rounded-full h-2">
            <div 
              class="bg-primary h-2 rounded-full transition-all"
              :style="{ width: `${(resolvedConflicts / totalConflicts) * 100}%` }"
            ></div>
          </div>
          <Button variant="ghost" @click="handleCancel">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
            </svg>
          </Button>
        </div>
      </div>

      <!-- 主要内容 -->
      <div class="flex-1 flex overflow-hidden">
        <!-- 左侧：文件列表 -->
        <div class="w-80 border-r bg-muted/30 overflow-y-auto">
          <div class="p-4">
            <h3 class="font-medium mb-3">冲突文件</h3>
            <div class="space-y-2">
              <div
                v-for="file in files"
                :key="file.path"
                class="p-3 rounded-lg border cursor-pointer transition-colors"
                :class="{
                  'bg-background border-primary': selectedFile === file.path,
                  'hover:bg-background/50': selectedFile !== file.path
                }"
                @click="selectFile(file.path)"
              >
                <div class="flex items-center justify-between">
                  <div class="flex items-center space-x-2">
                    <svg class="w-4 h-4 text-muted-foreground" fill="currentColor" viewBox="0 0 24 24">
                      <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                      <polyline points="14,2 14,8 20,8"/>
                    </svg>
                    <span class="font-medium text-sm truncate">{{ file.path }}</span>
                  </div>
                  <Badge 
                    :variant="file.resolved ? 'default' : 'destructive'"
                    class="text-xs"
                  >
                    {{ file.resolved ? '已解决' : file.conflicts.length }}
                  </Badge>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- 右侧：冲突详情 -->
        <div class="flex-1 flex flex-col">
          <div v-if="!currentFile" class="flex-1 flex items-center justify-center">
            <div class="text-center text-muted-foreground">
              <svg class="w-16 h-16 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
              </svg>
              <p>选择文件查看冲突详情</p>
            </div>
          </div>

          <div v-else class="flex-1 flex flex-col">
            <!-- 冲突列表 -->
            <div class="border-b p-4">
              <h3 class="font-medium mb-3">{{ currentFile.path }} 中的冲突</h3>
              <div class="flex flex-wrap gap-2">
                <Button
                  v-for="(conflict, index) in currentFile.conflicts"
                  :key="conflict.id"
                  :variant="selectedConflict === conflict.id ? 'default' : 'outline'"
                  size="sm"
                  @click="selectConflict(conflict.id)"
                >
                  冲突 {{ index + 1 }}
                  <svg v-if="conflict.resolution" class="w-3 h-3 ml-1 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                  </svg>
                </Button>
              </div>
            </div>

            <!-- 冲突内容 -->
            <div v-if="currentConflict" class="flex-1 overflow-hidden">
              <div class="h-full grid grid-cols-2 gap-4 p-4">
                <!-- 当前分支 -->
                <Card>
                  <CardHeader class="pb-3">
                    <CardTitle class="text-sm flex items-center justify-between">
                      <span>当前分支 (HEAD)</span>
                      <Button
                        size="sm"
                        variant="outline"
                        @click="resolveConflict(currentConflict.id, 'current')"
                      >
                        使用此版本
                      </Button>
                    </CardTitle>
                  </CardHeader>
                  <CardContent>
                    <pre class="text-sm bg-muted p-3 rounded overflow-auto max-h-40">{{ currentConflict.currentContent }}</pre>
                  </CardContent>
                </Card>

                <!-- 传入分支 -->
                <Card>
                  <CardHeader class="pb-3">
                    <CardTitle class="text-sm flex items-center justify-between">
                      <span>传入分支</span>
                      <Button
                        size="sm"
                        variant="outline"
                        @click="resolveConflict(currentConflict.id, 'incoming')"
                      >
                        使用此版本
                      </Button>
                    </CardTitle>
                  </CardHeader>
                  <CardContent>
                    <pre class="text-sm bg-muted p-3 rounded overflow-auto max-h-40">{{ currentConflict.incomingContent }}</pre>
                  </CardContent>
                </Card>
              </div>

              <!-- 解决方案选择 -->
              <div class="border-t p-4">
                <div class="flex items-center justify-between mb-4">
                  <h4 class="font-medium">选择解决方案</h4>
                  <div class="flex space-x-2">
                    <Button
                      size="sm"
                      variant="outline"
                      @click="resolveConflict(currentConflict.id, 'both')"
                    >
                      保留两者
                    </Button>
                    <Button
                      size="sm"
                      variant="outline"
                      @click="resolveConflict(currentConflict.id, 'manual')"
                    >
                      手动编辑
                    </Button>
                  </div>
                </div>

                <!-- 手动编辑区域 -->
                <div v-if="currentConflict.resolution === 'manual'" class="mt-4">
                  <label class="text-sm font-medium mb-2 block">手动编辑内容</label>
                  <textarea
                    :value="currentConflict.manualContent || ''"
                    @input="currentConflict.manualContent = ($event.target as HTMLTextAreaElement).value"
                    class="w-full h-32 p-3 border border-border rounded-md font-mono text-sm"
                    placeholder="输入解决后的内容..."
                  ></textarea>
                </div>

                <!-- 预览解决结果 -->
                <div v-if="currentConflict.resolution" class="mt-4">
                  <label class="text-sm font-medium mb-2 block">解决结果预览</label>
                  <pre class="text-sm bg-green-50 dark:bg-green-900/20 p-3 rounded border border-green-200 dark:border-green-800 overflow-auto max-h-32">{{ getResolutionContent(currentConflict) }}</pre>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 底部操作 -->
      <div class="flex items-center justify-between p-6 border-t bg-muted/30">
        <div class="text-sm text-muted-foreground">
          解决所有冲突后可以继续合并操作
        </div>
        <div class="flex space-x-3">
          <Button variant="outline" @click="handleCancel">
            取消合并
          </Button>
          <Button @click="handleResolve" :disabled="!canResolve">
            <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
            </svg>
            完成解决
          </Button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 确保代码内容不会换行导致布局问题 */
pre {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  white-space: pre-wrap;
  word-break: break-all;
}

/* 滚动条样式 */
.overflow-auto::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}

.overflow-auto::-webkit-scrollbar-track {
  background: hsl(var(--muted));
}

.overflow-auto::-webkit-scrollbar-thumb {
  background: hsl(var(--muted-foreground) / 0.3);
  border-radius: 3px;
}

.overflow-auto::-webkit-scrollbar-thumb:hover {
  background: hsl(var(--muted-foreground) / 0.5);
}
</style>

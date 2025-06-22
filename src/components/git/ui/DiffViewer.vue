<script setup lang="ts">
import { computed } from 'vue';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { File, FileText } from 'lucide-vue-next';

interface DiffLine {
  type: 'context' | 'addition' | 'deletion' | 'header';
  content: string;
  oldLineNumber?: number;
  newLineNumber?: number;
}

interface Props {
  fileName: string;
  oldContent?: string;
  newContent?: string;
  diff?: string;
  additions?: number;
  deletions?: number;
  showLineNumbers?: boolean;
  maxHeight?: string;
}

const props = withDefaults(defineProps<Props>(), {
  showLineNumbers: true,
  maxHeight: '400px'
});

// 解析差异内容
const diffLines = computed((): DiffLine[] => {
  if (props.diff) {
    return parseDiff(props.diff);
  }

  if (props.oldContent && props.newContent) {
    return generateDiff(props.oldContent, props.newContent);
  }

  return [];
});

// 解析 Git diff 格式
function parseDiff(diff: string): DiffLine[] {
  const lines = diff.split('\n');
  const result: DiffLine[] = [];
  let oldLineNum = 1;
  let newLineNum = 1;

  for (const line of lines) {
    if (line.startsWith('@@')) {
      // 解析行号信息
      const match = line.match(/@@ -(\d+),?\d* \+(\d+),?\d* @@/);
      if (match) {
        oldLineNum = parseInt(match[1]);
        newLineNum = parseInt(match[2]);
      }
      result.push({
        type: 'header',
        content: line,
        oldLineNumber: undefined,
        newLineNumber: undefined
      });
    } else if (line.startsWith('+')) {
      result.push({
        type: 'addition',
        content: line.substring(1),
        oldLineNumber: undefined,
        newLineNumber: newLineNum++
      });
    } else if (line.startsWith('-')) {
      result.push({
        type: 'deletion',
        content: line.substring(1),
        oldLineNumber: oldLineNum++,
        newLineNumber: undefined
      });
    } else if (line.startsWith(' ')) {
      result.push({
        type: 'context',
        content: line.substring(1),
        oldLineNumber: oldLineNum++,
        newLineNumber: newLineNum++
      });
    }
  }

  return result;
}

// 简单的差异生成（实际项目中应使用专业的 diff 库）
function generateDiff(oldContent: string, newContent: string): DiffLine[] {
  const oldLines = oldContent.split('\n');
  const newLines = newContent.split('\n');
  const result: DiffLine[] = [];

  // 这里是一个简化的实现，实际应该使用 Myers 算法或其他 diff 算法
  const maxLines = Math.max(oldLines.length, newLines.length);

  for (let i = 0; i < maxLines; i++) {
    const oldLine = oldLines[i];
    const newLine = newLines[i];

    if (oldLine === newLine) {
      result.push({
        type: 'context',
        content: oldLine || '',
        oldLineNumber: i + 1,
        newLineNumber: i + 1
      });
    } else if (oldLine && !newLine) {
      result.push({
        type: 'deletion',
        content: oldLine,
        oldLineNumber: i + 1,
        newLineNumber: undefined
      });
    } else if (!oldLine && newLine) {
      result.push({
        type: 'addition',
        content: newLine,
        oldLineNumber: undefined,
        newLineNumber: i + 1
      });
    } else {
      // 行被修改
      result.push({
        type: 'deletion',
        content: oldLine,
        oldLineNumber: i + 1,
        newLineNumber: undefined
      });
      result.push({
        type: 'addition',
        content: newLine,
        oldLineNumber: undefined,
        newLineNumber: i + 1
      });
    }
  }

  return result;
}

// 获取行的样式类
function getLineClasses(line: DiffLine): string {
  const baseClasses = 'flex font-mono text-sm leading-relaxed';

  switch (line.type) {
    case 'addition':
      return `${baseClasses} bg-green-50 text-green-800 dark:bg-green-900/20 dark:text-green-200`;
    case 'deletion':
      return `${baseClasses} bg-red-50 text-red-800 dark:bg-red-900/20 dark:text-red-200`;
    case 'header':
      return `${baseClasses} bg-muted text-muted-foreground font-semibold`;
    default:
      return `${baseClasses} hover:bg-muted/50`;
  }
}

// 获取行号的样式类
function getLineNumberClasses(line: DiffLine): string {
  const baseClasses = 'w-12 px-2 text-right text-xs text-muted-foreground select-none border-r border-border';

  switch (line.type) {
    case 'addition':
      return `${baseClasses} bg-green-100 dark:bg-green-900/30`;
    case 'deletion':
      return `${baseClasses} bg-red-100 dark:bg-red-900/30`;
    case 'header':
      return `${baseClasses} bg-muted`;
    default:
      return baseClasses;
  }
}

// 获取文件扩展名
const fileExtension = computed(() => {
  return props.fileName.split('.').pop()?.toLowerCase() || '';
});

// 获取文件类型图标组件
const getFileIcon = computed(() => {
  const extension = fileExtension.value;

  // 对于常见的代码文件类型，使用File图标
  const codeExtensions = ['js', 'ts', 'vue', 'css', 'html', 'json', 'xml', 'yaml', 'yml', 'py', 'java', 'cpp', 'c', 'h', 'php', 'rb', 'go', 'rs', 'swift', 'kt'];

  if (codeExtensions.includes(extension)) {
    return File;
  }

  // 对于文档类型，使用FileText图标
  const docExtensions = ['md', 'txt', 'doc', 'docx', 'pdf', 'rtf'];

  if (docExtensions.includes(extension)) {
    return FileText;
  }

  // 默认使用File图标
  return File;
});
</script>

<template>
  <Card class="w-full gap-4 shadow-none">
    <CardHeader class="gap-0">
      <CardTitle class="flex items-center justify-between p">
        <div class="flex items-center space-x-2">
          <component :is="getFileIcon" class="w-4 h-4" />
          <span class="font-mono text-sm">{{ fileName }}</span>
        </div>
        
        <div v-if="additions !== undefined || deletions !== undefined" class="flex items-center space-x-2">
          <span v-if="additions" class="text-green-600">
            +{{ additions }}
          </span>
          <span v-if="deletions" class="text-green-600">
            -{{ deletions }}
          </span>
        </div>
      </CardTitle>
    </CardHeader>
    
    <CardContent class="p-0">
      <div 
        class="overflow-auto border-t border-border"
        :style="{ maxHeight: maxHeight }"
      >
        <div v-if="diffLines.length === 0" class="p-4 text-center text-muted-foreground">
          <FileText class="w-8 h-8 mx-auto mb-2" />
          <p>无差异内容</p>
        </div>
        
        <div v-else>
          <div
            v-for="(line, index) in diffLines"
            :key="index"
            :class="getLineClasses(line)"
          >
            <!-- 行号 -->
            <div v-if="showLineNumbers" class="flex">
              <div :class="getLineNumberClasses(line)">
                {{ line.oldLineNumber || '' }}
              </div>
              <div :class="getLineNumberClasses(line)">
                {{ line.newLineNumber || '' }}
              </div>
            </div>
            
            <!-- 差异标识 -->
            <div class="w-6 px-1 text-center flex-shrink-0">
              <span v-if="line.type === 'addition'" class="text-green-600">+</span>
              <span v-else-if="line.type === 'deletion'" class="text-red-600">-</span>
              <span v-else-if="line.type === 'header'" class="text-muted-foreground">@</span>
            </div>
            
            <!-- 内容 -->
            <div class="flex-1 px-2 py-1 overflow-x-auto">
              <pre class="whitespace-pre-wrap break-all">{{ line.content }}</pre>
            </div>
          </div>
        </div>
      </div>
    </CardContent>
  </Card>
</template>

<style scoped>
/* 确保代码内容不会换行导致布局问题 */
pre {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 13px;
  line-height: 1.4;
}

/* 滚动条样式 */
.overflow-auto::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

.overflow-auto::-webkit-scrollbar-track {
  background: hsl(var(--muted));
}

.overflow-auto::-webkit-scrollbar-thumb {
  background: hsl(var(--muted-foreground) / 0.3);
  border-radius: 4px;
}

.overflow-auto::-webkit-scrollbar-thumb:hover {
  background: hsl(var(--muted-foreground) / 0.5);
}
</style>

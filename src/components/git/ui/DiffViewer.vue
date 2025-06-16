<script setup lang="ts">
import { computed } from 'vue';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';

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

// 获取文件类型图标
const fileIcon = computed(() => {
  const iconMap: Record<string, string> = {
    js: 'M3 3h18v18H3V3zm16.525 13.707c-.131-.821-.666-1.511-2.252-2.155-.552-.259-1.165-.438-1.349-.854-.068-.248-.078-.382-.034-.529.113-.484.687-.629 1.137-.495.293.09.563.315.732.676.775-.507.775-.507 1.316-.844-.203-.314-.304-.451-.439-.586-.473-.528-1.103-.798-2.126-.77l-.528.067c-.507.124-.991.395-1.283.754-.855.968-.608 2.655.427 3.354 1.023.765 2.521.933 2.712 1.653.18.878-.652 1.159-1.475 1.058-.607-.136-.945-.439-1.316-.998l-1.372.788c.157.359.337.517.607.832 1.305 1.316 4.568 1.249 5.153-.754.021-.067.18-.528.056-1.237l.034.049zm-6.737-5.434h-1.686c0 1.453-.007 2.898-.007 4.354 0 .924.047 1.772-.104 2.033-.247.517-.886.451-1.175.359-.297-.146-.448-.349-.623-.641-.047-.078-.082-.146-.095-.146l-1.368.844c.229.473.563.879.994 1.137.641.383 1.502.507 2.404.305.588-.17 1.095-.519 1.358-1.059.384-.697.302-1.553.299-2.509.008-1.541 0-3.083 0-4.635l.003-.042z',
    ts: 'M1.125 0C.502 0 0 .502 0 1.125v21.75C0 23.498.502 24 1.125 24h21.75c.623 0 1.125-.502 1.125-1.125V1.125C24 .502 23.498 0 22.875 0zm17.363 9.75c.612 0 1.154.037 1.627.111a6.38 6.38 0 0 1 1.306.34v2.458a3.95 3.95 0 0 0-.643-.361 5.093 5.093 0 0 0-.717-.26 5.453 5.453 0 0 0-1.426-.2c-.3 0-.573.028-.819.086a2.1 2.1 0 0 0-.623.242c-.17.104-.3.229-.393.374a.888.888 0 0 0-.14.49c0 .196.053.373.156.529.104.156.252.304.443.444s.423.276.696.41c.273.135.582.274.926.416.47.197.892.407 1.266.628.374.222.695.473.963.753.268.279.472.598.614.957.142.359.214.776.214 1.253 0 .657-.125 1.21-.373 1.656a3.033 3.033 0 0 1-1.012 1.085 4.38 4.38 0 0 1-1.487.596c-.566.12-1.163.18-1.79.18a9.916 9.916 0 0 1-1.84-.164 5.544 5.544 0 0 1-1.512-.493v-2.63a5.033 5.033 0 0 0 3.237 1.2c.333 0 .624-.03.872-.09.249-.06.456-.144.623-.25.166-.108.29-.234.373-.38a1.023 1.023 0 0 0-.074-1.089 2.12 2.12 0 0 0-.537-.5 5.597 5.597 0 0 0-.807-.444 27.72 27.72 0 0 0-1.007-.436c-.918-.383-1.602-.852-2.053-1.405-.45-.553-.676-1.222-.676-2.005 0-.614.123-1.141.369-1.582.246-.441.58-.804 1.004-1.089a4.494 4.494 0 0 1 1.47-.629 7.536 7.536 0 0 1 1.77-.201zm-15.113.188h9.563v2.166H9.506v9.646H6.789v-9.646H3.375z',
    vue: 'M24,1.61H14.06L12,5.16,9.94,1.61H0L12,22.39ZM12,14.08,5.16,2.23H9.59L12,6.41l2.41-4.18h4.43Z',
    css: 'M1.5 0h21l-1.91 21.563L11.977 24l-8.565-2.438L1.5 0zm17.09 4.413L5.41 4.41l.213 2.622 10.125.002-.255 2.716h-6.64l.24 2.573h6.182l-.366 3.523-2.91.804-2.956-.81-.188-2.11h-2.61l.29 3.855L12 19.288l5.373-1.53L18.59 4.414z',
    html: 'M1.5 0h21l-1.91 21.563L11.977 24l-8.564-2.438L1.5 0zm7.031 9.75l-.232-2.718 10.059.003.23-2.622L5.412 4.41l.698 8.01h9.126l-.326 3.426-2.91.804-2.955-.81-.188-2.11H6.248l.33 4.171L12 19.351l5.379-1.443.744-8.157H8.531z'
  };
  
  return iconMap[fileExtension.value] || 'M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z';
});
</script>

<template>
  <Card class="w-full">
    <CardHeader class="pb-3">
      <CardTitle class="flex items-center justify-between">
        <div class="flex items-center space-x-2">
          <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
            <path :d="fileIcon"/>
          </svg>
          <span class="font-mono text-sm">{{ fileName }}</span>
        </div>
        
        <div v-if="additions !== undefined || deletions !== undefined" class="flex items-center space-x-2">
          <Badge v-if="additions" variant="outline" class="text-green-600 border-green-600">
            +{{ additions }}
          </Badge>
          <Badge v-if="deletions" variant="outline" class="text-red-600 border-red-600">
            -{{ deletions }}
          </Badge>
        </div>
      </CardTitle>
    </CardHeader>
    
    <CardContent class="p-0">
      <div 
        class="overflow-auto border-t border-border"
        :style="{ maxHeight: maxHeight }"
      >
        <div v-if="diffLines.length === 0" class="p-4 text-center text-muted-foreground">
          <svg class="w-8 h-8 mx-auto mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
          </svg>
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

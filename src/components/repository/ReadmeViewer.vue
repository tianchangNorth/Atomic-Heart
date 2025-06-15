<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import MarkdownIt from 'markdown-it';
import type { ReadmeViewerProps } from '@/types/repository';
import { highlightAll } from '@/utils/prismLoader';

const props = withDefaults(defineProps<ReadmeViewerProps>(), {
  loading: false
});

// Markdown 渲染器配置
const md = new MarkdownIt({
  html: true,
  linkify: true,
  typographer: true,
  breaks: true
});

// 自定义代码块渲染
function escapeHtml(html: string): string {
  return html
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#039;');
}

md.renderer.rules.fence = function (tokens, idx) {
  const token = tokens[idx];
  const info = token.info ? token.info.trim() : '';
  const langName = info.split(/\s+/g)[0];

  const escapedContent = escapeHtml(token.content);

  if (langName) {
    return `<pre class="language-${langName}"><code class="language-${langName}">${escapedContent}</code></pre>`;
  }

  return `<pre><code>${escapedContent}</code></pre>`;
};

// 自定义链接渲染（处理相对链接）
md.renderer.rules.link_open = function (tokens, idx, options) {
  const token = tokens[idx];
  const hrefIndex = token.attrIndex('href');

  if (hrefIndex >= 0) {
    const href = token.attrs![hrefIndex][1];

    // 如果是相对链接，转换为绝对链接
    if (href.startsWith('./') || href.startsWith('../') || (!href.startsWith('http') && !href.startsWith('#'))) {
      // 这里可以根据仓库信息构建完整的 URL
      token.attrs![hrefIndex][1] = href;
      token.attrSet('target', '_blank');
      token.attrSet('rel', 'noopener noreferrer');
    } else if (href.startsWith('http')) {
      // 外部链接在新窗口打开
      token.attrSet('target', '_blank');
      token.attrSet('rel', 'noopener noreferrer');
    }
  }

  return md.renderer.renderToken(tokens, idx, options);
};

// 自定义图片渲染（处理相对路径）
md.renderer.rules.image = function (tokens, idx, options, renderer) {
  const token = tokens[idx];
  const srcIndex = token.attrIndex('src');

  if (srcIndex >= 0) {
    const src = token.attrs![srcIndex][1];

    // 如果是相对路径，转换为绝对路径
    if (src.startsWith('./') || src.startsWith('../') || (!src.startsWith('http') && !src.startsWith('data:'))) {
      // 这里可以根据仓库信息构建完整的图片 URL
      token.attrs![srcIndex][1] = src;
    }
  }

  token.attrSet('class', 'max-w-full h-auto rounded border');
  token.attrSet('loading', 'lazy');

  return renderer.renderToken(tokens, idx, options);
};

// 解码 Base64 内容
function decodeBase64Utf8(base64: string): string {
  try {
    const binary = atob(base64);
    // 转成 UTF-8 字符串
    return decodeURIComponent(escape(binary));
  } catch (e) {
    console.error('Base64 decode error:', e);
    return '';
  }
}

// 渲染的 HTML 内容
const renderedHtml = computed(() => {
  if (!props.readme) return '';

  try {
    const decodedContent = decodeBase64Utf8(props.readme.content);
    return md.render(decodedContent);
  } catch (err) {
    console.error('Markdown 渲染失败:', err);
    return '<p class="text-red-500">Markdown 渲染失败</p>';
  }
});

// 使用我们的安全高亮函数
const highlightAllCode = () => {
  highlightAll();
};

// 组件挂载后高亮代码
onMounted(() => {
  // 延迟执行以确保 DOM 已更新
  setTimeout(() => {
    highlightAllCode();
  }, 100);
});

// 监听内容变化，重新高亮
const observer = ref<MutationObserver | null>(null);

onMounted(() => {
  const readmeElement = document.getElementById('readme-content');
  if (readmeElement) {
    observer.value = new MutationObserver(() => {
      highlightAllCode();
    });

    observer.value.observe(readmeElement, {
      childList: true,
      subtree: true
    });
  }
});

onUnmounted(() => {
  if (observer.value) {
    observer.value.disconnect();
  }
});
</script>

<template>
  <div class="readme-viewer">
    <!-- 加载状态 -->
    <div v-if="loading" class="flex items-center justify-center py-8">
      <div class="animate-spin rounded-full h-6 w-6 border-b-2 border-primary"></div>
      <span class="ml-2 text-sm text-muted-foreground">加载 README...</span>
    </div>

    <!-- 无 README 文件 -->
    <div v-else-if="!readme" class="text-center py-8">
      <svg class="w-12 h-12 mx-auto text-muted-foreground mb-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
              d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
      </svg>
      <h3 class="text-lg font-medium text-foreground mb-2">没有 README 文件</h3>
      <p class="text-sm text-muted-foreground">
        此仓库没有 README 文件。README 文件可以帮助其他人了解您的项目。
      </p>
    </div>

    <!-- README 内容 -->
    <div v-else class="readme-content">
      <!-- README 文件信息 -->
      <div class="flex items-center justify-between mb-4 pb-3 border-b border-border">
        <div class="flex items-center space-x-2">
          <svg class="w-4 h-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                  d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.246 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"/>
          </svg>
          <span class="text-sm font-medium">{{ readme.name }}</span>
          <span class="text-xs text-muted-foreground">{{ formatFileSize(readme.size) }}</span>
        </div>
        
        <a 
          :href="readme.html_url" 
          target="_blank" 
          rel="noopener noreferrer"
          class="text-xs text-muted-foreground hover:text-foreground transition-colors"
        >
          查看原文件
          <svg class="w-3 h-3 inline ml-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"/>
          </svg>
        </a>
      </div>

      <!-- 渲染的 Markdown 内容 -->
      <div 
        id="readme-content"
        class="prose prose-sm max-w-none dark:prose-invert"
        v-html="renderedHtml"
      ></div>
    </div>
  </div>
</template>

<script lang="ts">
// 格式化文件大小的辅助函数
function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B';

  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
}

export { formatFileSize };
</script>

<style scoped>
/* README 内容样式 */
.readme-content :deep(.prose) {
  color: hsl(var(--foreground));
  max-width: none;
}

.readme-content :deep(.prose h1) {
  color: hsl(var(--foreground));
  font-size: 1.875rem;
  font-weight: 700;
  margin-top: 2rem;
  margin-bottom: 1rem;
  border-bottom: 1px solid hsl(var(--border));
  padding-bottom: 0.5rem;
}

.readme-content :deep(.prose h2) {
  color: hsl(var(--foreground));
  font-size: 1.5rem;
  font-weight: 600;
  margin-top: 1.5rem;
  margin-bottom: 0.75rem;
  border-bottom: 1px solid hsl(var(--border));
  padding-bottom: 0.25rem;
}

.readme-content :deep(.prose h3) {
  color: hsl(var(--foreground));
  font-size: 1.25rem;
  font-weight: 600;
  margin-top: 1.25rem;
  margin-bottom: 0.5rem;
}

.readme-content :deep(.prose h4),
.readme-content :deep(.prose h5),
.readme-content :deep(.prose h6) {
  color: hsl(var(--foreground));
  font-weight: 600;
  margin-top: 1rem;
  margin-bottom: 0.5rem;
}

.readme-content :deep(.prose p) {
  color: hsl(var(--foreground));
  margin-bottom: 1rem;
  line-height: 1.6;
}

.readme-content :deep(.prose a) {
  color: hsl(var(--primary));
  text-decoration: underline;
  text-decoration-color: transparent;
  transition: text-decoration-color 0.2s;
}

.readme-content :deep(.prose a:hover) {
  text-decoration-color: hsl(var(--primary));
}

.readme-content :deep(.prose code) {
  background-color: hsl(var(--muted));
  color: hsl(var(--foreground));
  padding: 0.125rem 0.25rem;
  border-radius: 0.25rem;
  font-size: 0.875em;
}

.readme-content :deep(.prose pre) {
  background-color: hsl(var(--muted));
  color: hsl(var(--foreground));
  padding: 1rem;
  border-radius: 0.5rem;
  overflow-x: auto;
  margin: 1rem 0;
  border: 1px solid hsl(var(--border));
}

.readme-content :deep(.prose pre code) {
  background-color: transparent;
  padding: 0;
  border-radius: 0;
}

.readme-content :deep(.prose blockquote) {
  border-left: 4px solid hsl(var(--border));
  padding-left: 1rem;
  margin: 1rem 0;
  color: hsl(var(--muted-foreground));
  font-style: italic;
}

.readme-content :deep(.prose ul),
.readme-content :deep(.prose ol) {
  margin: 1rem 0;
  padding-left: 1.5rem;
}

.readme-content :deep(.prose li) {
  margin: 0.25rem 0;
}

.readme-content :deep(.prose table) {
  width: 100%;
  border-collapse: collapse;
  margin: 1rem 0;
  border: 1px solid hsl(var(--border));
  border-radius: 0.5rem;
  overflow: hidden;
}

.readme-content :deep(.prose th),
.readme-content :deep(.prose td) {
  padding: 0.75rem;
  text-align: left;
  border-bottom: 1px solid hsl(var(--border));
}

.readme-content :deep(.prose th) {
  background-color: hsl(var(--muted));
  font-weight: 600;
}

.readme-content :deep(.prose img) {
  max-width: 100%;
  height: auto;
  border-radius: 0.5rem;
  border: 1px solid hsl(var(--border));
  margin: 1rem 0;
}

.readme-content :deep(.prose hr) {
  border: none;
  border-top: 1px solid hsl(var(--border));
  margin: 2rem 0;
}

/* 代码高亮样式 */
.readme-content :deep(.token.comment),
.readme-content :deep(.token.prolog),
.readme-content :deep(.token.doctype),
.readme-content :deep(.token.cdata) {
  color: hsl(var(--muted-foreground));
}

.readme-content :deep(.token.punctuation) {
  color: hsl(var(--foreground));
}

.readme-content :deep(.token.property),
.readme-content :deep(.token.tag),
.readme-content :deep(.token.boolean),
.readme-content :deep(.token.number),
.readme-content :deep(.token.constant),
.readme-content :deep(.token.symbol),
.readme-content :deep(.token.deleted) {
  color: #e06c75;
}

.readme-content :deep(.token.selector),
.readme-content :deep(.token.attr-name),
.readme-content :deep(.token.string),
.readme-content :deep(.token.char),
.readme-content :deep(.token.builtin),
.readme-content :deep(.token.inserted) {
  color: #98c379;
}

.readme-content :deep(.token.operator),
.readme-content :deep(.token.entity),
.readme-content :deep(.token.url),
.readme-content :deep(.language-css .token.string),
.readme-content :deep(.style .token.string) {
  color: #56b6c2;
}

.readme-content :deep(.token.atrule),
.readme-content :deep(.token.attr-value),
.readme-content :deep(.token.keyword) {
  color: #c678dd;
}

.readme-content :deep(.token.function),
.readme-content :deep(.token.class-name) {
  color: #61afef;
}

.readme-content :deep(.token.regex),
.readme-content :deep(.token.important),
.readme-content :deep(.token.variable) {
  color: #d19a66;
}
</style>

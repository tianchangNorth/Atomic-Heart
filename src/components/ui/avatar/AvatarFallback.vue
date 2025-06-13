<template>
  <div
    v-if="canRender"
    :class="cn('flex h-full w-full items-center justify-center rounded-full bg-muted', props.class)"
  >
    <slot />
  </div>
</template>

<script setup lang="ts">
import { computed, inject } from 'vue'
import { cn } from '@/lib/utils'

export interface AvatarFallbackProps {
  class?: string
}

const props = withDefaults(defineProps<AvatarFallbackProps>(), {})

// 注入 Avatar 上下文
const avatarContext = inject('avatarContext', null) as any

// 只在图片加载失败或没有图片时显示 fallback
const canRender = computed(() => {
  if (!avatarContext) {
    return true // 如果没有上下文，默认显示 fallback
  }
  // 当图片出错或者图片还没有加载成功时显示 fallback
  return avatarContext.imageError.value || !avatarContext.imageLoaded.value
})
</script>

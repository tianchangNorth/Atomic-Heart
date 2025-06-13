<template>
  <img
    v-if="canRender"
    :src="src"
    :alt="alt"
    :class="cn('aspect-square h-full w-full', props.class)"
    @load="onLoad"
    @error="onError"
  />
</template>

<script setup lang="ts">
import { ref, watch, computed, inject } from 'vue'
import { cn } from '@/lib/utils'

export interface AvatarImageProps {
  src?: string
  alt?: string
  class?: string
}

const props = withDefaults(defineProps<AvatarImageProps>(), {
  alt: ''
})

// 注入 Avatar 上下文
const avatarContext = inject('avatarContext', null) as any

const hasLoaded = ref(false)
const hasError = ref(false)

// 修复渲染逻辑：只要有 src 就尝试渲染，除非明确出错
const canRender = computed(() => !!props.src && !hasError.value)

const onLoad = () => {
  hasLoaded.value = true
  hasError.value = false
  // 通知 Avatar 容器图片已加载
  if (avatarContext) {
    avatarContext.setImageLoaded(true)
    avatarContext.setImageError(false)
  }
}

const onError = () => {
  hasLoaded.value = false
  hasError.value = true
  // 通知 Avatar 容器图片加载失败
  if (avatarContext) {
    avatarContext.setImageLoaded(false)
    avatarContext.setImageError(true)
  }
}

// Reset state when src changes
watch(() => props.src, () => {
  hasLoaded.value = false
  hasError.value = false
  // 重置 Avatar 容器状态
  if (avatarContext) {
    avatarContext.setImageLoaded(false)
    avatarContext.setImageError(false)
  }
}, { immediate: true })
</script>

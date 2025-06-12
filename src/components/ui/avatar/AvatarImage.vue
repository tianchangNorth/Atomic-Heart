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
import { ref, watch, computed } from 'vue'
import { cn } from '@/lib/utils'

export interface AvatarImageProps {
  src?: string
  alt?: string
  class?: string
}

const props = withDefaults(defineProps<AvatarImageProps>(), {
  alt: ''
})

const hasLoaded = ref(false)
const hasError = ref(false)

const canRender = computed(() => hasLoaded.value && !hasError.value)

const onLoad = () => {
  hasLoaded.value = true
  hasError.value = false
}

const onError = () => {
  hasLoaded.value = false
  hasError.value = true
}

// Reset state when src changes
watch(() => props.src, () => {
  hasLoaded.value = false
  hasError.value = false
})
</script>

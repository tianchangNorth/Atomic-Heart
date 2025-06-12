<template>
  <div
    v-if="canRender"
    :class="cn('flex h-full w-full items-center justify-center rounded-full bg-muted', props.class)"
  >
    <slot />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { cn } from '@/lib/utils'

export interface AvatarFallbackProps {
  class?: string
}

const props = withDefaults(defineProps<AvatarFallbackProps>(), {})

// For now, always render the fallback
// In a more complex implementation, this would check if the image failed to load
const canRender = computed(() => true)
</script>

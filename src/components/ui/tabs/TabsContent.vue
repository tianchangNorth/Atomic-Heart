<template>
  <div
    v-if="isActive"
    :class="cn(
      'mt-2 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2',
      props.class
    )"
  >
    <slot />
  </div>
</template>

<script setup lang="ts">
import { inject, computed } from 'vue'
import { cn } from '@/lib/utils'

export interface TabsContentProps {
  value: string
  class?: string
}

const props = defineProps<TabsContentProps>()

const tabsContext = inject('tabsContext') as any

const isActive = computed(() => tabsContext?.currentValue.value === props.value)
</script>

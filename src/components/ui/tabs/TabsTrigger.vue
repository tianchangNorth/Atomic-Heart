<template>
  <button
    :class="cn(
      'inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50',
      isActive 
        ? 'bg-background text-foreground shadow-sm' 
        : 'text-muted-foreground hover:text-foreground',
      props.class
    )"
    @click="handleClick"
  >
    <slot />
  </button>
</template>

<script setup lang="ts">
import { inject, computed } from 'vue'
import { cn } from '@/lib/utils'

export interface TabsTriggerProps {
  value: string
  class?: string
}

const props = defineProps<TabsTriggerProps>()

const tabsContext = inject('tabsContext') as any

const isActive = computed(() => tabsContext?.currentValue.value === props.value)

const handleClick = () => {
  tabsContext?.setValue(props.value)
}
</script>

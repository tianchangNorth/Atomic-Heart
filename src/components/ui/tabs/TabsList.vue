<template>
  <div
    :class="cn(
      'inline-flex h-10 items-center justify-center rounded-md bg-muted p-1 text-muted-foreground',
      props.class
    )"
  >
    <slot />
  </div>
</template>

<script setup lang="ts">
import { cn } from '@/lib/utils'

export interface TabsListProps {
  class?: string
}

const props = withDefaults(defineProps<TabsListProps>(), {})
</script>

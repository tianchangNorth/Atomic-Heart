<template>
  <div :class="cn('w-full', props.class)">
    <slot />
  </div>
</template>

<script setup lang="ts">
import { provide, ref, watch } from 'vue'
import { cn } from '@/lib/utils'

export interface TabsProps {
  defaultValue?: string
  value?: string
  class?: string
}

const props = withDefaults(defineProps<TabsProps>(), {})

const emit = defineEmits<{
  'update:value': [value: string]
}>()

const currentValue = ref(props.defaultValue || props.value || '')

// 监听外部 value 变化
watch(() => props.value, (newValue) => {
  if (newValue !== undefined && newValue !== currentValue.value) {
    currentValue.value = newValue
  }
}, { immediate: true })

const setValue = (value: string) => {
  currentValue.value = value
  emit('update:value', value)
}

provide('tabsContext', {
  currentValue,
  setValue
})
</script>

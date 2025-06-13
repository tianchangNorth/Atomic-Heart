<template>
  <div :class="avatarVariants({ size, class: props.class })">
    <slot />
  </div>
</template>

<script setup lang="ts">
import { provide, ref } from 'vue'
import { cva, type VariantProps } from 'class-variance-authority'

const avatarVariants = cva(
  'relative flex h-10 w-10 shrink-0 overflow-hidden rounded-full',
  {
    variants: {
      size: {
        sm: 'h-8 w-8',
        default: 'h-10 w-10',
        lg: 'h-12 w-12',
        xl: 'h-16 w-16',
      },
    },
    defaultVariants: {
      size: 'default',
    },
  }
)

export interface AvatarProps {
  size?: VariantProps<typeof avatarVariants>['size']
  class?: string
}

const props = withDefaults(defineProps<AvatarProps>(), {
  size: 'default'
})

// Avatar 上下文，用于协调 AvatarImage 和 AvatarFallback 的显示
const imageLoaded = ref(false)
const imageError = ref(false)

provide('avatarContext', {
  imageLoaded,
  imageError,
  setImageLoaded: (loaded: boolean) => { imageLoaded.value = loaded },
  setImageError: (error: boolean) => { imageError.value = error }
})
</script>

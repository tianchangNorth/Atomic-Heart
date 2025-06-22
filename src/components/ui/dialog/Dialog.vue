<script setup lang="ts">
import { computed, onMounted, onUnmounted } from 'vue';

interface Props {
  open?: boolean;
}

interface Emits {
  (e: 'update:open', value: boolean): void;
}

const props = withDefaults(defineProps<Props>(), {
  open: false
});

const emit = defineEmits<Emits>();

const isOpen = computed({
  get: () => props.open,
  set: (value) => emit('update:open', value)
});

const closeDialog = () => {
  isOpen.value = false;
};

const handleEscape = (event: KeyboardEvent) => {
  if (event.key === 'Escape' && props.open) {
    closeDialog();
  }
};

const handleBackdropClick = (event: MouseEvent) => {
  if (event.target === event.currentTarget) {
    closeDialog();
  }
};

onMounted(() => {
  document.addEventListener('keydown', handleEscape);
});

onUnmounted(() => {
  document.removeEventListener('keydown', handleEscape);
});
</script>

<template>
  <Teleport to="body">
    <Transition
      enter-active-class="transition-opacity duration-200"
      enter-from-class="opacity-0"
      enter-to-class="opacity-100"
      leave-active-class="transition-opacity duration-200"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0"
    >
      <div
        v-if="isOpen"
        class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
        @click="handleBackdropClick"
      >
        <slot />
      </div>
    </Transition>
  </Teleport>
</template>

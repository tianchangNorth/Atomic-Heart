<script setup lang="ts">
import Toast from './Toast.vue';
import { useToastStore } from './useToast.ts';

const toastStore = useToastStore();
const toasts = toastStore.toasts;

const removeToast = (id: string) => {
  toastStore.removeToast(id);
};
</script>

<template>
  <Teleport to="body">
    <div
      v-if="toasts.length > 0"
      class="fixed top-4 right-4 z-50 flex flex-col gap-2 max-w-sm w-full"
    >
      <Toast
        v-for="toast in toasts"
        :key="toast.id"
        v-bind="toast"
        @close="removeToast"
      />
    </div>
  </Teleport>
</template>

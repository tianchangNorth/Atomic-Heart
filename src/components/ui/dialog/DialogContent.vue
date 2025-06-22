<script setup lang="ts">
interface Props {
  class?: string;
}

const props = withDefaults(defineProps<Props>(), {
  class: ''
});
</script>

<template>
  <div
    :class="[
      'relative bg-background border border-border rounded-lg shadow-lg p-6 w-full max-w-lg mx-4',
      'transform transition-all duration-200',
      'animate-in fade-in-0 zoom-in-95',
      props.class
    ]"
    @click.stop
  >
    <slot />
  </div>
</template>

<style scoped>
@keyframes animate-in {
  from {
    opacity: 0;
    transform: scale(0.95);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

.animate-in {
  animation: animate-in 0.2s ease-out;
}

.fade-in-0 {
  animation-fill-mode: both;
}

.zoom-in-95 {
  animation-fill-mode: both;
}
</style>

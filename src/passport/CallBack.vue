<template>
  <div>
    loading...
  </div>
</template>

<script lang="ts" setup>
import { onMounted } from 'vue';
import { useRoute } from 'vue-router'
const route = useRoute()
const code = route.query.code;
const receivedState = route.query.state;

onMounted(() => {
  const redirectUrl = `http://localhost:5800?code=${code}&state=${receivedState}`
  window.location.replace(redirectUrl) // 更强制，避免 history 堆栈污染
})
</script>
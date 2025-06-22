<script setup lang="ts">
import { RouterView } from "vue-router";
import { ToastContainer } from "@/components/ui/toast";
</script>

<template>
  <RouterView />
  <ToastContainer />
</template>
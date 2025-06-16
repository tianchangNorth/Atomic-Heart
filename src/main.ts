import router from "./router/index";
import App from "./App.vue";

import { createPinia } from 'pinia'
import { createApp } from "vue";
import { initTheme } from '@/composables/useTheme';

import './index.css'

const app = createApp(App);

app.use(router);
app.use(createPinia());

// 初始化主题系统
initTheme().then(() => {
  app.mount("#app");
}).catch((error) => {
  console.error('主题初始化失败:', error);
  // 即使主题初始化失败，也要挂载应用
  app.mount("#app");
});

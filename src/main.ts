import router from "./router/index";
import App from "./App.vue";

import { createPinia } from 'pinia'
import { createApp } from "vue";
import { initTheme } from '@/composables/useTheme';
import { initLocalRepositories } from '@/composables/useLocalRepositories';
import { deepLinkService } from '@/services/deepLinkService';

import './index.css'

const app = createApp(App);

app.use(router);
app.use(createPinia());

// 初始化应用系统
Promise.all([
  initTheme(),
  initLocalRepositories(),
  deepLinkService.initialize()
]).then(() => {
  app.mount("#app");
}).catch((error) => {
  console.error('应用初始化失败:', error);
  // 即使初始化失败，也要挂载应用
  app.mount("#app");
});

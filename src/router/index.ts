import { createRouter, createWebHistory } from 'vue-router';
import { useUserStore } from '../stores/index';

import Login from '@/passport/Login.vue'
import CallBack from '@/passport/CallBack.vue'
import Home from '@/view/Home.vue'
import Overview from '@/view/Overview.vue'
import Issues from '@/view/Issues.vue'
import Repositories from '@/view/Repositories.vue'
import Projects from '@/view/Projects.vue'

const routes = [
  {
    path: '/',
    component: Home,
    redirect: '/overview',
    children: [
      { path: '/overview', component: Overview, name: 'Overview' },
      { path: '/issues', component: Issues, name: 'Issues' },
      { path: '/repositories', component: Repositories, name: 'Repositories' },
      { path: '/projects', component: Projects, name: 'Projects' },
    ]
  },
  { path: '/login', component: Login },
  { path: '/callback', component: CallBack },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

router.beforeEach(async (to) => {
  const userStore = useUserStore();
  if (userStore.user.id) {
    return true
  } else {
    if (to.path === '/login' || to.path === '/callback' || to.path === '/404') {
      return true
    } else {
      const { success } = await userStore.fetchInfo(); // 尝试获取用户信息
      if (!success) {
        return '/login'; // 重定向到登录页
      }
      return true
    }
  }
});

export default router
import { createRouter, createWebHistory } from 'vue-router';
import { useUserStore } from '../stores/index';
import { delToken } from '../utils/token';

import Login from '@/passport/Login.vue'
import Home from '@/view/Home.vue'
import Overview from '@/view/Overview.vue'
import Issues from '@/view/Issues.vue'
import Repositories from '@/view/Repositories.vue'
import RepositoryDetail from '@/view/RepositoryDetail.vue'
import LocalRepository from '@/view/LocalRepository.vue'
import LocalRepositoryDetail from '@/view/LocalRepositoryDetail.vue'
import Projects from '@/view/Projects.vue'
import Settings from '@/view/Settings.vue'

const routes = [
  {
    path: '/',
    component: Home,
    redirect: '/overview',
    children: [
      { path: '/overview', component: Overview, name: 'Overview' },
      { path: '/issues', component: Issues, name: 'Issues' },
      { path: '/repositories', component: Repositories, name: 'Repositories' },
      { path: '/local-repositories', component: LocalRepository, name: 'LocalRepository' },
      { path: '/local-repositories/:id', component: LocalRepositoryDetail, name: 'LocalRepositoryDetail', props: true },
      { path: '/projects', component: Projects, name: 'Projects' },
      { path: '/settings', component: Settings, name: 'Settings' },
    ]
  },
  {
    path: '/repository/:owner/:repo',
    component: RepositoryDetail,
    name: 'RepositoryDetail',
    props: true
  },
  { path: '/login', component: Login },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

router.beforeEach(async (to) => {
  const userStore = useUserStore();

  // 如果已经有用户信息，直接通过
  if (userStore.user.id) {
    return true;
  }

  // 这些路径不需要验证
  if (to.path === '/login' || to.path === '/callback' || to.path === '/404') {
    return true;
  }

  try {
    // 尝试获取用户信息
    const { success } = await userStore.fetchInfo();
    if (!success) {
      delToken();
      return '/login'; // 重定向到登录页
    }
    return true;
  } catch (error) {
    delToken();
    // console.error('Router guard error:', error);
    // 出错时也重定向到登录页，而不是阻止路由
    return '/login';
  }
});

export default router
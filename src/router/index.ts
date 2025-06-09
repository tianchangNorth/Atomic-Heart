import { createWebHistory, createRouter } from 'vue-router'

import Login from '@/passport/Login.vue'
import CallBack from '@/passport/CallBack.vue'
const routes = [
  { path: '/', component: Login },
  { path: '/callback', component: CallBack },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
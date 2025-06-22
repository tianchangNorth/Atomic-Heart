<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Loader2 } from 'lucide-vue-next'
import { saveToken } from '@/utils/token'
import { useUserStore } from '@/stores'
import { $fetch } from '@/utils/fetch'

const router = useRouter()
const userStore = useUserStore()
const loading = ref(false)
const loginSuccess = ref(false)
const authCode = ref('')
const state = ref('')

// OAuth配置
const oauthConfig = {
  clientId: import.meta.env.VITE_APP_CLIENT_ID,
  clientSecret: import.meta.env.VITE_APP_CLIENT_SECRET,
  redirectUri: '', // 将在启动时动态设置
  state: Math.random().toString(36).substring(2, 15)
}

let unlistenOAuthCallback: (() => void) | null = null

// 启动OAuth回调服务器并设置监听器
const setupOAuthCallback = async () => {
  try {
    console.log('设置OAuth回调');

    // 启动本地回调服务器
    const callbackUrl = await invoke('start_oauth_callback_server') as string
    console.log('回调URL:', callbackUrl);

    oauthConfig.redirectUri = callbackUrl

    // 监听OAuth回调事件
    unlistenOAuthCallback = await listen('oauth-callback', async (event: any) => {
      const { code, state: returnedState } = event.payload

      authCode.value = code
      state.value = returnedState
      await handleCallback()
    })
  } catch (error) {
    console.error('设置OAuth回调失败:', error)
  }
}

// 处理OAuth回调
const handleCallback = async () => {

  if (!authCode.value || !state.value) return

  loading.value = true
  try {
    const response = await $fetch('/login/oauth/access_token ', {
      method: 'POST',
      data: {
        client_id: oauthConfig.clientId,
        client_secret: oauthConfig.clientSecret,
        code: authCode.value,
      }
    })

    if (response.success && response.data.access_token) {
      saveToken(response.data.access_token)
      await userStore.fetchInfo()

      // 显示成功动画
      loginSuccess.value = true

      // 延迟跳转，让用户看到成功动画
      setTimeout(() => {
        router.push('/')
      }, 2000)
    }
  } catch (error) {
    console.error('OAuth token exchange failed:', error)
  } finally {
    loading.value = false
  }
}

// 开始OAuth流程
const startOAuth = async () => {
  if (!oauthConfig.redirectUri) {
    console.error('回调URL未设置')
    return
  }

  const authUrl = `https://atomgit.com/login/oauth/authorize?client_id=${oauthConfig.clientId}&redirect_uri=${encodeURIComponent(oauthConfig.redirectUri)}&state=${oauthConfig.state}`

  try {
    await invoke('open_url', { url: authUrl })
  } catch (error) {
    console.error('启动OAuth流程失败:', error)
  }
}

onMounted(() => {
  setupOAuthCallback()
})

onUnmounted(() => {
  if (unlistenOAuthCallback) {
    unlistenOAuthCallback()
  }
})
</script>

<template>
  <div class="min-h-screen relative overflow-hidden bg-gradient-to-br from-background via-muted/20 to-background">
    <!-- 动态背景效果 -->
    <div class="absolute inset-0 overflow-hidden">
      <!-- 渐变球体动画 -->
      <div class="absolute -top-40 -right-40 w-80 h-80 bg-primary/10 rounded-full blur-3xl animate-float"></div>
      <div class="absolute -bottom-40 -left-40 w-80 h-80 bg-blue-500/10 rounded-full blur-3xl animate-float-delayed"></div>
      <div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-96 h-96 bg-purple-500/5 rounded-full blur-3xl animate-pulse-slow"></div>

      <!-- 网格背景 -->
      <div class="absolute inset-0 bg-grid-pattern opacity-5"></div>
    </div>

    <!-- 主要内容 -->
    <div class="relative z-10 min-h-screen flex items-center justify-center p-4">
      <div class="w-full max-w-md">
        <!-- Logo 和品牌区域 -->
        <Transition
          appear
          enter-active-class="transition-all duration-1000 ease-out"
          enter-from-class="transform -translate-y-8 opacity-0"
          enter-to-class="transform translate-y-0 opacity-100"
        >
          <div class="text-center mb-8">
            <div class="inline-flex items-center bg-gray-50 dark:bg-gray-800 justify-center w-16 h-16 p-2 rounded-2xl shadow-lg mb-4 animate-bounce-gentle">
              <img src="@/assets/icon.png">
            </div>
            <h1 class="text-3xl font-bold text-foreground mb-2 bg-gradient-to-r from-primary to-blue-600 bg-clip-text">
              AtomGit
            </h1>
            <p class="text-muted-foreground">现代化的代码托管平台</p>
          </div>
        </Transition>

        <!-- 登录卡片 -->
        <Transition
          appear
          enter-active-class="transition-all duration-1000 ease-out delay-300"
          enter-from-class="transform translate-y-8 opacity-0 scale-95"
          enter-to-class="transform translate-y-0 opacity-100 scale-100"
        >
          <Card class="backdrop-blur-xl bg-card/80 border-border/50 shadow-2xl hover:shadow-3xl transition-all duration-500 hover:scale-[1.02]">
            <CardHeader class="text-center pb-2">
              <CardTitle class="text-2xl font-bold text-foreground">欢迎回来</CardTitle>
              <CardDescription class="text-muted-foreground mt-2">
                使用您的 AtomGit 账号登录以继续
              </CardDescription>
            </CardHeader>

            <CardContent class="pt-6">
              <!-- 登录按钮 -->
              <Button
                @click="startOAuth"
                :disabled="loading"
                class="w-full h-12 text-base font-medium relative overflow-hidden group transition-all duration-300 hover:scale-[1.02] active:scale-[0.98]"
                :class="loading ? 'animate-pulse' : ''"
              >
                <!-- 按钮背景动画 -->
                <div class="absolute inset-0 bg-gradient-to-r from-primary via-blue-600 to-primary bg-size-200 animate-gradient-x opacity-90 group-hover:opacity-100 transition-opacity"></div>

                <!-- 按钮内容 -->
                <div class="relative z-10 flex items-center justify-center">
                  <Transition
                    enter-active-class="transition-all duration-300"
                    enter-from-class="transform rotate-180 opacity-0"
                    enter-to-class="transform rotate-0 opacity-100"
                    leave-active-class="transition-all duration-300"
                    leave-from-class="transform rotate-0 opacity-100"
                    leave-to-class="transform rotate-180 opacity-0"
                  >
                    <Loader2 v-if="loading" class="mr-3 h-5 w-5 animate-spin" />
                  </Transition>

                  <span class="transition-all duration-300">
                    {{ loading ? '正在登录...' : '使用 AtomGit 登录' }}
                  </span>
                </div>

                <!-- 成功动画效果 -->
                <div v-if="loading" class="absolute inset-0 bg-green-500/20 animate-ping rounded-lg"></div>
              </Button>

              <!-- 分割线 -->
              <div class="relative my-6">
                <div class="absolute inset-0 flex items-center">
                  <div class="w-full border-t border-border/50"></div>
                </div>
                <div class="relative flex justify-center text-xs uppercase">
                  <span class="bg-card px-2 text-muted-foreground">安全登录</span>
                </div>
              </div>

              <!-- 安全提示 -->
              <div class="text-center space-y-2">
                <div class="flex items-center justify-center space-x-2 text-sm text-muted-foreground">
                  <svg class="w-4 h-4 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"/>
                  </svg>
                  <span>OAuth 2.0 安全认证</span>
                </div>
                <p class="text-xs text-muted-foreground">
                  我们使用业界标准的 OAuth 2.0 协议保护您的账号安全
                </p>
              </div>
            </CardContent>
          </Card>
        </Transition>

        <!-- 底部信息 -->
        <Transition
          appear
          enter-active-class="transition-all duration-1000 ease-out delay-600"
          enter-from-class="transform translate-y-4 opacity-0"
          enter-to-class="transform translate-y-0 opacity-100"
        >
          <div class="text-center mt-8 space-y-2">
            <p class="text-xs text-muted-foreground">
              登录即表示您同意我们的
              <a href="#" class="text-primary hover:underline transition-colors">服务条款</a>
              和
              <a href="#" class="text-primary hover:underline transition-colors">隐私政策</a>
            </p>
            <p class="text-xs text-muted-foreground">
              © 2024 AtomGit. 保留所有权利。
            </p>
          </div>
        </Transition>
      </div>
    </div>

    <!-- 成功登录的全屏动画 -->
    <Transition
      enter-active-class="transition-all duration-1000 ease-out"
      enter-from-class="transform scale-0 opacity-0"
      enter-to-class="transform scale-100 opacity-100"
      leave-active-class="transition-all duration-500 ease-in"
      leave-from-class="transform scale-100 opacity-100"
      leave-to-class="transform scale-110 opacity-0"
    >
      <div v-if="loginSuccess" class="fixed inset-0 z-50 flex items-center justify-center bg-primary/90 backdrop-blur-sm">
        <div class="text-center text-primary-foreground">
          <div class="w-20 h-20 mx-auto mb-4 bg-green-500 rounded-full flex items-center justify-center animate-bounce">
            <svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
            </svg>
          </div>
          <h2 class="text-2xl font-bold mb-2">登录成功！</h2>
          <p class="text-primary-foreground/80">正在跳转到工作台...</p>
        </div>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
/* 自定义动画 */
@keyframes float {

  0%,
  100% {
    transform: translateY(0px) rotate(0deg);
  }

  50% {
    transform: translateY(-20px) rotate(180deg);
  }
}

@keyframes float-delayed {

  0%,
  100% {
    transform: translateY(0px) rotate(0deg);
  }

  50% {
    transform: translateY(-30px) rotate(-180deg);
  }
}

@keyframes bounce-gentle {

  0%,
  100% {
    transform: translateY(0);
  }

  50% {
    transform: translateY(-5px);
  }
}

@keyframes gradient-x {

  0%,
  100% {
    background-position: 0% 50%;
  }

  50% {
    background-position: 100% 50%;
  }
}

@keyframes pulse-slow {

  0%,
  100% {
    opacity: 0.3;
    transform: scale(1);
  }

  50% {
    opacity: 0.1;
    transform: scale(1.1);
  }
}

.animate-float {
  animation: float 6s ease-in-out infinite;
}

.animate-float-delayed {
  animation: float-delayed 8s ease-in-out infinite;
  animation-delay: 2s;
}

.animate-bounce-gentle {
  animation: bounce-gentle 2s ease-in-out infinite;
}

.animate-gradient-x {
  animation: gradient-x 3s ease infinite;
}

.animate-pulse-slow {
  animation: pulse-slow 4s ease-in-out infinite;
}

.bg-size-200 {
  background-size: 200% 200%;
}

.shadow-3xl {
  box-shadow: 0 35px 60px -12px rgba(0, 0, 0, 0.25);
}

/* 网格背景 */
.bg-grid-pattern {
  background-image:
    linear-gradient(rgba(255, 255, 255, 0.1) 1px, transparent 1px),
    linear-gradient(90deg, rgba(255, 255, 255, 0.1) 1px, transparent 1px);
  background-size: 50px 50px;
}

/* 深色模式适配 */
@media (prefers-color-scheme: dark) {
  .bg-grid-pattern {
    background-image:
      linear-gradient(rgba(255, 255, 255, 0.05) 1px, transparent 1px),
      linear-gradient(90deg, rgba(255, 255, 255, 0.05) 1px, transparent 1px);
  }
}

/* 响应式优化 */
@media (max-width: 640px) {

  .animate-float,
  .animate-float-delayed {
    animation-duration: 4s;
  }
}

/* 减少动画效果（用户偏好） */
@media (prefers-reduced-motion: reduce) {

  .animate-float,
  .animate-float-delayed,
  .animate-bounce-gentle,
  .animate-gradient-x,
  .animate-pulse-slow {
    animation: none;
  }
}
</style>
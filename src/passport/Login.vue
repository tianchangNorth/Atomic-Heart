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
      body: {
        client_id: oauthConfig.clientId,
        client_secret: oauthConfig.clientSecret,
        code: authCode.value,
      }
    })

    if (response.success && response.data.access_token) {
      saveToken(response.data.access_token)
      await userStore.fetchInfo()
      router.push('/')
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
  <div class="min-h-screen flex items-center justify-center bg-gray-50">
    <Card class="w-full max-w-md">
      <CardHeader>
        <CardTitle>登录</CardTitle>
        <CardDescription>使用AtomGit账号登录</CardDescription>
      </CardHeader>
      <CardContent>
        <Button 
          @click="startOAuth" 
          :disabled="loading" 
          class="w-full"
        >
          <Loader2 v-if="loading" class="mr-2 h-4 w-4 animate-spin" />
          {{ loading ? '登录中...' : '使用AtomGit登录' }}
        </Button>
      </CardContent>
    </Card>
  </div>
</template>
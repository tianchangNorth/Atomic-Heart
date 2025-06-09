<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Loader2 } from 'lucide-vue-next'
import { invoke } from '@tauri-apps/api/core'
import { listen, UnlistenFn } from '@tauri-apps/api/event'
import { $fetch } from '@/utils/fetch'
import { saveToken } from '@/utils/token'
import { useUserStore } from '@/stores'

import router from '@/router'

const isLoading = ref(false)
const authCode = ref('')
const state = ref('')
const unlisten = ref<UnlistenFn | null>(null)

// OAuth配置
const oauthConfig = {
  clientId: import.meta.env.VITE_APP_CLIENT_ID, // 您的客户端ID
  clientSecret: import.meta.env.VITE_APP_CLIENT_SECRET, // 您的客户端密钥
  redirectUri: 'http://localhost:1420/callback', // 使用 OAuth 插件的本地服务器地址
  state: 'state_test' // 用于防止CSRF攻击的状态参数
}

// 监听 OAuth 回调
const setupOAuthListener = async () => {
  console.log('设置OAuth监听器...');
  unlisten.value = await listen('oauth://callback', (event) => {
    console.log('收到OAuth回调事件:', event);
    const url = new URL(event.payload as string);
    const code = url.searchParams.get('code');
    const receivedState = url.searchParams.get('state');

    if (code) {
      authCode.value = code;
      state.value = receivedState || '';

      // 验证state参数，防止CSRF攻击
      if (receivedState !== oauthConfig.state) {
        console.error('state参数不匹配，可能存在CSRF攻击');
        return;
      }
      // 获取到授权码后，可以调用后端API交换访问令牌
      exchangeCodeForToken(code);
      // 重置加载状态
      isLoading.value = false;
    } else {
      console.error('未在回调URL中找到授权码');
      isLoading.value = false;
    }
  });
  console.log('OAuth监听器设置完成');
}

const handleLogin = async () => {
  isLoading.value = true;

  try {
    // 构建OAuth授权URL
    const authUrl = `https://atomgit.com/login/oauth/authorize?client_id=${oauthConfig.clientId}&state=${oauthConfig.state}&redirect_uri=${encodeURIComponent(oauthConfig.redirectUri)}`;

    // 调用 Rust 后端命令启动 OAuth 服务器并打开授权 URL
    const port = await invoke('start_oauth_server', { url: authUrl });
    console.log('已启动 OAuth 流程，服务器端口:', port);

    // 注意：授权码将通过事件监听器获取
    // 不要在这里重置 isLoading，而是在收到授权码后重置
  } catch (error) {
    console.error('授权失败', error);
    isLoading.value = false;
  }
}

// 使用授权码交换访问令牌
const exchangeCodeForToken = async (code: string) => {
  isLoading.value = true

  try {
    // 这里应该调用您的后端API或直接调用OAuth提供商的令牌端点
    const { success, data } = await $fetch('/login/oauth/access_token',
      {
        method: 'POST',
        body: {
          client_id: oauthConfig.clientId,
          client_secret: oauthConfig.clientSecret,
          code,
        }
      })
    // 模拟API调用
    if (success) {
      console.log('获取访问令牌成功', data.access_token);
      saveToken(data.access_token);

      // 获取用户信息并跳转
      const userStore = useUserStore();
      await userStore.fetchInfo();
      router.push('/'); // 跳转到首页
    }

  } catch (error) {
    console.error('获取访问令牌失败', error)
    isLoading.value = false
  }
}

// 组件挂载时设置 OAuth 监听器
onMounted(() => {
  setupOAuthListener()
})

// 组件卸载时移除监听器
onUnmounted(async () => {
  if (unlisten.value) {
    await unlisten.value()
  }
})
</script>

<template>
  <div class="flex min-h-screen w-full items-center justify-center bg-background">
    <Card class="w-[400px] shadow-lg">
      <CardHeader class="text-center">
        <CardTitle class="text-2xl font-bold">AtomGit</CardTitle>
        <CardDescription>欢迎使用AtomGit</CardDescription>
      </CardHeader>
      <CardContent class="flex flex-col items-center justify-center space-y-6 py-8">
        <img src="/tauri.svg" alt="Logo" class="h-24 w-24" />
        <p class="text-center text-muted-foreground">
          点击下方按钮使用AtomGit账号登录
        </p>
        <!-- 显示授权码和状态（仅用于演示） -->
        <div v-if="authCode" class="w-full text-sm">
          <div class="rounded bg-gray-100 p-2 mb-2">
            <p class="font-semibold">授权码:</p>
            <p class="break-all">{{ authCode }}</p>
          </div>
          <div class="rounded bg-gray-100 p-2">
            <p class="font-semibold">状态:</p>
            <p>{{ state }}</p>
          </div>
        </div>
      </CardContent>
      <CardFooter class="flex flex-col space-y-4">
        <Button class="w-full" @click="handleLogin" :disabled="isLoading">
          <Loader2 v-if="isLoading" class="mr-2 h-4 w-4 animate-spin" />
          <span v-else>登录 AtomGit</span>
        </Button>
      </CardFooter>
      <div class="px-8 pb-8 text-center text-sm text-muted-foreground">
        <p>登录即表示您同意 AtomGit 的
          <a href="#" class="underline underline-offset-4 hover:text-primary">服务条款</a>
          和
          <a href="#" class="underline underline-offset-4 hover:text-primary">隐私政策</a>
        </p>
      </div>
    </Card>
  </div>
</template>
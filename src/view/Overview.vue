<template>
  <div class="space-y-6">
    <!-- 欢迎标题 -->
    <div class="mb-8">
      <h1 class="text-3xl font-bold text-foreground mb-2">欢迎回来，{{ user.name || '用户' }}！</h1>
      <p class="text-muted-foreground">这是您的 AtomGit 工作台概览</p>
    </div>

    <!-- 统计卡片 -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
      <Card>
        <CardContent class="p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-muted-foreground">总仓库数</p>
              <p class="text-2xl font-bold text-foreground">{{ user.total_repos || 0 }}</p>
            </div>
            <div class="w-12 h-12 bg-primary/10 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"/>
              </svg>
            </div>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardContent class="p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-muted-foreground">关注者</p>
              <p class="text-2xl font-bold text-foreground">{{ user.followers || 0 }}</p>
            </div>
            <div class="w-12 h-12 bg-green-500/10 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/>
              </svg>
            </div>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardContent class="p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-muted-foreground">正在关注</p>
              <p class="text-2xl font-bold text-foreground">{{ user.following || 0 }}</p>
            </div>
            <div class="w-12 h-12 bg-blue-500/10 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-blue-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/>
              </svg>
            </div>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardContent class="p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-muted-foreground">活跃 Issues</p>
              <p class="text-2xl font-bold text-foreground">12</p>
            </div>
            <div class="w-12 h-12 bg-orange-500/10 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-orange-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.34 16.5c-.77.833.192 2.5 1.732 2.5z"/>
              </svg>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>

    <!-- 最近活动和快速操作 -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- 最近活动 -->
      <Card>
        <CardHeader>
          <CardTitle>最近活动</CardTitle>
          <CardDescription>您最近的代码活动记录</CardDescription>
        </CardHeader>
        <CardContent>
          <div class="space-y-4">
            <div class="flex items-start space-x-3">
              <div class="w-2 h-2 bg-green-500 rounded-full mt-2"></div>
              <div class="flex-1">
                <p class="text-sm font-medium text-foreground">完成了 Issue #123</p>
                <p class="text-xs text-muted-foreground">修复登录问题 • 2小时前</p>
              </div>
            </div>
            <div class="flex items-start space-x-3">
              <div class="w-2 h-2 bg-blue-500 rounded-full mt-2"></div>
              <div class="flex-1">
                <p class="text-sm font-medium text-foreground">创建了新分支</p>
                <p class="text-xs text-muted-foreground">feature/user-profile • 4小时前</p>
              </div>
            </div>
            <div class="flex items-start space-x-3">
              <div class="w-2 h-2 bg-purple-500 rounded-full mt-2"></div>
              <div class="flex-1">
                <p class="text-sm font-medium text-foreground">推送了 3 个提交</p>
                <p class="text-xs text-muted-foreground">到 main 分支 • 1天前</p>
              </div>
            </div>
            <div class="flex items-start space-x-3">
              <div class="w-2 h-2 bg-yellow-500 rounded-full mt-2"></div>
              <div class="flex-1">
                <p class="text-sm font-medium text-foreground">更新了项目文档</p>
                <p class="text-xs text-muted-foreground">README.md • 2天前</p>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      <!-- 快速操作 -->
      <Card>
        <CardHeader>
          <CardTitle>快速操作</CardTitle>
          <CardDescription>常用功能的快速入口</CardDescription>
        </CardHeader>
        <CardContent>
          <div class="grid grid-cols-2 gap-4">
            <Button variant="outline" class="h-20 flex-col space-y-2">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
              </svg>
              <span class="text-sm">新建仓库</span>
            </Button>
            <Button variant="outline" class="h-20 flex-col space-y-2">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10"/>
              </svg>
              <span class="text-sm">导入仓库</span>
            </Button>
            <Button variant="outline" class="h-20 flex-col space-y-2">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/>
              </svg>
              <span class="text-sm">加入组织</span>
            </Button>
            <Button variant="outline" class="h-20 flex-col space-y-2">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
              </svg>
              <span class="text-sm">设置</span>
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useUserStore } from '@/stores/index';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';

const userStore = useUserStore();
const { user } = userStore;
</script>

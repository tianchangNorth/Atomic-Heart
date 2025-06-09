<template>
  <div class="space-y-6">
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <div class="bg-gradient-to-r from-blue-500 to-blue-600 rounded-lg p-6 text-white">
        <h3 class="text-lg font-semibold mb-2">活跃Issues</h3>
        <p class="text-3xl font-bold">12</p>
        <p class="text-blue-100 text-sm">本周新增 3 个</p>
      </div>
      <div class="bg-gradient-to-r from-green-500 to-green-600 rounded-lg p-6 text-white">
        <h3 class="text-lg font-semibold mb-2">仓库数量</h3>
        <p class="text-3xl font-bold">8</p>
        <p class="text-green-100 text-sm">2 个私有仓库</p>
      </div>
      <div class="bg-gradient-to-r from-purple-500 to-purple-600 rounded-lg p-6 text-white">
        <h3 class="text-lg font-semibold mb-2">进行中项目</h3>
        <p class="text-3xl font-bold">5</p>
        <p class="text-purple-100 text-sm">1 个即将截止</p>
      </div>
    </div>
    
    <div class="bg-gray-50 rounded-lg p-6">
      <h3 class="text-lg font-semibold mb-4">最近活动</h3>
      <div class="space-y-3">
        <div class="flex items-center space-x-3">
          <div class="w-2 h-2 bg-green-500 rounded-full"></div>
          <span class="text-sm text-gray-700">完成了 Issue #123: 修复登录问题</span>
          <span class="text-xs text-gray-500">2小时前</span>
        </div>
        <div class="flex items-center space-x-3">
          <div class="w-2 h-2 bg-blue-500 rounded-full"></div>
          <span class="text-sm text-gray-700">创建了新分支 feature/user-profile</span>
          <span class="text-xs text-gray-500">4小时前</span>
        </div>
        <div class="flex items-center space-x-3">
          <div class="w-2 h-2 bg-yellow-500 rounded-full"></div>
          <span class="text-sm text-gray-700">更新了项目文档</span>
          <span class="text-xs text-gray-500">1天前</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// 概览页面逻辑
</script>
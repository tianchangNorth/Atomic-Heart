<template>
  <div class="space-y-4">
    <div class="flex justify-between items-center">
      <h3 class="text-lg font-semibold">我的仓库</h3>
      <button class="bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700 transition-colors">
        新建仓库
      </button>
    </div>
    
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div class="border rounded-lg p-4 hover:shadow-md transition-shadow">
        <div class="flex items-center justify-between mb-3">
          <h4 class="font-semibold text-blue-600">tauri-app</h4>
          <span class="bg-gray-100 text-gray-800 text-xs px-2 py-1 rounded">Public</span>
        </div>
        <p class="text-sm text-gray-600 mb-3">基于Tauri和Vue3的桌面应用程序</p>
        <div class="flex items-center space-x-4 text-xs text-gray-500">
          <span class="flex items-center space-x-1">
            <div class="w-3 h-3 bg-green-500 rounded-full"></div>
            <span>Vue</span>
          </span>
          <span>⭐ 15</span>
          <span>🍴 3</span>
          <span>更新于 2小时前</span>
        </div>
      </div>
      
      <div class="border rounded-lg p-4 hover:shadow-md transition-shadow">
        <div class="flex items-center justify-between mb-3">
          <h4 class="font-semibold text-blue-600">api-server</h4>
          <span class="bg-yellow-100 text-yellow-800 text-xs px-2 py-1 rounded">Private</span>
        </div>
        <p class="text-sm text-gray-600 mb-3">后端API服务器，提供用户认证和数据管理</p>
        <div class="flex items-center space-x-4 text-xs text-gray-500">
          <span class="flex items-center space-x-1">
            <div class="w-3 h-3 bg-blue-500 rounded-full"></div>
            <span>TypeScript</span>
          </span>
          <span>⭐ 8</span>
          <span>🍴 1</span>
          <span>更新于 1天前</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// 仓库页面逻辑
</script>
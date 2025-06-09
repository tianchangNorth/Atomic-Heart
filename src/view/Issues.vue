<template>
  <div class="space-y-4">
    <div class="flex justify-between items-center">
      <h3 class="text-lg font-semibold">我的Issues</h3>
      <button class="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 transition-colors">
        新建Issue
      </button>
    </div>
    
    <div class="space-y-3">
      <div class="border rounded-lg p-4 hover:bg-gray-50 transition-colors">
        <div class="flex items-center justify-between">
          <div class="flex items-center space-x-3">
            <span class="bg-red-100 text-red-800 text-xs px-2 py-1 rounded-full">Bug</span>
            <h4 class="font-medium">登录页面样式错乱</h4>
          </div>
          <span class="text-sm text-gray-500">#001</span>
        </div>
        <p class="text-sm text-gray-600 mt-2">在移动端设备上登录页面布局出现问题</p>
        <div class="flex items-center space-x-4 mt-3 text-xs text-gray-500">
          <span>创建于 2024-01-15</span>
          <span>分配给: 张三</span>
          <span class="bg-yellow-100 text-yellow-800 px-2 py-1 rounded">进行中</span>
        </div>
      </div>
      
      <div class="border rounded-lg p-4 hover:bg-gray-50 transition-colors">
        <div class="flex items-center justify-between">
          <div class="flex items-center space-x-3">
            <span class="bg-blue-100 text-blue-800 text-xs px-2 py-1 rounded-full">Feature</span>
            <h4 class="font-medium">添加用户头像上传功能</h4>
          </div>
          <span class="text-sm text-gray-500">#002</span>
        </div>
        <p class="text-sm text-gray-600 mt-2">用户需要能够上传和更换个人头像</p>
        <div class="flex items-center space-x-4 mt-3 text-xs text-gray-500">
          <span>创建于 2024-01-14</span>
          <span>分配给: 李四</span>
          <span class="bg-green-100 text-green-800 px-2 py-1 rounded">待开始</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// Issues页面逻辑
</script>
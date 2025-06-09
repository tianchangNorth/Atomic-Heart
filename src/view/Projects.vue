<template>
  <div class="space-y-4">
    <div class="flex justify-between items-center">
      <h3 class="text-lg font-semibold">我的项目</h3>
      <button class="bg-purple-600 text-white px-4 py-2 rounded-md hover:bg-purple-700 transition-colors">
        新建项目
      </button>
    </div>
    
    <div class="space-y-4">
      <div class="border rounded-lg p-6">
        <div class="flex items-center justify-between mb-4">
          <h4 class="text-lg font-semibold">开发者工作台 v2.0</h4>
          <span class="bg-green-100 text-green-800 text-sm px-3 py-1 rounded-full">进行中</span>
        </div>
        <p class="text-gray-600 mb-4">升级现有工作台，添加更多功能和改进用户体验</p>
        <div class="mb-4">
          <div class="flex justify-between text-sm mb-2">
            <span>进度</span>
            <span>65%</span>
          </div>
          <div class="w-full bg-gray-200 rounded-full h-2">
            <div class="bg-blue-600 h-2 rounded-full" style="width: 65%"></div>
          </div>
        </div>
        <div class="flex items-center space-x-6 text-sm text-gray-500">
          <span>截止日期: 2024-02-15</span>
          <span>团队成员: 5人</span>
          <span>剩余任务: 8个</span>
        </div>
      </div>
      
      <div class="border rounded-lg p-6">
        <div class="flex items-center justify-between mb-4">
          <h4 class="text-lg font-semibold">移动端适配</h4>
          <span class="bg-yellow-100 text-yellow-800 text-sm px-3 py-1 rounded-full">计划中</span>
        </div>
        <p class="text-gray-600 mb-4">为现有应用添加移动端支持，优化移动设备体验</p>
        <div class="mb-4">
          <div class="flex justify-between text-sm mb-2">
            <span>进度</span>
            <span>15%</span>
          </div>
          <div class="w-full bg-gray-200 rounded-full h-2">
            <div class="bg-yellow-500 h-2 rounded-full" style="width: 15%"></div>
          </div>
        </div>
        <div class="flex items-center space-x-6 text-sm text-gray-500">
          <span>预计开始: 2024-02-01</span>
          <span>团队成员: 3人</span>
          <span>预估工期: 6周</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// 项目页面逻辑
</script>
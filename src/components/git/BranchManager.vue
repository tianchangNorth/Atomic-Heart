<script setup lang="ts">
import { ref, computed, reactive } from 'vue';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import type { BranchInfo } from '@/types/git';

// 模拟分支数据
const branches = ref<BranchInfo[]>([
  {
    name: 'main',
    type: 'local',
    current: true,
    upstream: 'origin/main',
    lastCommit: {
      sha: 'abc123def456',
      message: 'feat: 添加分支管理界面',
      author: { name: 'Developer', email: 'dev@example.com', date: '2024-12-15T14:20:00Z' },
      committer: { name: 'Developer', email: 'dev@example.com', date: '2024-12-15T14:20:00Z' },
      url: ''
    },
    ahead: 2,
    behind: 0
  },
  {
    name: 'develop',
    type: 'local',
    current: false,
    upstream: 'origin/develop',
    lastCommit: {
      sha: 'def456ghi789',
      message: 'fix: 修复文件树展开问题',
      author: { name: 'Developer', email: 'dev@example.com', date: '2024-12-14T16:45:00Z' },
      committer: { name: 'Developer', email: 'dev@example.com', date: '2024-12-14T16:45:00Z' },
      url: ''
    },
    ahead: 0,
    behind: 1
  },
  {
    name: 'feature/new-ui',
    type: 'local',
    current: false,
    lastCommit: {
      sha: 'ghi789jkl012',
      message: 'wip: 新UI设计进行中',
      author: { name: 'Designer', email: 'design@example.com', date: '2024-12-13T09:15:00Z' },
      committer: { name: 'Designer', email: 'design@example.com', date: '2024-12-13T09:15:00Z' },
      url: ''
    },
    ahead: 5,
    behind: 3
  },
  {
    name: 'origin/main',
    type: 'remote',
    current: false,
    lastCommit: {
      sha: 'jkl012mno345',
      message: 'docs: 更新README文档',
      author: { name: 'Maintainer', email: 'maintainer@example.com', date: '2024-12-15T12:00:00Z' },
      committer: { name: 'Maintainer', email: 'maintainer@example.com', date: '2024-12-15T12:00:00Z' },
      url: ''
    },
    ahead: 0,
    behind: 0
  },
  {
    name: 'origin/develop',
    type: 'remote',
    current: false,
    lastCommit: {
      sha: 'mno345pqr678',
      message: 'test: 添加单元测试',
      author: { name: 'Tester', email: 'test@example.com', date: '2024-12-14T18:30:00Z' },
      committer: { name: 'Tester', email: 'test@example.com', date: '2024-12-14T18:30:00Z' },
      url: ''
    },
    ahead: 0,
    behind: 0
  }
]);

const newBranchForm = reactive({
  name: '',
  baseBranch: 'main',
  checkout: true
});

const searchQuery = ref('');
const selectedBranch = ref<string | null>(null);
const isCreating = ref(false);
const showCreateForm = ref(false);

// 计算属性
const localBranches = computed(() =>
  branches.value.filter(b => b.type === 'local')
);

const remoteBranches = computed(() =>
  branches.value.filter(b => b.type === 'remote')
);

const filteredLocalBranches = computed(() => {
  if (!searchQuery.value) return localBranches.value;
  return localBranches.value.filter(b =>
    b.name.toLowerCase().includes(searchQuery.value.toLowerCase())
  );
});

const filteredRemoteBranches = computed(() => {
  if (!searchQuery.value) return remoteBranches.value;
  return remoteBranches.value.filter(b =>
    b.name.toLowerCase().includes(searchQuery.value.toLowerCase())
  );
});

const currentBranch = computed(() =>
  branches.value.find(b => b.current)
);

const canCreateBranch = computed(() =>
  newBranchForm.name.trim() && !isCreating.value
);

// 方法
const formatDate = (dateString: string): string => {
  const date = new Date(dateString);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMins = Math.floor(diffMs / (1000 * 60));

  if (diffMins < 1) return '刚刚';
  if (diffMins < 60) return `${diffMins} 分钟前`;
  if (diffMins < 1440) return `${Math.floor(diffMins / 60)} 小时前`;
  return `${Math.floor(diffMins / 1440)} 天前`;
};

const getBranchStatusColor = (branch: BranchInfo): string => {
  if (branch.current) return 'text-green-600';
  if (branch.ahead > 0 && branch.behind > 0) return 'text-yellow-600';
  if (branch.ahead > 0) return 'text-blue-600';
  if (branch.behind > 0) return 'text-orange-600';
  return 'text-muted-foreground';
};

const getBranchStatusText = (branch: BranchInfo): string => {
  if (branch.current) return '当前分支';
  if (branch.ahead > 0 && branch.behind > 0) return `领先 ${branch.ahead}，落后 ${branch.behind}`;
  if (branch.ahead > 0) return `领先 ${branch.ahead} 个提交`;
  if (branch.behind > 0) return `落后 ${branch.behind} 个提交`;
  return '已同步';
};

const switchBranch = async (branchName: string) => {
  if (branchName === currentBranch.value?.name) return;

  // 模拟切换分支
  const targetBranch = branches.value.find(b => b.name === branchName && b.type === 'local');
  if (!targetBranch) return;

  // 更新当前分支状态
  branches.value.forEach(b => {
    b.current = b.name === branchName && b.type === 'local';
  });

  selectedBranch.value = null;
};

const createBranch = async () => {
  if (!canCreateBranch.value) return;

  isCreating.value = true;

  // 模拟创建分支
  await new Promise(resolve => setTimeout(resolve, 1500));

  const baseBranch = branches.value.find(b => b.name === newBranchForm.baseBranch && b.type === 'local');
  const newBranch: BranchInfo = {
    name: newBranchForm.name,
    type: 'local',
    current: newBranchForm.checkout,
    lastCommit: baseBranch?.lastCommit || {
      sha: 'new123branch456',
      message: '创建新分支',
      author: { name: 'Developer', email: 'dev@example.com', date: new Date().toISOString() },
      committer: { name: 'Developer', email: 'dev@example.com', date: new Date().toISOString() },
      url: ''
    },
    ahead: 0,
    behind: 0
  };

  branches.value.push(newBranch);

  if (newBranchForm.checkout) {
    branches.value.forEach(b => {
      b.current = b.name === newBranch.name && b.type === 'local';
    });
  }

  // 重置表单
  newBranchForm.name = '';
  newBranchForm.baseBranch = 'main';
  newBranchForm.checkout = true;
  showCreateForm.value = false;
  isCreating.value = false;
};

const deleteBranch = async (branchName: string) => {
  if (branchName === currentBranch.value?.name) return;

  const index = branches.value.findIndex(b => b.name === branchName && b.type === 'local');
  if (index > -1) {
    branches.value.splice(index, 1);
  }

  selectedBranch.value = null;
};

const mergeBranch = async (branchName: string) => {
  // 模拟合并分支
  console.log(`合并分支: ${branchName} -> ${currentBranch.value?.name}`);
  selectedBranch.value = null;
};

const selectBranch = (branchName: string) => {
  selectedBranch.value = selectedBranch.value === branchName ? null : branchName;
};
</script>

<template>
  <div class="space-y-6">
    <!-- 顶部操作栏 -->
    <Card>
      <CardHeader>
        <CardTitle class="flex items-center justify-between">
          <div class="flex items-center space-x-2">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16l2.879-2.879m0 0a3 3 0 104.243-4.242 3 3 0 00-4.243 4.242zM21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
            </svg>
            <span>分支管理</span>
            <Badge v-if="currentBranch" variant="default" class="ml-2">
              {{ currentBranch.name }}
            </Badge>
          </div>
          <Button @click="showCreateForm = !showCreateForm">
            <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"/>
            </svg>
            新建分支
          </Button>
        </CardTitle>
      </CardHeader>
      <CardContent>
        <!-- 搜索框 -->
        <div class="flex space-x-4">
          <div class="flex-1">
            <Input
              v-model="searchQuery"
              placeholder="搜索分支..."
              class="w-full"
            >
              <template #prefix>
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                </svg>
              </template>
            </Input>
          </div>
        </div>

        <!-- 创建分支表单 -->
        <div v-if="showCreateForm" class="mt-4 p-4 border rounded-lg bg-muted/50">
          <h3 class="font-medium mb-3">创建新分支</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="space-y-2">
              <label class="text-sm font-medium">分支名称</label>
              <Input
                v-model="newBranchForm.name"
                placeholder="feature/new-feature"
              />
            </div>
            <div class="space-y-2">
              <label class="text-sm font-medium">基于分支</label>
              <select 
                v-model="newBranchForm.baseBranch"
                class="w-full px-3 py-2 border border-border rounded-md bg-background"
              >
                <option v-for="branch in localBranches" :key="branch.name" :value="branch.name">
                  {{ branch.name }}
                </option>
              </select>
            </div>
          </div>
          <div class="flex items-center justify-between mt-4">
            <label class="flex items-center space-x-2">
              <input v-model="newBranchForm.checkout" type="checkbox" class="rounded border-border">
              <span class="text-sm">创建后切换到新分支</span>
            </label>
            <div class="space-x-2">
              <Button variant="outline" @click="showCreateForm = false">
                取消
              </Button>
              <Button @click="createBranch" :disabled="!canCreateBranch">
                <svg v-if="isCreating" class="w-4 h-4 mr-2 animate-spin" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
                </svg>
                {{ isCreating ? '创建中...' : '创建分支' }}
              </Button>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- 本地分支 -->
      <Card>
        <CardHeader>
          <CardTitle class="flex items-center space-x-2">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
            </svg>
            <span>本地分支</span>
            <Badge variant="secondary">{{ filteredLocalBranches.length }}</Badge>
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div class="space-y-2">
            <div
              v-for="branch in filteredLocalBranches"
              :key="branch.name"
              class="p-3 rounded-lg border hover:bg-accent cursor-pointer transition-colors"
              :class="{ 'bg-accent': selectedBranch === branch.name, 'border-primary': branch.current }"
              @click="selectBranch(branch.name)"
            >
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-3 flex-1 min-w-0">
                  <div class="flex items-center space-x-2">
                    <svg 
                      class="w-4 h-4" 
                      :class="getBranchStatusColor(branch)"
                      fill="none" 
                      stroke="currentColor" 
                      viewBox="0 0 24 24"
                    >
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16l2.879-2.879m0 0a3 3 0 104.243-4.242 3 3 0 00-4.243 4.242zM21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                    </svg>
                    <span class="font-medium">{{ branch.name }}</span>
                    <Badge v-if="branch.current" variant="default" class="text-xs">
                      当前
                    </Badge>
                  </div>
                </div>
                <div class="text-right">
                  <div class="text-xs text-muted-foreground">
                    {{ formatDate(branch.lastCommit.date!) }}
                  </div>
                  <div class="text-xs" :class="getBranchStatusColor(branch)">
                    {{ getBranchStatusText(branch) }}
                  </div>
                </div>
              </div>
              
              <div class="mt-2 text-sm text-muted-foreground truncate">
                {{ branch.lastCommit.message }}
              </div>
              
              <!-- 分支操作按钮 -->
              <div v-if="selectedBranch === branch.name" class="mt-3 flex space-x-2">
                <Button 
                  v-if="!branch.current"
                  size="sm" 
                  @click.stop="switchBranch(branch.name)"
                >
                  切换
                </Button>
                <Button 
                  v-if="!branch.current && currentBranch"
                  variant="outline" 
                  size="sm" 
                  @click.stop="mergeBranch(branch.name)"
                >
                  合并到 {{ currentBranch.name }}
                </Button>
                <Button 
                  v-if="!branch.current"
                  variant="destructive" 
                  size="sm" 
                  @click.stop="deleteBranch(branch.name)"
                >
                  删除
                </Button>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      <!-- 远程分支 -->
      <Card>
        <CardHeader>
          <CardTitle class="flex items-center space-x-2">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9v-9m0-9v9"/>
            </svg>
            <span>远程分支</span>
            <Badge variant="outline">{{ filteredRemoteBranches.length }}</Badge>
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div class="space-y-2">
            <div
              v-for="branch in filteredRemoteBranches"
              :key="branch.name"
              class="p-3 rounded-lg border hover:bg-accent cursor-pointer transition-colors"
              :class="{ 'bg-accent': selectedBranch === branch.name }"
              @click="selectBranch(branch.name)"
            >
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-3 flex-1 min-w-0">
                  <div class="flex items-center space-x-2">
                    <svg class="w-4 h-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9v-9m0-9v9"/>
                    </svg>
                    <span class="font-medium">{{ branch.name }}</span>
                  </div>
                </div>
                <div class="text-right">
                  <div class="text-xs text-muted-foreground">
                    {{ formatDate(branch.lastCommit.date!) }}
                  </div>
                </div>
              </div>
              
              <div class="mt-2 text-sm text-muted-foreground truncate">
                {{ branch.lastCommit.message }}
              </div>
              
              <!-- 远程分支操作按钮 -->
              <div v-if="selectedBranch === branch.name" class="mt-3 flex space-x-2">
                <Button size="sm" @click.stop="console.log('检出远程分支')">
                  检出
                </Button>
                <Button variant="outline" size="sm" @click.stop="console.log('拉取远程分支')">
                  拉取
                </Button>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Input } from '@/components/ui/input'
import { useToast } from '@/components/ui/toast'
import {
  GitBranch,
  Plus,
  Trash2,
  RefreshCw,
  Clock,
  ArrowUp,
  ArrowDown,
  GitCommit,
  Upload,
} from 'lucide-vue-next'
import { GitOperationsApi } from '@/api/git-operations'

// 分支信息接口
interface BranchInfo {
  name: string
  is_current: boolean
  is_remote: boolean
  upstream?: string
  ahead: number
  behind: number
  last_commit?: {
    sha: string
    message: string
    author_name: string
    author_email: string
    timestamp: number
  }
}

// Props
interface Props {
  repoPath: string
}

const props = defineProps<Props>()

// Toast通知
const { success, error, warning } = useToast()

// 响应式数据
const branches = ref<BranchInfo[]>([])
const loading = ref(false)
const gitApi = new GitOperationsApi()

// 创建分支相关状态
const showCreateForm = ref(false)
const newBranchName = ref('')
const selectedBaseBranch = ref('')
const checkoutAfterCreate = ref(true)
const pushToRemote = ref(false)
const creating = ref(false)
const pushing = ref(false)

// 计算属性
const localBranches = computed(() =>
  branches.value.filter(branch => !branch.is_remote)
)

const remoteBranches = computed(() =>
  branches.value.filter(branch => branch.is_remote)
)

const currentBranch = computed(() =>
  branches.value.find(branch => branch.is_current)
)

// 可选择的基础分支列表（本地分支）
const availableBaseBranches = computed(() =>
  localBranches.value.map(branch => ({
    value: branch.name,
    label: branch.name,
    isCurrent: branch.is_current
  }))
)

// 获取分支列表
const fetchBranches = async () => {
  if (!props.repoPath) return

  loading.value = true
  try {
    const result = await invoke<BranchInfo[]>('list_branches', {
      repoPath: props.repoPath
    })

    branches.value = result
    initializeBaseBranch()
  } catch (e) {
    error('获取分支失败')
  } finally {
    loading.value = false
  }
}

// 刷新分支列表
const refreshBranches = async () => {
  await fetchBranches()
  success('分支已刷新')
}

// 格式化时间
const formatTime = (timestamp: number) => {
  const date = new Date(timestamp * 1000)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffMins = Math.floor(diffMs / (1000 * 60))

  if (diffMins < 1) return '刚刚'
  if (diffMins < 60) return `${diffMins} 分钟前`
  if (diffMins < 1440) return `${Math.floor(diffMins / 60)} 小时前`
  return `${Math.floor(diffMins / 1440)} 天前`
}

// 格式化提交消息
const formatCommitMessage = (message: string) => {
  const firstLine = message.split('\n')[0]
  return firstLine.length > 50 ? firstLine.substring(0, 50) + '...' : firstLine
}

// 获取分支状态徽章
const getBranchStatusBadge = (branch: BranchInfo) => {
  if (branch.is_current) {
    return { text: '当前', variant: 'default' as const }
  }
  if (branch.ahead > 0 && branch.behind > 0) {
    return { text: '分歧', variant: 'destructive' as const }
  }
  if (branch.ahead > 0) {
    return { text: '领先', variant: 'secondary' as const }
  }
  if (branch.behind > 0) {
    return { text: '落后', variant: 'outline' as const }
  }
  // 检查是否需要推送（本地分支但没有远程跟踪）
  if (needsPush(branch)) {
    return { text: '未推送', variant: 'outline' as const }
  }
  return null
}

// 创建分支
const createBranch = async () => {
  if (!newBranchName.value.trim() || creating.value) return

  creating.value = true
  const branchName = newBranchName.value.trim()

  try {
    // 第一步：创建分支
    const createResult = await gitApi.createBranch(
      props.repoPath,
      branchName,
      selectedBaseBranch.value || undefined,
      checkoutAfterCreate.value
    )

    if (!createResult.success) {
      error(`分支创建失败: ${createResult.message}`)
      return
    }

    success(`分支 '${branchName}' 创建成功`)

    // 第二步：如果需要推送到远程
    if (pushToRemote.value) {
      pushing.value = true
      try {
        const pushResult = await gitApi.smartPushRemote(props.repoPath)

        if (pushResult.success) {
          success(`分支 '${branchName}' 推送成功`)
        } else {
          warning(`分支推送失败: ${pushResult.message}`)
          // 推送失败不影响分支创建成功的状态
        }
      } catch (pushError) {
        warning(`分支推送失败: ${pushError}`)
        // 推送失败不影响分支创建成功的状态
      } finally {
        pushing.value = false
      }
    }

    // 第三步：重新获取分支列表和重置表单
    await fetchBranches()
    resetCreateForm()

  } catch (createError) {
    error(`创建分支失败: ${createError}`)
  } finally {
    creating.value = false
  }
}

// 切换分支
const switchToBranch = async (branchName: string) => {
  if (loading.value) return

  loading.value = true
  try {
    const result = await gitApi.switchBranch(props.repoPath, branchName)

    if (result.success) {
      success('分支切换成功')
      await fetchBranches()
    } else {
      error('分支切换失败')
      if (result.has_uncommitted_changes) {
        console.log('未提交的文件:', result.uncommitted_files)
      }
    }
  } catch (e) {
    error(`分支切换发生错误${e}`)
  } finally {
    loading.value = false
  }
}

// 删除分支
const deleteBranch = async (branchName: string, force = false) => {
  if (loading.value) return

  loading.value = true
  try {
    const result = await gitApi.deleteBranch(props.repoPath, branchName, force)

    if (result.success) {
      success(`分支 '${branchName}' 删除成功`)
      // 重新获取分支列表
      await fetchBranches()
    } else {
      error(`分支删除失败: ${result.message}`)
    }
  } catch (deleteError) {
    error(`删除分支失败: ${deleteError}`)
  } finally {
    loading.value = false
  }
}

// 推送单个分支到远程
const pushBranchToRemote = async (branchName: string) => {
  if (loading.value) return

  loading.value = true
  try {
    // 如果不是当前分支，需要先切换
    const needSwitch = currentBranch.value?.name !== branchName
    if (needSwitch) {
      const switchResult = await gitApi.switchBranch(props.repoPath, branchName)
      if (!switchResult.success) {
        if (switchResult.has_uncommitted_changes) {
          warning(`无法切换分支: 存在未提交的变更`)
          return
        } else {
          error(`切换分支失败: ${switchResult.message}`)
          return
        }
      }
    }

    // 推送分支
    const pushResult = await gitApi.smartPushRemote(props.repoPath)

    if (pushResult.success) {
      success(`分支 '${branchName}' 推送成功`)
      // 重新获取分支列表以更新状态
      await fetchBranches()
    } else {
      error(`分支推送失败: ${pushResult.message}`)
    }
  } catch (pushError) {
    error(`推送分支失败: ${pushError}`)
  } finally {
    loading.value = false
  }
}

// 重置创建表单
const resetCreateForm = () => {
  newBranchName.value = ''
  selectedBaseBranch.value = currentBranch.value?.name || ''
  checkoutAfterCreate.value = true
  pushToRemote.value = false
  showCreateForm.value = false
}

// 初始化基础分支选择
const initializeBaseBranch = () => {
  if (!selectedBaseBranch.value && currentBranch.value) {
    selectedBaseBranch.value = currentBranch.value.name
  }
}

// 获取创建按钮文本
const getCreateButtonText = () => {
  if (creating.value && !pushing.value) {
    return '创建中...'
  }
  if (pushing.value) {
    return '推送中...'
  }
  return '创建分支'
}

// 检查分支是否需要推送（没有远程跟踪分支）
const needsPush = (branch: BranchInfo) => {
  // 如果是本地分支且没有对应的远程分支，则需要推送
  if (!branch.is_remote) {
    const remoteBranchName = `origin/${branch.name}`
    return !remoteBranches.value.some(remote => remote.name === remoteBranchName)
  }
  return false
}

// 检出远程分支
const checkoutRemoteBranch = async (remoteBranchName: string) => {
  if (loading.value) return

  loading.value = true
  try {
    const result = await gitApi.checkoutRemoteBranch(props.repoPath, remoteBranchName)

    if (result.success) {
      success(result.message)
      // 重新获取分支列表以更新状态
      await fetchBranches()
    } else {
      error(`检出失败: ${result.message}`)
    }
  } catch (checkoutError) {
    error(`检出失败: ${checkoutError}`)
  } finally {
    loading.value = false
  }
}

// 组件挂载时获取分支列表
onMounted(() => {
  fetchBranches()
})
</script>

<template>
  <Card class="h-full gap-0 border-0 shadow-none py-0">
    <CardHeader>
      <div class="flex items-center justify-between">
        <div>
          <CardTitle class="flex items-center gap-2">
            <GitBranch class="h-5 w-5" />
            分支管理
          </CardTitle>
          <CardDescription>
            管理Git分支，查看分支状态和历史
          </CardDescription>
        </div>
        <div class="flex items-center gap-2">
          <Button
            variant="outline"
            size="sm"
            @click="refreshBranches"
            :disabled="loading"
          >
            <RefreshCw class="h-4 w-4" :class="{ 'animate-spin': loading }" />
          </Button>
          <Button size="sm" @click="showCreateForm = !showCreateForm">
            <Plus class="h-4 w-4 mr-2" />
            新建分支
          </Button>
        </div>
      </div>
    </CardHeader>
    <CardContent class="p-6">
      <div class="h-auto overflow-y-auto space-y-6">
        <!-- 当前分支信息 -->
        <div v-if="currentBranch" class="space-y-3">
            <h3 class="text-sm font-medium text-muted-foreground">当前分支</h3>
            <div class="p-4 border rounded-lg bg-muted/50">
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-3">
                  <GitBranch class="h-4 w-4 text-primary" />
                  <span class="font-medium">{{ currentBranch.name }}</span>
                  <Badge variant="default">当前</Badge>
                </div>
                <div class="flex items-center gap-4 text-sm text-muted-foreground">
                  <div v-if="currentBranch.ahead > 0" class="flex items-center gap-1">
                    <ArrowUp class="h-3 w-3" />
                    {{ currentBranch.ahead }}
                  </div>
                  <div v-if="currentBranch.behind > 0" class="flex items-center gap-1">
                    <ArrowDown class="h-3 w-3" />
                    {{ currentBranch.behind }}
                  </div>
                </div>
              </div>

              <div v-if="currentBranch.last_commit" class="mt-3 pt-3 border-t">
                <div class="flex items-start gap-3">
                  <GitCommit class="h-4 w-4 mt-0.5 text-muted-foreground" />
                  <div class="flex-1 min-w-0">
                    <p class="text-sm font-medium">
                      {{ formatCommitMessage(currentBranch.last_commit.message) }}
                    </p>
                    <div class="flex items-center gap-4 mt-1 text-xs text-muted-foreground">
                      <span>{{ currentBranch.last_commit.author_name }}</span>
                      <span class="flex items-center gap-1">
                        <Clock class="h-3 w-3" />
                        {{ formatTime(currentBranch.last_commit.timestamp) }}
                      </span>
                      <span class="font-mono">{{ currentBranch.last_commit.sha.substring(0, 7) }}</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>

        <!-- 创建分支表单 -->
        <div v-if="showCreateForm" class="space-y-4 p-4 border rounded-lg bg-muted/30">
          <h3 class="text-sm font-medium">创建新分支</h3>

          <div class="space-y-3">
            <div>
              <label class="text-xs font-medium text-muted-foreground">分支名称</label>
              <Input
                v-model="newBranchName"
                placeholder="feature/new-feature"
                class="mt-1"
                @keyup.enter="createBranch"
              />
            </div>

            <div>
              <label class="text-xs font-medium text-muted-foreground">基于分支</label>
              <select
                v-model="selectedBaseBranch"
                class="mt-1 w-full px-3 py-2 text-sm border border-border rounded-md bg-background focus:outline-none focus:ring-2 focus:ring-ring focus:border-transparent"
              >
                <option
                  v-for="branch in availableBaseBranches"
                  :key="branch.value"
                  :value="branch.value"
                >
                  {{ branch.label }}{{ branch.isCurrent ? ' (当前)' : '' }}
                </option>
              </select>
            </div>

            <div class="space-y-2">
              <div class="flex items-center space-x-2">
                <input
                  id="checkout-after-create"
                  v-model="checkoutAfterCreate"
                  type="checkbox"
                  class="rounded border-border"
                />
                <label for="checkout-after-create" class="text-xs">创建后切换到新分支</label>
              </div>

              <div class="flex items-center space-x-2">
                <input
                  id="push-to-remote"
                  v-model="pushToRemote"
                  type="checkbox"
                  class="rounded border-border"
                />
                <label for="push-to-remote" class="text-xs">创建后推送到远程</label>
              </div>
            </div>
          </div>

          <div class="flex items-center gap-2">
            <Button
              size="sm"
              @click="createBranch"
              :disabled="!newBranchName.trim() || creating || pushing"
            >
              <Plus v-if="!creating && !pushing" class="h-3 w-3 mr-1" />
              <RefreshCw v-else class="h-3 w-3 mr-1 animate-spin" />
              {{ getCreateButtonText() }}
            </Button>
            <Button
              variant="outline"
              size="sm"
              @click="resetCreateForm"
              :disabled="creating || pushing"
            >
              取消
            </Button>
          </div>
        </div>

        <!-- 本地分支列表 -->
        <div class="space-y-3">
            <div class="flex items-center justify-between">
              <h3 class="text-sm font-medium text-muted-foreground">
                本地分支 ({{ localBranches.length }})
              </h3>
            </div>

            <div class="space-y-2">
              <div
                v-for="branch in localBranches"
                :key="branch.name"
                class="p-3 border rounded-lg hover:bg-muted/50 transition-colors"
                :class="{ 'bg-muted/30': branch.is_current }"
              >
                <div class="flex items-center justify-between">
                  <div class="flex items-center gap-3 flex-1 min-w-0">
                    <GitBranch class="h-4 w-4 text-muted-foreground" />
                    <span class="font-medium truncate">{{ branch.name }}</span>

                    <div class="flex items-center gap-2">
                      <Badge
                        v-if="getBranchStatusBadge(branch)"
                        :variant="getBranchStatusBadge(branch)!.variant"
                        class="text-xs"
                      >
                        {{ getBranchStatusBadge(branch)!.text }}
                      </Badge>
                    </div>
                  </div>

                  <div class="flex items-center gap-2">
                    <div class="flex items-center gap-3 text-sm text-muted-foreground">
                      <div v-if="branch.ahead > 0" class="flex items-center gap-1">
                        <ArrowUp class="h-3 w-3" />
                        {{ branch.ahead }}
                      </div>
                      <div v-if="branch.behind > 0" class="flex items-center gap-1">
                        <ArrowDown class="h-3 w-3" />
                        {{ branch.behind }}
                      </div>
                    </div>

                    <!-- 推送按钮：仅对需要推送的分支显示 -->
                    <Button
                      v-if="needsPush(branch)"
                      variant="ghost"
                      size="sm"
                      @click="pushBranchToRemote(branch.name)"
                      :disabled="loading"
                      title="推送到远程"
                    >
                      <Upload class="h-4 w-4" />
                    </Button>

                    <Button
                      v-if="!branch.is_current"
                      variant="ghost"
                      size="sm"
                      @click="switchToBranch(branch.name)"
                      :disabled="loading"
                    >
                      切换
                    </Button>

                    <Button
                      v-if="!branch.is_current"
                      variant="ghost"
                      size="sm"
                      @click="deleteBranch(branch.name)"
                      :disabled="loading"
                    >
                      <Trash2 class="h-4 w-4" />
                    </Button>
                  </div>
                </div>

                <div v-if="branch.last_commit" class="mt-2 ml-7 text-sm text-muted-foreground">
                  <p class="truncate">{{ formatCommitMessage(branch.last_commit.message) }}</p>
                  <div class="flex items-center gap-3 mt-1 text-xs">
                    <span>{{ branch.last_commit.author_name }}</span>
                    <span>{{ formatTime(branch.last_commit.timestamp) }}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>

        <div class="border-t my-4"></div>

        <!-- 远程分支列表 -->
        <div class="space-y-3">
            <h3 class="text-sm font-medium text-muted-foreground">
              远程分支 ({{ remoteBranches.length }})
            </h3>

            <div class="space-y-2">
              <div
                v-for="branch in remoteBranches"
                :key="branch.name"
                class="p-3 border rounded-lg hover:bg-muted/50 transition-colors"
              >
                <div class="flex items-center justify-between">
                  <div class="flex items-center gap-3 flex-1 min-w-0">
                    <GitBranch class="h-4 w-4 text-muted-foreground" />
                    <span class="font-medium truncate">{{ branch.name }}</span>
                    <Badge variant="outline" class="text-xs">远程</Badge>
                  </div>

                  <Button
                    variant="ghost"
                    size="sm"
                    @click="checkoutRemoteBranch(branch.name)"
                    :disabled="loading"
                  >
                    检出
                  </Button>
                </div>

                <div v-if="branch.last_commit" class="mt-2 ml-7 text-sm text-muted-foreground">
                  <p class="truncate">{{ formatCommitMessage(branch.last_commit.message) }}</p>
                  <div class="flex items-center gap-3 mt-1 text-xs">
                    <span>{{ branch.last_commit.author_name }}</span>
                    <span>{{ formatTime(branch.last_commit.timestamp) }}</span>
                  </div>
                </div>
              </div>
            </div>
        </div>
      </div>
    </CardContent>
  </Card>
</template>

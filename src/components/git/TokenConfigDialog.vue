<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Badge } from '@/components/ui/badge';
import { useToast } from '@/components/ui/toast';
import { gitOperationsApi, type TokenConfig } from '@/api/git-operations';
import { Key, Github, GitlabIcon as Gitlab, Globe, Info } from 'lucide-vue-next';

// Props
interface Props {
  open: boolean;
  domain?: string;
  onClose: () => void;
  onSuccess?: (token: TokenConfig) => void;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  'update:open': [value: boolean];
}>();

// Toast
const { success, error } = useToast();

// 表单状态
const formData = ref({
  domain: props.domain || '',
  token: '',
  username: ''
});

const loading = ref(false);
const validationError = ref('');

// 计算属性
const isFormValid = computed(() => {
  return formData.value.domain.trim() !== '' && formData.value.token.trim() !== '';
});

const domainInfo = computed(() => {
  const domain = formData.value.domain.toLowerCase();

  if (domain.includes('github.com')) {
    return {
      name: 'GitHub',
      icon: Github,
      color: 'bg-gray-900',
      helpUrl: 'https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token',
      description: '在GitHub设置中创建Personal Access Token，需要repo权限'
    };
  } else if (domain.includes('gitlab.com') || domain.includes('gitlab')) {
    return {
      name: 'GitLab',
      icon: Gitlab,
      color: 'bg-orange-600',
      helpUrl: 'https://docs.gitlab.com/ee/user/profile/personal_access_tokens.html',
      description: '在GitLab设置中创建Personal Access Token，需要read_repository和write_repository权限'
    };
  } else if (domain.includes('atomgit.com')) {
    return {
      name: 'AtomGit',
      icon: Globe,
      color: 'bg-green-600',
      helpUrl: 'https://atomgit.com/help/user/profile/personal_access_tokens',
      description: '在AtomGit设置中创建Personal Access Token，需要repo权限'
    };
  } else if (domain.includes('gitee.com')) {
    return {
      name: 'Gitee',
      icon: Globe,
      color: 'bg-red-600',
      helpUrl: 'https://gitee.com/help/articles/4191',
      description: '在Gitee设置中创建私人令牌，需要projects权限'
    };
  } else if (domain.includes('bitbucket.org')) {
    return {
      name: 'Bitbucket',
      icon: Globe,
      color: 'bg-blue-700',
      helpUrl: 'https://support.atlassian.com/bitbucket-cloud/docs/app-passwords/',
      description: '在Bitbucket设置中创建App Password，需要Repositories权限'
    };
  } else {
    return {
      name: '自定义Git服务',
      icon: Globe,
      color: 'bg-blue-600',
      helpUrl: '',
      description: '请参考您的Git服务提供商文档创建Personal Access Token'
    };
  }
});

// 方法
const handleClose = () => {
  emit('update:open', false);
  props.onClose();
};

const handleSubmit = async () => {
  if (!isFormValid.value) {
    validationError.value = '请填写必填字段';
    return;
  }

  loading.value = true;
  validationError.value = '';

  try {
    await gitOperationsApi.storeAccessToken(
      formData.value.domain,
      formData.value.token,
      formData.value.username || undefined
    );

    const tokenConfig: TokenConfig = {
      domain: formData.value.domain,
      token: formData.value.token,
      username: formData.value.username || undefined,
      created_at: Date.now(),
    };

    success('Token配置成功');
    props.onSuccess?.(tokenConfig);
    handleClose();
  } catch (err) {
    const errorMessage = err instanceof Error ? err.message : 'Token配置失败';
    error(errorMessage);
    validationError.value = errorMessage;
  } finally {
    loading.value = false;
  }
};

const openHelpUrl = () => {
  if (domainInfo.value.helpUrl) {
    window.open(domainInfo.value.helpUrl, '_blank');
  }
};

// 重置表单
const resetForm = () => {
  formData.value = {
    domain: props.domain || '',
    token: '',
    username: ''
  };
  validationError.value = '';
};

// 监听props变化
watch(() => props.open, (newValue) => {
  if (newValue) {
    resetForm();
  }
});

watch(() => props.domain, (newValue) => {
  if (newValue) {
    formData.value.domain = newValue;
  }
});
</script>

<template>
  <Dialog :open="open" @update:open="(value) => emit('update:open', value)">
    <DialogContent class="sm:max-w-[500px]">
      <DialogHeader>
        <DialogTitle class="flex items-center space-x-2">
          <Key class="w-5 h-5" />
          <span>配置Personal Access Token</span>
        </DialogTitle>
        <p class="text-sm text-muted-foreground">
          为HTTPS协议的Git操作配置Personal Access Token认证
        </p>
      </DialogHeader>

      <div class="space-y-6">
        <!-- 域名信息展示 -->
        <div v-if="formData.domain" class="flex items-center space-x-3 p-3 bg-muted rounded-lg">
          <div :class="['w-8 h-8 rounded-full flex items-center justify-center text-white', domainInfo.color]">
            <component :is="domainInfo.icon" class="w-4 h-4" />
          </div>
          <div class="flex-1">
            <div class="font-medium">{{ domainInfo.name }}</div>
            <div class="text-sm text-muted-foreground">{{ formData.domain }}</div>
          </div>
          <Badge variant="outline">HTTPS</Badge>
        </div>

        <!-- 表单 -->
        <div class="space-y-4">
          <div class="space-y-2">
            <label for="domain" class="text-sm font-medium">Git服务域名 *</label>
            <Input
              id="domain"
              v-model="formData.domain"
              placeholder="例如: github.com, gitlab.com"
              :disabled="!!props.domain"
            />
          </div>

          <div class="space-y-2">
            <label for="token" class="text-sm font-medium">Personal Access Token *</label>
            <Input
              id="token"
              v-model="formData.token"
              type="password"
              placeholder="输入您的Personal Access Token"
            />
          </div>

          <div class="space-y-2">
            <label for="username" class="text-sm font-medium">用户名（可选）</label>
            <Input
              id="username"
              v-model="formData.username"
              placeholder="Git用户名"
            />
          </div>
        </div>

        <!-- 帮助信息 -->
        <div class="p-3 bg-blue-50 border border-blue-200 rounded-lg">
          <div class="flex items-start space-x-2">
            <Info class="w-4 h-4 text-blue-600 mt-0.5" />
            <div class="text-sm text-blue-800">
              {{ domainInfo.description }}
              <Button
                v-if="domainInfo.helpUrl"
                variant="link"
                size="sm"
                class="p-0 h-auto ml-2 text-blue-600"
                @click="openHelpUrl"
              >
                查看帮助文档
              </Button>
            </div>
          </div>
        </div>

        <!-- 错误信息 -->
        <div v-if="validationError" class="p-3 bg-red-50 border border-red-200 rounded-lg">
          <div class="text-sm text-red-800">{{ validationError }}</div>
        </div>
      </div>

      <DialogFooter class="mt-4">
        <Button variant="outline" @click="handleClose" :disabled="loading">
          取消
        </Button>
        <Button @click="handleSubmit" :disabled="!isFormValid || loading">
          <Key class="w-4 h-4 mr-2" />
          {{ loading ? '保存中...' : '保存Token' }}
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>

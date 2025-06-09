import { defineStore } from 'pinia';
import { ref } from 'vue';
import { $fetch } from '@/utils/fetch';
interface UserState {
  userid?: string;
  name?: string;
  position?: string;
  department?: number[];
}

const useUserStore = defineStore('user', () => {
  const getInitUser = (): UserState => ({
    userid: undefined,
    name: undefined,
    position: undefined,
  });

  const user = ref<UserState>(getInitUser());

  const setInfo = (partial: Partial<UserState>) => {
    user.value = { ...user.value, ...partial };
  }

  const resetInfo = () => {
    user.value = getInitUser();
  }

  // Get user's information
  const fetchInfo = async () => {
    const { success, data } = await $fetch('/user/info', { method: 'get' });
    if (success) {
      setInfo(data)
    }
    return { success, data }
  }

  return { user, setInfo, resetInfo, fetchInfo };
});

export default useUserStore;

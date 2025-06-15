import { createPinia } from 'pinia';
import useUserStore from './user/index';
import { useRepositoryStore } from './repository/index';

const pinia = createPinia();

export { useUserStore, useRepositoryStore };

export default pinia;
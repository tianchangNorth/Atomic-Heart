import { createPinia } from 'pinia';
import useUserStore from './user/index';

const pinia = createPinia();

export { useUserStore };

export default pinia;
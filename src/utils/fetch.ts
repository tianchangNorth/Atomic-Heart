// src/utils/http.ts
import { invoke } from '@tauri-apps/api/core';
import { getToken } from './token';
// import router from '@/router';
export interface ApiResponse {
  success: boolean;
  code: number;
  message?: string;
  data?: any;
}

type HttpMethod = 'get' | 'post' | 'GET' | 'POST';

interface HttpOptions {
  method?: HttpMethod;
  data?: Record<string, any>;
  headers?: Record<string, string>;
}
const baseUrl = import.meta.env.VITE_APP_BASE_API;
/**
 * 统一的请求方法，支持 GET / POST，并自动附带本地 token
 */
export async function $fetch(url: string, options: HttpOptions): Promise<ApiResponse> {
  const { method = 'get', data, headers = {} } = options;
  const fullUrl = `${baseUrl}${url}`;

  // 自动添加 token
  const token = getToken();
  if (token && !headers['Authorization']) {
    headers['Authorization'] = `Bearer ${token}`;
  }

  const invokeName = method.toLowerCase() === 'post' ? 'http_post' : 'http_get';

  const response = await invoke<ApiResponse>(invokeName, {
    url: fullUrl,
    data,
    headers,
  });
  if (response.success) {
    return response;
  } else {
    // router.push('/login');
    throw new Error(response.message);
  }
}

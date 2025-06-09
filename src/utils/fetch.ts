// src/utils/http.ts
import { invoke } from '@tauri-apps/api/core';

export interface ApiResponse {
  success: boolean;
  code: number;
  message?: string;
  data?: any;
}

type HttpMethod = 'get' | 'post'|'GET'|'POST';

interface HttpOptions {
  method?: HttpMethod;
  body?: Record<string, any>;
  headers?: Record<string, string>;
}

/**
 * 统一的请求方法，支持 GET / POST，并自动附带本地 token
 */
export async function $fetch(url:string,options: HttpOptions): Promise<ApiResponse> {
  const { method = 'get', body, headers = {} } = options;

  // 自动添加 token
  const token = localStorage.getItem('access_token');
  if (token && !headers['Authorization']) {
    headers['Authorization'] = `Bearer ${token}`;
  }

  const invokeName = method === 'post' ? 'http_post' : 'http_get';

  const response =  await invoke<ApiResponse>(invokeName, {
    path:url,
    body,
    headers,
  });
  if(response.success){
    return response.data;
  }else{
    throw new Error(response.message);  
  }
}

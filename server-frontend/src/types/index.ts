/**
 * TaskFleet - 中央类型导出
 */

// 导出所有类型定义
export * from './user';
export * from './task';
export * from './project';
export * from './analytics';

/**
 * API响应格式
 */
export interface ApiResponse<T> {
  success: boolean;
  message?: string;
  data?: T;
  error?: string;
}

/**
 * 分页响应格式
 */
export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  limit: number;
}

/**
 * 登录请求
 */
export interface LoginRequest {
  username: string;
  password: string;
}

/**
 * 登录响应
 */
export interface LoginResponse {
  token: string;
  user: any; // 使用User类型
}

/**
 * 注册请求
 */
export interface RegisterRequest {
  username: string;
  email: string;
  password: string;
  full_name: string;
}

/**
 * TaskFleet - 用户相关类型定义
 */

/**
 * 用户角色枚举
 */
export enum UserRole {
  SystemAdmin = 'system_admin',      // 系统管理员 - 可管理所有公司
  CompanyAdmin = 'company_admin',    // 公司管理员 - 只能管理本公司数据
  Employee = 'employee',             // 普通员工 - 只能查看自己的任务
}

/**
 * 用户实体
 */
export interface User {
  id: string;
  username: string;
  email: string;
  role: UserRole;
  full_name: string;
  is_active: boolean;
  company_id?: number;               // 所属公司ID(SystemAdmin可为空)
  created_at: string;
  updated_at: string;
  last_login?: string;
}

/**
 * 创建用户请求
 */
export interface CreateUserRequest {
  username: string;
  email: string;
  password: string;
  role: UserRole;
  full_name: string;
}

/**
 * 更新用户请求
 */
export interface UpdateUserRequest {
  username?: string;
  email?: string;
  password?: string;
  role?: UserRole;
  full_name?: string;
  is_active?: boolean;
}

/**
 * 用户列表响应
 */
export interface UserListResponse {
  users: User[];
  total: number;
  page: number;
  limit: number;
}

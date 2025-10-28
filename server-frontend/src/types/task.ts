/**
 * TaskFleet - 任务相关类型定义
 */

/**
 * 任务状态枚举
 */
export enum TaskStatus {
  Pending = 'pending',
  InProgress = 'in_progress',
  Completed = 'completed',
  Cancelled = 'cancelled',
}

/**
 * 任务优先级枚举
 */
export enum TaskPriority {
  Low = 'low',
  Medium = 'medium',
  High = 'high',
  Urgent = 'urgent',
}

/**
 * 任务实体
 */
export interface Task {
  id: string;
  title: string;
  description: string;
  status: TaskStatus;
  priority: TaskPriority;
  
  // 关联关系
  project_id?: string;
  assigned_to?: string;
  created_by: string;
  
  // 时间管理
  due_date?: string;
  estimated_hours?: number;
  actual_hours?: number;
  
  // 元数据
  created_at: string;
  updated_at: string;
  completed_at?: string;
}

/**
 * 任务详细信息（包含关联数据）
 */
export interface TaskInfo extends Task {
  project_name?: string;
  assigned_to_name?: string;
  created_by_name: string;
}

/**
 * 创建任务请求
 */
export interface CreateTaskRequest {
  title: string;
  description: string;
  priority: TaskPriority;
  project_id?: string;
  assigned_to?: string;
  due_date?: string;
  estimated_hours?: number;
}

/**
 * 更新任务请求
 */
export interface UpdateTaskRequest {
  title?: string;
  description?: string;
  status?: TaskStatus;
  priority?: TaskPriority;
  assigned_to?: string;
  due_date?: string;
  estimated_hours?: number;
}

/**
 * 任务查询参数
 */
export interface TaskQueryParams {
  project_id?: string;
  assigned_to?: string;
  status?: TaskStatus;
  page?: number;
  limit?: number;
}

/**
 * 分配任务请求
 */
export interface AssignTaskRequest {
  assigned_to: string;
}

/**
 * 任务列表响应
 */
export interface TaskListResponse {
  tasks: TaskInfo[];
  total: number;
  page: number;
  limit: number;
}

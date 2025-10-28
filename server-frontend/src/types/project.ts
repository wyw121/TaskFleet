/**
 * TaskFleet - 项目相关类型定义
 */

/**
 * 项目状态枚举
 */
export enum ProjectStatus {
  Planning = 'planning',
  Active = 'active',
  OnHold = 'on_hold',
  Completed = 'completed',
  Cancelled = 'cancelled',
}

/**
 * 项目实体
 */
export interface Project {
  id: string;
  name: string;
  description?: string;
  status: ProjectStatus;
  manager_id: string;
  start_date?: string;
  end_date?: string;
  budget?: number;
  actual_cost?: number;
  created_at: string;
  updated_at: string;
}

/**
 * 项目详细信息（包含统计数据）
 */
export interface ProjectInfo extends Project {
  manager_name?: string;
  task_count?: number;
  completed_tasks?: number;
  progress?: number;
}

/**
 * 创建项目请求
 */
export interface CreateProjectRequest {
  name: string;
  description?: string;
  status?: ProjectStatus;
  manager_id: string;
  start_date?: string;
  end_date?: string;
  budget?: number;
}

/**
 * 更新项目请求
 */
export interface UpdateProjectRequest {
  name?: string;
  description?: string;
  status?: ProjectStatus;
  manager_id?: string;
  start_date?: string;
  end_date?: string;
  budget?: number;
  actual_cost?: number;
}

/**
 * 项目查询参数
 */
export interface ProjectQueryParams {
  status?: ProjectStatus;
  manager_id?: string;
  page?: number;
  limit?: number;
}

/**
 * 项目列表响应
 */
export interface ProjectListResponse {
  projects: ProjectInfo[];
  total: number;
  page: number;
  limit: number;
}

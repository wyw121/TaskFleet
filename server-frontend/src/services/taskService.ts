/**
 * TaskFleet - 任务管理API服务
 */

import api from './api';
import {
  Task,
  TaskInfo,
  CreateTaskRequest,
  UpdateTaskRequest,
  TaskQueryParams,
  AssignTaskRequest,
  TaskListResponse,
} from '../types';

/**
 * 任务服务类
 */
class TaskService {
  private baseUrl = '/api/v1/tasks';

  /**
   * 获取任务列表
   */
  async getTasks(params?: TaskQueryParams): Promise<TaskInfo[]> {
    const response = await api.get<TaskInfo[]>(this.baseUrl, { params });
    return response.data;
  }

  /**
   * 获取单个任务详情
   */
  async getTask(id: string): Promise<TaskInfo> {
    const response = await api.get<TaskInfo>(`${this.baseUrl}/${id}`);
    return response.data;
  }

  /**
   * 创建新任务
   */
  async createTask(data: CreateTaskRequest): Promise<Task> {
    const response = await api.post<Task>(this.baseUrl, data);
    return response.data;
  }

  /**
   * 更新任务
   */
  async updateTask(id: string, data: UpdateTaskRequest): Promise<Task> {
    const response = await api.put<Task>(`${this.baseUrl}/${id}`, data);
    return response.data;
  }

  /**
   * 删除任务
   */
  async deleteTask(id: string): Promise<void> {
    await api.delete(`${this.baseUrl}/${id}`);
  }

  /**
   * 开始任务
   */
  async startTask(id: string): Promise<Task> {
    const response = await api.post<Task>(`${this.baseUrl}/${id}/start`);
    return response.data;
  }

  /**
   * 完成任务
   */
  async completeTask(id: string): Promise<Task> {
    const response = await api.post<Task>(`${this.baseUrl}/${id}/complete`);
    return response.data;
  }

  /**
   * 取消任务
   */
  async cancelTask(id: string): Promise<Task> {
    const response = await api.post<Task>(`${this.baseUrl}/${id}/cancel`);
    return response.data;
  }

  /**
   * 分配任务
   */
  async assignTask(id: string, data: AssignTaskRequest): Promise<Task> {
    const response = await api.post<Task>(`${this.baseUrl}/${id}/assign`, data);
    return response.data;
  }

  /**
   * 按项目获取任务
   */
  async getTasksByProject(projectId: string): Promise<TaskInfo[]> {
    return this.getTasks({ project_id: projectId });
  }

  /**
   * 按用户获取任务
   */
  async getTasksByAssignee(userId: string): Promise<TaskInfo[]> {
    return this.getTasks({ assigned_to: userId });
  }

  /**
   * 按状态获取任务
   */
  async getTasksByStatus(status: string): Promise<TaskInfo[]> {
    return this.getTasks({ status: status as any });
  }
}

export default new TaskService();

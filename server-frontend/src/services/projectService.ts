/**
 * TaskFleet - 项目管理API服务
 */

import api from './api';
import {
  Project,
  ProjectInfo,
  CreateProjectRequest,
  UpdateProjectRequest,
  ProjectQueryParams,
} from '../types';

/**
 * 项目服务类
 */
class ProjectService {
  private baseUrl = '/api/v1/projects';

  /**
   * 获取项目列表
   */
  async getProjects(params?: ProjectQueryParams): Promise<ProjectInfo[]> {
    const response = await api.get<ProjectInfo[]>(this.baseUrl, { params });
    return response.data;
  }

  /**
   * 获取单个项目详情
   */
  async getProject(id: string): Promise<ProjectInfo> {
    const response = await api.get<ProjectInfo>(`${this.baseUrl}/${id}`);
    return response.data;
  }

  /**
   * 创建新项目
   */
  async createProject(data: CreateProjectRequest): Promise<Project> {
    const response = await api.post<Project>(this.baseUrl, data);
    return response.data;
  }

  /**
   * 更新项目
   */
  async updateProject(id: string, data: UpdateProjectRequest): Promise<Project> {
    const response = await api.put<Project>(`${this.baseUrl}/${id}`, data);
    return response.data;
  }

  /**
   * 删除项目
   */
  async deleteProject(id: string): Promise<void> {
    await api.delete(`${this.baseUrl}/${id}`);
  }

  /**
   * 启动项目
   */
  async startProject(id: string): Promise<Project> {
    const response = await api.post<Project>(`${this.baseUrl}/${id}/start`);
    return response.data;
  }

  /**
   * 暂停项目
   */
  async holdProject(id: string): Promise<Project> {
    const response = await api.post<Project>(`${this.baseUrl}/${id}/hold`);
    return response.data;
  }

  /**
   * 完成项目
   */
  async completeProject(id: string): Promise<Project> {
    const response = await api.post<Project>(`${this.baseUrl}/${id}/complete`);
    return response.data;
  }

  /**
   * 取消项目
   */
  async cancelProject(id: string): Promise<Project> {
    const response = await api.post<Project>(`${this.baseUrl}/${id}/cancel`);
    return response.data;
  }
}

export default new ProjectService();

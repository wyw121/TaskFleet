/**
 * TaskFleet - 数据分析API服务
 */

import api from './api';
import {
  TaskStatistics,
  ProjectStatistics,
  UserWorkloadStatistics,
  ProjectProgressStatistics,
  DashboardData,
} from '../types';

/**
 * 分析服务类
 */
class AnalyticsService {
  private baseUrl = '/api/v1/statistics';

  /**
   * 获取任务统计数据
   */
  async getTaskStatistics(): Promise<TaskStatistics> {
    const response = await api.get<TaskStatistics>(`${this.baseUrl}/tasks`);
    return response.data;
  }

  /**
   * 获取项目统计数据
   */
  async getProjectStatistics(): Promise<ProjectStatistics> {
    const response = await api.get<ProjectStatistics>(`${this.baseUrl}/projects`);
    return response.data;
  }

  /**
   * 获取用户工作量统计
   */
  async getUserWorkload(userId: string): Promise<UserWorkloadStatistics> {
    const response = await api.get<UserWorkloadStatistics>(
      `${this.baseUrl}/users/${userId}/workload`
    );
    return response.data;
  }

  /**
   * 获取所有用户工作量统计
   */
  async getAllUsersWorkload(): Promise<UserWorkloadStatistics[]> {
    const response = await api.get<UserWorkloadStatistics[]>(
      `${this.baseUrl}/users/workload`
    );
    return response.data;
  }

  /**
   * 获取项目进度统计
   */
  async getProjectProgress(projectId: string): Promise<ProjectProgressStatistics> {
    const response = await api.get<ProjectProgressStatistics>(
      `${this.baseUrl}/projects/${projectId}/progress`
    );
    return response.data;
  }

  /**
   * 获取仪表板概览数据
   */
  async getDashboardData(): Promise<DashboardData> {
    const [taskStats, projectStats] = await Promise.all([
      this.getTaskStatistics(),
      this.getProjectStatistics(),
    ]);

    return {
      task_statistics: taskStats,
      project_statistics: projectStats,
      recent_tasks: [],
      recent_projects: [],
    };
  }
}

export default new AnalyticsService();

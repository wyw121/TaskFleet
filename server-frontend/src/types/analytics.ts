/**
 * TaskFleet - 数据分析和统计相关类型定义
 */

/**
 * 任务统计
 */
export interface TaskStatistics {
  total_tasks: number;
  pending_tasks: number;
  in_progress_tasks: number;
  completed_tasks: number;
  cancelled_tasks: number;
  completion_rate: number;
}

/**
 * 项目统计
 */
export interface ProjectStatistics {
  total_projects: number;
  planning_projects: number;
  active_projects: number;
  on_hold_projects: number;
  completed_projects: number;
  cancelled_projects: number;
}

/**
 * 用户工作量统计
 */
export interface UserWorkloadStatistics {
  user_id: string;
  user_name: string;
  total_tasks: number;
  pending_tasks: number;
  in_progress_tasks: number;
  completed_tasks: number;
  total_hours: number;
  completion_rate: number;
}

/**
 * 项目进度统计
 */
export interface ProjectProgressStatistics {
  project_id: string;
  project_name: string;
  total_tasks: number;
  completed_tasks: number;
  in_progress_tasks: number;
  pending_tasks: number;
  progress_percentage: number;
  total_estimated_hours: number;
  total_actual_hours: number;
}

/**
 * 仪表板数据
 */
export interface DashboardData {
  task_statistics: TaskStatistics;
  project_statistics: ProjectStatistics;
  recent_tasks: Array<{
    id: string;
    title: string;
    status: string;
    priority: string;
    updated_at: string;
  }>;
  recent_projects: Array<{
    id: string;
    name: string;
    status: string;
    progress: number;
    updated_at: string;
  }>;
}

/**
 * 图表数据点
 */
export interface ChartDataPoint {
  label: string;
  value: number;
  date?: string;
}

/**
 * 趋势数据
 */
export interface TrendData {
  period: string;
  data_points: ChartDataPoint[];
}

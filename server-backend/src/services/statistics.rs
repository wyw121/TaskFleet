use crate::database::Database;
use crate::errors::AppError;
use serde::Serialize;
use uuid::Uuid;

/// 任务统计数据
#[derive(Debug, Serialize)]
pub struct TaskStatistics {
    /// 总任务数
    pub total_tasks: i64,
    /// 待处理任务数
    pub pending_tasks: i64,
    /// 进行中任务数
    pub in_progress_tasks: i64,
    /// 已完成任务数
    pub completed_tasks: i64,
    /// 已取消任务数
    pub cancelled_tasks: i64,
    /// 完成率(%)
    pub completion_rate: f64,
}

/// 项目统计数据
#[derive(Debug, Serialize)]
pub struct ProjectStatistics {
    /// 总项目数
    pub total_projects: i64,
    /// 规划中项目数
    pub planning_projects: i64,
    /// 进行中项目数
    pub active_projects: i64,
    /// 暂停项目数
    pub on_hold_projects: i64,
    /// 已完成项目数
    pub completed_projects: i64,
    /// 已取消项目数
    pub cancelled_projects: i64,
}

/// 员工工作量统计
#[derive(Debug, Serialize)]
pub struct UserWorkloadStatistics {
    /// 员工ID
    pub user_id: Uuid,
    /// 员工姓名
    pub user_name: Option<String>,
    /// 分配任务总数
    pub assigned_tasks: i64,
    /// 已完成任务数
    pub completed_tasks: i64,
    /// 进行中任务数
    pub in_progress_tasks: i64,
    /// 工作时长(小时)
    pub total_hours: f64,
}

/// 项目进度统计
#[derive(Debug, Serialize)]
pub struct ProjectProgressStatistics {
    /// 项目ID
    pub project_id: Uuid,
    /// 项目名称
    pub project_name: String,
    /// 任务总数
    pub total_tasks: i64,
    /// 已完成任务数
    pub completed_tasks: i64,
    /// 进行中任务数
    pub in_progress_tasks: i64,
    /// 完成率(%)
    pub progress: f64,
    /// 预估总工时
    pub estimated_hours: f64,
    /// 实际工时
    pub actual_hours: f64,
}

/// 数据统计服务
pub struct StatisticsService {
    db: Database,
}

impl StatisticsService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// 获取任务统计
    pub async fn get_task_statistics(&self) -> Result<TaskStatistics, AppError> {
        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tasks")
            .fetch_one(&self.db.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let pending: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tasks WHERE status = 'pending'")
            .fetch_one(&self.db.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let in_progress: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tasks WHERE status = 'in_progress'")
            .fetch_one(&self.db.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let completed: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tasks WHERE status = 'completed'")
            .fetch_one(&self.db.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let cancelled: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tasks WHERE status = 'cancelled'")
            .fetch_one(&self.db.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let completion_rate = if total.0 > 0 {
            (completed.0 as f64 / total.0 as f64) * 100.0
        } else {
            0.0
        };

        Ok(TaskStatistics {
            total_tasks: total.0,
            pending_tasks: pending.0,
            in_progress_tasks: in_progress.0,
            completed_tasks: completed.0,
            cancelled_tasks: cancelled.0,
            completion_rate,
        })
    }

    /// 获取项目统计
    pub async fn get_project_statistics(&self) -> Result<ProjectStatistics, AppError> {
        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM projects")
            .fetch_one(&self.db.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let planning: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM projects WHERE status = 'planning'")
            .fetch_one(&self.db.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let active: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM projects WHERE status = 'active'")
            .fetch_one(&self.db.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let on_hold: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM projects WHERE status = 'on_hold'")
            .fetch_one(&self.db.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let completed: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM projects WHERE status = 'completed'")
            .fetch_one(&self.db.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let cancelled: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM projects WHERE status = 'cancelled'")
            .fetch_one(&self.db.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(ProjectStatistics {
            total_projects: total.0,
            planning_projects: planning.0,
            active_projects: active.0,
            on_hold_projects: on_hold.0,
            completed_projects: completed.0,
            cancelled_projects: cancelled.0,
        })
    }

    /// 获取员工工作量统计
    pub async fn get_user_workload(&self, user_id: Uuid) -> Result<UserWorkloadStatistics, AppError> {
        let assigned: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM tasks WHERE assigned_to = ?"
        )
        .bind(user_id)
        .fetch_one(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let completed: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM tasks WHERE assigned_to = ? AND status = 'completed'"
        )
        .bind(user_id)
        .fetch_one(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let in_progress: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM tasks WHERE assigned_to = ? AND status = 'in_progress'"
        )
        .bind(user_id)
        .fetch_one(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let hours_result: Option<(Option<f64>,)> = sqlx::query_as(
            "SELECT SUM(actual_hours) FROM tasks WHERE assigned_to = ?"
        )
        .bind(user_id)
        .fetch_optional(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let total_hours = hours_result
            .and_then(|r| r.0)
            .unwrap_or(0.0);

        Ok(UserWorkloadStatistics {
            user_id,
            user_name: None,
            assigned_tasks: assigned.0,
            completed_tasks: completed.0,
            in_progress_tasks: in_progress.0,
            total_hours,
        })
    }

    /// 获取项目进度统计
    pub async fn get_project_progress(&self, project_id: Uuid) -> Result<ProjectProgressStatistics, AppError> {
        // 获取项目名称
        let project: (String,) = sqlx::query_as(
            "SELECT name FROM projects WHERE id = ?"
        )
        .bind(project_id)
        .fetch_one(&self.db.pool)
        .await
        .map_err(|_| AppError::NotFound("项目不存在".to_string()))?;

        let total: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM tasks WHERE project_id = ?"
        )
        .bind(project_id)
        .fetch_one(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let completed: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM tasks WHERE project_id = ? AND status = 'completed'"
        )
        .bind(project_id)
        .fetch_one(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let in_progress: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM tasks WHERE project_id = ? AND status = 'in_progress'"
        )
        .bind(project_id)
        .fetch_one(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let estimated_result: Option<(Option<f64>,)> = sqlx::query_as(
            "SELECT SUM(estimated_hours) FROM tasks WHERE project_id = ?"
        )
        .bind(project_id)
        .fetch_optional(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let actual_result: Option<(Option<f64>,)> = sqlx::query_as(
            "SELECT SUM(actual_hours) FROM tasks WHERE project_id = ?"
        )
        .bind(project_id)
        .fetch_optional(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let estimated_hours = estimated_result
            .and_then(|r| r.0)
            .unwrap_or(0.0);

        let actual_hours = actual_result
            .and_then(|r| r.0)
            .unwrap_or(0.0);

        let progress = if total.0 > 0 {
            (completed.0 as f64 / total.0 as f64) * 100.0
        } else {
            0.0
        };

        Ok(ProjectProgressStatistics {
            project_id,
            project_name: project.0,
            total_tasks: total.0,
            completed_tasks: completed.0,
            in_progress_tasks: in_progress.0,
            progress,
            estimated_hours,
            actual_hours,
        })
    }

    /// 获取所有员工工作量统计
    pub async fn get_all_users_workload(&self) -> Result<Vec<UserWorkloadStatistics>, AppError> {
        let user_ids: Vec<(Uuid,)> = sqlx::query_as(
            "SELECT DISTINCT assigned_to FROM tasks WHERE assigned_to IS NOT NULL"
        )
        .fetch_all(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut workloads = Vec::new();
        for (user_id,) in user_ids {
            let workload = self.get_user_workload(user_id).await?;
            workloads.push(workload);
        }

        Ok(workloads)
    }
}

use axum::{
    extract::State,
    response::Json,
};
use crate::database::Database;
use crate::errors::AppError;
use crate::services::statistics::{ProjectStatistics, TaskStatistics};
use crate::Config;

type AppState = (Database, Config);

/// 获取任务统计 (轻量实现：如果 tasks 表不存在则返回 0 值统计)
/// GET /api/v1/statistics/tasks
pub async fn get_task_statistics(
    State((db, _config)): State<AppState>,
) -> Result<Json<TaskStatistics>, AppError> {
    // 尝试在数据库中执行计数查询；如果表不存在或查询失败，返回默认值以避免 404/500
    let total = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM tasks")
        .fetch_one(&db.pool)
        .await
        .map(|r| r.0)
        .unwrap_or(0);

    let pending = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM tasks WHERE status = 'pending'")
        .fetch_one(&db.pool)
        .await
        .map(|r| r.0)
        .unwrap_or(0);

    let in_progress = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM tasks WHERE status = 'in_progress'")
        .fetch_one(&db.pool)
        .await
        .map(|r| r.0)
        .unwrap_or(0);

    let completed = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM tasks WHERE status = 'completed'")
        .fetch_one(&db.pool)
        .await
        .map(|r| r.0)
        .unwrap_or(0);

    let cancelled = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM tasks WHERE status = 'cancelled'")
        .fetch_one(&db.pool)
        .await
        .map(|r| r.0)
        .unwrap_or(0);

    let completion_rate = if total > 0 { (completed as f64 / total as f64) * 100.0 } else { 0.0 };

    Ok(Json(TaskStatistics {
        total_tasks: total,
        pending_tasks: pending,
        in_progress_tasks: in_progress,
        completed_tasks: completed,
        cancelled_tasks: cancelled,
        completion_rate,
    }))
}

/// 获取项目统计 (轻量实现：如果 projects 表不存在则返回 0 值统计)
/// GET /api/v1/statistics/projects
pub async fn get_project_statistics(
    State((db, _config)): State<AppState>,
) -> Result<Json<ProjectStatistics>, AppError> {
    let total = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM projects")
        .fetch_one(&db.pool)
        .await
        .map(|r| r.0)
        .unwrap_or(0);

    let planning = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM projects WHERE status = 'planning'")
        .fetch_one(&db.pool)
        .await
        .map(|r| r.0)
        .unwrap_or(0);

    let active = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM projects WHERE status = 'active'")
        .fetch_one(&db.pool)
        .await
        .map(|r| r.0)
        .unwrap_or(0);

    let on_hold = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM projects WHERE status = 'on_hold'")
        .fetch_one(&db.pool)
        .await
        .map(|r| r.0)
        .unwrap_or(0);

    let completed = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM projects WHERE status = 'completed'")
        .fetch_one(&db.pool)
        .await
        .map(|r| r.0)
        .unwrap_or(0);

    let cancelled = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM projects WHERE status = 'cancelled'")
        .fetch_one(&db.pool)
        .await
        .map(|r| r.0)
        .unwrap_or(0);

    Ok(Json(ProjectStatistics {
        total_projects: total,
        planning_projects: planning,
        active_projects: active,
        on_hold_projects: on_hold,
        completed_projects: completed,
        cancelled_projects: cancelled,
    }))
}

// 其余统计端点（按员工/按项目进度）仍在迁移中，若需要可在后续迭代中实现。

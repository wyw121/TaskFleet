use axum::{
    extract::{Path, State},
    response::Json,
};
use uuid::Uuid;

use crate::database::Database;
use crate::errors::AppError;
use crate::services::statistics::{
    ProjectProgressStatistics, ProjectStatistics, StatisticsService, TaskStatistics,
    UserWorkloadStatistics,
};
use crate::Config;

type AppState = (Database, Config);

/// 获取任务统计
/// GET /api/statistics/tasks
pub async fn get_task_statistics(
    State((db, _config)): State<AppState>,
) -> Result<Json<TaskStatistics>, AppError> {
    let service = StatisticsService::new(db);
    let stats = service.get_task_statistics().await?;
    Ok(Json(stats))
}

/// 获取项目统计
/// GET /api/statistics/projects
pub async fn get_project_statistics(
    State((db, _config)): State<AppState>,
) -> Result<Json<ProjectStatistics>, AppError> {
    let service = StatisticsService::new(db);
    let stats = service.get_project_statistics().await?;
    Ok(Json(stats))
}

/// 获取指定员工工作量统计
/// GET /api/statistics/users/:user_id/workload
pub async fn get_user_workload(
    State((db, _config)): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserWorkloadStatistics>, AppError> {
    let service = StatisticsService::new(db);
    let stats = service.get_user_workload(user_id).await?;
    Ok(Json(stats))
}

/// 获取所有员工工作量统计
/// GET /api/statistics/users/workload
pub async fn get_all_users_workload(
    State((db, _config)): State<AppState>,
) -> Result<Json<Vec<UserWorkloadStatistics>>, AppError> {
    let service = StatisticsService::new(db);
    let stats = service.get_all_users_workload().await?;
    Ok(Json(stats))
}

/// 获取项目进度统计
/// GET /api/statistics/projects/:project_id/progress
pub async fn get_project_progress(
    State((db, _config)): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<ProjectProgressStatistics>, AppError> {
    let service = StatisticsService::new(db);
    let stats = service.get_project_progress(project_id).await?;
    Ok(Json(stats))
}

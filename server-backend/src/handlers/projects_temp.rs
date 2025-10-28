use axum::{
    extract::State,
    response::Json,
};
use crate::database::Database;
use crate::errors::AppError;
use crate::Config;
use serde::{Deserialize, Serialize};

type AppState = (Database, Config);

/// 项目模型 (临时占位符)
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
}

/// 用户工作量统计
#[derive(Debug, Serialize, Deserialize)]
pub struct UserWorkloadStatistics {
    pub user_id: i64,
    pub username: String,
    pub total_tasks: i64,
    pub completed_tasks: i64,
    pub in_progress_tasks: i64,
    pub pending_tasks: i64,
}

/// 获取所有项目 (临时实现：返回空数组，避免404)
/// GET /api/v1/projects
pub async fn list_projects(
    State((_db, _config)): State<AppState>,
) -> Result<Json<Vec<Project>>, AppError> {
    // 返回空数组而不是404，等待数据库迁移后实现完整功能
    Ok(Json(vec![]))
}

/// 获取所有用户工作量统计 (临时实现：返回空数组)
/// GET /api/v1/statistics/users/workload
pub async fn get_all_users_workload(
    State((_db, _config)): State<AppState>,
) -> Result<Json<Vec<UserWorkloadStatistics>>, AppError> {
    // 临时实现：返回空数组
    // TODO: 等待 tasks 表创建后,实现完整的用户工作量统计
    Ok(Json(vec![]))
}

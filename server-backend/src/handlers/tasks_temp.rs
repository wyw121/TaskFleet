use axum::{
    extract::State,
    response::Json,
};
use serde::{Deserialize, Serialize};

use crate::database::Database;
use crate::errors::AppError;
use crate::Config;

type AppState = (Database, Config);

/// 临时任务响应结构（简化版，避免前端报错）
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub created_at: String,
}

/// 获取任务列表 (临时实现：返回空数组)
/// GET /api/v1/tasks
pub async fn list_tasks(
    State((_db, _config)): State<AppState>,
) -> Result<Json<Vec<Task>>, AppError> {
    // 临时返回空数组，避免前端 404 错误
    // 等待 tasks 表创建和完整 handler 实现后再替换
    Ok(Json(vec![]))
}

/// 获取单个任务 (临时实现)
/// GET /api/v1/tasks/:id
pub async fn get_task(
    State((_db, _config)): State<AppState>,
) -> Result<Json<Task>, AppError> {
    Err(AppError::NotFound("Tasks功能正在迁移中".to_string()))
}

/// 创建任务 (临时实现)
/// POST /api/v1/tasks
pub async fn create_task(
    State((_db, _config)): State<AppState>,
) -> Result<Json<Task>, AppError> {
    Err(AppError::BadRequest("Tasks功能正在迁移中".to_string()))
}

/// 更新任务 (临时实现)
/// PUT /api/v1/tasks/:id
pub async fn update_task(
    State((_db, _config)): State<AppState>,
) -> Result<Json<Task>, AppError> {
    Err(AppError::BadRequest("Tasks功能正在迁移中".to_string()))
}

/// 删除任务 (临时实现)
/// DELETE /api/v1/tasks/:id
pub async fn delete_task(
    State((_db, _config)): State<AppState>,
) -> Result<Json<()>, AppError> {
    Err(AppError::BadRequest("Tasks功能正在迁移中".to_string()))
}

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    Extension,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::database::Database;
use crate::errors::AppError;
use crate::models::{CreateTaskRequest, TaskInfo, TaskStatus, UpdateTaskRequest, User};
use crate::services::task::TaskService;
use crate::Config;

type AppState = (Database, Config);

/// 任务列表查询参数
#[derive(Debug, Deserialize)]
pub struct TaskQueryParams {
    /// 按项目ID筛选
    pub project_id: Option<Uuid>,
    /// 按分配人ID筛选
    pub assignee_id: Option<Uuid>,
    /// 按状态筛选
    pub status: Option<String>,
}

/// 任务状态更新请求
#[derive(Debug, Deserialize)]
pub struct UpdateTaskStatusRequest {
    pub status: TaskStatus,
}

/// 任务分配请求
#[derive(Debug, Deserialize)]
pub struct AssignTaskRequest {
    pub assignee_id: Uuid,
}

/// 创建任务
/// POST /api/tasks
pub async fn create_task(
    State((db, _config)): State<AppState>,
    Extension(user): Extension<User>,
    Json(request): Json<CreateTaskRequest>,
) -> Result<Json<TaskInfo>, AppError> {
    let service = TaskService::new(db);
    let task = service.create_task(request, user.id).await?;
    Ok(Json(task))
}

/// 获取任务列表
/// GET /api/tasks?project_id=xxx&assignee_id=xxx&status=pending
pub async fn list_tasks(
    State((db, _config)): State<AppState>,
    Query(params): Query<TaskQueryParams>,
) -> Result<Json<Vec<TaskInfo>>, AppError> {
    let service = TaskService::new(db);

    let tasks = if let Some(project_id) = params.project_id {
        // 按项目筛选
        service.list_tasks_by_project(project_id).await?
    } else if let Some(assignee_id) = params.assignee_id {
        // 按分配人筛选
        service.list_tasks_by_assignee(assignee_id).await?
    } else if let Some(status_str) = params.status {
        // 按状态筛选
        let status = match status_str.to_lowercase().as_str() {
            "pending" => TaskStatus::Pending,
            "in_progress" | "inprogress" => TaskStatus::InProgress,
            "completed" => TaskStatus::Completed,
            "cancelled" => TaskStatus::Cancelled,
            _ => return Err(AppError::BadRequest("无效的任务状态".to_string())),
        };
        service.list_tasks_by_status(status).await?
    } else {
        // 获取所有任务
        service.list_tasks().await?
    };

    Ok(Json(tasks))
}

/// 获取任务详情
/// GET /api/tasks/:id
pub async fn get_task(
    State((db, _config)): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<TaskInfo>, AppError> {
    let service = TaskService::new(db);
    let task = service.get_task(id).await?;
    Ok(Json(task))
}

/// 更新任务
/// PUT /api/tasks/:id
pub async fn update_task(
    State((db, _config)): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateTaskRequest>,
) -> Result<Json<TaskInfo>, AppError> {
    let service = TaskService::new(db);
    let task = service.update_task(id, request).await?;
    Ok(Json(task))
}

/// 删除任务
/// DELETE /api/tasks/:id
pub async fn delete_task(
    State((db, _config)): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let service = TaskService::new(db);
    service.delete_task(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// 开始任务
/// POST /api/tasks/:id/start
pub async fn start_task(
    State((db, _config)): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<TaskInfo>, AppError> {
    let service = TaskService::new(db);
    let task = service.start_task(id).await?;
    Ok(Json(task))
}

/// 完成任务
/// POST /api/tasks/:id/complete
pub async fn complete_task(
    State((db, _config)): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<TaskInfo>, AppError> {
    let service = TaskService::new(db);
    let task = service.complete_task(id).await?;
    Ok(Json(task))
}

/// 取消任务
/// POST /api/tasks/:id/cancel
pub async fn cancel_task(
    State((db, _config)): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<TaskInfo>, AppError> {
    let service = TaskService::new(db);
    let task = service.cancel_task(id).await?;
    Ok(Json(task))
}

/// 分配任务
/// POST /api/tasks/:id/assign
pub async fn assign_task(
    State((db, _config)): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<AssignTaskRequest>,
) -> Result<Json<TaskInfo>, AppError> {
    let service = TaskService::new(db);
    let task = service.assign_task(id, request.assignee_id).await?;
    Ok(Json(task))
}

/// 更新任务状态
/// PATCH /api/tasks/:id/status
pub async fn update_task_status(
    State((db, _config)): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateTaskStatusRequest>,
) -> Result<Json<TaskInfo>, AppError> {
    let service = TaskService::new(db);
    let task = service.update_task_status(id, request.status).await?;
    Ok(Json(task))
}

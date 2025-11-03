use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Json, IntoResponse},
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
    // TODO: 暂时禁用创建,因为数据类型不匹配
    return Err(AppError::BadRequest("任务创建功能正在维护中".to_string()));
}

/// 获取任务列表
/// GET /api/tasks?project_id=xxx&assignee_id=xxx&status=pending
pub async fn list_tasks(
    State((db, _config)): State<AppState>,
    Extension(user): Extension<User>,
    Query(params): Query<TaskQueryParams>,
) -> Result<axum::response::Response, AppError> {
    // 角色权限过滤
    match user.role.as_str() {
        "platform_admin" => {
            // PlatformAdmin不应查看业务数据,返回空列表
            Ok(axum::response::Json(serde_json::json!([])).into_response())
        }
        "project_manager" => {
            let service = TaskService::new(db.clone());
            // ProjectManager只能看自己公司的任务
            if let Some(company_id) = user.company_id {
                let tasks = if let Some(project_id) = params.project_id {
                    service.list_tasks_by_project(project_id, Some(company_id)).await?
                } else if let Some(status_str) = params.status {
                    let status = match status_str.to_lowercase().as_str() {
                        "pending" => TaskStatus::Pending,
                        "in_progress" | "inprogress" => TaskStatus::InProgress,
                        "completed" => TaskStatus::Completed,
                        "cancelled" => TaskStatus::Cancelled,
                        _ => return Err(AppError::BadRequest("无效的任务状态".to_string())),
                    };
                    service.list_tasks_by_status(status, Some(company_id)).await?
                } else {
                    service.list_tasks_by_company(company_id).await?
                };
                Ok(axum::response::Json(tasks).into_response())
            } else {
                Err(AppError::BadRequest("项目经理必须有company_id".to_string()))
            }
        }
        "task_executor" => {
            // TaskExecutor只能看分配给自己的任务
            let user_id_str = user.id.to_string();
            
            let rows = sqlx::query!(
                "SELECT id, title, description, status, priority, 
                        project_id, assigned_to, created_by,
                        due_date, estimated_hours, actual_hours,
                        created_at, updated_at
                 FROM tasks WHERE assigned_to = ?",
                user_id_str
            )
            .fetch_all(&db.pool)
            .await
            .map_err(|e| AppError::DatabaseQuery(e.to_string()))?;
            
            // 直接返回JSON数组
            let tasks: Vec<_> = rows.into_iter().map(|row| {
                serde_json::json!({
                    "id": row.id,
                    "title": row.title,
                    "description": row.description,
                    "status": row.status,
                    "priority": row.priority,
                    "project_id": row.project_id,
                    "project_name": null,
                    "assigned_to": row.assigned_to,
                    "assigned_to_name": null,
                    "created_by": row.created_by,
                    "created_by_name": "",
                    "due_date": row.due_date,
                    "estimated_hours": row.estimated_hours,
                    "actual_hours": row.actual_hours,
                    "created_at": row.created_at,
                    "updated_at": row.updated_at,
                    "completed_at": null,
                })
            }).collect();
            
            Ok(axum::response::Json(tasks).into_response())
        }
        _ => Err(AppError::BadRequest("未知角色".to_string())),
    }
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

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::database::Database;
use crate::errors::AppError;
use crate::models::{CreateProjectRequest, ProjectInfo, ProjectStatus, UpdateProjectRequest};
use crate::services::project::ProjectService;
use crate::Config;

type AppState = (Database, Config);

/// 项目列表查询参数
#[derive(Debug, Deserialize)]
pub struct ProjectQueryParams {
    /// 按项目经理ID筛选
    pub manager_id: Option<Uuid>,
    /// 按状态筛选
    pub status: Option<String>,
}

/// 创建项目
/// POST /api/projects
pub async fn create_project(
    State((db, _config)): State<AppState>,
    Json(request): Json<CreateProjectRequest>,
) -> Result<Json<ProjectInfo>, AppError> {
    let service = ProjectService::new(db);
    let project = service.create_project(request).await?;
    Ok(Json(project))
}

/// 获取项目列表
/// GET /api/projects?manager_id=xxx&status=active
pub async fn list_projects(
    State((db, _config)): State<AppState>,
    Query(params): Query<ProjectQueryParams>,
) -> Result<Json<Vec<ProjectInfo>>, AppError> {
    let service = ProjectService::new(db);

    let projects = if let Some(manager_id) = params.manager_id {
        // 按项目经理筛选
        service.list_projects_by_manager(manager_id).await?
    } else if let Some(status_str) = params.status {
        // 按状态筛选
        let status = match status_str.to_lowercase().as_str() {
            "planning" => ProjectStatus::Planning,
            "active" => ProjectStatus::Active,
            "on_hold" | "onhold" => ProjectStatus::OnHold,
            "completed" => ProjectStatus::Completed,
            "cancelled" => ProjectStatus::Cancelled,
            _ => return Err(AppError::BadRequest("无效的项目状态".to_string())),
        };
        service.list_projects_by_status(status).await?
    } else {
        // 获取所有项目
        service.list_projects().await?
    };

    Ok(Json(projects))
}

/// 获取项目详情
/// GET /api/projects/:id
pub async fn get_project(
    State((db, _config)): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ProjectInfo>, AppError> {
    let service = ProjectService::new(db);
    let project = service.get_project(id).await?;
    Ok(Json(project))
}

/// 更新项目
/// PUT /api/projects/:id
pub async fn update_project(
    State((db, _config)): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateProjectRequest>,
) -> Result<Json<ProjectInfo>, AppError> {
    let service = ProjectService::new(db);
    let project = service.update_project(id, request).await?;
    Ok(Json(project))
}

/// 删除项目
/// DELETE /api/projects/:id
pub async fn delete_project(
    State((db, _config)): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let service = ProjectService::new(db);
    service.delete_project(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// 开始项目
/// POST /api/projects/:id/start
pub async fn start_project(
    State((db, _config)): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ProjectInfo>, AppError> {
    let service = ProjectService::new(db);
    let project = service.start_project(id).await?;
    Ok(Json(project))
}

/// 暂停项目
/// POST /api/projects/:id/hold
pub async fn hold_project(
    State((db, _config)): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ProjectInfo>, AppError> {
    let service = ProjectService::new(db);
    let project = service.hold_project(id).await?;
    Ok(Json(project))
}

/// 完成项目
/// POST /api/projects/:id/complete
pub async fn complete_project(
    State((db, _config)): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ProjectInfo>, AppError> {
    let service = ProjectService::new(db);
    let project = service.complete_project(id).await?;
    Ok(Json(project))
}

/// 取消项目
/// POST /api/projects/:id/cancel
pub async fn cancel_project(
    State((db, _config)): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ProjectInfo>, AppError> {
    let service = ProjectService::new(db);
    let project = service.cancel_project(id).await?;
    Ok(Json(project))
}

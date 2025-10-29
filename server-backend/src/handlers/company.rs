use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::{
    database::Database,
    errors::AppError,
    models::UserRole,
    middleware::auth::AuthContext,
    services::company::{CompanyService, CreateCompanyRequest, UpdateCompanyRequest},
    Config,
};

type AppState = (Database, Config);

/// 查询参数:是否仅返回活跃公司
#[derive(Debug, Deserialize)]
pub struct ListCompaniesQuery {
    #[serde(default)]
    active_only: bool,
}

/// 创建新公司
/// POST /api/companies
pub async fn create_company(
    State((database, _config)): State<AppState>,
    auth_context: AuthContext,
    Json(request): Json<CreateCompanyRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = CompanyService::new(database);
    
    let user_role: UserRole = auth_context.claims.role.parse()
        .map_err(|_| AppError::BadRequest("无效的用户角色".to_string()))?;
    
    let company = service.create_company(request, user_role).await?;
    
    Ok((StatusCode::CREATED, Json(company)))
}

/// 获取所有公司列表(仅SystemAdmin)
/// GET /api/companies?active_only=true
pub async fn list_companies(
    State((database, _config)): State<AppState>,
    auth_context: AuthContext,
    Query(query): Query<ListCompaniesQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = CompanyService::new(database);
    
    let user_role: UserRole = auth_context.claims.role.parse()
        .map_err(|_| AppError::BadRequest("无效的用户角色".to_string()))?;
    
    let companies = service.list_companies(user_role, query.active_only).await?;
    
    Ok(Json(companies))
}

/// 获取公司详情
/// GET /api/companies/:id
pub async fn get_company(
    State((database, _config)): State<AppState>,
    auth_context: AuthContext,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let service = CompanyService::new(database);
    
    let user_role: UserRole = auth_context.claims.role.parse()
        .map_err(|_| AppError::BadRequest("无效的用户角色".to_string()))?;
    
    // 从用户信息获取company_id
    let user_company_id = auth_context.user.company_id;
    
    let company = service.get_company(id, user_role, user_company_id).await?;
    
    Ok(Json(company))
}

/// 更新公司信息(仅SystemAdmin)
/// PUT /api/companies/:id
pub async fn update_company(
    State((database, _config)): State<AppState>,
    auth_context: AuthContext,
    Path(id): Path<i64>,
    Json(request): Json<UpdateCompanyRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = CompanyService::new(database);
    
    let user_role: UserRole = auth_context.claims.role.parse()
        .map_err(|_| AppError::BadRequest("无效的用户角色".to_string()))?;
    
    let company = service.update_company(id, request, user_role).await?;
    
    Ok(Json(company))
}

/// 删除公司(仅SystemAdmin)
/// DELETE /api/companies/:id
pub async fn delete_company(
    State((database, _config)): State<AppState>,
    auth_context: AuthContext,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let service = CompanyService::new(database);
    
    let user_role: UserRole = auth_context.claims.role.parse()
        .map_err(|_| AppError::BadRequest("无效的用户角色".to_string()))?;
    
    service.delete_company(id, user_role).await?;
    
    Ok(StatusCode::NO_CONTENT)
}

/// 切换公司状态(激活/停用)(仅SystemAdmin)
/// POST /api/companies/:id/toggle-status
pub async fn toggle_company_status(
    State((database, _config)): State<AppState>,
    auth_context: AuthContext,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let service = CompanyService::new(database);
    
    let user_role: UserRole = auth_context.claims.role.parse()
        .map_err(|_| AppError::BadRequest("无效的用户角色".to_string()))?;
    
    let company = service.toggle_company_status(id, user_role).await?;
    
    Ok(Json(company))
}

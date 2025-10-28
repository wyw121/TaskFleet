use axum::{
    extract::{Path, Query, State},
    response::Json as ResponseJson,
    Json,
};
use serde::Deserialize;

use crate::{
    errors::AppError,
    middleware::auth::AuthContext,
    models::{ApiResponse, CreateUserRequest, UpdateUserRequest, UserInfo},
    services::user::UserService,
    Config, Database,
};

type AppState = (Database, Config);

#[derive(Deserialize)]
pub struct ListUsersQuery {
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub role: Option<String>,
}

pub async fn list_users(
    State((database, _config)): State<AppState>,
    auth_context: AuthContext,
    Query(_query): Query<ListUsersQuery>,
) -> Result<ResponseJson<ApiResponse<Vec<UserInfo>>>, AppError> {
    let user_service = UserService::new(database);

    let users = user_service
        .list_users(&auth_context.user)
        .await?;

    Ok(ResponseJson(ApiResponse::success(users)))
}

pub async fn create_user(
    State((database, _config)): State<AppState>,
    auth_context: AuthContext,
    Json(request): Json<CreateUserRequest>,
) -> Result<ResponseJson<ApiResponse<UserInfo>>, AppError> {
    tracing::info!("创建用户请求: {:?}", request);
    tracing::info!("请求用户: {:?}", auth_context.user);

    let user_service = UserService::new(database);
    let user = user_service.create_user(request, &auth_context.user).await?;

    tracing::info!("用户创建成功: {:?}", user);
    Ok(ResponseJson(ApiResponse::success(user)))
}

pub async fn get_user(
    State((database, _config)): State<AppState>,
    auth_context: AuthContext,
    Path(user_id): Path<i64>,
) -> Result<ResponseJson<ApiResponse<UserInfo>>, AppError> {
    let user_service = UserService::new(database);
    let user = user_service.get_user(user_id, &auth_context.user).await?;

    Ok(ResponseJson(ApiResponse::success(user)))
}

pub async fn update_user(
    State((database, _config)): State<AppState>,
    auth_context: AuthContext,
    Path(user_id): Path<i64>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<ResponseJson<ApiResponse<UserInfo>>, AppError> {
    let user_service = UserService::new(database);
    let user = user_service
        .update_user(user_id, request, &auth_context.user)
        .await?;

    Ok(ResponseJson(ApiResponse::success(user)))
}

pub async fn delete_user(
    State((database, _config)): State<AppState>,
    auth_context: AuthContext,
    Path(user_id): Path<i64>,
) -> Result<ResponseJson<ApiResponse<()>>, AppError> {
    let user_service = UserService::new(database);
    user_service.delete_user(user_id, &auth_context.user).await?;

    Ok(ResponseJson(ApiResponse::success(())))
}

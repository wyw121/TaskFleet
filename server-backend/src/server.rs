use axum::{
    http::Method,
    middleware,
    routing::{delete, get, post, put},
    Extension, Router,
};
use std::path::PathBuf;
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, services::ServeDir,
    trace::TraceLayer,
};

use crate::{handlers, Config, Database};

pub async fn create_app(database: Database, config: Config) -> Router {
    // 创建事件广播器
    let event_broadcaster = handlers::websocket::create_event_broadcaster();
    
    // 创建CORS中间件
    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
            Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::HeaderName::from_static("authorization"),
            axum::http::HeaderName::from_static("content-type"),
            axum::http::HeaderName::from_static("x-requested-with"),
        ])
        .allow_credentials(false);

    // 静态文件服务配置
    let static_files_service = {
        let static_dir = PathBuf::from(&config.static_dir);
        if static_dir.exists() {
            tracing::info!(
                "📁 静态文件目录: {:?}",
                static_dir.canonicalize().unwrap_or(static_dir.clone())
            );
            ServeDir::new(&config.static_dir)
                .precompressed_gzip()
                .precompressed_br()
                .append_index_html_on_directories(true)
        } else {
            tracing::warn!("⚠️  静态文件目录不存在: {:?}", static_dir);
            tracing::info!("💡 请确保运行 'npm run build' 构建前端");
            ServeDir::new(".")
        }
    };

    // 公开路由（不需要认证）
    let public_routes = Router::new()
        .route("/health", get(handlers::health::health_check))
        .route("/api/v1/auth/login", post(handlers::auth::login))
        .route("/api/v1/auth/register", post(handlers::auth::register))
        .route("/api/v1/auth/logout", post(handlers::auth::logout))
        .route("/docs", get(handlers::docs::api_docs))
        .with_state((database.clone(), config.clone()));

    // 受保护路由（需要认证）
    let protected_routes = Router::new()
        .route("/api/v1/auth/me", get(handlers::auth::get_current_user))
        .route("/api/v1/auth/refresh", post(handlers::auth::refresh_token))
        
        // 用户管理
        .route("/api/v1/users", get(handlers::users::list_users))
        .route("/api/v1/users", post(handlers::users::create_user))
        .route("/api/v1/users/:id", get(handlers::users::get_user))
        .route("/api/v1/users/:id", put(handlers::users::update_user))
        .route("/api/v1/users/:id", delete(handlers::users::delete_user))
        
        // 任务管理 API
        .route("/api/v1/tasks", get(handlers::tasks::list_tasks))
        .route("/api/v1/tasks", post(handlers::tasks::create_task))
        .route("/api/v1/tasks/:id", get(handlers::tasks::get_task))
        .route("/api/v1/tasks/:id", put(handlers::tasks::update_task))
        .route("/api/v1/tasks/:id", delete(handlers::tasks::delete_task))
        .route("/api/v1/tasks/:id/start", post(handlers::tasks::start_task))
        .route("/api/v1/tasks/:id/complete", post(handlers::tasks::complete_task))
        .route("/api/v1/tasks/:id/cancel", post(handlers::tasks::cancel_task))
        .route("/api/v1/tasks/:id/assign", post(handlers::tasks::assign_task))
        
        // 项目管理 API
        .route("/api/v1/projects", get(handlers::projects::list_projects))
        .route("/api/v1/projects", post(handlers::projects::create_project))
        .route("/api/v1/projects/:id", get(handlers::projects::get_project))
        .route("/api/v1/projects/:id", put(handlers::projects::update_project))
        .route("/api/v1/projects/:id", delete(handlers::projects::delete_project))
        .route("/api/v1/projects/:id/start", post(handlers::projects::start_project))
        .route("/api/v1/projects/:id/hold", post(handlers::projects::hold_project))
        .route("/api/v1/projects/:id/complete", post(handlers::projects::complete_project))
        .route("/api/v1/projects/:id/cancel", post(handlers::projects::cancel_project))
        
        // 数据统计 API
        .route("/api/v1/statistics/tasks", get(handlers::statistics::get_task_statistics))
        .route("/api/v1/statistics/projects", get(handlers::statistics::get_project_statistics))
        .route("/api/v1/statistics/users/workload", get(handlers::statistics::get_all_users_workload))
        .route("/api/v1/statistics/users/:user_id/workload", get(handlers::statistics::get_user_workload))
        .route("/api/v1/statistics/projects/:project_id/progress", get(handlers::statistics::get_project_progress))
        
        // WebSocket实时通信
        .route("/ws/task-updates", get(handlers::websocket::task_updates_websocket))
        
        .layer(middleware::from_fn_with_state(
            (database.clone(), config.clone()),
            crate::middleware::auth::AuthLayer::middleware,
        ))
        .layer(Extension(event_broadcaster))
        .with_state((database, config));

    // 合并路由
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        // 静态文件服务 (优先级最低，放在最后)
        .fallback_service(static_files_service)
        // 全局中间件
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(cors)
}

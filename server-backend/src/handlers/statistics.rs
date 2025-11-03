use axum::{
    extract::State,
    response::Json,
    Extension,
};
use crate::database::Database;
use crate::errors::AppError;
use crate::models::User;
use crate::services::statistics::{ProjectStatistics, TaskStatistics};
use crate::Config;

type AppState = (Database, Config);

/// 获取任务统计 (支持角色权限和多租户隔离)
/// GET /api/v1/statistics/tasks
pub async fn get_task_statistics(
    State((db, _config)): State<AppState>,
    Extension(user): Extension<User>,
) -> Result<Json<TaskStatistics>, AppError> {
    // 角色权限过滤
    let (total, pending, in_progress, completed, cancelled) = match user.role.as_str() {
        "platform_admin" => {
            // PlatformAdmin不查看业务数据
            (0, 0, 0, 0, 0)
        }
        "project_manager" => {
            // ProjectManager查看本公司数据
            if let Some(company_id) = user.company_id {
                let total = sqlx::query_as::<_, (i64,)>(
                    "SELECT COUNT(*) FROM tasks t 
                     JOIN projects p ON t.project_id = p.id 
                     WHERE p.company_id = ?")
                    .bind(company_id)
                    .fetch_one(&db.pool)
                    .await
                    .map(|r| r.0)
                    .unwrap_or(0);

                let pending = sqlx::query_as::<_, (i64,)>(
                    "SELECT COUNT(*) FROM tasks t 
                     JOIN projects p ON t.project_id = p.id 
                     WHERE p.company_id = ? AND t.status = 'pending'")
                    .bind(company_id)
                    .fetch_one(&db.pool)
                    .await
                    .map(|r| r.0)
                    .unwrap_or(0);

                let in_progress = sqlx::query_as::<_, (i64,)>(
                    "SELECT COUNT(*) FROM tasks t 
                     JOIN projects p ON t.project_id = p.id 
                     WHERE p.company_id = ? AND t.status = 'in_progress'")
                    .bind(company_id)
                    .fetch_one(&db.pool)
                    .await
                    .map(|r| r.0)
                    .unwrap_or(0);

                let completed = sqlx::query_as::<_, (i64,)>(
                    "SELECT COUNT(*) FROM tasks t 
                     JOIN projects p ON t.project_id = p.id 
                     WHERE p.company_id = ? AND t.status = 'completed'")
                    .bind(company_id)
                    .fetch_one(&db.pool)
                    .await
                    .map(|r| r.0)
                    .unwrap_or(0);

                let cancelled = sqlx::query_as::<_, (i64,)>(
                    "SELECT COUNT(*) FROM tasks t 
                     JOIN projects p ON t.project_id = p.id 
                     WHERE p.company_id = ? AND t.status = 'cancelled'")
                    .bind(company_id)
                    .fetch_one(&db.pool)
                    .await
                    .map(|r| r.0)
                    .unwrap_or(0);

                (total, pending, in_progress, completed, cancelled)
            } else {
                return Err(AppError::BadRequest("项目经理必须有company_id".to_string()));
            }
        }
        "task_executor" => {
            // TaskExecutor只看分配给自己的任务
            let user_id = user.id.to_string();
            let total = sqlx::query_as::<_, (i64,)>(
                "SELECT COUNT(*) FROM tasks WHERE assigned_to = ?")
                .bind(&user_id)
                .fetch_one(&db.pool)
                .await
                .map(|r| r.0)
                .unwrap_or(0);

            let pending = sqlx::query_as::<_, (i64,)>(
                "SELECT COUNT(*) FROM tasks WHERE assigned_to = ? AND status = 'pending'")
                .bind(&user_id)
                .fetch_one(&db.pool)
                .await
                .map(|r| r.0)
                .unwrap_or(0);

            let in_progress = sqlx::query_as::<_, (i64,)>(
                "SELECT COUNT(*) FROM tasks WHERE assigned_to = ? AND status = 'in_progress'")
                .bind(&user_id)
                .fetch_one(&db.pool)
                .await
                .map(|r| r.0)
                .unwrap_or(0);

            let completed = sqlx::query_as::<_, (i64,)>(
                "SELECT COUNT(*) FROM tasks WHERE assigned_to = ? AND status = 'completed'")
                .bind(&user_id)
                .fetch_one(&db.pool)
                .await
                .map(|r| r.0)
                .unwrap_or(0);

            let cancelled = sqlx::query_as::<_, (i64,)>(
                "SELECT COUNT(*) FROM tasks WHERE assigned_to = ? AND status = 'cancelled'")
                .bind(&user_id)
                .fetch_one(&db.pool)
                .await
                .map(|r| r.0)
                .unwrap_or(0);

            (total, pending, in_progress, completed, cancelled)
        }
        _ => return Err(AppError::BadRequest("未知角色".to_string())),
    };

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

/// 获取项目统计 (支持角色权限和多租户隔离)
/// GET /api/v1/statistics/projects
pub async fn get_project_statistics(
    State((db, _config)): State<AppState>,
    Extension(user): Extension<User>,
) -> Result<Json<ProjectStatistics>, AppError> {
    // 角色权限过滤
    let (total, planning, active, on_hold, completed, cancelled) = match user.role.as_str() {
        "platform_admin" => {
            // PlatformAdmin不查看业务数据
            (0, 0, 0, 0, 0, 0)
        }
        "project_manager" => {
            // ProjectManager查看本公司项目
            if let Some(company_id) = user.company_id {
                let total = sqlx::query_as::<_, (i64,)>(
                    "SELECT COUNT(*) FROM projects WHERE company_id = ?")
                    .bind(company_id)
                    .fetch_one(&db.pool)
                    .await
                    .map(|r| r.0)
                    .unwrap_or(0);

                let planning = sqlx::query_as::<_, (i64,)>(
                    "SELECT COUNT(*) FROM projects WHERE company_id = ? AND status = 'planning'")
                    .bind(company_id)
                    .fetch_one(&db.pool)
                    .await
                    .map(|r| r.0)
                    .unwrap_or(0);

                let active = sqlx::query_as::<_, (i64,)>(
                    "SELECT COUNT(*) FROM projects WHERE company_id = ? AND status = 'active'")
                    .bind(company_id)
                    .fetch_one(&db.pool)
                    .await
                    .map(|r| r.0)
                    .unwrap_or(0);

                let on_hold = sqlx::query_as::<_, (i64,)>(
                    "SELECT COUNT(*) FROM projects WHERE company_id = ? AND status = 'on_hold'")
                    .bind(company_id)
                    .fetch_one(&db.pool)
                    .await
                    .map(|r| r.0)
                    .unwrap_or(0);

                let completed = sqlx::query_as::<_, (i64,)>(
                    "SELECT COUNT(*) FROM projects WHERE company_id = ? AND status = 'completed'")
                    .bind(company_id)
                    .fetch_one(&db.pool)
                    .await
                    .map(|r| r.0)
                    .unwrap_or(0);

                let cancelled = sqlx::query_as::<_, (i64,)>(
                    "SELECT COUNT(*) FROM projects WHERE company_id = ? AND status = 'cancelled'")
                    .bind(company_id)
                    .fetch_one(&db.pool)
                    .await
                    .map(|r| r.0)
                    .unwrap_or(0);

                (total, planning, active, on_hold, completed, cancelled)
            } else {
                return Err(AppError::BadRequest("项目经理必须有company_id".to_string()));
            }
        }
        "task_executor" => {
            // TaskExecutor不直接查看项目统计(因为他们没有项目管理权限)
            // 但如果需要查看参与的项目,可以通过JOIN tasks表实现
            // 目前返回0
            (0, 0, 0, 0, 0, 0)
        }
        _ => return Err(AppError::BadRequest("未知角色".to_string())),
    };

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


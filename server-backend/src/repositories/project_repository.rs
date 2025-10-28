use crate::database::Database;
use crate::errors::AppError;
use crate::models::{Project, ProjectStatus, CreateProjectRequest, UpdateProjectRequest};
use chrono::Utc;
use uuid::Uuid;

/// 项目数据仓库
pub struct ProjectRepository {
    db: Database,
}

impl ProjectRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// 创建新项目
    pub async fn create(&self, request: CreateProjectRequest) -> Result<Project, AppError> {
        let project = Project {
            id: Uuid::new_v4(),
            name: request.name,
            description: request.description,
            status: request.status.unwrap_or(ProjectStatus::Planning),
            manager_id: request.manager_id,
            start_date: request.start_date,
            end_date: request.end_date,
            budget: request.budget,
            actual_cost: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        sqlx::query(
            r#"
            INSERT INTO projects (
                id, name, description, status, manager_id, 
                start_date, end_date, budget, actual_cost,
                created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&project.id)
        .bind(&project.name)
        .bind(&project.description)
        .bind(&project.status)
        .bind(&project.manager_id)
        .bind(&project.start_date)
        .bind(&project.end_date)
        .bind(&project.budget)
        .bind(&project.actual_cost)
        .bind(&project.created_at)
        .bind(&project.updated_at)
        .execute(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(project)
    }

    /// 根据ID查询项目
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Project>, AppError> {
        let project = sqlx::query_as::<_, Project>(
            "SELECT * FROM projects WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(project)
    }

    /// 更新项目
    pub async fn update(&self, id: Uuid, request: UpdateProjectRequest) -> Result<Project, AppError> {
        let mut project = self.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("项目不存在".to_string()))?;

        if let Some(name) = request.name {
            project.name = name;
        }
        if let Some(description) = request.description {
            project.description = Some(description);
        }
        if let Some(status) = request.status {
            project.status = status;
        }
        if let Some(manager_id) = request.manager_id {
            project.manager_id = manager_id;
        }
        if let Some(start_date) = request.start_date {
            project.start_date = Some(start_date);
        }
        if let Some(end_date) = request.end_date {
            project.end_date = Some(end_date);
        }
        if let Some(budget) = request.budget {
            project.budget = Some(budget);
        }
        if let Some(actual_cost) = request.actual_cost {
            project.actual_cost = Some(actual_cost);
        }

        project.updated_at = Utc::now();

        sqlx::query(
            r#"
            UPDATE projects 
            SET name = ?, description = ?, status = ?, manager_id = ?,
                start_date = ?, end_date = ?, budget = ?, actual_cost = ?,
                updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&project.name)
        .bind(&project.description)
        .bind(&project.status)
        .bind(&project.manager_id)
        .bind(&project.start_date)
        .bind(&project.end_date)
        .bind(&project.budget)
        .bind(&project.actual_cost)
        .bind(&project.updated_at)
        .bind(&project.id)
        .execute(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(project)
    }

    /// 删除项目
    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM projects WHERE id = ?")
            .bind(id)
            .execute(&self.db.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("项目不存在".to_string()));
        }

        Ok(())
    }

    /// 获取所有项目列表
    pub async fn list_all(&self) -> Result<Vec<Project>, AppError> {
        let projects = sqlx::query_as::<_, Project>(
            "SELECT * FROM projects ORDER BY created_at DESC"
        )
        .fetch_all(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(projects)
    }

    /// 根据项目经理获取项目列表
    pub async fn find_by_manager(&self, manager_id: Uuid) -> Result<Vec<Project>, AppError> {
        let projects = sqlx::query_as::<_, Project>(
            "SELECT * FROM projects WHERE manager_id = ? ORDER BY created_at DESC"
        )
        .bind(manager_id)
        .fetch_all(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(projects)
    }

    /// 根据状态获取项目列表
    pub async fn find_by_status(&self, status: ProjectStatus) -> Result<Vec<Project>, AppError> {
        let projects = sqlx::query_as::<_, Project>(
            "SELECT * FROM projects WHERE status = ? ORDER BY created_at DESC"
        )
        .bind(status)
        .fetch_all(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(projects)
    }

    /// 获取项目任务数量统计
    pub async fn get_task_count(&self, project_id: Uuid) -> Result<i64, AppError> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM tasks WHERE project_id = ?"
        )
        .bind(project_id)
        .fetch_one(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.0)
    }

    /// 获取项目已完成任务数量
    pub async fn get_completed_task_count(&self, project_id: Uuid) -> Result<i64, AppError> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM tasks WHERE project_id = ? AND status = 'completed'"
        )
        .bind(project_id)
        .fetch_one(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.0)
    }
}

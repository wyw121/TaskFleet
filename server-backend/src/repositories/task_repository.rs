use crate::database::Database;
use crate::errors::AppError;
use crate::models::{Task, TaskStatus, CreateTaskRequest, UpdateTaskRequest};
use chrono::Utc;
use uuid::Uuid;

/// 任务数据仓库
pub struct TaskRepository {
    db: Database,
}

impl TaskRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// 创建新任务
    pub async fn create(&self, request: CreateTaskRequest, created_by: Uuid) -> Result<Task, AppError> {
        let task = Task {
            id: Uuid::new_v4(),
            title: request.title.clone(),
            description: request.description.clone(),
            priority: request.priority.clone(),
            project_id: request.project_id,
            assigned_to: request.assigned_to,
            created_by,
            status: TaskStatus::Pending,  // 新任务总是待处理状态
            due_date: request.due_date,
            estimated_hours: request.estimated_hours,
            actual_hours: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            completed_at: None,
        };

        sqlx::query(
            r#"
            INSERT INTO tasks (
                id, title, description, status, priority, project_id, 
                assigned_to, created_by, due_date, estimated_hours, actual_hours,
                created_at, updated_at, completed_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&task.id)
        .bind(&task.title)
        .bind(&task.description)
        .bind(&task.status)
        .bind(&task.priority)
        .bind(&task.project_id)
        .bind(&task.assigned_to)
        .bind(&task.created_by)
        .bind(&task.due_date)
        .bind(&task.estimated_hours)
        .bind(&task.actual_hours)
        .bind(&task.created_at)
        .bind(&task.updated_at)
        .bind(&task.completed_at)
        .execute(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(task)
    }

    /// 根据ID查询任务
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Task>, AppError> {
        let task = sqlx::query_as::<_, Task>(
            "SELECT * FROM tasks WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(task)
    }

    /// 更新任务
    pub async fn update(&self, id: Uuid, request: UpdateTaskRequest) -> Result<Task, AppError> {
        let mut task = self.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("任务不存在".to_string()))?;

        if let Some(title) = request.title {
            task.title = title;
        }
        if let Some(description) = request.description {
            task.description = description;
        }
        if let Some(status) = request.status {
            task.status = status;
            // 如果任务完成,记录完成时间
            if task.status == TaskStatus::Completed && task.completed_at.is_none() {
                task.completed_at = Some(Utc::now());
            }
        }
        if let Some(priority) = request.priority {
            task.priority = priority;
        }
        if let Some(assigned_to) = request.assigned_to {
            task.assigned_to = Some(assigned_to);
        }
        if let Some(due_date) = request.due_date {
            task.due_date = Some(due_date);
        }
        if let Some(estimated_hours) = request.estimated_hours {
            task.estimated_hours = Some(estimated_hours);
        }

        task.updated_at = Utc::now();

        sqlx::query(
            r#"
            UPDATE tasks 
            SET title = ?, description = ?, status = ?, priority = ?, 
                assigned_to = ?, due_date = ?, estimated_hours = ?, 
                actual_hours = ?, updated_at = ?, completed_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&task.title)
        .bind(&task.description)
        .bind(&task.status)
        .bind(&task.priority)
        .bind(&task.assigned_to)
        .bind(&task.due_date)
        .bind(&task.estimated_hours)
        .bind(&task.actual_hours)
        .bind(&task.updated_at)
        .bind(&task.completed_at)
        .bind(&task.id)
        .execute(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(task)
    }

    /// 删除任务
    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM tasks WHERE id = ?")
            .bind(id)
            .execute(&self.db.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("任务不存在".to_string()));
        }

        Ok(())
    }

    /// 获取所有任务列表
    pub async fn list_all(&self) -> Result<Vec<Task>, AppError> {
        let tasks = sqlx::query_as::<_, Task>(
            "SELECT * FROM tasks ORDER BY created_at DESC"
        )
        .fetch_all(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(tasks)
    }

    /// 根据项目ID获取任务列表
    pub async fn find_by_project(&self, project_id: Uuid) -> Result<Vec<Task>, AppError> {
        let tasks = sqlx::query_as::<_, Task>(
            "SELECT * FROM tasks WHERE project_id = ? ORDER BY created_at DESC"
        )
        .bind(project_id)
        .fetch_all(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(tasks)
    }

    /// 根据分配人获取任务列表
    pub async fn find_by_assignee(&self, user_id: Uuid) -> Result<Vec<Task>, AppError> {
        let tasks = sqlx::query_as::<_, Task>(
            "SELECT * FROM tasks WHERE assigned_to = ? ORDER BY due_date ASC, priority DESC"
        )
        .bind(user_id)
        .fetch_all(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(tasks)
    }

    /// 根据状态获取任务列表
    pub async fn find_by_status(&self, status: TaskStatus) -> Result<Vec<Task>, AppError> {
        let tasks = sqlx::query_as::<_, Task>(
            "SELECT * FROM tasks WHERE status = ? ORDER BY created_at DESC"
        )
        .bind(status)
        .fetch_all(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(tasks)
    }

    /// 更新任务状态
    pub async fn update_status(&self, id: Uuid, status: TaskStatus) -> Result<Task, AppError> {
        let mut task = self.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("任务不存在".to_string()))?;

        task.status = status;
        task.updated_at = Utc::now();

        // 如果任务完成,记录完成时间
        if task.status == TaskStatus::Completed && task.completed_at.is_none() {
            task.completed_at = Some(Utc::now());
        }

        sqlx::query(
            "UPDATE tasks SET status = ?, updated_at = ?, completed_at = ? WHERE id = ?"
        )
        .bind(&task.status)
        .bind(&task.updated_at)
        .bind(&task.completed_at)
        .bind(&task.id)
        .execute(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(task)
    }

    /// 分配任务给员工
    pub async fn assign_task(&self, id: Uuid, assignee_id: Uuid) -> Result<Task, AppError> {
        let mut task = self.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("任务不存在".to_string()))?;

        task.assigned_to = Some(assignee_id);
        task.updated_at = Utc::now();

        sqlx::query(
            "UPDATE tasks SET assigned_to = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&task.assigned_to)
        .bind(&task.updated_at)
        .bind(&task.id)
        .execute(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(task)
    }
}

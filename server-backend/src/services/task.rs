use crate::database::Database;
use crate::errors::AppError;
use crate::models::{TaskInfo, TaskStatus, CreateTaskRequest, UpdateTaskRequest};
use crate::repositories::TaskRepository;
use uuid::Uuid;
use validator::Validate;

/// 任务管理服务
pub struct TaskService {
    task_repo: TaskRepository,
}

impl TaskService {
    pub fn new(db: Database) -> Self {
        Self {
            task_repo: TaskRepository::new(db),
        }
    }

    /// 创建新任务
    pub async fn create_task(&self, request: CreateTaskRequest, created_by: Uuid, company_id: Option<i64>) -> Result<TaskInfo, AppError> {
        // 验证请求参数
        request.validate()
            .map_err(|e| AppError::BadRequest(format!("参数验证失败: {}", e)))?;

        // 创建任务
        let task = self.task_repo.create(request, created_by, company_id).await?;

        Ok(TaskInfo::from(task))
    }

    /// 获取任务详情
    pub async fn get_task(&self, id: Uuid) -> Result<TaskInfo, AppError> {
        let task = self.task_repo.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("任务不存在".to_string()))?;

        Ok(TaskInfo::from(task))
    }

    /// 更新任务
    pub async fn update_task(&self, id: Uuid, request: UpdateTaskRequest) -> Result<TaskInfo, AppError> {
        // 验证请求参数
        request.validate()
            .map_err(|e| AppError::BadRequest(format!("参数验证失败: {}", e)))?;

        // 更新任务
        let task = self.task_repo.update(id, request).await?;

        Ok(TaskInfo::from(task))
    }

    /// 删除任务
    pub async fn delete_task(&self, id: Uuid) -> Result<(), AppError> {
        self.task_repo.delete(id).await
    }

    /// 获取所有任务列表
    pub async fn list_tasks(&self) -> Result<Vec<TaskInfo>, AppError> {
        let tasks = self.task_repo.list_all().await?;
        Ok(tasks.into_iter().map(TaskInfo::from).collect())
    }

    /// 根据公司ID获取任务列表(多租户隔离)
    pub async fn list_tasks_by_company(&self, company_id: i64) -> Result<Vec<TaskInfo>, AppError> {
        let tasks = self.task_repo.list_by_company_id(company_id).await?;
        Ok(tasks.into_iter().map(TaskInfo::from).collect())
    }

    /// 获取项目的任务列表(支持company_id过滤)
    pub async fn list_tasks_by_project(&self, project_id: Uuid, company_id: Option<i64>) -> Result<Vec<TaskInfo>, AppError> {
        let tasks = self.task_repo.find_by_project(project_id, company_id).await?;
        Ok(tasks.into_iter().map(TaskInfo::from).collect())
    }

    /// 获取分配给某员工的任务列表(支持company_id过滤)
    pub async fn list_tasks_by_assignee(&self, user_id: Uuid, company_id: Option<i64>) -> Result<Vec<TaskInfo>, AppError> {
        let tasks = self.task_repo.find_by_assignee(user_id, company_id).await?;
        Ok(tasks.into_iter().map(TaskInfo::from).collect())
    }

    /// 获取特定状态的任务列表(支持company_id过滤)
    pub async fn list_tasks_by_status(&self, status: TaskStatus, company_id: Option<i64>) -> Result<Vec<TaskInfo>, AppError> {
        let tasks = self.task_repo.find_by_status(status, company_id).await?;
        Ok(tasks.into_iter().map(TaskInfo::from).collect())
    }

    /// 更新任务状态
    pub async fn update_task_status(&self, id: Uuid, status: TaskStatus) -> Result<TaskInfo, AppError> {
        let task = self.task_repo.update_status(id, status).await?;
        Ok(TaskInfo::from(task))
    }

    /// 开始任务
    pub async fn start_task(&self, id: Uuid) -> Result<TaskInfo, AppError> {
        let task = self.task_repo.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("任务不存在".to_string()))?;

        // 检查任务状态
        if task.status != TaskStatus::Pending {
            return Err(AppError::BadRequest("只有待处理的任务可以开始".to_string()));
        }

        let task = self.task_repo.update_status(id, TaskStatus::InProgress).await?;
        Ok(TaskInfo::from(task))
    }

    /// 完成任务
    pub async fn complete_task(&self, id: Uuid) -> Result<TaskInfo, AppError> {
        let task = self.task_repo.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("任务不存在".to_string()))?;

        // 检查任务状态
        if task.status == TaskStatus::Completed {
            return Err(AppError::BadRequest("任务已经完成".to_string()));
        }
        if task.status == TaskStatus::Cancelled {
            return Err(AppError::BadRequest("已取消的任务无法完成".to_string()));
        }

        let task = self.task_repo.update_status(id, TaskStatus::Completed).await?;
        Ok(TaskInfo::from(task))
    }

    /// 取消任务
    pub async fn cancel_task(&self, id: Uuid) -> Result<TaskInfo, AppError> {
        let task = self.task_repo.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("任务不存在".to_string()))?;

        // 检查任务状态
        if task.status == TaskStatus::Completed {
            return Err(AppError::BadRequest("已完成的任务无法取消".to_string()));
        }
        if task.status == TaskStatus::Cancelled {
            return Err(AppError::BadRequest("任务已经取消".to_string()));
        }

        let task = self.task_repo.update_status(id, TaskStatus::Cancelled).await?;
        Ok(TaskInfo::from(task))
    }

    /// 分配任务
    pub async fn assign_task(&self, id: Uuid, assignee_id: Uuid) -> Result<TaskInfo, AppError> {
        let task = self.task_repo.assign_task(id, assignee_id).await?;
        Ok(TaskInfo::from(task))
    }
}

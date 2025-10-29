use crate::database::Database;
use crate::errors::AppError;
use crate::models::{ProjectInfo, ProjectStatus, CreateProjectRequest, UpdateProjectRequest};
use crate::repositories::ProjectRepository;
use uuid::Uuid;
use validator::Validate;

/// 项目管理服务
pub struct ProjectService {
    project_repo: ProjectRepository,
}

impl ProjectService {
    pub fn new(db: Database) -> Self {
        Self {
            project_repo: ProjectRepository::new(db),
        }
    }

    /// 创建新项目
    pub async fn create_project(&self, request: CreateProjectRequest, company_id: Option<i64>) -> Result<ProjectInfo, AppError> {
        // 验证请求参数
        request.validate()
            .map_err(|e| AppError::BadRequest(format!("参数验证失败: {}", e)))?;

        // 验证日期逻辑
        if let (Some(start), Some(end)) = (request.start_date, request.end_date) {
            if end < start {
                return Err(AppError::BadRequest("结束日期不能早于开始日期".to_string()));
            }
        }

        // 创建项目
        let project = self.project_repo.create(request, company_id).await?;

        Ok(ProjectInfo::from(project))
    }

    /// 获取项目详情（包含统计信息）
    pub async fn get_project(&self, id: Uuid) -> Result<ProjectInfo, AppError> {
        let project = self.project_repo.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("项目不存在".to_string()))?;

        // 获取任务统计
        let task_count = self.project_repo.get_task_count(id).await?;
        let completed_tasks = self.project_repo.get_completed_task_count(id).await?;
        let progress = if task_count > 0 {
            Some((completed_tasks as f64 / task_count as f64) * 100.0)
        } else {
            Some(0.0)
        };

        let mut info = ProjectInfo::from(project);
        info.task_count = Some(task_count);
        info.completed_tasks = Some(completed_tasks);
        info.progress = progress;

        Ok(info)
    }

    /// 更新项目
    pub async fn update_project(&self, id: Uuid, request: UpdateProjectRequest) -> Result<ProjectInfo, AppError> {
        // 验证请求参数
        request.validate()
            .map_err(|e| AppError::BadRequest(format!("参数验证失败: {}", e)))?;

        // 更新项目
        let project = self.project_repo.update(id, request).await?;

        Ok(ProjectInfo::from(project))
    }

    /// 删除项目
    pub async fn delete_project(&self, id: Uuid) -> Result<(), AppError> {
        // 检查项目是否有任务
        let task_count = self.project_repo.get_task_count(id).await?;
        if task_count > 0 {
            return Err(AppError::BadRequest("无法删除包含任务的项目，请先删除所有任务".to_string()));
        }

        self.project_repo.delete(id).await
    }

    /// 获取所有项目列表
    pub async fn list_projects(&self) -> Result<Vec<ProjectInfo>, AppError> {
        let projects = self.project_repo.list_all().await?;
        Ok(projects.into_iter().map(ProjectInfo::from).collect())
    }

    /// 根据公司ID获取项目列表(多租户隔离)
    pub async fn list_projects_by_company(&self, company_id: i64) -> Result<Vec<ProjectInfo>, AppError> {
        let projects = self.project_repo.list_by_company_id(company_id).await?;
        Ok(projects.into_iter().map(ProjectInfo::from).collect())
    }

    /// 获取项目经理的项目列表(支持company_id过滤)
    pub async fn list_projects_by_manager(&self, manager_id: Uuid, company_id: Option<i64>) -> Result<Vec<ProjectInfo>, AppError> {
        let projects = self.project_repo.find_by_manager(manager_id, company_id).await?;
        Ok(projects.into_iter().map(ProjectInfo::from).collect())
    }

    /// 获取特定状态的项目列表(支持company_id过滤)
    pub async fn list_projects_by_status(&self, status: ProjectStatus, company_id: Option<i64>) -> Result<Vec<ProjectInfo>, AppError> {
        let projects = self.project_repo.find_by_status(status, company_id).await?;
        Ok(projects.into_iter().map(ProjectInfo::from).collect())
    }

    /// 开始项目（从Planning到Active）
    pub async fn start_project(&self, id: Uuid) -> Result<ProjectInfo, AppError> {
        let project = self.project_repo.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("项目不存在".to_string()))?;

        if project.status != ProjectStatus::Planning {
            return Err(AppError::BadRequest("只有规划中的项目可以开始".to_string()));
        }

        let update_request = UpdateProjectRequest {
            status: Some(ProjectStatus::Active),
            ..Default::default()
        };

        let project = self.project_repo.update(id, update_request).await?;
        Ok(ProjectInfo::from(project))
    }

    /// 暂停项目
    pub async fn hold_project(&self, id: Uuid) -> Result<ProjectInfo, AppError> {
        let project = self.project_repo.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("项目不存在".to_string()))?;

        if project.status != ProjectStatus::Active {
            return Err(AppError::BadRequest("只有进行中的项目可以暂停".to_string()));
        }

        let update_request = UpdateProjectRequest {
            status: Some(ProjectStatus::OnHold),
            ..Default::default()
        };

        let project = self.project_repo.update(id, update_request).await?;
        Ok(ProjectInfo::from(project))
    }

    /// 完成项目
    pub async fn complete_project(&self, id: Uuid) -> Result<ProjectInfo, AppError> {
        let project = self.project_repo.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("项目不存在".to_string()))?;

        if project.status == ProjectStatus::Completed {
            return Err(AppError::BadRequest("项目已经完成".to_string()));
        }
        if project.status == ProjectStatus::Cancelled {
            return Err(AppError::BadRequest("已取消的项目无法完成".to_string()));
        }

        // 检查所有任务是否完成
        let task_count = self.project_repo.get_task_count(id).await?;
        let completed_tasks = self.project_repo.get_completed_task_count(id).await?;
        
        if task_count > 0 && completed_tasks < task_count {
            return Err(AppError::BadRequest(
                format!("项目还有 {} 个未完成的任务", task_count - completed_tasks)
            ));
        }

        let update_request = UpdateProjectRequest {
            status: Some(ProjectStatus::Completed),
            ..Default::default()
        };

        let project = self.project_repo.update(id, update_request).await?;
        Ok(ProjectInfo::from(project))
    }

    /// 取消项目
    pub async fn cancel_project(&self, id: Uuid) -> Result<ProjectInfo, AppError> {
        let project = self.project_repo.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("项目不存在".to_string()))?;

        if project.status == ProjectStatus::Completed {
            return Err(AppError::BadRequest("已完成的项目无法取消".to_string()));
        }
        if project.status == ProjectStatus::Cancelled {
            return Err(AppError::BadRequest("项目已经取消".to_string()));
        }

        let update_request = UpdateProjectRequest {
            status: Some(ProjectStatus::Cancelled),
            ..Default::default()
        };

        let project = self.project_repo.update(id, update_request).await?;
        Ok(ProjectInfo::from(project))
    }
}

// 为 UpdateProjectRequest 实现 Default trait
impl Default for UpdateProjectRequest {
    fn default() -> Self {
        Self {
            name: None,
            description: None,
            status: None,
            manager_id: None,
            start_date: None,
            end_date: None,
            budget: None,
            actual_cost: None,
        }
    }
}

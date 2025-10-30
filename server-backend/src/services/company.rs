use crate::database::Database;
use crate::errors::AppError;
use crate::models::{Company, CompanyInfo, UserRole};
use crate::repositories::CompanyRepository;
use validator::Validate;
use serde::Deserialize;
use chrono::Utc;

/// 公司管理服务
pub struct CompanyService {
    company_repo: CompanyRepository,
}

/// 创建公司请求
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateCompanyRequest {
    #[validate(length(min = 1, max = 100, message = "公司名称长度必须在1-100个字符之间"))]
    pub name: String,
    
    #[validate(email(message = "请提供有效的电子邮件地址"))]
    pub contact_email: String,
    
    #[validate(range(min = 1, message = "最大员工数必须至少为1"))]
    pub max_employees: i32,
}

/// 更新公司请求
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateCompanyRequest {
    #[validate(length(min = 1, max = 100, message = "公司名称长度必须在1-100个字符之间"))]
    pub name: Option<String>,
    
    #[validate(email(message = "请提供有效的电子邮件地址"))]
    pub contact_email: Option<String>,
    
    #[validate(range(min = 1, message = "最大员工数必须至少为1"))]
    pub max_employees: Option<i32>,
    
    pub is_active: Option<bool>,
}

impl CompanyService {
    pub fn new(db: Database) -> Self {
        Self {
            company_repo: CompanyRepository::new(db),
        }
    }

    /// 创建新公司(仅SystemAdmin)
    pub async fn create_company(
        &self,
        request: CreateCompanyRequest,
        user_role: UserRole,
    ) -> Result<CompanyInfo, AppError> {
        // 权限检查:仅PlatformAdmin可创建公司
        if user_role != UserRole::PlatformAdmin {
            return Err(AppError::Forbidden);
        }

        // 验证请求参数
        request.validate()
            .map_err(|e| AppError::BadRequest(format!("参数验证失败: {}", e)))?;

        // 检查公司名是否已存在
        if let Some(_) = self.company_repo.find_by_name(&request.name)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))? 
        {
            return Err(AppError::BadRequest("公司名称已存在".to_string()));
        }

        // 创建公司对象
        let now = Utc::now();
        let company = Company {
            id: 0,  // SQLite会自动生成
            name: request.name,
            contact_email: Some(request.contact_email),
            contact_phone: None,
            max_employees: request.max_employees,
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        // 创建公司
        let created_company = self.company_repo.create(company)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(CompanyInfo::from(created_company))
    }

    /// 获取公司详情(包含员工数量)
    pub async fn get_company(
        &self,
        id: i64,
        user_role: UserRole,
        user_company_id: Option<i64>,
    ) -> Result<CompanyInfo, AppError> {
        // 权限检查:ProjectManager只能查看自己的公司
        if user_role == UserRole::ProjectManager {
            if user_company_id != Some(id) {
                return Err(AppError::Forbidden);
            }
        }

        let company = self.company_repo.find_by_id(id)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("公司不存在".to_string()))?;

        // 获取员工数量
        let employee_count = self.company_repo.get_employee_count(id)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut info = CompanyInfo::from(company);
        info.current_employees = employee_count;

        Ok(info)
    }

    /// 获取所有公司列表(仅PlatformAdmin)
    pub async fn list_companies(
        &self,
        user_role: UserRole,
        active_only: bool,
    ) -> Result<Vec<CompanyInfo>, AppError> {
        // 权限检查:仅PlatformAdmin可查看所有公司
        if user_role != UserRole::PlatformAdmin {
            return Err(AppError::Forbidden);
        }

        let companies = if active_only {
            self.company_repo.list_active()
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?
        } else {
            self.company_repo.list_all()
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?
        };

        Ok(companies.into_iter().map(CompanyInfo::from).collect())
    }

    /// 更新公司信息(仅SystemAdmin)
    pub async fn update_company(
        &self,
        id: i64,
        request: UpdateCompanyRequest,
        user_role: UserRole,
    ) -> Result<CompanyInfo, AppError> {
        // 权限检查:仅PlatformAdmin可更新公司
        if user_role != UserRole::PlatformAdmin {
            return Err(AppError::Forbidden);
        }

        // 验证请求参数
        request.validate()
            .map_err(|e| AppError::BadRequest(format!("参数验证失败: {}", e)))?;

        // 检查公司是否存在
        let mut company = self.company_repo.find_by_id(id)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("公司不存在".to_string()))?;

        // 如果更新名称,检查是否与其他公司重复
        if let Some(ref new_name) = request.name {
            if new_name != &company.name {
                if let Some(_) = self.company_repo.find_by_name(new_name)
                    .await
                    .map_err(|e| AppError::DatabaseError(e.to_string()))? 
                {
                    return Err(AppError::BadRequest("公司名称已被使用".to_string()));
                }
            }
        }

        // 应用更新
        if let Some(name) = request.name {
            company.name = name;
        }
        if let Some(email) = request.contact_email {
            company.contact_email = Some(email);
        }
        if let Some(max_emp) = request.max_employees {
            company.max_employees = max_emp;
        }
        if let Some(active) = request.is_active {
            company.is_active = active;
        }
        company.updated_at = Utc::now();

        // 执行更新
        self.company_repo.update(company.clone())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(CompanyInfo::from(company))
    }

    /// 删除公司(仅PlatformAdmin)
    pub async fn delete_company(
        &self,
        id: i64,
        user_role: UserRole,
    ) -> Result<(), AppError> {
        // 权限检查:仅PlatformAdmin可删除公司
        if user_role != UserRole::PlatformAdmin {
            return Err(AppError::Forbidden);
        }

        // 检查公司是否存在
        self.company_repo.find_by_id(id)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("公司不存在".to_string()))?;

        // 检查公司是否有员工
        let employee_count = self.company_repo.get_employee_count(id)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if employee_count > 0 {
            return Err(AppError::BadRequest(
                format!("无法删除拥有 {} 名员工的公司,请先转移或删除所有员工", employee_count)
            ));
        }

        // 执行删除
        self.company_repo.delete(id)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// 停用/激活公司(仅PlatformAdmin)
    pub async fn toggle_company_status(
        &self,
        id: i64,
        user_role: UserRole,
    ) -> Result<CompanyInfo, AppError> {
        // 权限检查:仅PlatformAdmin可修改公司状态
        if user_role != UserRole::PlatformAdmin {
            return Err(AppError::Forbidden);
        }

        // 获取公司
        let mut company = self.company_repo.find_by_id(id)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("公司不存在".to_string()))?;

        // 切换状态
        company.is_active = !company.is_active;
        company.updated_at = Utc::now();

        // 更新数据库
        self.company_repo.update(company.clone())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(CompanyInfo::from(company))
    }
}

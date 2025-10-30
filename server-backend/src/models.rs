use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use std::str::FromStr;

// ==================== 数据模型说明 ====================
// 
// TaskFleet核心数据模型包含以下几大类:
//
// 0. Company（公司）模型 - 多租户隔离的核心，所有数据都归属于公司
//    - Company: 公司实体
//    - CreateCompanyRequest/UpdateCompanyRequest: 创建/更新公司的DTO
//    - CompanyInfo: 公司响应信息
//
// 1. User（用户）模型 - 系统用户，包含平台管理员、项目经理和任务执行者三种角色
//    - User: 用户实体
//    - UserRole: 用户角色枚举
//    - CreateUserRequest/UpdateUserRequest: 创建/更新用户的DTO
//    - UserInfo: 用户响应信息
//
// 2. Task（任务）模型 - 核心业务实体，任务执行的基本单元
//    - Task: 任务实体
//    - TaskStatus: 任务状态枚举 (Pending/InProgress/Completed/Cancelled)
//    - TaskPriority: 任务优先级枚举 (Low/Medium/High/Urgent)
//    - CreateTaskRequest/UpdateTaskRequest: 创建/更新任务的DTO
//    - TaskInfo: 任务响应信息（包含关联数据）
//
// 3. Project（项目）模型 - 任务的容器和组织单元
//    - Project: 项目实体
//    - ProjectStatus: 项目状态枚举 (Planning/Active/OnHold/Completed/Cancelled)
//    - CreateProjectRequest/UpdateProjectRequest: 创建/更新项目的DTO
//    - ProjectInfo: 项目响应信息（包含统计数据）
//
// 4. WorkLog（工作记录）模型 - 员工的工作时间跟踪
//    - WorkLog: 工作记录实体
//    - CreateWorkLogRequest/UpdateWorkLogRequest: 创建/更新工作记录的DTO
//    - WorkLogInfo: 工作记录响应信息（包含关联数据）
//
// ==================== Company（公司）模型 ====================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Company {
    pub id: i64,
    pub name: String,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub max_employees: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateCompanyRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: String,
    #[validate(email)]
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub max_employees: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateCompanyRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: Option<String>,
    #[validate(email)]
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub max_employees: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyInfo {
    pub id: i64,
    pub name: String,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub max_employees: i32,
    pub current_employees: i32,  // 当前员工数
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Company> for CompanyInfo {
    fn from(company: Company) -> Self {
        Self {
            id: company.id,
            name: company.name,
            contact_email: company.contact_email,
            contact_phone: company.contact_phone,
            max_employees: company.max_employees,
            current_employees: 0,  // 需要单独查询
            is_active: company.is_active,
            created_at: company.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            updated_at: company.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

// ==================== User（用户）模型 ====================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,  // SQLite使用INTEGER类型
    pub username: String,
    pub email: String,
    pub hashed_password: String,
    pub role: UserRole,
    pub full_name: String,
    pub is_active: bool,
    pub company_id: Option<i64>,  // 所属公司ID,PlatformAdmin为NULL
    pub parent_id: Option<i64>,  // 上级用户ID,用于层级隔离(临时方案,最终用company_id)
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    PlatformAdmin,    // 平台管理员 - 可以查看和管理所有公司的所有数据
    ProjectManager,   // 项目经理 - 只能查看和管理本公司的数据
    TaskExecutor,     // 任务执行者 - 只能查看和更新自己的任务
}

// 手动实现sqlx类型转换,支持旧的role值
impl sqlx::Type<sqlx::Sqlite> for UserRole {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <String as sqlx::Type<sqlx::Sqlite>>::type_info()
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Sqlite> for UserRole {
    fn decode(value: sqlx::sqlite::SqliteValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Sqlite>>::decode(value)?;
        UserRole::from_str(s)
            .ok_or_else(|| format!("Unknown role: {}", s).into())
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Sqlite> for UserRole {
    fn encode_by_ref(&self, buf: &mut Vec<sqlx::sqlite::SqliteArgumentValue<'q>>) -> sqlx::encode::IsNull {
        <&str as sqlx::Encode<sqlx::Sqlite>>::encode(self.as_str(), buf)
    }
}

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::PlatformAdmin => "platform_admin",
            UserRole::ProjectManager => "project_manager",
            UserRole::TaskExecutor => "task_executor",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            // 新的角色名称
            "platform_admin" => Some(UserRole::PlatformAdmin),
            "project_manager" => Some(UserRole::ProjectManager),
            "task_executor" => Some(UserRole::TaskExecutor),
            // 兼容旧的角色名称
            "system_admin" => Some(UserRole::PlatformAdmin),
            "user_admin" | "company_admin" => Some(UserRole::ProjectManager),
            "employee" => Some(UserRole::TaskExecutor),
            _ => None,
        }
    }
}

// 实现FromStr trait用于handlers中解析
impl FromStr for UserRole {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s).ok_or_else(|| format!("未知的角色: {}", s))
    }
}

impl PartialEq<&str> for UserRole {
    fn eq(&self, other: &&str) -> bool {
        match (self, *other) {
            // 新的角色名称
            (UserRole::PlatformAdmin, "platform_admin") => true,
            (UserRole::ProjectManager, "project_manager") => true,
            (UserRole::TaskExecutor, "task_executor") => true,
            // 兼容旧的角色名称
            (UserRole::PlatformAdmin, "system_admin") => true,
            (UserRole::ProjectManager, "user_admin") => true,
            (UserRole::ProjectManager, "company_admin") => true,
            (UserRole::TaskExecutor, "employee") => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::PlatformAdmin => write!(f, "platform_admin"),
            UserRole::ProjectManager => write!(f, "project_manager"),
            UserRole::TaskExecutor => write!(f, "task_executor"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
    pub role: UserRole,
    pub full_name: String,
    pub company_id: Option<i64>,  // 所属公司ID
    pub parent_id: Option<i64>,  // 上级用户ID,用于层级隔离(临时方案)
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub password: Option<String>,
    pub full_name: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1))]
    pub username: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserInfo {
    pub id: i64,  // SQLite使用INTEGER类型
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub role: UserRole,
    pub is_active: bool,
    pub company_id: Option<i64>,  // 所属公司ID
    pub parent_id: Option<i64>,  // 上级用户ID
    pub created_at: String,
    pub last_login: Option<String>,
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            full_name: user.full_name,
            role: user.role,
            is_active: user.is_active,
            company_id: user.company_id,
            parent_id: user.parent_id,
            created_at: user.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            last_login: user
                .last_login
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}

// 通用API响应结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            message: "操作成功".to_string(),
            data: Some(data),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            message,
            data: None,
        }
    }
}

// ============================================================================
// TaskFleet 核心业务模型
// ============================================================================

// ----------------------------------------------------------------------------
// Task（任务）模型
// ----------------------------------------------------------------------------

/// 任务状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,      // 待处理 - 任务已创建，等待分配或开始
    InProgress,   // 进行中 - 任务正在执行
    Completed,    // 已完成 - 任务已完成
    Cancelled,    // 已取消 - 任务被取消
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Pending => "pending",
            TaskStatus::InProgress => "in_progress",
            TaskStatus::Completed => "completed",
            TaskStatus::Cancelled => "cancelled",
        }
    }
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// 任务优先级
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "snake_case")]
pub enum TaskPriority {
    Low,       // 低优先级
    Medium,    // 中优先级
    High,      // 高优先级
    Urgent,    // 紧急
}

impl TaskPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskPriority::Low => "low",
            TaskPriority::Medium => "medium",
            TaskPriority::High => "high",
            TaskPriority::Urgent => "urgent",
        }
    }
}

impl std::fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// 任务主模型
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    
    // 多租户隔离
    pub company_id: Option<i64>,          // 所属公司ID,用于多租户数据隔离
    
    // 关联关系
    pub project_id: Option<Uuid>,        // 所属项目（可选）
    pub assigned_to: Option<Uuid>,        // 分配给的员工（可选）
    pub created_by: Uuid,                 // 创建者
    
    // 时间管理
    pub due_date: Option<DateTime<Utc>>, // 截止日期（可选）
    pub estimated_hours: Option<f64>,     // 预估工时（小时）
    pub actual_hours: Option<f64>,        // 实际工时（小时）
    
    // 元数据
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>, // 完成时间
}

/// 创建任务请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateTaskRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    
    #[validate(length(max = 2000))]
    pub description: String,
    
    pub priority: TaskPriority,
    pub project_id: Option<Uuid>,
    pub assigned_to: Option<Uuid>,
    pub due_date: Option<DateTime<Utc>>,
    pub estimated_hours: Option<f64>,
}

/// 更新任务请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateTaskRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: Option<String>,
    
    #[validate(length(max = 2000))]
    pub description: Option<String>,
    
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub assigned_to: Option<Uuid>,
    pub due_date: Option<DateTime<Utc>>,
    pub estimated_hours: Option<f64>,
}

/// 任务信息响应（包含额外的计算字段）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInfo {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    
    pub project_id: Option<Uuid>,
    pub project_name: Option<String>,      // 项目名称（关联查询）
    
    pub assigned_to: Option<Uuid>,
    pub assigned_to_name: Option<String>,  // 分配员工姓名（关联查询）
    
    pub created_by: Uuid,
    pub created_by_name: String,           // 创建者姓名（关联查询）
    
    pub due_date: Option<String>,
    pub estimated_hours: Option<f64>,
    pub actual_hours: Option<f64>,
    
    pub created_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
}

impl From<Task> for TaskInfo {
    fn from(task: Task) -> Self {
        Self {
            id: task.id,
            title: task.title,
            description: task.description,
            status: task.status,
            priority: task.priority,
            project_id: task.project_id,
            project_name: None,
            assigned_to: task.assigned_to,
            assigned_to_name: None,
            created_by: task.created_by,
            created_by_name: String::new(),
            due_date: task.due_date.map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string()),
            estimated_hours: task.estimated_hours,
            actual_hours: task.actual_hours,
            created_at: task.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            updated_at: task.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            completed_at: task.completed_at.map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}

// ==================== PROJECT（项目）模型 ====================

/// 项目状态
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
pub enum ProjectStatus {
    /// 规划中
    Planning,
    /// 进行中
    Active,
    /// 已暂停
    OnHold,
    /// 已完成
    Completed,
    /// 已取消
    Cancelled,
}

/// 项目模型
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Project {
    /// 项目ID
    pub id: Uuid,
    /// 项目名称
    pub name: String,
    /// 项目描述
    pub description: Option<String>,
    /// 项目状态
    pub status: ProjectStatus,
    /// 多租户隔离 - 所属公司ID
    pub company_id: Option<i64>,
    /// 项目经理ID
    pub manager_id: Uuid,
    /// 项目开始日期
    pub start_date: Option<chrono::NaiveDate>,
    /// 项目结束日期
    pub end_date: Option<chrono::NaiveDate>,
    /// 预算（单位：元）
    pub budget: Option<f64>,
    /// 实际支出（单位：元）
    pub actual_cost: Option<f64>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

/// 创建项目请求
#[derive(Debug, Deserialize, Validate)]
pub struct CreateProjectRequest {
    /// 项目名称（必填，2-100字符）
    #[validate(length(min = 2, max = 100, message = "项目名称必须在2-100个字符之间"))]
    pub name: String,
    
    /// 项目描述（可选，最多1000字符）
    #[validate(length(max = 1000, message = "项目描述不能超过1000个字符"))]
    pub description: Option<String>,
    
    /// 项目状态（可选，默认Planning）
    pub status: Option<ProjectStatus>,
    
    /// 项目经理ID（必填）
    pub manager_id: Uuid,
    
    /// 项目开始日期（可选）
    pub start_date: Option<chrono::NaiveDate>,
    
    /// 项目结束日期（可选）
    pub end_date: Option<chrono::NaiveDate>,
    
    /// 预算（可选，必须为正数）
    #[validate(range(min = 0.0, message = "预算必须为正数"))]
    pub budget: Option<f64>,
}

/// 更新项目请求
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProjectRequest {
    /// 项目名称（可选，2-100字符）
    #[validate(length(min = 2, max = 100, message = "项目名称必须在2-100个字符之间"))]
    pub name: Option<String>,
    
    /// 项目描述（可选，最多1000字符）
    #[validate(length(max = 1000, message = "项目描述不能超过1000个字符"))]
    pub description: Option<String>,
    
    /// 项目状态（可选）
    pub status: Option<ProjectStatus>,
    
    /// 项目经理ID（可选）
    pub manager_id: Option<Uuid>,
    
    /// 项目开始日期（可选）
    pub start_date: Option<chrono::NaiveDate>,
    
    /// 项目结束日期（可选）
    pub end_date: Option<chrono::NaiveDate>,
    
    /// 预算（可选，必须为正数）
    #[validate(range(min = 0.0, message = "预算必须为正数"))]
    pub budget: Option<f64>,
    
    /// 实际支出（可选，必须为正数）
    #[validate(range(min = 0.0, message = "实际支出必须为正数"))]
    pub actual_cost: Option<f64>,
}

/// 项目信息（包含关联数据）
#[derive(Debug, Serialize)]
pub struct ProjectInfo {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: ProjectStatus,
    pub manager_id: Uuid,
    pub manager_name: Option<String>,  // 项目经理姓名
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub budget: Option<f64>,
    pub actual_cost: Option<f64>,
    pub task_count: Option<i64>,       // 任务总数
    pub completed_tasks: Option<i64>,  // 已完成任务数
    pub progress: Option<f64>,         // 进度百分比
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Project> for ProjectInfo {
    fn from(project: Project) -> Self {
        Self {
            id: project.id,
            name: project.name,
            description: project.description,
            status: project.status,
            manager_id: project.manager_id,
            manager_name: None,
            start_date: project.start_date,
            end_date: project.end_date,
            budget: project.budget,
            actual_cost: project.actual_cost,
            task_count: None,
            completed_tasks: None,
            progress: None,
            created_at: project.created_at,
            updated_at: project.updated_at,
        }
    }
}

// ==================== WORKLOG（工作记录）模型 ====================

/// 工作记录模型
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WorkLog {
    /// 工作记录ID
    pub id: Uuid,
    /// 关联任务ID
    pub task_id: Uuid,
    /// 员工ID
    pub user_id: Uuid,
    /// 工作描述
    pub description: Option<String>,
    /// 工作时长（小时）
    pub hours: f64,
    /// 工作日期
    pub work_date: chrono::NaiveDate,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

/// 创建工作记录请求
#[derive(Debug, Deserialize, Validate)]
pub struct CreateWorkLogRequest {
    /// 关联任务ID（必填）
    pub task_id: Uuid,
    
    /// 员工ID（必填）
    pub user_id: Uuid,
    
    /// 工作描述（可选，最多500字符）
    #[validate(length(max = 500, message = "工作描述不能超过500个字符"))]
    pub description: Option<String>,
    
    /// 工作时长（必填，必须大于0且不超过24小时）
    #[validate(range(min = 0.1, max = 24.0, message = "工作时长必须在0.1-24小时之间"))]
    pub hours: f64,
    
    /// 工作日期（可选，默认为今天）
    pub work_date: Option<chrono::NaiveDate>,
}

/// 更新工作记录请求
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateWorkLogRequest {
    /// 工作描述（可选，最多500字符）
    #[validate(length(max = 500, message = "工作描述不能超过500个字符"))]
    pub description: Option<String>,
    
    /// 工作时长（可选，必须大于0且不超过24小时）
    #[validate(range(min = 0.1, max = 24.0, message = "工作时长必须在0.1-24小时之间"))]
    pub hours: Option<f64>,
    
    /// 工作日期（可选）
    pub work_date: Option<chrono::NaiveDate>,
}

/// 工作记录信息（包含关联数据）
#[derive(Debug, Serialize)]
pub struct WorkLogInfo {
    pub id: Uuid,
    pub task_id: Uuid,
    pub task_title: Option<String>,    // 任务标题
    pub user_id: Uuid,
    pub user_name: Option<String>,     // 员工姓名
    pub description: Option<String>,
    pub hours: f64,
    pub work_date: String,              // 格式化为YYYY-MM-DD
    pub created_at: String,             // 格式化为YYYY-MM-DD HH:MM:SS
    pub updated_at: String,             // 格式化为YYYY-MM-DD HH:MM:SS
}

impl From<WorkLog> for WorkLogInfo {
    fn from(log: WorkLog) -> Self {
        Self {
            id: log.id,
            task_id: log.task_id,
            task_title: None,
            user_id: log.user_id,
            user_name: None,
            description: log.description,
            hours: log.hours,
            work_date: log.work_date.format("%Y-%m-%d").to_string(),
            created_at: log.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            updated_at: log.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}



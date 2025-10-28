// TaskFleet Employee Client - 数据模型
// 定义与服务器API通信的数据结构

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// ==================== 认证相关 ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub full_name: String,
    pub email: String,
    pub role: String,
}

// ==================== 任务相关 ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub project_id: Option<i64>,
    pub assigned_to: Option<i64>,
    pub created_by: i64,
    pub estimated_hours: Option<f64>,
    pub actual_hours: Option<f64>,
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTaskStatusRequest {
    pub status: TaskStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTaskNoteRequest {
    pub note: String,
}

// ==================== 工作记录相关 ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLog {
    pub id: i64,
    pub task_id: i64,
    pub user_id: i64,
    pub hours: f64,
    pub notes: Option<String>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkLogRequest {
    pub task_id: i64,
    pub hours: f64,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSession {
    pub task_id: i64,
    pub task_title: String,
    pub started_at: DateTime<Utc>,
}

// ==================== 通知相关 ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotificationType {
    TaskAssigned,
    TaskDue,
    TaskCompleted,
    SystemMessage,
}

// ==================== API错误响应 ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub error: String,
    pub message: String,
}

// ==================== 辅助类型 ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskListResponse {
    pub tasks: Vec<Task>,
    pub total: i64,
}

#[allow(dead_code)]
impl TaskStatus {
    pub fn to_display_string(&self) -> &str {
        match self {
            TaskStatus::Pending => "待处理",
            TaskStatus::InProgress => "进行中",
            TaskStatus::Completed => "已完成",
            TaskStatus::Cancelled => "已取消",
        }
    }
}

#[allow(dead_code)]
impl TaskPriority {
    pub fn to_display_string(&self) -> &str {
        match self {
            TaskPriority::Low => "低",
            TaskPriority::Medium => "中",
            TaskPriority::High => "高",
            TaskPriority::Urgent => "紧急",
        }
    }

    pub fn to_color(&self) -> &str {
        match self {
            TaskPriority::Low => "#52c41a",
            TaskPriority::Medium => "#1890ff",
            TaskPriority::High => "#fa8c16",
            TaskPriority::Urgent => "#f5222d",
        }
    }
}

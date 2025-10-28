// TaskFleet Employee Client - Tauri命令
// 这些命令从前端JavaScript调用

use crate::taskfleet_api::TaskFleetApiClient;
use crate::taskfleet_models::*;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

// 应用状态
pub struct AppState {
    pub api_client: Arc<Mutex<TaskFleetApiClient>>,
    pub current_user: Arc<Mutex<Option<User>>>,
    pub active_work_session: Arc<Mutex<Option<WorkSession>>>,
}

impl AppState {
    pub fn new(server_url: String) -> Self {
        Self {
            api_client: Arc::new(Mutex::new(TaskFleetApiClient::new(server_url))),
            current_user: Arc::new(Mutex::new(None)),
            active_work_session: Arc::new(Mutex::new(None)),
        }
    }
}

// ==================== 认证命令 ====================

#[tauri::command]
pub async fn login(
    username: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<LoginResponse, String> {
    let mut api = state.api_client.lock().await;

    match api.login(username, password).await {
        Ok(response) => {
            // 保存当前用户
            let mut current_user = state.current_user.lock().await;
            *current_user = Some(response.user.clone());

            Ok(response)
        }
        Err(e) => Err(format!("登录失败: {}", e)),
    }
}

#[tauri::command]
pub async fn logout(state: State<'_, AppState>) -> Result<(), String> {
    let mut api = state.api_client.lock().await;

    match api.logout().await {
        Ok(_) => {
            // 清除当前用户
            let mut current_user = state.current_user.lock().await;
            *current_user = None;

            // 清除工作会话
            let mut session = state.active_work_session.lock().await;
            *session = None;

            Ok(())
        }
        Err(e) => Err(format!("退出登录失败: {}", e)),
    }
}

#[tauri::command]
pub async fn get_current_user(state: State<'_, AppState>) -> Result<User, String> {
    let current_user = state.current_user.lock().await;

    current_user
        .clone()
        .ok_or_else(|| "未登录".to_string())
}

// ==================== 任务命令 ====================

#[tauri::command]
pub async fn get_my_tasks(state: State<'_, AppState>) -> Result<Vec<Task>, String> {
    let api = state.api_client.lock().await;

    match api.get_my_tasks().await {
        Ok(tasks) => Ok(tasks),
        Err(e) => Err(format!("获取任务列表失败: {}", e)),
    }
}

#[tauri::command]
pub async fn get_task(task_id: i64, state: State<'_, AppState>) -> Result<Task, String> {
    let api = state.api_client.lock().await;

    match api.get_task(task_id).await {
        Ok(task) => Ok(task),
        Err(e) => Err(format!("获取任务详情失败: {}", e)),
    }
}

#[tauri::command]
pub async fn start_task(task_id: i64, state: State<'_, AppState>) -> Result<Task, String> {
    let api = state.api_client.lock().await;

    match api.start_task(task_id).await {
        Ok(task) => {
            // 开始工作会话
            let mut session = state.active_work_session.lock().await;
            *session = Some(WorkSession {
                task_id: task.id,
                task_title: task.title.clone(),
                started_at: chrono::Utc::now(),
            });

            Ok(task)
        }
        Err(e) => Err(format!("开始任务失败: {}", e)),
    }
}

#[tauri::command]
pub async fn complete_task(task_id: i64, state: State<'_, AppState>) -> Result<Task, String> {
    let api = state.api_client.lock().await;

    match api.complete_task(task_id).await {
        Ok(task) => {
            // 结束工作会话
            let mut session = state.active_work_session.lock().await;
            if let Some(active_session) = session.as_ref() {
                if active_session.task_id == task_id {
                    *session = None;
                }
            }

            Ok(task)
        }
        Err(e) => Err(format!("完成任务失败: {}", e)),
    }
}

#[tauri::command]
pub async fn cancel_task(task_id: i64, state: State<'_, AppState>) -> Result<Task, String> {
    let api = state.api_client.lock().await;

    match api.cancel_task(task_id).await {
        Ok(task) => Ok(task),
        Err(e) => Err(format!("取消任务失败: {}", e)),
    }
}

// ==================== 工作记录命令 ====================

#[tauri::command]
pub async fn get_active_work_session(
    state: State<'_, AppState>,
) -> Result<Option<WorkSession>, String> {
    let session = state.active_work_session.lock().await;
    Ok(session.clone())
}

#[tauri::command]
pub async fn create_work_log(
    task_id: i64,
    hours: f64,
    notes: Option<String>,
    state: State<'_, AppState>,
) -> Result<WorkLog, String> {
    let api = state.api_client.lock().await;

    let request = CreateWorkLogRequest {
        task_id,
        hours,
        notes,
    };

    match api.create_work_log(request).await {
        Ok(log) => Ok(log),
        Err(e) => Err(format!("创建工作记录失败: {}", e)),
    }
}

#[tauri::command]
pub async fn get_my_work_logs(state: State<'_, AppState>) -> Result<Vec<WorkLog>, String> {
    let api = state.api_client.lock().await;

    match api.get_my_work_logs().await {
        Ok(logs) => Ok(logs),
        Err(e) => Err(format!("获取工作记录失败: {}", e)),
    }
}

// ==================== 系统命令 ====================

#[tauri::command]
pub async fn get_app_version() -> Result<String, String> {
    Ok(env!("CARGO_PKG_VERSION").to_string())
}

#[tauri::command]
pub async fn check_server_connection(server_url: String) -> Result<bool, String> {
    let _client = TaskFleetApiClient::new(server_url);
    // 简单的连接检查 - 尝试访问健康检查端点
    Ok(true)
}

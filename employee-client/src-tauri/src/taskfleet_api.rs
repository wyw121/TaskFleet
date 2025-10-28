// TaskFleet Employee Client - API客户端服务
// 负责与TaskFleet服务器通信

use crate::taskfleet_models::*;
use anyhow::{Context, Result};
use reqwest::Client;
use std::time::Duration;

pub struct TaskFleetApiClient {
    client: Client,
    base_url: String,
    token: Option<String>,
}

impl TaskFleetApiClient {
    pub fn new(base_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url,
            token: None,
        }
    }

    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    pub fn clear_token(&mut self) {
        self.token = None;
    }

    fn get_auth_header(&self) -> Result<String> {
        self.token
            .as_ref()
            .map(|t| format!("Bearer {}", t))
            .context("Not authenticated")
    }

    // ==================== 认证API ====================

    pub async fn login(&mut self, username: String, password: String) -> Result<LoginResponse> {
        let url = format!("{}/api/auth/login", self.base_url);
        let request = LoginRequest { username, password };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send login request")?;

        if response.status().is_success() {
            let login_response: LoginResponse = response.json().await?;
            self.set_token(login_response.token.clone());
            Ok(login_response)
        } else {
            let error: ApiError = response.json().await?;
            Err(anyhow::anyhow!("Login failed: {}", error.message))
        }
    }

    pub async fn logout(&mut self) -> Result<()> {
        self.clear_token();
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn get_current_user(&self) -> Result<User> {
        let url = format!("{}/api/auth/me", self.base_url);
        let auth = self.get_auth_header()?;

        let response = self
            .client
            .get(&url)
            .header("Authorization", auth)
            .send()
            .await
            .context("Failed to get current user")?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let error: ApiError = response.json().await?;
            Err(anyhow::anyhow!("Failed to get user: {}", error.message))
        }
    }

    // ==================== 任务API ====================

    pub async fn get_my_tasks(&self) -> Result<Vec<Task>> {
        let url = format!("{}/api/tasks?assigned_to=me", self.base_url);
        let auth = self.get_auth_header()?;

        let response = self
            .client
            .get(&url)
            .header("Authorization", auth)
            .send()
            .await
            .context("Failed to get tasks")?;

        if response.status().is_success() {
            let task_response: TaskListResponse = response.json().await?;
            Ok(task_response.tasks)
        } else {
            let error: ApiError = response.json().await?;
            Err(anyhow::anyhow!("Failed to get tasks: {}", error.message))
        }
    }

    pub async fn get_task(&self, task_id: i64) -> Result<Task> {
        let url = format!("{}/api/tasks/{}", self.base_url, task_id);
        let auth = self.get_auth_header()?;

        let response = self
            .client
            .get(&url)
            .header("Authorization", auth)
            .send()
            .await
            .context("Failed to get task")?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let error: ApiError = response.json().await?;
            Err(anyhow::anyhow!("Failed to get task: {}", error.message))
        }
    }

    pub async fn start_task(&self, task_id: i64) -> Result<Task> {
        let url = format!("{}/api/tasks/{}/start", self.base_url, task_id);
        let auth = self.get_auth_header()?;

        let response = self
            .client
            .put(&url)
            .header("Authorization", auth)
            .send()
            .await
            .context("Failed to start task")?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let error: ApiError = response.json().await?;
            Err(anyhow::anyhow!("Failed to start task: {}", error.message))
        }
    }

    pub async fn complete_task(&self, task_id: i64) -> Result<Task> {
        let url = format!("{}/api/tasks/{}/complete", self.base_url, task_id);
        let auth = self.get_auth_header()?;

        let response = self
            .client
            .put(&url)
            .header("Authorization", auth)
            .send()
            .await
            .context("Failed to complete task")?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let error: ApiError = response.json().await?;
            Err(anyhow::anyhow!("Failed to complete task: {}", error.message))
        }
    }

    pub async fn cancel_task(&self, task_id: i64) -> Result<Task> {
        let url = format!("{}/api/tasks/{}/cancel", self.base_url, task_id);
        let auth = self.get_auth_header()?;

        let response = self
            .client
            .put(&url)
            .header("Authorization", auth)
            .send()
            .await
            .context("Failed to cancel task")?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let error: ApiError = response.json().await?;
            Err(anyhow::anyhow!("Failed to cancel task: {}", error.message))
        }
    }

    // ==================== 工作记录API ====================

    pub async fn create_work_log(&self, request: CreateWorkLogRequest) -> Result<WorkLog> {
        let url = format!("{}/api/work-logs", self.base_url);
        let auth = self.get_auth_header()?;

        let response = self
            .client
            .post(&url)
            .header("Authorization", auth)
            .json(&request)
            .send()
            .await
            .context("Failed to create work log")?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let error: ApiError = response.json().await?;
            Err(anyhow::anyhow!("Failed to create work log: {}", error.message))
        }
    }

    pub async fn get_my_work_logs(&self) -> Result<Vec<WorkLog>> {
        let url = format!("{}/api/work-logs?user=me", self.base_url);
        let auth = self.get_auth_header()?;

        let response = self
            .client
            .get(&url)
            .header("Authorization", auth)
            .send()
            .await
            .context("Failed to get work logs")?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let error: ApiError = response.json().await?;
            Err(anyhow::anyhow!("Failed to get work logs: {}", error.message))
        }
    }
}

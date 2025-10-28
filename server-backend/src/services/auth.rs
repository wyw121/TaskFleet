use anyhow::{anyhow, Result};
use chrono::Utc;

use crate::{
    models::{CreateUserRequest, LoginResponse, User, UserInfo},
    utils::jwt::create_jwt_token,
    Config, Database,
};

pub struct AuthService {
    database: Database,
    config: Config,
}

impl AuthService {
    pub fn new(database: Database, config: Config) -> Self {
        Self { database, config }
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<LoginResponse> {
        // 查找用户 - 支持通过用户名、邮箱或手机号登录
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE (username = ? OR email = ? OR phone = ?) AND (is_active IS NULL OR is_active = 1)",
        )
        .bind(username)
        .bind(username)
        .bind(username)
        .fetch_optional(&self.database.pool)
        .await?
        .ok_or_else(|| anyhow!("用户不存在或已被禁用"))?;

        // 验证密码
        if !bcrypt::verify(password, &user.hashed_password)? {
            return Err(anyhow!("密码错误"));
        }

        // 生成JWT token
        let token = create_jwt_token(
            &user.id.to_string(),
            user.role.as_str(),
            &self.config.jwt_secret,
            self.config.jwt_expires_in,
        )?;

        Ok(LoginResponse {
            token,
            user: user.into(),
        })
    }

    pub async fn register(&self, request: CreateUserRequest) -> Result<UserInfo> {
        // 检查用户名是否已存在
        let existing_user = sqlx::query("SELECT id FROM users WHERE username = ? OR email = ?")
            .bind(&request.username)
            .bind(&request.email)
            .fetch_optional(&self.database.pool)
            .await?;

        if existing_user.is_some() {
            return Err(anyhow!("用户名或邮箱已存在"));
        }

        // 生成密码哈希
        let hashed_password = bcrypt::hash(&request.password, self.config.bcrypt_rounds)?;
        let now = Utc::now();

        // 插入新用户 - 使用简化的TaskFleet字段(id会自动生成)
        let result = sqlx::query(
            r#"
            INSERT INTO users (username, email, hashed_password, role, full_name, is_active, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&request.username)
        .bind(&request.email)
        .bind(&hashed_password)
        .bind(request.role.as_str())
        .bind(if request.full_name.is_empty() { request.username.clone() } else { request.full_name.clone() })
        .bind(true)
        .bind(now)
        .bind(now)
        .execute(&self.database.pool)
        .await?;

        let user_id = result.last_insert_rowid();

        // 返回用户信息
        Ok(UserInfo {
            id: user_id,
            username: request.username.clone(),
            email: request.email.clone(),
            full_name: if request.full_name.is_empty() { request.username.clone() } else { request.full_name.clone() },
            role: request.role,
            is_active: true,
            company_id: None,  // 注册的新用户没有公司
            parent_id: None,  // 注册的新用户没有上级
            created_at: now.format("%Y-%m-%d %H:%M:%S").to_string(),
            last_login: None,
        })
    }

    pub async fn refresh_token(&self, user_id: &str) -> Result<String> {
        // 查找用户
        let user_id: i64 = user_id.parse()?;
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = ? AND is_active = 1",
        )
        .bind(user_id)
        .fetch_optional(&self.database.pool)
        .await?
        .ok_or_else(|| anyhow!("用户不存在或已被禁用"))?;

        // 生成新的JWT token
        create_jwt_token(
            &user.id.to_string(),
            user.role.as_str(),
            &self.config.jwt_secret,
            self.config.jwt_expires_in,
        )
    }
}

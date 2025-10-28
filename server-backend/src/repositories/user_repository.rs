use crate::{models::User, Database};
use anyhow::Result;

/// UserRepository: 负责所有用户相关的数据库操作
pub struct UserRepository {
    database: Database,
}

impl UserRepository {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    /// 根据ID查询用户
    pub async fn find_by_id(&self, id: i64) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.database.pool)
            .await?;
        Ok(user)
    }

    /// 根据用户名查询用户
    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(&self.database.pool)
            .await?;
        Ok(user)
    }

    /// 根据邮箱查询用户
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(&self.database.pool)
            .await?;
        Ok(user)
    }

    /// 获取所有用户列表
    pub async fn list_all(&self) -> Result<Vec<User>> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
            .fetch_all(&self.database.pool)
            .await?;
        Ok(users)
    }

    /// 创建新用户
    pub async fn create(&self, user: User) -> Result<User> {
        let result = sqlx::query(
            r#"
            INSERT INTO users (username, email, hashed_password, role, full_name, is_active, created_at, updated_at, last_login)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.hashed_password)
        .bind(user.role.as_str())
        .bind(&user.full_name)
        .bind(user.is_active)
        .bind(user.created_at)
        .bind(user.updated_at)
        .bind(user.last_login)
        .execute(&self.database.pool)
        .await?;
        
        let user_id = result.last_insert_rowid();
        let created_user = self.find_by_id(user_id).await?
            .ok_or_else(|| anyhow::anyhow!("创建用户后无法查询到"))?;
        Ok(created_user)
    }

    /// 更新用户信息
    pub async fn update(&self, user: User) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE users 
            SET username = ?, email = ?, hashed_password = ?, role = ?, full_name = ?, 
                is_active = ?, updated_at = ?, last_login = ?
            WHERE id = ?
            "#
        )
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.hashed_password)
        .bind(user.role.as_str())
        .bind(&user.full_name)
        .bind(user.is_active)
        .bind(user.updated_at)
        .bind(user.last_login)
        .bind(user.id)
        .execute(&self.database.pool)
        .await?;
        Ok(())
    }

    /// 删除用户
    pub async fn delete(&self, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(&self.database.pool)
            .await?;
        Ok(())
    }

    /// 更新用户最后登录时间
    pub async fn update_last_login(&self, id: i64) -> Result<()> {
        let now = chrono::Utc::now();
        sqlx::query("UPDATE users SET last_login = ? WHERE id = ?")
            .bind(now)
            .bind(id)
            .execute(&self.database.pool)
            .await?;
        Ok(())
    }
}

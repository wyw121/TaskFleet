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
            INSERT INTO users (username, email, hashed_password, role, full_name, is_active, company_id, parent_id, created_at, updated_at, last_login)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.hashed_password)
        .bind(user.role.as_str())
        .bind(&user.full_name)
        .bind(user.is_active)
        .bind(user.company_id)
        .bind(user.parent_id)
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
                is_active = ?, company_id = ?, parent_id = ?, updated_at = ?, last_login = ?
            WHERE id = ?
            "#
        )
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.hashed_password)
        .bind(user.role.as_str())
        .bind(&user.full_name)
        .bind(user.is_active)
        .bind(user.company_id)
        .bind(user.parent_id)
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

    /// 根据parent_id查询用户(包括parent_id为NULL或等于指定值的用户)
    /// 用于公司管理员查看自己管理的用户
    pub async fn list_by_parent(&self, parent_id: i64) -> Result<Vec<User>> {
        let users = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE parent_id = ? OR id = ? ORDER BY created_at DESC"
        )
        .bind(parent_id)
        .bind(parent_id)
        .fetch_all(&self.database.pool)
        .await?;
        Ok(users)
    }

    /// 查询所有顶级用户(parent_id为NULL的用户)及其下级
    /// 用于系统管理员查看所有用户
    pub async fn list_all_hierarchy(&self) -> Result<Vec<User>> {
        let users = sqlx::query_as::<_, User>(
            "SELECT * FROM users ORDER BY COALESCE(parent_id, id), created_at DESC"
        )
        .fetch_all(&self.database.pool)
        .await?;
        Ok(users)
    }

    /// 根据company_id查询用户
    /// 用于公司管理员查看本公司所有用户
    pub async fn list_by_company_id(&self, company_id: i64) -> Result<Vec<User>> {
        let users = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE company_id = ? ORDER BY created_at DESC"
        )
        .bind(company_id)
        .fetch_all(&self.database.pool)
        .await?;
        Ok(users)
    }
}

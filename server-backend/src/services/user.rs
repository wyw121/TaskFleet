use anyhow::{anyhow, Result};
use uuid::Uuid;

use crate::{
    models::{CreateUserRequest, UpdateUserRequest, User, UserInfo, UserRole},
    repositories::UserRepository,
    utils::hash_password,
    Database,
};

pub struct UserService {
    database: Database,
    user_repository: UserRepository,
}

impl UserService {
    pub fn new(database: Database) -> Self {
        let user_repository = UserRepository::new(database.clone());
        Self {
            database,
            user_repository,
        }
    }

    pub async fn list_users(
        &self,
        current_user: &UserInfo,
    ) -> Result<Vec<UserInfo>> {
        // 只有项目管理员可以查看所有用户列表
        if current_user.role != UserRole::ProjectManager {
            return Err(anyhow!("权限不足：只有项目管理员可以查看用户列表"));
        }

        // 获取所有用户
        let users = self.user_repository.list_all().await?;
        Ok(users.into_iter().map(|user| user.into()).collect())
    }

    pub async fn get_user(
        &self,
        user_id: Uuid,
        current_user: &UserInfo,
    ) -> Result<UserInfo> {
        // 用户只能查看自己的信息，项目管理员可以查看所有用户
        if current_user.role != UserRole::ProjectManager && current_user.id != user_id {
            return Err(anyhow!("权限不足：只能查看自己的信息"));
        }

        let user = self.user_repository.find_by_id(user_id).await?
            .ok_or_else(|| anyhow!("用户不存在"))?;

        Ok(user.into())
    }

    pub async fn create_user(
        &self,
        request: CreateUserRequest,
        current_user: &UserInfo,
    ) -> Result<UserInfo> {
        // 只有项目管理员可以创建用户
        if current_user.role != UserRole::ProjectManager {
            return Err(anyhow!("权限不足：只有项目管理员可以创建用户"));
        }

        // 检查用户名和邮箱是否已存在
        if self.user_repository.find_by_username(&request.username).await?.is_some() {
            return Err(anyhow!("用户名已存在"));
        }

        if self.user_repository.find_by_email(&request.email).await?.is_some() {
            return Err(anyhow!("邮箱已存在"));
        }

        // 创建用户
        let user_id = Uuid::new_v4();
        let hashed_password = hash_password(&request.password)?;
        let now = chrono::Utc::now();

        let new_user = User {
            id: user_id,
            username: request.username.clone(),
            email: request.email.clone(),
            hashed_password,
            role: request.role.clone(),
            full_name: if request.full_name.is_empty() { request.username.clone() } else { request.full_name.clone() },
            is_active: true,
            created_at: now,
            updated_at: now,
            last_login: None,
        };

        self.user_repository.create(new_user).await?;

        Ok(UserInfo {
            id: user_id,
            username: request.username,
            email: request.email,
            full_name: request.full_name,
            role: request.role,
            is_active: true,
            created_at: now.format("%Y-%m-%d %H:%M:%S").to_string(),
            last_login: None,
        })
    }

    pub async fn update_user(
        &self,
        user_id: Uuid,
        request: UpdateUserRequest,
        current_user: &UserInfo,
    ) -> Result<UserInfo> {
        // 用户只能更新自己的信息，项目管理员可以更新所有用户
        if current_user.role != UserRole::ProjectManager && current_user.id != user_id {
            return Err(anyhow!("权限不足：只能更新自己的信息"));
        }

        let mut user = self.user_repository.find_by_id(user_id).await?
            .ok_or_else(|| anyhow!("用户不存在"))?;

        // 更新字段
        if let Some(username) = request.username {
            // 检查用户名是否被其他用户使用
            if let Some(existing_user) = self.user_repository.find_by_username(&username).await? {
                if existing_user.id != user_id {
                    return Err(anyhow!("用户名已被使用"));
                }
            }
            user.username = username;
        }

        if let Some(email) = request.email {
            // 检查邮箱是否被其他用户使用
            if let Some(existing_user) = self.user_repository.find_by_email(&email).await? {
                if existing_user.id != user_id {
                    return Err(anyhow!("邮箱已被使用"));
                }
            }
            user.email = email;
        }

        if let Some(password) = request.password {
            user.hashed_password = hash_password(&password)?;
        }

        if let Some(full_name) = request.full_name {
            user.full_name = full_name;
        }

        if let Some(is_active) = request.is_active {
            // 只有项目管理员可以更改用户状态
            if current_user.role != UserRole::ProjectManager {
                return Err(anyhow!("权限不足：只有项目管理员可以更改用户状态"));
            }
            user.is_active = is_active;
        }

        user.updated_at = chrono::Utc::now();

        self.user_repository.update(user.clone()).await?;

        Ok(user.into())
    }

    pub async fn delete_user(
        &self,
        user_id: Uuid,
        current_user: &UserInfo,
    ) -> Result<()> {
        // 只有项目管理员可以删除用户
        if current_user.role != UserRole::ProjectManager {
            return Err(anyhow!("权限不足：只有项目管理员可以删除用户"));
        }

        // 不能删除自己
        if current_user.id == user_id {
            return Err(anyhow!("不能删除自己的账户"));
        }

        // 验证用户是否存在
        let _user = self.user_repository.find_by_id(user_id).await?
            .ok_or_else(|| anyhow!("用户不存在"))?;

        // 删除用户
        self.user_repository.delete(user_id).await?;

        Ok(())
    }
}
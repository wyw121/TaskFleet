use anyhow::{anyhow, Result};

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
        // 根据角色返回不同范围的用户列表
        let users = match current_user.role {
            // 平台管理员可以查看所有用户
            UserRole::PlatformAdmin => {
                self.user_repository.list_all_hierarchy().await?
            }
            // 项目经理只能查看本公司用户
            UserRole::ProjectManager => {
                let company_id = current_user.company_id
                    .ok_or_else(|| anyhow!("项目经理必须关联公司"))?;
                self.user_repository.list_by_company_id(company_id).await?
            }
            // 任务执行者不能查看用户列表
            UserRole::TaskExecutor => {
                return Err(anyhow!("权限不足：任务执行者无法查看用户列表"));
            }
        };

        Ok(users.into_iter().map(|user| user.into()).collect())
    }

    pub async fn get_user(
        &self,
        user_id: i64,
        current_user: &UserInfo,
    ) -> Result<UserInfo> {
        let user = self.user_repository.find_by_id(user_id).await?
            .ok_or_else(|| anyhow!("用户不存在"))?;

        // 权限检查
        match current_user.role {
            // 平台管理员可以查看所有用户
            UserRole::PlatformAdmin => {}
            // 项目经理只能查看本公司用户
            UserRole::ProjectManager => {
                if user.company_id != current_user.company_id {
                    return Err(anyhow!("权限不足：只能查看本公司用户"));
                }
            }
            // 任务执行者只能查看自己
            UserRole::TaskExecutor => {
                if user.id != current_user.id {
                    return Err(anyhow!("权限不足：只能查看自己的信息"));
                }
            }
        }

        Ok(user.into())
    }

    pub async fn create_user(
        &self,
        request: CreateUserRequest,
        current_user: &UserInfo,
    ) -> Result<UserInfo> {
        // 只有管理员可以创建用户
        let (company_id, parent_id) = match current_user.role {
            UserRole::PlatformAdmin => {
                // 平台管理员可以创建任何用户,使用请求中的company_id
                (request.company_id, request.parent_id)
            }
            UserRole::ProjectManager => {
                // 项目经理只能创建任务执行者,且必须在自己公司
                if request.role != UserRole::TaskExecutor {
                    return Err(anyhow!("权限不足：项目经理只能创建任务执行者账号"));
                }
                let company_id = current_user.company_id
                    .ok_or_else(|| anyhow!("项目经理必须关联公司"))?;
                (Some(company_id), Some(current_user.id))
            }
            UserRole::TaskExecutor => {
                return Err(anyhow!("权限不足：任务执行者无法创建用户"));
            }
        };

        // 检查用户名和邮箱是否已存在
        if self.user_repository.find_by_username(&request.username).await?.is_some() {
            return Err(anyhow!("用户名已存在"));
        }

        if self.user_repository.find_by_email(&request.email).await?.is_some() {
            return Err(anyhow!("邮箱已存在"));
        }

        // 创建用户
        let hashed_password = hash_password(&request.password)?;
        let now = chrono::Utc::now();

        let new_user = User {
            id: 0,  // 数据库自动生成
            username: request.username.clone(),
            email: request.email.clone(),
            hashed_password,
            role: request.role.clone(),
            full_name: if request.full_name.is_empty() { request.username.clone() } else { request.full_name.clone() },
            is_active: true,
            company_id,
            parent_id,
            created_at: now,
            updated_at: now,
            last_login: None,
        };

        let created_user = self.user_repository.create(new_user).await?;

        Ok(UserInfo {
            id: created_user.id,
            username: request.username,
            email: request.email,
            full_name: request.full_name,
            role: request.role,
            is_active: true,
            company_id,
            parent_id,
            created_at: now.format("%Y-%m-%d %H:%M:%S").to_string(),
            last_login: None,
        })
    }

    pub async fn update_user(
        &self,
        user_id: i64,
        request: UpdateUserRequest,
        current_user: &UserInfo,
    ) -> Result<UserInfo> {
        let mut user = self.user_repository.find_by_id(user_id).await?
            .ok_or_else(|| anyhow!("用户不存在"))?;

        // 权限检查
        match current_user.role {
            UserRole::PlatformAdmin => {}  // 平台管理员可以更新所有用户
            UserRole::ProjectManager => {
                // 项目经理只能更新本公司用户
                if user.company_id != current_user.company_id {
                    return Err(anyhow!("权限不足：只能更新本公司用户"));
                }
            }
            UserRole::TaskExecutor => {
                // 任务执行者只能更新自己
                if user.id != current_user.id {
                    return Err(anyhow!("权限不足：只能更新自己的信息"));
                }
            }
        }

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
            // 只有管理员可以更改用户状态
            if current_user.role == UserRole::TaskExecutor {
                return Err(anyhow!("权限不足：任务执行者无法更改用户状态"));
            }
            user.is_active = is_active;
        }

        user.updated_at = chrono::Utc::now();

        self.user_repository.update(user.clone()).await?;

        Ok(user.into())
    }

    pub async fn delete_user(
        &self,
        user_id: i64,
        current_user: &UserInfo,
    ) -> Result<()> {
        // 不能删除自己
        if current_user.id == user_id {
            return Err(anyhow!("不能删除自己的账户"));
        }

        // 验证用户是否存在
        let user = self.user_repository.find_by_id(user_id).await?
            .ok_or_else(|| anyhow!("用户不存在"))?;

        // 权限检查
        match current_user.role {
            UserRole::PlatformAdmin => {}  // 平台管理员可以删除任何用户
            UserRole::ProjectManager => {
                // 项目经理只能删除本公司用户
                if user.company_id != current_user.company_id {
                    return Err(anyhow!("权限不足：只能删除本公司用户"));
                }
            }
            UserRole::TaskExecutor => {
                return Err(anyhow!("权限不足：任务执行者无法删除用户"));
            }
        }

        // 删除用户
        self.user_repository.delete(user_id).await?;

        Ok(())
    }
}
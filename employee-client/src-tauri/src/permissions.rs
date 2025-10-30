// TaskFleet Desktop Client - 权限检查模块
// 与后端权限保持一致,前端仅用于UI控制

use crate::taskfleet_models::UserRole;

/// 权限检查辅助结构
pub struct Permissions {
    role: UserRole,
}

impl Permissions {
    pub fn new(role: UserRole) -> Self {
        Self { role }
    }

    // ==================== 角色检查 ====================

    /// 是否为平台管理员
    pub fn is_platform_admin(&self) -> bool {
        matches!(self.role, UserRole::PlatformAdmin)
    }

    /// 是否为项目经理
    pub fn is_project_manager(&self) -> bool {
        matches!(self.role, UserRole::ProjectManager)
    }

    /// 是否为任务执行者
    pub fn is_task_executor(&self) -> bool {
        matches!(self.role, UserRole::TaskExecutor)
    }

    /// 是否有管理权限(平台管理员或项目经理)
    pub fn has_admin_role(&self) -> bool {
        self.is_platform_admin() || self.is_project_manager()
    }

    // ==================== 功能权限检查 ====================

    /// 是否可以管理公司
    pub fn can_manage_companies(&self) -> bool {
        self.is_platform_admin()
    }

    /// 是否可以管理用户
    pub fn can_manage_users(&self) -> bool {
        self.has_admin_role()
    }

    /// 是否可以创建任务
    pub fn can_create_task(&self) -> bool {
        self.has_admin_role()
    }

    /// 是否可以创建项目
    pub fn can_create_project(&self) -> bool {
        self.has_admin_role()
    }

    /// 是否可以分配任务
    pub fn can_assign_tasks(&self) -> bool {
        self.has_admin_role()
    }

    /// 是否可以查看数据分析
    pub fn can_view_analytics(&self) -> bool {
        self.has_admin_role()
    }

    /// 是否可以删除任务/项目
    pub fn can_delete(&self) -> bool {
        self.has_admin_role()
    }

    /// 是否可以查看所有任务
    pub fn can_view_all_tasks(&self) -> bool {
        self.has_admin_role()
    }

    /// 是否只能查看自己的任务
    pub fn can_only_view_own_tasks(&self) -> bool {
        self.is_task_executor()
    }

    // ==================== UI显示控制 ====================

    /// 获取角色的中文显示名称
    pub fn get_role_display_name(&self) -> &'static str {
        match self.role {
            UserRole::PlatformAdmin => "平台管理员",
            UserRole::ProjectManager => "项目经理",
            UserRole::TaskExecutor => "任务执行者",
        }
    }

    /// 获取角色对应的颜色(用于UI标识)
    pub fn get_role_color(&self) -> &'static str {
        match self.role {
            UserRole::PlatformAdmin => "red",
            UserRole::ProjectManager => "blue",
            UserRole::TaskExecutor => "green",
        }
    }

    /// 获取桌面端应该显示的功能列表
    pub fn get_desktop_features(&self) -> Vec<DesktopFeature> {
        let mut features = vec![
            DesktopFeature::ViewTasks,
            DesktopFeature::UpdateTaskStatus,
            DesktopFeature::ViewProfile,
            DesktopFeature::WorkLogs,
        ];

        if self.has_admin_role() {
            features.extend_from_slice(&[
                DesktopFeature::CreateTask,
                DesktopFeature::AssignTask,
                DesktopFeature::ViewAnalytics,
                DesktopFeature::ManageProjects,
            ]);
        }

        if self.is_platform_admin() {
            features.extend_from_slice(&[
                DesktopFeature::ManageCompanies,
                DesktopFeature::ManageUsers,
            ]);
        }

        features
    }
}

/// 桌面端功能枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopFeature {
    // 基础功能(所有角色)
    ViewTasks,
    UpdateTaskStatus,
    ViewProfile,
    WorkLogs,

    // 管理功能(管理员)
    CreateTask,
    AssignTask,
    ViewAnalytics,
    ManageProjects,

    // 高级功能(平台管理员)
    ManageCompanies,
    ManageUsers,
}

impl DesktopFeature {
    /// 获取功能的显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::ViewTasks => "查看任务",
            Self::UpdateTaskStatus => "更新任务状态",
            Self::ViewProfile => "个人信息",
            Self::WorkLogs => "工作记录",
            Self::CreateTask => "创建任务",
            Self::AssignTask => "分配任务",
            Self::ViewAnalytics => "数据分析",
            Self::ManageProjects => "项目管理",
            Self::ManageCompanies => "公司管理",
            Self::ManageUsers => "用户管理",
        }
    }

    /// 获取功能图标(用于UI)
    pub fn icon(&self) -> &'static str {
        match self {
            Self::ViewTasks => "📋",
            Self::UpdateTaskStatus => "✅",
            Self::ViewProfile => "👤",
            Self::WorkLogs => "📊",
            Self::CreateTask => "➕",
            Self::AssignTask => "👥",
            Self::ViewAnalytics => "📈",
            Self::ManageProjects => "🗂️",
            Self::ManageCompanies => "🏢",
            Self::ManageUsers => "👨‍💼",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_admin_permissions() {
        let perms = Permissions::new(UserRole::PlatformAdmin);

        assert!(perms.is_platform_admin());
        assert!(perms.has_admin_role());
        assert!(perms.can_manage_companies());
        assert!(perms.can_manage_users());
        assert!(perms.can_create_task());
        assert!(perms.can_view_all_tasks());
    }

    #[test]
    fn test_project_manager_permissions() {
        let perms = Permissions::new(UserRole::ProjectManager);

        assert!(perms.is_project_manager());
        assert!(perms.has_admin_role());
        assert!(!perms.can_manage_companies());
        assert!(perms.can_manage_users());
        assert!(perms.can_create_task());
    }

    #[test]
    fn test_task_executor_permissions() {
        let perms = Permissions::new(UserRole::TaskExecutor);

        assert!(perms.is_task_executor());
        assert!(!perms.has_admin_role());
        assert!(!perms.can_manage_companies());
        assert!(!perms.can_manage_users());
        assert!(!perms.can_create_task());
        assert!(perms.can_only_view_own_tasks());
    }

    #[test]
    fn test_desktop_features() {
        let admin = Permissions::new(UserRole::PlatformAdmin);
        let features = admin.get_desktop_features();

        assert!(features.contains(&DesktopFeature::ViewTasks));
        assert!(features.contains(&DesktopFeature::ManageCompanies));
        assert!(features.len() > 4); // 应该有更多功能
    }
}

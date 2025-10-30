import { useSelector } from 'react-redux';
import { RootState } from '../store';
import { UserRole } from '../types/user';

/**
 * 权限检查 Hook
 * 用于UI元素显示/隐藏控制
 */
export const usePermissions = () => {
  const { user } = useSelector((state: RootState) => state.auth);

  /**
   * 检查是否有指定角色
   */
  const hasRole = (roles: UserRole | UserRole[]): boolean => {
    if (!user) return false;
    const roleArray = Array.isArray(roles) ? roles : [roles];
    return roleArray.includes(user.role);
  };

  /**
   * 检查是否为平台管理员
   */
  const isPlatformAdmin = (): boolean => {
    return hasRole(UserRole.PlatformAdmin);
  };

  /**
   * 检查是否为系统管理员 (兼容旧函数名)
   * @deprecated 使用 isPlatformAdmin() 代替
   */
  const isSystemAdmin = (): boolean => {
    return isPlatformAdmin();
  };

  /**
   * 检查是否为项目经理
   */
  const isProjectManager = (): boolean => {
    return hasRole(UserRole.ProjectManager);
  };

  /**
   * 检查是否为公司管理员 (兼容旧函数名)
   * @deprecated 使用 isProjectManager() 代替
   */
  const isCompanyAdmin = (): boolean => {
    return isProjectManager();
  };

  /**
   * 检查是否为任务执行者
   */
  const isTaskExecutor = (): boolean => {
    return hasRole(UserRole.TaskExecutor);
  };

  /**
   * 检查是否为普通员工 (兼容旧函数名)
   * @deprecated 使用 isTaskExecutor() 代替
   */
  const isEmployee = (): boolean => {
    return isTaskExecutor();
  };

  /**
   * 检查是否有管理权限(平台管理员或项目经理)
   */
  const hasAdminRole = (): boolean => {
    return hasRole([UserRole.PlatformAdmin, UserRole.ProjectManager]);
  };

  /**
   * 检查是否可以管理公司
   * 仅平台管理员可以管理公司
   */
  const canManageCompanies = (): boolean => {
    return isPlatformAdmin();
  };

  /**
   * 检查是否可以管理用户
   * 平台管理员可以管理所有用户
   * 项目经理可以管理自己公司的用户
   */
  const canManageUsers = (): boolean => {
    return hasAdminRole();
  };

  // ==================== 页面访问权限 ====================
  
  /**
   * 检查是否可以访问任务管理页面
   * ProjectManager 和 TaskExecutor 可以访问
   * PlatformAdmin 不应参与具体业务
   */
  const canAccessTasks = (): boolean => {
    return hasRole([UserRole.ProjectManager, UserRole.TaskExecutor]);
  };

  /**
   * 检查是否可以访问项目管理页面
   * ProjectManager 和 TaskExecutor 可以访问
   */
  const canAccessProjects = (): boolean => {
    return hasRole([UserRole.ProjectManager, UserRole.TaskExecutor]);
  };

  /**
   * 检查是否可以查看数据分析
   * PlatformAdmin 查看全平台数据
   * ProjectManager 查看本公司数据
   */
  const canViewAnalytics = (): boolean => {
    return hasRole([UserRole.PlatformAdmin, UserRole.ProjectManager]);
  };

  // ==================== 任务操作权限 ====================

  /**
   * 检查是否可以创建任务
   * ProjectManager: 可以创建任何任务
   * TaskExecutor: 可以创建子任务和问题反馈
   */
  const canCreateTask = (): boolean => {
    return hasRole([UserRole.ProjectManager, UserRole.TaskExecutor]);
  };

  /**
   * 检查是否可以编辑任务
   * ProjectManager: 可以编辑任何任务
   * TaskExecutor: 只能编辑分配给自己的任务
   * 需要在组件中进一步检查任务所有权
   */
  const canEditTask = (taskAssigneeId?: number | string): boolean => {
    if (isProjectManager()) return true;
    if (isTaskExecutor() && user && taskAssigneeId) {
      // 统一转换为字符串进行比较
      return String(taskAssigneeId) === String(user.id);
    }
    return false;
  };

  /**
   * 检查是否可以删除任务
   * 仅 ProjectManager 可以删除
   */
  const canDeleteTask = (): boolean => {
    return isProjectManager();
  };

  /**
   * 检查是否可以分配任务给其他人
   * 仅 ProjectManager 可以分配
   */
  const canAssignTasks = (): boolean => {
    return isProjectManager();
  };

  /**
   * 检查是否可以更新任务状态（开始/完成/暂停）
   * ProjectManager 和 TaskExecutor 都可以
   */
  const canUpdateTaskStatus = (): boolean => {
    return hasRole([UserRole.ProjectManager, UserRole.TaskExecutor]);
  };

  // ==================== 项目操作权限 ====================

  /**
   * 检查是否可以创建项目
   * 仅 ProjectManager 可以创建
   */
  const canCreateProject = (): boolean => {
    return isProjectManager();
  };

  /**
   * 检查是否可以编辑项目
   * 仅 ProjectManager 可以编辑（应只能编辑自己创建的）
   */
  const canEditProject = (): boolean => {
    return isProjectManager();
  };

  /**
   * 检查是否可以删除项目
   * 仅 ProjectManager 可以删除（需二次确认）
   */
  const canDeleteProject = (): boolean => {
    return isProjectManager();
  };

  // ==================== 用户管理权限 ====================

  /**
   * 检查是否可以创建用户
   * PlatformAdmin: 可以创建任意公司的用户
   * ProjectManager: 只能创建本公司的用户
   */
  const canCreateUser = (): boolean => {
    return hasAdminRole();
  };

  /**
   * 检查是否可以编辑用户
   * PlatformAdmin: 可以编辑任意用户
   * ProjectManager: 只能编辑本公司用户
   */
  const canEditUser = (): boolean => {
    return hasAdminRole();
  };

  /**
   * 检查是否可以删除用户
   * PlatformAdmin: 可以删除任意用户
   * ProjectManager: 只能删除本公司用户
   */
  const canDeleteUser = (): boolean => {
    return hasAdminRole();
  };

  /**
   * 检查是否可以查看团队成员列表
   * ProjectManager 和 TaskExecutor 可以查看本公司成员
   */
  const canViewTeamMembers = (): boolean => {
    return hasRole([UserRole.ProjectManager, UserRole.TaskExecutor]);
  };

  // ==================== 通用操作权限 ====================

  /**
   * 检查是否可以导出数据
   * PlatformAdmin 和 ProjectManager 可以导出
   */
  const canExportData = (): boolean => {
    return hasAdminRole();
  };

  return {
    user,
    hasRole,
    
    // 角色检查
    isPlatformAdmin,
    isProjectManager,
    isTaskExecutor,
    isSystemAdmin,  // 兼容旧名称
    isCompanyAdmin, // 兼容旧名称
    isEmployee,     // 兼容旧名称
    hasAdminRole,
    
    // 页面访问权限
    canAccessTasks,
    canAccessProjects,
    canViewAnalytics,
    
    // 模块管理权限
    canManageCompanies,
    canManageUsers,
    
    // 任务操作权限
    canCreateTask,
    canEditTask,
    canDeleteTask,
    canAssignTasks,
    canUpdateTaskStatus,
    
    // 项目操作权限
    canCreateProject,
    canEditProject,
    canDeleteProject,
    
    // 用户管理权限
    canCreateUser,
    canEditUser,
    canDeleteUser,
    canViewTeamMembers,
    
    // 通用权限
    canExportData,
  };
};

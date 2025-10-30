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

  /**
   * 检查是否可以管理任务
   * 所有角色都可以查看和管理任务
   */
  const canManageTasks = (): boolean => {
    return true; // 所有已认证用户都可以
  };

  /**
   * 检查是否可以管理项目
   * 所有角色都可以查看和管理项目
   */
  const canManageProjects = (): boolean => {
    return true; // 所有已认证用户都可以
  };

  /**
   * 检查是否可以查看数据分析
   * 仅管理员可以查看
   */
  const canViewAnalytics = (): boolean => {
    return hasAdminRole();
  };

  /**
   * 检查是否可以创建任务
   */
  const canCreateTask = (): boolean => {
    return hasAdminRole(); // 仅管理员可以创建任务
  };

  /**
   * 检查是否可以创建项目
   */
  const canCreateProject = (): boolean => {
    return hasAdminRole(); // 仅管理员可以创建项目
  };

  /**
   * 检查是否可以删除任务/项目
   */
  const canDelete = (): boolean => {
    return hasAdminRole(); // 仅管理员可以删除
  };

  /**
   * 检查是否可以分配任务给其他人
   */
  const canAssignTasks = (): boolean => {
    return hasAdminRole(); // 仅管理员可以分配任务
  };

  return {
    user,
    hasRole,
    // 新的函数名
    isPlatformAdmin,
    isProjectManager,
    isTaskExecutor,
    // 兼容旧的函数名
    isSystemAdmin,
    isCompanyAdmin,
    isEmployee,
    hasAdminRole,
    canManageCompanies,
    canManageUsers,
    canManageTasks,
    canManageProjects,
    canViewAnalytics,
    canCreateTask,
    canCreateProject,
    canDelete,
    canAssignTasks,
  };
};

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
   * 检查是否为系统管理员
   */
  const isSystemAdmin = (): boolean => {
    return hasRole(UserRole.SystemAdmin);
  };

  /**
   * 检查是否为公司管理员
   */
  const isCompanyAdmin = (): boolean => {
    return hasRole(UserRole.CompanyAdmin);
  };

  /**
   * 检查是否为普通员工
   */
  const isEmployee = (): boolean => {
    return hasRole(UserRole.Employee);
  };

  /**
   * 检查是否有管理权限(系统管理员或公司管理员)
   */
  const hasAdminRole = (): boolean => {
    return hasRole([UserRole.SystemAdmin, UserRole.CompanyAdmin]);
  };

  /**
   * 检查是否可以管理公司
   * 仅系统管理员可以管理公司
   */
  const canManageCompanies = (): boolean => {
    return isSystemAdmin();
  };

  /**
   * 检查是否可以管理用户
   * 系统管理员可以管理所有用户
   * 公司管理员可以管理自己公司的用户
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

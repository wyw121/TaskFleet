import React from 'react';
import { Navigate } from 'react-router-dom';
import { useSelector } from 'react-redux';
import { RootState } from '../store';
import { UserRole } from '../types/user';
import { Result, Button } from 'antd';

/**
 * 路由权限配置
 */
interface RoutePermission {
  /** 允许访问的角色列表 */
  allowedRoles: UserRole[];
  /** 是否需要认证 */
  requireAuth?: boolean;
}

/**
 * 权限守卫组件属性
 */
interface ProtectedRouteProps {
  /** 子组件 */
  children: React.ReactNode;
  /** 允许访问的角色 */
  allowedRoles?: UserRole[];
  /** 重定向路径(无权限时) */
  redirectTo?: string;
}

/**
 * 权限守卫组件
 * 根据用户角色控制路由访问
 */
export const ProtectedRoute: React.FC<ProtectedRouteProps> = ({
  children,
  allowedRoles = [],
  redirectTo = '/dashboard',
}) => {
  const { isAuthenticated, user } = useSelector((state: RootState) => state.auth);

  // 未认证则重定向到登录页
  if (!isAuthenticated || !user) {
    return <Navigate to="/login" replace />;
  }

  // 如果未指定角色限制,则允许所有已认证用户访问
  if (allowedRoles.length === 0) {
    return <>{children}</>;
  }

  // 检查用户角色是否在允许列表中
  const hasPermission = allowedRoles.includes(user.role);

  if (!hasPermission) {
    // 无权限时显示403页面
    return (
      <Result
        status="403"
        title="403"
        subTitle="抱歉,您没有权限访问此页面。"
        extra={
          <Button type="primary" onClick={() => window.location.href = redirectTo}>
            返回主页
          </Button>
        }
      />
    );
  }

  return <>{children}</>;
};

/**
 * 检查用户是否有指定角色
 */
export const useHasRole = (roles: UserRole | UserRole[]): boolean => {
  const { user } = useSelector((state: RootState) => state.auth);
  
  if (!user) return false;
  
  const roleArray = Array.isArray(roles) ? roles : [roles];
  return roleArray.includes(user.role);
};

/**
 * 检查用户是否为系统管理员
 */
export const useIsSystemAdmin = (): boolean => {
  return useHasRole(UserRole.SystemAdmin);
};

/**
 * 检查用户是否为公司管理员
 */
export const useIsCompanyAdmin = (): boolean => {
  return useHasRole(UserRole.CompanyAdmin);
};

/**
 * 检查用户是否为普通员工
 */
export const useIsEmployee = (): boolean => {
  return useHasRole(UserRole.Employee);
};

/**
 * 检查用户是否有管理权限(系统管理员或公司管理员)
 */
export const useHasAdminRole = (): boolean => {
  return useHasRole([UserRole.SystemAdmin, UserRole.CompanyAdmin]);
};

/**
 * 路由权限配置表
 */
export const ROUTE_PERMISSIONS: Record<string, RoutePermission> = {
  // 公司管理 - 仅系统管理员
  '/companies': {
    allowedRoles: [UserRole.SystemAdmin],
    requireAuth: true,
  },
  
  // 用户管理 - 系统管理员和公司管理员
  '/users': {
    allowedRoles: [UserRole.SystemAdmin, UserRole.CompanyAdmin],
    requireAuth: true,
  },
  
  // 任务管理 - 所有角色
  '/tasks': {
    allowedRoles: [UserRole.SystemAdmin, UserRole.CompanyAdmin, UserRole.Employee],
    requireAuth: true,
  },
  
  // 项目管理 - 所有角色
  '/projects': {
    allowedRoles: [UserRole.SystemAdmin, UserRole.CompanyAdmin, UserRole.Employee],
    requireAuth: true,
  },
  
  // 数据分析 - 系统管理员和公司管理员
  '/analytics': {
    allowedRoles: [UserRole.SystemAdmin, UserRole.CompanyAdmin],
    requireAuth: true,
  },
  
  // 仪表板 - 所有角色
  '/dashboard': {
    allowedRoles: [UserRole.SystemAdmin, UserRole.CompanyAdmin, UserRole.Employee],
    requireAuth: true,
  },
};

/**
 * 获取指定路径的权限配置
 */
export const getRoutePermission = (path: string): RoutePermission | undefined => {
  return ROUTE_PERMISSIONS[path];
};

/**
 * 检查用户是否有访问指定路径的权限
 */
export const useCanAccessRoute = (path: string): boolean => {
  const { user } = useSelector((state: RootState) => state.auth);
  const permission = getRoutePermission(path);
  
  if (!permission) return true; // 未配置权限的路由默认允许访问
  if (!user) return false;
  
  return permission.allowedRoles.includes(user.role);
};

export default ProtectedRoute;

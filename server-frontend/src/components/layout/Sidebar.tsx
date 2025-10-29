/**
 * TaskFleet - Sidebar导航组件
 * 根据用户角色动态显示菜单项
 */

import React, { useMemo } from 'react';
import { Link, useLocation } from 'react-router-dom';
import {
  DashboardOutlined,
  CheckSquareOutlined,
  ProjectOutlined,
  BarChartOutlined,
  UserOutlined,
  LogoutOutlined,
  ApartmentOutlined,
} from '@ant-design/icons';
import { Menu } from 'antd';
import type { MenuProps } from 'antd';
import { useDispatch } from 'react-redux';
import { logout } from '../../store/authSlice';
import { usePermissions } from '../../hooks/usePermissions';

type MenuItem = Required<MenuProps>['items'][number];

const Sidebar: React.FC = () => {
  const location = useLocation();
  const dispatch = useDispatch();
  const { canViewAnalytics, canManageUsers, canManageCompanies } = usePermissions();

  const handleLogout = () => {
    dispatch(logout() as any);
  };

  // 根据用户权限动态生成菜单项
  const menuItems: MenuItem[] = useMemo(() => {
    const items: MenuItem[] = [
      // 仪表板 - 所有角色可访问
      {
        key: '/dashboard',
        icon: <DashboardOutlined />,
        label: <Link to="/dashboard">仪表板</Link>,
      },
      // 任务管理 - 所有角色可访问
      {
        key: '/tasks',
        icon: <CheckSquareOutlined />,
        label: <Link to="/tasks">任务管理</Link>,
      },
      // 项目管理 - 所有角色可访问
      {
        key: '/projects',
        icon: <ProjectOutlined />,
        label: <Link to="/projects">项目管理</Link>,
      },
    ];

    // 数据分析 - 仅管理员可见
    if (canViewAnalytics()) {
      items.push({
        key: '/analytics',
        icon: <BarChartOutlined />,
        label: <Link to="/analytics">数据分析</Link>,
      });
    }

    // 公司管理 - 仅系统管理员可见
    if (canManageCompanies()) {
      items.push({
        key: '/companies',
        icon: <ApartmentOutlined />,
        label: <Link to="/companies">公司管理</Link>,
      });
    }

    // 用户管理 - 系统管理员和公司管理员可见
    if (canManageUsers()) {
      items.push({
        key: '/users',
        icon: <UserOutlined />,
        label: <Link to="/users">员工管理</Link>,
      });
    }

    // 添加分隔线和退出按钮
    items.push(
      {
        type: 'divider',
      },
      {
        key: 'logout',
        icon: <LogoutOutlined />,
        label: '退出登录',
        onClick: handleLogout,
        danger: true,
      }
    );

    return items;
  }, [canViewAnalytics, canManageUsers, canManageCompanies, handleLogout]);

  return (
    <div style={{ 
      height: '100vh',
      background: '#001529',
    }}>
      <div style={{
        height: '64px',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        color: '#fff',
        fontSize: '20px',
        fontWeight: 'bold',
        borderBottom: '1px solid #002140',
      }}>
        TaskFleet
      </div>
      <Menu
        mode="inline"
        theme="dark"
        selectedKeys={[location.pathname]}
        items={menuItems}
        style={{ border: 'none' }}
      />
    </div>
  );
};

export default Sidebar;

/**
 * TaskFleet - Sidebar导航组件
 */

import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import {
  DashboardOutlined,
  CheckSquareOutlined,
  ProjectOutlined,
  BarChartOutlined,
  UserOutlined,
  LogoutOutlined,
} from '@ant-design/icons';
import { Menu } from 'antd';
import type { MenuProps } from 'antd';
import { useDispatch } from 'react-redux';
import { logout } from '../../store/authSlice';

type MenuItem = Required<MenuProps>['items'][number];

const Sidebar: React.FC = () => {
  const location = useLocation();
  const dispatch = useDispatch();

  const handleLogout = () => {
    dispatch(logout());
  };

  const menuItems: MenuItem[] = [
    {
      key: '/dashboard',
      icon: <DashboardOutlined />,
      label: <Link to="/dashboard">仪表板</Link>,
    },
    {
      key: '/tasks',
      icon: <CheckSquareOutlined />,
      label: <Link to="/tasks">任务管理</Link>,
    },
    {
      key: '/projects',
      icon: <ProjectOutlined />,
      label: <Link to="/projects">项目管理</Link>,
    },
    {
      key: '/analytics',
      icon: <BarChartOutlined />,
      label: <Link to="/analytics">数据分析</Link>,
    },
    {
      key: '/users',
      icon: <UserOutlined />,
      label: <Link to="/users">员工管理</Link>,
    },
    {
      type: 'divider',
    },
    {
      key: 'logout',
      icon: <LogoutOutlined />,
      label: '退出登录',
      onClick: handleLogout,
      danger: true,
    },
  ];

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

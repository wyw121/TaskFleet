/**
 * TaskFleet - Header头部组件
 * 显示用户信息和角色
 */

import React from 'react';
import { Layout, Avatar, Dropdown, Space, Tag } from 'antd';
import { UserOutlined, SettingOutlined, LogoutOutlined } from '@ant-design/icons';
import type { MenuProps } from 'antd';
import { useSelector, useDispatch } from 'react-redux';
import { RootState } from '../../store';
import { logout } from '../../store/authSlice';
import { UserRole } from '../../types/user';

const { Header: AntHeader } = Layout;

/**
 * 获取角色显示名称和颜色
 */
const getRoleDisplay = (role: UserRole): { label: string; color: string } => {
  switch (role) {
    case UserRole.SystemAdmin:
      return { label: '系统管理员', color: 'red' };
    case UserRole.CompanyAdmin:
      return { label: '公司管理员', color: 'blue' };
    case UserRole.Employee:
      return { label: '员工', color: 'green' };
    default:
      return { label: '未知', color: 'default' };
  }
};

const Header: React.FC = () => {
  const dispatch = useDispatch();
  const user = useSelector((state: RootState) => state.auth.user);

  const handleLogout = () => {
    dispatch(logout() as any);
  };

  const menuItems: MenuProps['items'] = [
    {
      key: 'profile',
      icon: <UserOutlined />,
      label: '个人资料',
    },
    {
      key: 'settings',
      icon: <SettingOutlined />,
      label: '设置',
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

  const roleDisplay = user ? getRoleDisplay(user.role) : { label: '未知', color: 'default' };

  return (
    <AntHeader style={{
      padding: '0 24px',
      background: '#fff',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'flex-end',
      borderBottom: '1px solid #f0f0f0',
    }}>
      <Dropdown menu={{ items: menuItems }} placement="bottomRight">
        <Space style={{ cursor: 'pointer' }}>
          <Avatar icon={<UserOutlined />} />
          <Space direction="vertical" size={0} style={{ lineHeight: 1.2 }}>
            <span>{user?.full_name || user?.username || '用户'}</span>
            <Tag color={roleDisplay.color} style={{ fontSize: '12px', margin: 0 }}>
              {roleDisplay.label}
            </Tag>
          </Space>
        </Space>
      </Dropdown>
    </AntHeader>
  );
};

export default Header;

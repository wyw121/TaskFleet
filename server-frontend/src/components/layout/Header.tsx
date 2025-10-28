/**
 * TaskFleet - Header头部组件
 */

import React from 'react';
import { Layout, Avatar, Dropdown, Space } from 'antd';
import { UserOutlined, SettingOutlined, LogoutOutlined } from '@ant-design/icons';
import type { MenuProps } from 'antd';
import { useSelector, useDispatch } from 'react-redux';
import { RootState } from '../../store';
import { logout } from '../../store/authSlice';

const { Header: AntHeader } = Layout;

const Header: React.FC = () => {
  const dispatch = useDispatch();
  const user = useSelector((state: RootState) => state.auth.user);

  const handleLogout = () => {
    dispatch(logout());
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
          <span>{user?.full_name || user?.username || '用户'}</span>
        </Space>
      </Dropdown>
    </AntHeader>
  );
};

export default Header;

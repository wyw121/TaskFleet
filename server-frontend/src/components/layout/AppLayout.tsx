/**
 * TaskFleet - AppLayout主布局组件
 */

import React from 'react';
import { Layout } from 'antd';
import { Outlet } from 'react-router-dom';
import Sidebar from './Sidebar';
import Header from './Header';

const { Content } = Layout;

const AppLayout: React.FC = () => {
  return (
    <Layout style={{ minHeight: '100vh' }}>
      <Layout.Sider
        width={250}
        style={{
          overflow: 'auto',
          height: '100vh',
          position: 'fixed',
          left: 0,
          top: 0,
          bottom: 0,
        }}
      >
        <Sidebar />
      </Layout.Sider>
      <Layout style={{ marginLeft: 250 }}>
        <Header />
        <Content
          style={{
            margin: '24px',
            padding: '24px',
            background: '#fff',
            minHeight: 'calc(100vh - 112px)',
          }}
        >
          <Outlet />
        </Content>
      </Layout>
    </Layout>
  );
};

export default AppLayout;

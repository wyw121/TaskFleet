/**
 * TaskFleet - 用户管理页面
 * 支持多租户权限控制
 */

import React, { useEffect, useState } from 'react';
import { Table, Button, Space, Tag, message, Modal, Form, Input, Select, Card } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { apiClient } from '../services/api';
import { usePermissions } from '../hooks/usePermissions';
import { UserRole } from '../types/user';

interface User {
  id: number;
  username: string;
  email: string;
  full_name: string;
  role: string;
  is_active: boolean;
  created_at: string;
  last_login: string | null;
}

const UserManagement: React.FC = () => {
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState(false);
  const [isModalVisible, setIsModalVisible] = useState(false);
  const [editingUser, setEditingUser] = useState<User | null>(null);
  const [form] = Form.useForm();
  const { isSystemAdmin, canManageUsers } = usePermissions();

  // 加载用户列表
  const loadUsers = async () => {
    try {
      setLoading(true);
      const response = await apiClient.get('/api/v1/users');
      if (response.data.success) {
        setUsers(response.data.data);
      }
    } catch (error: any) {
      message.error(error.response?.data?.message || '加载用户列表失败');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadUsers();
  }, []);

  // 打开创建/编辑对话框
  const showModal = (user?: User) => {
    if (user) {
      setEditingUser(user);
      form.setFieldsValue(user);
    } else {
      setEditingUser(null);
      form.resetFields();
    }
    setIsModalVisible(true);
  };

  // 关闭对话框
  const handleCancel = () => {
    setIsModalVisible(false);
    setEditingUser(null);
    form.resetFields();
  };

  // 保存用户
  const handleSave = async () => {
    try {
      const values = await form.validateFields();
      
      if (editingUser) {
        // 更新用户
        await apiClient.put(`/api/v1/users/${editingUser.id}`, values);
        message.success('用户更新成功');
      } else {
        // 创建新用户
        await apiClient.post('/api/v1/users', values);
        message.success('用户创建成功');
      }
      
      handleCancel();
      loadUsers();
    } catch (error: any) {
      message.error(error.response?.data?.message || '保存失败');
    }
  };

  // 删除用户
  const handleDelete = (user: User) => {
    Modal.confirm({
      title: '确认删除',
      content: `确定要删除用户 "${user.username}" 吗?`,
      okText: '确定',
      cancelText: '取消',
      okType: 'danger',
      onOk: async () => {
        try {
          await apiClient.delete(`/api/v1/users/${user.id}`);
          message.success('用户删除成功');
          loadUsers();
        } catch (error: any) {
          message.error(error.response?.data?.message || '删除失败');
        }
      },
    });
  };

  // 角色标签颜色
  const getRoleColor = (role: string) => {
    const roleColors: Record<string, string> = {
      PlatformAdmin: 'red',
      ProjectManager: 'blue',
      TaskExecutor: 'green',
      // 兼容旧的角色名称
      SystemAdmin: 'red',
      CompanyAdmin: 'blue',
      Employee: 'green',
    };
    return roleColors[role] || 'default';
  };

  // 角色中文名
  const getRoleName = (role: string) => {
    const roleNames: Record<string, string> = {
      PlatformAdmin: '平台管理员',
      ProjectManager: '项目经理',
      TaskExecutor: '任务执行者',
      // 兼容旧的角色名称
      SystemAdmin: '平台管理员',
      CompanyAdmin: '项目经理',
      Employee: '任务执行者',
    };
    return roleNames[role] || role;
  };

  // 表格列定义
  const columns: ColumnsType<User> = [
    {
      title: 'ID',
      dataIndex: 'id',
      key: 'id',
      width: 80,
    },
    {
      title: '用户名',
      dataIndex: 'username',
      key: 'username',
    },
    {
      title: '邮箱',
      dataIndex: 'email',
      key: 'email',
    },
    {
      title: '姓名',
      dataIndex: 'full_name',
      key: 'full_name',
      render: (text) => text || '-',
    },
    {
      title: '角色',
      dataIndex: 'role',
      key: 'role',
      render: (role) => (
        <Tag color={getRoleColor(role)}>
          {getRoleName(role)}
        </Tag>
      ),
    },
    {
      title: '状态',
      dataIndex: 'is_active',
      key: 'is_active',
      render: (isActive) => (
        <Tag color={isActive ? 'green' : 'red'}>
          {isActive ? '启用' : '禁用'}
        </Tag>
      ),
    },
    {
      title: '创建时间',
      dataIndex: 'created_at',
      key: 'created_at',
      render: (text) => text ? new Date(text).toLocaleString('zh-CN') : '-',
    },
    {
      title: '最后登录',
      dataIndex: 'last_login',
      key: 'last_login',
      render: (text) => text ? new Date(text).toLocaleString('zh-CN') : '从未登录',
    },
    {
      title: '操作',
      key: 'action',
      fixed: 'right',
      width: 150,
      render: (_, record) => (
        <Space size="small">
          <Button
            type="link"
            size="small"
            icon={<EditOutlined />}
            onClick={() => showModal(record)}
            disabled={!canManageUsers()}
          >
            编辑
          </Button>
          <Button
            type="link"
            size="small"
            danger
            icon={<DeleteOutlined />}
            onClick={() => handleDelete(record)}
            disabled={!isSystemAdmin()} // 仅系统管理员可以删除用户
          >
            删除
          </Button>
        </Space>
      ),
    },
  ];

  return (
    <div>
      <Card
        title="用户管理"
        extra={
          canManageUsers() && (
            <Button
              type="primary"
              icon={<PlusOutlined />}
              onClick={() => showModal()}
            >
              新建用户
            </Button>
          )
        }
      >
        <Table
          columns={columns}
          dataSource={users}
          rowKey="id"
          loading={loading}
          pagination={{
            pageSize: 10,
            showTotal: (total) => `共 ${total} 个用户`,
            showSizeChanger: true,
            showQuickJumper: true,
          }}
          scroll={{ x: 1200 }}
        />
      </Card>

      {/* 创建/编辑用户对话框 */}
      <Modal
        title={editingUser ? '编辑用户' : '新建用户'}
        open={isModalVisible}
        onOk={handleSave}
        onCancel={handleCancel}
        okText="保存"
        cancelText="取消"
        width={600}
      >
        <Form
          form={form}
          layout="vertical"
          autoComplete="off"
        >
          <Form.Item
            label="用户名"
            name="username"
            rules={[
              { required: true, message: '请输入用户名' },
              { min: 3, message: '用户名至少3个字符' },
              { max: 50, message: '用户名最多50个字符' },
            ]}
          >
            <Input placeholder="请输入用户名" disabled={!!editingUser} />
          </Form.Item>

          <Form.Item
            label="邮箱"
            name="email"
            rules={[
              { required: true, message: '请输入邮箱' },
              { type: 'email', message: '请输入有效的邮箱地址' },
            ]}
          >
            <Input placeholder="请输入邮箱" />
          </Form.Item>

          <Form.Item
            label="姓名"
            name="full_name"
          >
            <Input placeholder="请输入姓名(可选)" />
          </Form.Item>

          {!editingUser && (
            <Form.Item
              label="密码"
              name="password"
              rules={[
                { required: true, message: '请输入密码' },
                { min: 6, message: '密码至少6个字符' },
              ]}
            >
              <Input.Password placeholder="请输入密码" />
            </Form.Item>
          )}

          <Form.Item
            label="角色"
            name="role"
            rules={[{ required: true, message: '请选择角色' }]}
          >
            <Select placeholder="请选择角色">
              <Select.Option value="PlatformAdmin">平台管理员</Select.Option>
              <Select.Option value="ProjectManager">项目经理</Select.Option>
              <Select.Option value="TaskExecutor">任务执行者</Select.Option>
            </Select>
          </Form.Item>

          <Form.Item
            label="状态"
            name="is_active"
            initialValue={true}
            rules={[{ required: true, message: '请选择状态' }]}
          >
            <Select>
              <Select.Option value={true}>启用</Select.Option>
              <Select.Option value={false}>禁用</Select.Option>
            </Select>
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
};

export default UserManagement;

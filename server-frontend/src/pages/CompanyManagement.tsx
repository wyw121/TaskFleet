/**
 * TaskFleet - 公司管理页面
 * 仅平台管理员可访问
 */

import React, { useEffect, useState } from 'react';
import { Table, Button, Modal, Form, Input, Space, message, Tag } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import axios from '../services/api';

interface Company {
  id: number;
  name: string;
  code: string;
  description?: string;
  is_active: boolean;
  created_at: string;
  updated_at?: string;
}

const CompanyManagement: React.FC = () => {
  const [companies, setCompanies] = useState<Company[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [editingCompany, setEditingCompany] = useState<Company | null>(null);
  const [form] = Form.useForm();

  // 加载公司列表
  const loadCompanies = async () => {
    try {
      setLoading(true);
      const response = await axios.get('/api/v1/companies');
      if (response.data.success) {
        setCompanies(response.data.data);
      }
    } catch (error: any) {
      message.error(error.response?.data?.message || '加载公司列表失败');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadCompanies();
  }, []);

  // 打开新建/编辑对话框
  const handleOpenModal = (company?: Company) => {
    if (company) {
      setEditingCompany(company);
      form.setFieldsValue(company);
    } else {
      setEditingCompany(null);
      form.resetFields();
    }
    setModalVisible(true);
  };

  // 关闭对话框
  const handleCloseModal = () => {
    setModalVisible(false);
    setEditingCompany(null);
    form.resetFields();
  };

  // 保存公司
  const handleSave = async () => {
    try {
      const values = await form.validateFields();
      
      if (editingCompany) {
        // 更新
        await axios.put(`/api/v1/companies/${editingCompany.id}`, values);
        message.success('公司更新成功');
      } else {
        // 创建
        await axios.post('/api/v1/companies', values);
        message.success('公司创建成功');
      }
      
      handleCloseModal();
      loadCompanies();
    } catch (error: any) {
      message.error(error.response?.data?.message || '保存失败');
    }
  };

  // 删除公司
  const handleDelete = (company: Company) => {
    Modal.confirm({
      title: '确认删除',
      content: `确定要删除公司 "${company.name}" 吗？`,
      okText: '确定',
      cancelText: '取消',
      okType: 'danger',
      onOk: async () => {
        try {
          await axios.delete(`/api/v1/companies/${company.id}`);
          message.success('公司删除成功');
          loadCompanies();
        } catch (error: any) {
          message.error(error.response?.data?.message || '删除失败');
        }
      },
    });
  };

  const columns: ColumnsType<Company> = [
    {
      title: 'ID',
      dataIndex: 'id',
      key: 'id',
      width: 80,
    },
    {
      title: '公司名称',
      dataIndex: 'name',
      key: 'name',
    },
    {
      title: '公司代码',
      dataIndex: 'code',
      key: 'code',
    },
    {
      title: '描述',
      dataIndex: 'description',
      key: 'description',
      render: (text) => text || '-',
    },
    {
      title: '状态',
      dataIndex: 'is_active',
      key: 'is_active',
      render: (isActive: boolean) => (
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
            onClick={() => handleOpenModal(record)}
          >
            编辑
          </Button>
          <Button
            type="link"
            size="small"
            danger
            icon={<DeleteOutlined />}
            onClick={() => handleDelete(record)}
          >
            删除
          </Button>
        </Space>
      ),
    },
  ];

  return (
    <div>
      <div style={{ marginBottom: 16, display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <h1 style={{ margin: 0 }}>公司管理</h1>
        <Button
          type="primary"
          icon={<PlusOutlined />}
          onClick={() => handleOpenModal()}
        >
          新建公司
        </Button>
      </div>

      <Table
        columns={columns}
        dataSource={companies}
        rowKey="id"
        loading={loading}
        pagination={{
          pageSize: 10,
          showTotal: (total) => `共 ${total} 家公司`,
          showSizeChanger: true,
          showQuickJumper: true,
        }}
        scroll={{ x: 1000 }}
      />

      <Modal
        title={editingCompany ? '编辑公司' : '新建公司'}
        open={modalVisible}
        onOk={handleSave}
        onCancel={handleCloseModal}
        okText="保存"
        cancelText="取消"
        width={600}
      >
        <Form form={form} layout="vertical" autoComplete="off">
          <Form.Item
            label="公司名称"
            name="name"
            rules={[
              { required: true, message: '请输入公司名称' },
              { min: 2, message: '公司名称至少2个字符' },
              { max: 100, message: '公司名称最多100个字符' },
            ]}
          >
            <Input placeholder="请输入公司名称" />
          </Form.Item>

          <Form.Item
            label="公司代码"
            name="code"
            rules={[
              { required: true, message: '请输入公司代码' },
              { min: 2, message: '公司代码至少2个字符' },
              { max: 50, message: '公司代码最多50个字符' },
              { pattern: /^[a-zA-Z0-9_-]+$/, message: '公司代码只能包含字母、数字、下划线和横线' },
            ]}
          >
            <Input placeholder="请输入公司代码（唯一标识）" disabled={!!editingCompany} />
          </Form.Item>

          <Form.Item
            label="描述"
            name="description"
          >
            <Input.TextArea rows={4} placeholder="请输入公司描述（可选）" />
          </Form.Item>

          <Form.Item
            label="状态"
            name="is_active"
            initialValue={true}
            rules={[{ required: true, message: '请选择状态' }]}
          >
            <Input.Group>
              <Button
                type={form.getFieldValue('is_active') === true ? 'primary' : 'default'}
                onClick={() => form.setFieldValue('is_active', true)}
              >
                启用
              </Button>
              <Button
                type={form.getFieldValue('is_active') === false ? 'primary' : 'default'}
                onClick={() => form.setFieldValue('is_active', false)}
                style={{ marginLeft: 8 }}
              >
                禁用
              </Button>
            </Input.Group>
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
};

export default CompanyManagement;

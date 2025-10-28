/**
 * TaskFleet - 项目管理页面
 */

import React, { useEffect, useState } from 'react';
import { useDispatch, useSelector } from 'react-redux';
import {
  Table,
  Button,
  Space,
  Tag,
  Input,
  Modal,
  Form,
  DatePicker,
  message,
  Progress,
} from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined, PlayCircleOutlined, PauseCircleOutlined, CheckCircleOutlined } from '@ant-design/icons';
import { AppDispatch, RootState } from '../store';
import {
  fetchProjects,
  createProject,
  updateProject,
  deleteProject,
  startProject,
  holdProject,
  completeProject,
} from '../store/projectSlice';
import { ProjectStatus, Project, CreateProjectRequest, UpdateProjectRequest } from '../types/project';
import dayjs from 'dayjs';

const { Search } = Input;

const ProjectManagement: React.FC = () => {
  const dispatch = useDispatch<AppDispatch>();
  const { projects, loading } = useSelector((state: RootState) => state.project);
  const [isModalVisible, setIsModalVisible] = useState(false);
  const [editingProject, setEditingProject] = useState<Project | null>(null);
  const [form] = Form.useForm();

  useEffect(() => {
    dispatch(fetchProjects());
  }, [dispatch]);

  const handleCreateProject = () => {
    setEditingProject(null);
    form.resetFields();
    setIsModalVisible(true);
  };

  const handleEditProject = (project: Project) => {
    setEditingProject(project);
    form.setFieldsValue({
      ...project,
      start_date: project.start_date ? dayjs(project.start_date) : null,
      end_date: project.end_date ? dayjs(project.end_date) : null,
    });
    setIsModalVisible(true);
  };

  const handleDeleteProject = (projectId: number) => {
    Modal.confirm({
      title: '确认删除',
      content: '确定要删除这个项目吗？此操作无法撤销。',
      onOk: async () => {
        try {
          await dispatch(deleteProject(projectId)).unwrap();
          message.success('项目删除成功');
        } catch (error) {
          message.error('项目删除失败');
        }
      },
    });
  };

  const handleStartProject = async (projectId: number) => {
    try {
      await dispatch(startProject(projectId)).unwrap();
      message.success('项目已启动');
    } catch (error) {
      message.error('操作失败');
    }
  };

  const handleHoldProject = async (projectId: number) => {
    try {
      await dispatch(holdProject(projectId)).unwrap();
      message.success('项目已暂停');
    } catch (error) {
      message.error('操作失败');
    }
  };

  const handleCompleteProject = async (projectId: number) => {
    try {
      await dispatch(completeProject(projectId)).unwrap();
      message.success('项目已完成');
    } catch (error) {
      message.error('操作失败');
    }
  };

  const handleModalOk = async () => {
    try {
      const values = await form.validateFields();
      const projectData = {
        ...values,
        start_date: values.start_date ? values.start_date.format('YYYY-MM-DD') : null,
        end_date: values.end_date ? values.end_date.format('YYYY-MM-DD') : null,
      };

      if (editingProject) {
        await dispatch(updateProject({ id: editingProject.id, data: projectData as UpdateProjectRequest })).unwrap();
        message.success('项目更新成功');
      } else {
        await dispatch(createProject(projectData as CreateProjectRequest)).unwrap();
        message.success('项目创建成功');
      }

      setIsModalVisible(false);
      form.resetFields();
    } catch (error) {
      message.error('操作失败');
    }
  };

  const columns = [
    {
      title: 'ID',
      dataIndex: 'id',
      key: 'id',
      width: 80,
    },
    {
      title: '项目名称',
      dataIndex: 'name',
      key: 'name',
      width: 200,
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      width: 100,
      render: (status: ProjectStatus) => {
        const statusConfig: Record<ProjectStatus, { color: string; text: string }> = {
          [ProjectStatus.Planning]: { color: 'default', text: '规划中' },
          [ProjectStatus.Active]: { color: 'processing', text: '进行中' },
          [ProjectStatus.OnHold]: { color: 'warning', text: '已暂停' },
          [ProjectStatus.Completed]: { color: 'success', text: '已完成' },
          [ProjectStatus.Cancelled]: { color: 'error', text: '已取消' },
        };
        const config = statusConfig[status];
        return <Tag color={config.color}>{config.text}</Tag>;
      },
    },
    {
      title: '进度',
      dataIndex: 'progress',
      key: 'progress',
      width: 150,
      render: (progress: number) => <Progress percent={Math.round(progress)} size="small" />,
    },
    {
      title: '开始日期',
      dataIndex: 'start_date',
      key: 'start_date',
      width: 120,
      render: (date: string | null) => (date ? dayjs(date).format('YYYY-MM-DD') : '-'),
    },
    {
      title: '结束日期',
      dataIndex: 'end_date',
      key: 'end_date',
      width: 120,
      render: (date: string | null) => (date ? dayjs(date).format('YYYY-MM-DD') : '-'),
    },
    {
      title: '操作',
      key: 'action',
      width: 250,
      render: (_: any, record: Project) => (
        <Space size="small">
          {record.status === ProjectStatus.Planning && (
            <Button
              type="link"
              icon={<PlayCircleOutlined />}
              onClick={() => handleStartProject(record.id)}
            >
              启动
            </Button>
          )}
          {record.status === ProjectStatus.Active && (
            <>
              <Button
                type="link"
                icon={<PauseCircleOutlined />}
                onClick={() => handleHoldProject(record.id)}
              >
                暂停
              </Button>
              <Button
                type="link"
                icon={<CheckCircleOutlined />}
                onClick={() => handleCompleteProject(record.id)}
              >
                完成
              </Button>
            </>
          )}
          <Button
            type="link"
            icon={<EditOutlined />}
            onClick={() => handleEditProject(record)}
          >
            编辑
          </Button>
          <Button
            type="link"
            danger
            icon={<DeleteOutlined />}
            onClick={() => handleDeleteProject(record.id)}
          >
            删除
          </Button>
        </Space>
      ),
    },
  ];

  return (
    <div>
      <div style={{ marginBottom: '16px', display: 'flex', justifyContent: 'space-between' }}>
        <Search
          placeholder="搜索项目名称"
          style={{ width: 250 }}
        />
        <Button type="primary" icon={<PlusOutlined />} onClick={handleCreateProject}>
          新建项目
        </Button>
      </div>

      <Table
        columns={columns}
        dataSource={projects}
        rowKey="id"
        loading={loading}
        pagination={{ pageSize: 10 }}
      />

      <Modal
        title={editingProject ? '编辑项目' : '新建项目'}
        open={isModalVisible}
        onOk={handleModalOk}
        onCancel={() => setIsModalVisible(false)}
        width={600}
      >
        <Form form={form} layout="vertical">
          <Form.Item
            name="name"
            label="项目名称"
            rules={[{ required: true, message: '请输入项目名称' }]}
          >
            <Input placeholder="请输入项目名称" />
          </Form.Item>
          <Form.Item name="description" label="项目描述">
            <Input.TextArea rows={4} placeholder="请输入项目描述" />
          </Form.Item>
          <Form.Item name="start_date" label="开始日期">
            <DatePicker style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item name="end_date" label="结束日期">
            <DatePicker style={{ width: '100%' }} />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
};

export default ProjectManagement;

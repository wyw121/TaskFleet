/**
 * TaskFleet - 任务管理页面
 */

import React, { useEffect, useState } from 'react';
import { useDispatch, useSelector } from 'react-redux';
import {
  Table,
  Button,
  Space,
  Tag,
  Input,
  Select,
  Modal,
  Form,
  DatePicker,
  InputNumber,
  message,
} from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined, PlayCircleOutlined, CheckCircleOutlined } from '@ant-design/icons';
import { AppDispatch, RootState } from '../store';
import {
  fetchTasks,
  createTask,
  updateTask,
  deleteTask,
  startTask,
  completeTask,
  setFilters,
} from '../store/taskSlice';
import { TaskStatus, TaskPriority, Task, CreateTaskRequest, UpdateTaskRequest } from '../types/task';
import dayjs from 'dayjs';

const { Search } = Input;
const { Option } = Select;

const TaskManagement: React.FC = () => {
  const dispatch = useDispatch<AppDispatch>();
  const { tasks, loading, filters } = useSelector((state: RootState) => state.task);
  const [isModalVisible, setIsModalVisible] = useState(false);
  const [editingTask, setEditingTask] = useState<Task | null>(null);
  const [form] = Form.useForm();

  useEffect(() => {
    dispatch(fetchTasks(filters));
  }, [dispatch, filters]);

  const handleCreateTask = () => {
    setEditingTask(null);
    form.resetFields();
    setIsModalVisible(true);
  };

  const handleEditTask = (task: Task) => {
    setEditingTask(task);
    form.setFieldsValue({
      ...task,
      due_date: task.due_date ? dayjs(task.due_date) : null,
    });
    setIsModalVisible(true);
  };

  const handleDeleteTask = (taskId: number) => {
    Modal.confirm({
      title: '确认删除',
      content: '确定要删除这个任务吗？此操作无法撤销。',
      onOk: async () => {
        try {
          await dispatch(deleteTask(taskId)).unwrap();
          message.success('任务删除成功');
        } catch (error) {
          message.error('任务删除失败');
        }
      },
    });
  };

  const handleStartTask = async (taskId: number) => {
    try {
      await dispatch(startTask(taskId)).unwrap();
      message.success('任务已开始');
    } catch (error) {
      message.error('操作失败');
    }
  };

  const handleCompleteTask = async (taskId: number) => {
    try {
      await dispatch(completeTask(taskId)).unwrap();
      message.success('任务已完成');
    } catch (error) {
      message.error('操作失败');
    }
  };

  const handleModalOk = async () => {
    try {
      const values = await form.validateFields();
      const taskData = {
        ...values,
        due_date: values.due_date ? values.due_date.format('YYYY-MM-DD HH:mm:ss') : null,
      };

      if (editingTask) {
        await dispatch(updateTask({ id: editingTask.id, data: taskData as UpdateTaskRequest })).unwrap();
        message.success('任务更新成功');
      } else {
        await dispatch(createTask(taskData as CreateTaskRequest)).unwrap();
        message.success('任务创建成功');
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
      title: '标题',
      dataIndex: 'title',
      key: 'title',
      width: 200,
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      width: 100,
      render: (status: TaskStatus) => {
        const statusConfig: Record<TaskStatus, { color: string; text: string }> = {
          [TaskStatus.Pending]: { color: 'default', text: '待处理' },
          [TaskStatus.InProgress]: { color: 'processing', text: '进行中' },
          [TaskStatus.Completed]: { color: 'success', text: '已完成' },
          [TaskStatus.Cancelled]: { color: 'error', text: '已取消' },
        };
        const config = statusConfig[status];
        return <Tag color={config.color}>{config.text}</Tag>;
      },
    },
    {
      title: '优先级',
      dataIndex: 'priority',
      key: 'priority',
      width: 100,
      render: (priority: TaskPriority) => {
        const priorityConfig: Record<TaskPriority, { color: string; text: string }> = {
          [TaskPriority.Low]: { color: 'blue', text: '低' },
          [TaskPriority.Medium]: { color: 'orange', text: '中' },
          [TaskPriority.High]: { color: 'red', text: '高' },
          [TaskPriority.Urgent]: { color: 'volcano', text: '紧急' },
        };
        const config = priorityConfig[priority];
        return <Tag color={config.color}>{config.text}</Tag>;
      },
    },
    {
      title: '截止日期',
      dataIndex: 'due_date',
      key: 'due_date',
      width: 180,
      render: (date: string | null) => (date ? dayjs(date).format('YYYY-MM-DD HH:mm') : '-'),
    },
    {
      title: '操作',
      key: 'action',
      width: 200,
      render: (_: any, record: Task) => (
        <Space size="small">
          {record.status === TaskStatus.Pending && (
            <Button
              type="link"
              icon={<PlayCircleOutlined />}
              onClick={() => handleStartTask(record.id)}
            >
              开始
            </Button>
          )}
          {record.status === TaskStatus.InProgress && (
            <Button
              type="link"
              icon={<CheckCircleOutlined />}
              onClick={() => handleCompleteTask(record.id)}
            >
              完成
            </Button>
          )}
          <Button
            type="link"
            icon={<EditOutlined />}
            onClick={() => handleEditTask(record)}
          >
            编辑
          </Button>
          <Button
            type="link"
            danger
            icon={<DeleteOutlined />}
            onClick={() => handleDeleteTask(record.id)}
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
        <Space>
          <Search
            placeholder="搜索任务标题"
            onSearch={(value) => dispatch(setFilters({ ...filters, search: value }))}
            style={{ width: 250 }}
          />
          <Select
            placeholder="状态筛选"
            allowClear
            style={{ width: 150 }}
            onChange={(value) => dispatch(setFilters({ ...filters, status: value }))}
          >
            <Option value={TaskStatus.Pending}>待处理</Option>
            <Option value={TaskStatus.InProgress}>进行中</Option>
            <Option value={TaskStatus.Completed}>已完成</Option>
            <Option value={TaskStatus.Cancelled}>已取消</Option>
          </Select>
          <Select
            placeholder="优先级筛选"
            allowClear
            style={{ width: 150 }}
            onChange={(value) => dispatch(setFilters({ ...filters, priority: value }))}
          >
            <Option value={TaskPriority.Low}>低</Option>
            <Option value={TaskPriority.Medium}>中</Option>
            <Option value={TaskPriority.High}>高</Option>
            <Option value={TaskPriority.Urgent}>紧急</Option>
          </Select>
        </Space>
        <Button type="primary" icon={<PlusOutlined />} onClick={handleCreateTask}>
          新建任务
        </Button>
      </div>

      <Table
        columns={columns}
        dataSource={tasks}
        rowKey="id"
        loading={loading}
        pagination={{ pageSize: 10 }}
      />

      <Modal
        title={editingTask ? '编辑任务' : '新建任务'}
        open={isModalVisible}
        onOk={handleModalOk}
        onCancel={() => setIsModalVisible(false)}
        width={600}
      >
        <Form form={form} layout="vertical">
          <Form.Item
            name="title"
            label="任务标题"
            rules={[{ required: true, message: '请输入任务标题' }]}
          >
            <Input placeholder="请输入任务标题" />
          </Form.Item>
          <Form.Item name="description" label="任务描述">
            <Input.TextArea rows={4} placeholder="请输入任务描述" />
          </Form.Item>
          <Form.Item
            name="priority"
            label="优先级"
            rules={[{ required: true, message: '请选择优先级' }]}
          >
            <Select>
              <Option value={TaskPriority.Low}>低</Option>
              <Option value={TaskPriority.Medium}>中</Option>
              <Option value={TaskPriority.High}>高</Option>
              <Option value={TaskPriority.Urgent}>紧急</Option>
            </Select>
          </Form.Item>
          <Form.Item name="due_date" label="截止日期">
            <DatePicker showTime style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item name="estimated_hours" label="预估工时">
            <InputNumber min={0} step={0.5} style={{ width: '100%' }} />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
};

export default TaskManagement;

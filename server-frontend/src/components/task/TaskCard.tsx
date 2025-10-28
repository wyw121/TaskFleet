/**
 * TaskFleet - 任务卡片组件
 */

import React from 'react';
import { Card, Tag, Space, Button } from 'antd';
import { ClockCircleOutlined, UserOutlined } from '@ant-design/icons';
import { Task, TaskStatus, TaskPriority } from '../../types/task';
import dayjs from 'dayjs';

interface TaskCardProps {
  task: Task;
  onStart?: (taskId: number) => void;
  onComplete?: (taskId: number) => void;
  onEdit?: (task: Task) => void;
}

const TaskCard: React.FC<TaskCardProps> = ({ task, onStart, onComplete, onEdit }) => {
  const statusConfig: Record<TaskStatus, { color: string; text: string }> = {
    [TaskStatus.Pending]: { color: 'default', text: '待处理' },
    [TaskStatus.InProgress]: { color: 'processing', text: '进行中' },
    [TaskStatus.Completed]: { color: 'success', text: '已完成' },
    [TaskStatus.Cancelled]: { color: 'error', text: '已取消' },
  };

  const priorityConfig: Record<TaskPriority, { color: string; text: string }> = {
    [TaskPriority.Low]: { color: 'blue', text: '低' },
    [TaskPriority.Medium]: { color: 'orange', text: '中' },
    [TaskPriority.High]: { color: 'red', text: '高' },
    [TaskPriority.Urgent]: { color: 'volcano', text: '紧急' },
  };

  return (
    <Card
      hoverable
      style={{ marginBottom: '16px' }}
      extra={
        <Space>
          <Tag color={statusConfig[task.status].color}>
            {statusConfig[task.status].text}
          </Tag>
          <Tag color={priorityConfig[task.priority].color}>
            {priorityConfig[task.priority].text}
          </Tag>
        </Space>
      }
      actions={[
        task.status === TaskStatus.Pending && onStart && (
          <Button type="link" onClick={() => onStart(task.id)}>
            开始任务
          </Button>
        ),
        task.status === TaskStatus.InProgress && onComplete && (
          <Button type="link" onClick={() => onComplete(task.id)}>
            完成任务
          </Button>
        ),
        onEdit && (
          <Button type="link" onClick={() => onEdit(task)}>
            编辑
          </Button>
        ),
      ].filter(Boolean)}
    >
      <Card.Meta
        title={task.title}
        description={
          <div>
            <p>{task.description || '无描述'}</p>
            <Space style={{ marginTop: '12px' }}>
              {task.due_date && (
                <span>
                  <ClockCircleOutlined /> {dayjs(task.due_date).format('YYYY-MM-DD HH:mm')}
                </span>
              )}
              {task.assigned_to && (
                <span>
                  <UserOutlined /> 负责人: {task.assigned_to}
                </span>
              )}
            </Space>
          </div>
        }
      />
    </Card>
  );
};

export default TaskCard;

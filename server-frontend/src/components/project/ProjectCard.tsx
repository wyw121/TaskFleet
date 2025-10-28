/**
 * TaskFleet - 项目卡片组件
 */

import React from 'react';
import { Card, Tag, Progress, Space, Button } from 'antd';
import { CalendarOutlined, UserOutlined } from '@ant-design/icons';
import { Project, ProjectStatus } from '../../types/project';
import dayjs from 'dayjs';

interface ProjectCardProps {
  project: Project;
  onStart?: (projectId: number) => void;
  onComplete?: (projectId: number) => void;
  onEdit?: (project: Project) => void;
}

const ProjectCard: React.FC<ProjectCardProps> = ({ project, onStart, onComplete, onEdit }) => {
  const statusConfig: Record<ProjectStatus, { color: string; text: string }> = {
    [ProjectStatus.Planning]: { color: 'default', text: '规划中' },
    [ProjectStatus.Active]: { color: 'processing', text: '进行中' },
    [ProjectStatus.OnHold]: { color: 'warning', text: '已暂停' },
    [ProjectStatus.Completed]: { color: 'success', text: '已完成' },
    [ProjectStatus.Cancelled]: { color: 'error', text: '已取消' },
  };

  return (
    <Card
      hoverable
      style={{ marginBottom: '16px' }}
      extra={
        <Tag color={statusConfig[project.status].color}>
          {statusConfig[project.status].text}
        </Tag>
      }
      actions={[
        project.status === ProjectStatus.Planning && onStart && (
          <Button type="link" onClick={() => onStart(project.id)}>
            启动项目
          </Button>
        ),
        project.status === ProjectStatus.Active && onComplete && (
          <Button type="link" onClick={() => onComplete(project.id)}>
            完成项目
          </Button>
        ),
        onEdit && (
          <Button type="link" onClick={() => onEdit(project)}>
            编辑
          </Button>
        ),
      ].filter(Boolean)}
    >
      <Card.Meta
        title={project.name}
        description={
          <div>
            <p>{project.description || '无描述'}</p>
            <Progress
              percent={Math.round(project.progress)}
              style={{ marginTop: '12px', marginBottom: '12px' }}
            />
            <Space>
              {project.start_date && (
                <span>
                  <CalendarOutlined /> 开始: {dayjs(project.start_date).format('YYYY-MM-DD')}
                </span>
              )}
              {project.end_date && (
                <span>
                  <CalendarOutlined /> 结束: {dayjs(project.end_date).format('YYYY-MM-DD')}
                </span>
              )}
              {project.owner_id && (
                <span>
                  <UserOutlined /> 负责人: {project.owner_id}
                </span>
              )}
            </Space>
          </div>
        }
      />
    </Card>
  );
};

export default ProjectCard;

/**
 * TaskFleet - Dashboard仪表板页面
 */

import React, { useEffect } from 'react';
import { Row, Col, Card, Statistic, Progress, Spin } from 'antd';
import {
  CheckCircleOutlined,
  ClockCircleOutlined,
  LoadingOutlined,
  ProjectOutlined,
} from '@ant-design/icons';
import analyticsService from '../services/analyticsService';

const Dashboard: React.FC = () => {
  const [loading, setLoading] = React.useState(true);
  const [taskStats, setTaskStats] = React.useState<any>(null);
  const [projectStats, setProjectStats] = React.useState<any>(null);

  useEffect(() => {
    loadDashboardData();
  }, []);

  const loadDashboardData = async () => {
    try {
      setLoading(true);
      const data = await analyticsService.getDashboardData();
      setTaskStats(data.task_statistics);
      setProjectStats(data.project_statistics);
    } catch (error) {
      console.error('Failed to load dashboard data:', error);
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return (
      <div style={{ textAlign: 'center', padding: '50px' }}>
        <Spin size="large" />
      </div>
    );
  }

  const taskCompletionRate = taskStats
    ? Math.round((taskStats.completed_tasks / taskStats.total_tasks) * 100) || 0
    : 0;

  return (
    <div>
      <h1 style={{ marginBottom: '24px' }}>仪表板</h1>

      {/* 任务统计卡片 */}
      <Row gutter={[16, 16]} style={{ marginBottom: '24px' }}>
        <Col xs={24} sm={12} lg={6}>
          <Card>
            <Statistic
              title="总任务数"
              value={taskStats?.total_tasks || 0}
              prefix={<CheckCircleOutlined />}
            />
          </Card>
        </Col>
        <Col xs={24} sm={12} lg={6}>
          <Card>
            <Statistic
              title="已完成"
              value={taskStats?.completed_tasks || 0}
              valueStyle={{ color: '#3f8600' }}
              prefix={<CheckCircleOutlined />}
            />
          </Card>
        </Col>
        <Col xs={24} sm={12} lg={6}>
          <Card>
            <Statistic
              title="进行中"
              value={taskStats?.in_progress_tasks || 0}
              valueStyle={{ color: '#1890ff' }}
              prefix={<LoadingOutlined />}
            />
          </Card>
        </Col>
        <Col xs={24} sm={12} lg={6}>
          <Card>
            <Statistic
              title="待处理"
              value={taskStats?.pending_tasks || 0}
              valueStyle={{ color: '#faad14' }}
              prefix={<ClockCircleOutlined />}
            />
          </Card>
        </Col>
      </Row>

      {/* 项目统计卡片 */}
      <Row gutter={[16, 16]} style={{ marginBottom: '24px' }}>
        <Col xs={24} sm={12} lg={6}>
          <Card>
            <Statistic
              title="总项目数"
              value={projectStats?.total_projects || 0}
              prefix={<ProjectOutlined />}
            />
          </Card>
        </Col>
        <Col xs={24} sm={12} lg={6}>
          <Card>
            <Statistic
              title="进行中"
              value={projectStats?.active_projects || 0}
              valueStyle={{ color: '#1890ff' }}
            />
          </Card>
        </Col>
        <Col xs={24} sm={12} lg={6}>
          <Card>
            <Statistic
              title="已完成"
              value={projectStats?.completed_projects || 0}
              valueStyle={{ color: '#3f8600' }}
            />
          </Card>
        </Col>
        <Col xs={24} sm={12} lg={6}>
          <Card>
            <Statistic
              title="规划中"
              value={projectStats?.planning_projects || 0}
            />
          </Card>
        </Col>
      </Row>

      {/* 任务完成率 */}
      <Row gutter={[16, 16]}>
        <Col xs={24} lg={12}>
          <Card title="任务完成率">
            <Progress
              type="circle"
              percent={taskCompletionRate}
              format={(percent) => `${percent}%`}
              size={200}
            />
            <div style={{ textAlign: 'center', marginTop: '16px' }}>
              <p style={{ fontSize: '16px', marginBottom: '8px' }}>
                已完成 {taskStats?.completed_tasks || 0} / {taskStats?.total_tasks || 0} 个任务
              </p>
            </div>
          </Card>
        </Col>
        <Col xs={24} lg={12}>
          <Card title="快速操作">
            <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
              <a href="/tasks">查看所有任务</a>
              <a href="/projects">查看所有项目</a>
              <a href="/analytics">查看数据分析</a>
            </div>
          </Card>
        </Col>
      </Row>
    </div>
  );
};

export default Dashboard;

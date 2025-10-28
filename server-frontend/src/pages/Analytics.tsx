/**
 * TaskFleet - 数据分析页面
 */

import React, { useEffect, useState } from 'react';
import { Card, Row, Col, Table, Spin } from 'antd';
import { Column } from '@ant-design/plots';
import analyticsService from '../services/analyticsService';
import { UserWorkloadStatistics } from '../types/analytics';

const Analytics: React.FC = () => {
  const [loading, setLoading] = useState(true);
  const [userWorkload, setUserWorkload] = useState<UserWorkloadStatistics[]>([]);

  useEffect(() => {
    loadAnalytics();
  }, []);

  const loadAnalytics = async () => {
    try {
      setLoading(true);
      const workload = await analyticsService.getUserWorkload();
      setUserWorkload(workload);
    } catch (error) {
      console.error('Failed to load analytics:', error);
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

  const chartData = userWorkload.map((item) => ({
    user: item.user_name,
    value: item.total_tasks,
    type: '总任务数',
  }));

  const chartConfig = {
    data: chartData,
    xField: 'user',
    yField: 'value',
    seriesField: 'type',
    label: {
      position: 'middle' as const,
      style: {
        fill: '#FFFFFF',
        opacity: 0.6,
      },
    },
    xAxis: {
      label: {
        autoHide: true,
        autoRotate: false,
      },
    },
  };

  const columns = [
    {
      title: '用户',
      dataIndex: 'user_name',
      key: 'user_name',
    },
    {
      title: '总任务数',
      dataIndex: 'total_tasks',
      key: 'total_tasks',
      sorter: (a: UserWorkloadStatistics, b: UserWorkloadStatistics) => a.total_tasks - b.total_tasks,
    },
    {
      title: '已完成',
      dataIndex: 'completed_tasks',
      key: 'completed_tasks',
      sorter: (a: UserWorkloadStatistics, b: UserWorkloadStatistics) => a.completed_tasks - b.completed_tasks,
    },
    {
      title: '进行中',
      dataIndex: 'in_progress_tasks',
      key: 'in_progress_tasks',
      sorter: (a: UserWorkloadStatistics, b: UserWorkloadStatistics) => a.in_progress_tasks - b.in_progress_tasks,
    },
    {
      title: '总工时',
      dataIndex: 'total_hours',
      key: 'total_hours',
      render: (hours: number) => `${hours.toFixed(2)} 小时`,
      sorter: (a: UserWorkloadStatistics, b: UserWorkloadStatistics) => a.total_hours - b.total_hours,
    },
  ];

  return (
    <div>
      <h1 style={{ marginBottom: '24px' }}>数据分析</h1>

      <Row gutter={[16, 16]}>
        <Col span={24}>
          <Card title="用户工作负载统计">
            <Column {...chartConfig} />
          </Card>
        </Col>
        <Col span={24}>
          <Card title="详细数据">
            <Table
              columns={columns}
              dataSource={userWorkload}
              rowKey="user_id"
              pagination={{ pageSize: 10 }}
            />
          </Card>
        </Col>
      </Row>
    </div>
  );
};

export default Analytics;

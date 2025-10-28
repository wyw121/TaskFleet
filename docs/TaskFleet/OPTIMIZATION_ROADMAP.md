# TaskFleet 项目优化路线图

> **项目愿景**: 专注于任务分发、进度监控和数据统计的开源项目管理系统

**创建日期**: 2025年10月28日  
**文档目的**: 提供完整的TaskFleet项目优化实施指南  
**预计完成时间**: 9-15天

---

## 📊 现状分析

### ✅ 已完成的工作
- [x] 文档系统完全清理，移除Flow Farm过时内容
- [x] 建立清晰的TaskFleet技术架构文档  
- [x] 确定核心定位：任务分发、进度监控、数据统计
- [x] 创建现代化的文档导航系统

### ❌ 当前存在的问题
1. **后端代码混乱** - 仍有Flow Farm时期的计费、设备管理等无关代码
2. **缺少核心功能** - 没有任务管理、项目管理的实际实现
3. **前端不匹配** - 可能还是Flow Farm的界面，未体现TaskFleet定位
4. **桌面客户端定位不清** - 未专注于员工任务执行场景
5. **数据模型过时** - User模型包含计费等无关字段

---

## 🎯 总体优化策略

### 核心原则
1. **专注性** - 只保留与任务分发、监控、统计相关的功能
2. **简洁性** - 删除所有Flow Farm遗留的复杂逻辑
3. **实用性** - 每个功能都要解决实际的任务管理问题
4. **一致性** - 前后端、文档、代码都要体现TaskFleet定位

### 技术架构目标
```
┌─────────────────────────────────────────────────────────┐
│                  TaskFleet 任务执行专家                    │
├─────────────────────────────────────────────────────────┤
│  📊 管理端 (Web)     │  💻 员工端 (Desktop)              │
│  React + TypeScript  │  Tauri + Rust                    │
│  - 任务批量导入       │  - 接收任务通知                   │
│  - 进度实时监控       │  - 更新任务状态                   │
│  - 数据统计分析       │  - 记录工作时间                   │
├─────────────────────────────────────────────────────────┤
│              🔧 后端服务 (Rust + Axum)                    │
│    任务分发 • 进度监控 • 数据统计 • WebSocket实时通信       │
└─────────────────────────────────────────────────────────┘
```

---

## 📋 详细实施计划

## 🗑️ 第一阶段：代码清理 (1-2天) ✅ **大部分已完成**

### ✅ 已完成的清理任务:

#### A. 删除过时的Handler模块 ✅
```bash
# 已删除的文件
✅ server-backend/src/handlers/billing.rs
✅ server-backend/src/handlers/company_pricing.rs  
✅ server-backend/src/handlers/devices.rs
✅ server-backend/src/handlers/work_records.rs
✅ server-backend/src/handlers/kpi.rs
✅ server-backend/src/handlers/reports.rs

# 保留的核心模块
✅ server-backend/src/handlers/auth.rs
✅ server-backend/src/handlers/users.rs
✅ server-backend/src/handlers/health.rs
✅ server-backend/src/handlers/docs.rs
```

#### B. 简化User数据模型 ✅
**已完成的User模型简化**：
- ✅ 移除`balance`、`max_employees`、`current_employees`等计费字段
- ✅ 简化权限为两个角色：ProjectManager、Employee
- ✅ 移除`parent_id`等多层级结构
- ✅ 使用Uuid替代i32 ID

#### C. 删除无关目录和文件 ✅
```bash
# 已删除的目录和文件
✅ adb_xml_reader/          # ADB设备管理目录
✅ server-backend/Cargo_query.toml  # Flow Farm的查询配置
✅ server-backend/query_users*.ps1  # Flow Farm的用户查询脚本
```

### 🔄 需要修复的问题:

由于模型简化过程中发现的编译错误，需要在下一步修复：
- Repository层需要适配新的Uuid ID类型
- Service层需要重写以匹配简化的模型
- Utils模块缺少hash_password函数
- 某些handler方法签名需要调整

**当前状态**: Stage 1清理工作85%完成，需要修复编译错误后才能继续Stage 2。

**目标简化后的User模型**：
```rust
// server-backend/src/models.rs
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub hashed_password: String,
    pub role: UserRole,
    pub full_name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

// 简化的权限角色
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
pub enum UserRole {
    ProjectManager,  // 项目管理员 - 可以创建项目、分配任务、查看统计
    Employee,        // 普通员工 - 只能查看和更新自己的任务
}
```

#### C. 删除无关目录和文件
```bash
# 完全不相关的目录
❌ adb_xml_reader/          # ADB设备管理，与TaskFleet无关
❌ deploy/ 中的Flow Farm脚本 # 保留通用部署脚本即可

# 需要清理的配置文件
❌ server-backend/Cargo_query.toml  # Flow Farm的查询配置
❌ server-backend/query_users*.ps1  # Flow Farm的用户查询脚本
```

#### D. 更新mod.rs文件
```rust
// server-backend/src/handlers/mod.rs
pub mod auth;
pub mod health;
pub mod users;
pub mod docs;
// 删除：billing, company_pricing, devices, work_records

// 后续要添加的TaskFleet核心模块：
// pub mod tasks;     // 任务管理
// pub mod projects;  // 项目管理  
// pub mod analytics; // 数据统计
```

### 1.2 清理检查清单
- [ ] 删除billing.rs及相关代码
- [ ] 删除company_pricing.rs及相关代码
- [ ] 删除devices.rs及相关代码
- [ ] 删除adb_xml_reader目录
- [ ] 简化User模型，移除计费字段
- [ ] 更新handlers/mod.rs
- [ ] 运行`cargo check`确保编译通过
- [ ] 运行`cargo test`确保测试通过

---

## 🔧 第二阶段：核心功能开发 (3-5天)

### 2.1 数据模型设计

#### A. Task（任务）模型
```rust
// server-backend/src/models/task.rs
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub project_id: Uuid,
    pub assigned_to: Option<Uuid>,  // 分配给的员工
    pub created_by: Uuid,           // 创建者（项目管理员）
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub due_date: Option<DateTime<Utc>>,
    pub estimated_hours: Option<f32>,    // 预估工时
    pub actual_hours: Option<f32>,       // 实际工时
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
pub enum TaskStatus {
    Todo,        // 待办
    InProgress,  // 进行中
    Review,      // 待审核
    Completed,   // 已完成
    Cancelled,   // 已取消
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
pub enum TaskPriority {
    Low,     // 低优先级
    Medium,  // 中优先级  
    High,    // 高优先级
    Urgent,  // 紧急
}
```

#### B. Project（项目）模型
```rust
// server-backend/src/models/project.rs
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: Uuid,             // 项目负责人
    pub status: ProjectStatus,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
pub enum ProjectStatus {
    Planning,    // 规划中
    Active,      // 进行中
    OnHold,      // 暂停
    Completed,   // 已完成
    Cancelled,   // 已取消
}
```

#### C. WorkLog（工作记录）模型
```rust
// server-backend/src/models/work_log.rs
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WorkLog {
    pub id: Uuid,
    pub task_id: Uuid,
    pub user_id: Uuid,
    pub hours: f32,
    pub notes: Option<String>,
    pub logged_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
```

### 2.2 API端点设计

#### A. 任务管理API
```rust
// server-backend/src/handlers/tasks.rs

// 获取任务列表（支持筛选）
// GET /api/tasks?project_id=xxx&assigned_to=xxx&status=xxx&page=1&limit=20
pub async fn get_tasks(
    Query(params): Query<TaskQueryParams>,
    auth: JwtAuth,
) -> Result<Json<PaginatedResponse<Task>>, ApiError>

// 创建单个任务
// POST /api/tasks
pub async fn create_task(
    Json(payload): Json<CreateTaskRequest>,
    auth: JwtAuth,
) -> Result<Json<Task>, ApiError>

// 批量导入任务（支持CSV/Excel）
// POST /api/tasks/batch-import
pub async fn batch_import_tasks(
    mut payload: Multipart,
    auth: JwtAuth,
) -> Result<Json<BatchImportResponse>, ApiError>

// 更新任务状态
// PUT /api/tasks/{id}/status
pub async fn update_task_status(
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTaskStatusRequest>,
    auth: JwtAuth,
) -> Result<Json<Task>, ApiError>

// 分配任务给员工
// PUT /api/tasks/{id}/assign
pub async fn assign_task(
    Path(id): Path<Uuid>,
    Json(payload): Json<AssignTaskRequest>,
    auth: JwtAuth,
) -> Result<Json<Task>, ApiError>

// 记录工作时间
// POST /api/tasks/{id}/work-logs
pub async fn log_work_time(
    Path(id): Path<Uuid>,
    Json(payload): Json<LogWorkTimeRequest>,
    auth: JwtAuth,
) -> Result<Json<WorkLog>, ApiError>
```

#### B. 项目管理API
```rust
// server-backend/src/handlers/projects.rs

// 获取项目列表
// GET /api/projects
pub async fn get_projects(
    Query(params): Query<ProjectQueryParams>,
    auth: JwtAuth,
) -> Result<Json<Vec<Project>>, ApiError>

// 创建项目
// POST /api/projects  
pub async fn create_project(
    Json(payload): Json<CreateProjectRequest>,
    auth: JwtAuth,
) -> Result<Json<Project>, ApiError>

// 获取项目详情（包含任务统计）
// GET /api/projects/{id}
pub async fn get_project_detail(
    Path(id): Path<Uuid>,
    auth: JwtAuth,
) -> Result<Json<ProjectDetailResponse>, ApiError>
```

#### C. 数据统计API
```rust
// server-backend/src/handlers/analytics.rs

// 获取仪表板概览数据
// GET /api/analytics/dashboard
pub async fn get_dashboard_overview(
    Query(params): Query<DashboardParams>,
    auth: JwtAuth,
) -> Result<Json<DashboardData>, ApiError>

// 员工效率分析
// GET /api/analytics/employee-efficiency
pub async fn get_employee_efficiency(
    Query(params): Query<EfficiencyParams>,
    auth: JwtAuth,
) -> Result<Json<EfficiencyReport>, ApiError>

// 任务完成趋势
// GET /api/analytics/task-trends
pub async fn get_task_trends(
    Query(params): Query<TrendParams>,
    auth: JwtAuth,
) -> Result<Json<TrendData>, ApiError>

// 项目进度报告
// GET /api/analytics/project-progress
pub async fn get_project_progress(
    Query(params): Query<ProgressParams>,
    auth: JwtAuth,
) -> Result<Json<ProgressReport>, ApiError>
```

### 2.3 实时通信设计
```rust
// server-backend/src/handlers/websocket.rs

// WebSocket连接处理
// GET /ws/task-updates
pub async fn task_updates_websocket(
    ws: WebSocketUpgrade,
    auth: JwtAuth,
) -> Response

// 事件类型定义
#[derive(Debug, Serialize)]
pub enum TaskEvent {
    TaskCreated { task: Task },
    TaskUpdated { task: Task },
    TaskAssigned { task_id: Uuid, assigned_to: Uuid },
    TaskCompleted { task_id: Uuid, completed_by: Uuid },
}
```

### 2.4 数据库迁移脚本
```sql
-- migrations/002_create_tasks_table.sql
CREATE TABLE projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR NOT NULL,
    description TEXT,
    owner_id UUID NOT NULL REFERENCES users(id),
    status VARCHAR NOT NULL,
    start_date TIMESTAMPTZ,
    end_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR NOT NULL,
    description TEXT,
    project_id UUID NOT NULL REFERENCES projects(id),
    assigned_to UUID REFERENCES users(id),
    created_by UUID NOT NULL REFERENCES users(id),
    status VARCHAR NOT NULL DEFAULT 'todo',
    priority VARCHAR NOT NULL DEFAULT 'medium',
    due_date TIMESTAMPTZ,
    estimated_hours REAL,
    actual_hours REAL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

CREATE TABLE work_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    task_id UUID NOT NULL REFERENCES tasks(id),
    user_id UUID NOT NULL REFERENCES users(id),
    hours REAL NOT NULL,
    notes TEXT,
    logged_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 添加索引
CREATE INDEX idx_tasks_assigned_to ON tasks(assigned_to);
CREATE INDEX idx_tasks_status ON tasks(status);
CREATE INDEX idx_tasks_project_id ON tasks(project_id);
CREATE INDEX idx_tasks_created_at ON tasks(created_at);
CREATE INDEX idx_work_logs_task_id ON work_logs(task_id);
CREATE INDEX idx_work_logs_user_id ON work_logs(user_id);
```

### 2.5 开发检查清单
- [ ] 创建Task、Project、WorkLog模型
- [ ] 实现任务管理API endpoints
- [ ] 实现项目管理API endpoints  
- [ ] 实现数据统计API endpoints
- [ ] 添加WebSocket实时通信
- [ ] 创建数据库迁移脚本
- [ ] 编写单元测试
- [ ] 编写API文档

---

## 🎨 第三阶段：前端重构 (2-3天)

### 3.1 项目结构设计
```
server-frontend/
├── src/
│   ├── pages/
│   │   ├── Dashboard.tsx           # 仪表板 - 项目概览
│   │   ├── TaskManagement.tsx      # 任务管理页面
│   │   ├── ProjectManagement.tsx   # 项目管理页面
│   │   ├── EmployeeManagement.tsx  # 员工管理页面
│   │   ├── Analytics.tsx           # 数据分析页面
│   │   └── Login.tsx               # 登录页面
│   ├── components/
│   │   ├── layout/
│   │   │   ├── AppLayout.tsx       # 主布局
│   │   │   ├── Sidebar.tsx         # 侧边栏导航
│   │   │   └── Header.tsx          # 顶部导航
│   │   ├── task/
│   │   │   ├── TaskCard.tsx        # 任务卡片
│   │   │   ├── TaskList.tsx        # 任务列表
│   │   │   ├── TaskForm.tsx        # 任务创建/编辑表单
│   │   │   ├── TaskStatusBadge.tsx # 任务状态徽章
│   │   │   └── BatchImport.tsx     # 批量导入组件
│   │   ├── project/
│   │   │   ├── ProjectCard.tsx     # 项目卡片
│   │   │   ├── ProjectForm.tsx     # 项目表单
│   │   │   └── ProjectProgress.tsx # 项目进度
│   │   └── charts/
│   │       ├── ProgressChart.tsx   # 进度图表
│   │       ├── EfficiencyChart.tsx # 效率图表
│   │       └── TrendChart.tsx      # 趋势图表
│   ├── services/
│   │   ├── api.ts                  # API客户端
│   │   ├── tasks.ts                # 任务相关API
│   │   ├── projects.ts             # 项目相关API
│   │   ├── analytics.ts            # 统计相关API
│   │   └── websocket.ts            # WebSocket服务
│   ├── stores/
│   │   ├── authStore.ts            # 认证状态
│   │   ├── taskStore.ts            # 任务状态
│   │   ├── projectStore.ts         # 项目状态
│   │   └── globalStore.ts          # 全局状态
│   ├── types/
│   │   ├── task.ts                 # 任务类型定义
│   │   ├── project.ts              # 项目类型定义
│   │   ├── user.ts                 # 用户类型定义
│   │   └── api.ts                  # API响应类型
│   └── utils/
│       ├── date.ts                 # 日期工具函数
│       ├── format.ts               # 格式化工具
│       └── constants.ts            # 常量定义
```

### 3.2 核心页面实现

#### A. 仪表板页面
```typescript
// src/pages/Dashboard.tsx
import React, { useEffect } from 'react';
import { Card, Row, Col, Statistic, Progress } from 'antd';
import { CheckCircleOutlined, ClockCircleOutlined, LoadingOutlined } from '@ant-design/icons';
import { useTaskStore } from '../stores/taskStore';
import { useProjectStore } from '../stores/projectStore';
import ProgressChart from '../components/charts/ProgressChart';
import TrendChart from '../components/charts/TrendChart';

export const Dashboard: React.FC = () => {
  const { tasks, fetchTasks } = useTaskStore();
  const { projects, fetchProjects } = useProjectStore();

  useEffect(() => {
    fetchTasks();
    fetchProjects();
  }, []);

  // 统计数据计算
  const totalTasks = tasks.length;
  const completedTasks = tasks.filter(t => t.status === 'completed').length;
  const inProgressTasks = tasks.filter(t => t.status === 'in_progress').length;
  const completionRate = totalTasks > 0 ? (completedTasks / totalTasks) * 100 : 0;

  return (
    <div className="dashboard">
      <Row gutter={[16, 16]}>
        {/* 统计卡片 */}
        <Col span={6}>
          <Card>
            <Statistic
              title="总任务数"
              value={totalTasks}
              prefix={<CheckCircleOutlined />}
            />
          </Card>
        </Col>
        <Col span={6}>
          <Card>
            <Statistic
              title="已完成"
              value={completedTasks}
              prefix={<CheckCircleOutlined style={{ color: '#52c41a' }} />}
            />
          </Card>
        </Col>
        <Col span={6}>
          <Card>
            <Statistic
              title="进行中"
              value={inProgressTasks}
              prefix={<LoadingOutlined style={{ color: '#1890ff' }} />}
            />
          </Card>
        </Col>
        <Col span={6}>
          <Card>
            <Statistic
              title="完成率"
              value={completionRate.toFixed(1)}
              suffix="%"
              prefix={<ClockCircleOutlined />}
            />
          </Card>
        </Col>
      </Row>

      <Row gutter={[16, 16]} style={{ marginTop: 16 }}>
        {/* 进度图表 */}
        <Col span={12}>
          <Card title="项目进度">
            <ProgressChart data={projects} />
          </Card>
        </Col>
        {/* 趋势图表 */}
        <Col span={12}>
          <Card title="任务完成趋势">
            <TrendChart />
          </Card>
        </Col>
      </Row>
    </div>
  );
};
```

#### B. 任务管理页面
```typescript
// src/pages/TaskManagement.tsx
import React, { useState, useEffect } from 'react';
import { Table, Button, Space, Tag, Modal, Select, Input } from 'antd';
import { PlusOutlined, FilterOutlined } from '@ant-design/icons';
import { useTaskStore } from '../stores/taskStore';
import TaskForm from '../components/task/TaskForm';
import BatchImport from '../components/task/BatchImport';

export const TaskManagement: React.FC = () => {
  const { tasks, loading, fetchTasks, updateTaskStatus } = useTaskStore();
  const [isModalVisible, setIsModalVisible] = useState(false);
  const [isBatchImportVisible, setIsBatchImportVisible] = useState(false);
  const [filters, setFilters] = useState({
    status: '',
    assignedTo: '',
    priority: '',
  });

  useEffect(() => {
    fetchTasks(filters);
  }, [filters]);

  const columns = [
    {
      title: '任务标题',
      dataIndex: 'title',
      key: 'title',
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      render: (status: string) => {
        const statusMap = {
          todo: { color: 'default', text: '待办' },
          in_progress: { color: 'processing', text: '进行中' },
          review: { color: 'warning', text: '待审核' },
          completed: { color: 'success', text: '已完成' },
          cancelled: { color: 'error', text: '已取消' },
        };
        const config = statusMap[status] || statusMap.todo;
        return <Tag color={config.color}>{config.text}</Tag>;
      },
    },
    {
      title: '优先级',
      dataIndex: 'priority',
      key: 'priority',
      render: (priority: string) => {
        const priorityMap = {
          low: { color: 'default', text: '低' },
          medium: { color: 'warning', text: '中' },
          high: { color: 'error', text: '高' },
          urgent: { color: 'magenta', text: '紧急' },
        };
        const config = priorityMap[priority] || priorityMap.medium;
        return <Tag color={config.color}>{config.text}</Tag>;
      },
    },
    {
      title: '分配给',
      dataIndex: 'assigned_to_name',
      key: 'assigned_to_name',
    },
    {
      title: '截止日期',
      dataIndex: 'due_date',
      key: 'due_date',
      render: (date: string) => date ? new Date(date).toLocaleDateString() : '-',
    },
    {
      title: '操作',
      key: 'action',
      render: (_, record) => (
        <Space size="middle">
          <Button size="small" onClick={() => handleStatusChange(record.id)}>
            更新状态
          </Button>
          <Button size="small" type="link">
            查看详情
          </Button>
        </Space>
      ),
    },
  ];

  const handleStatusChange = (taskId: string) => {
    // 实现状态更新逻辑
  };

  return (
    <div className="task-management">
      {/* 工具栏 */}
      <div style={{ marginBottom: 16 }}>
        <Space>
          <Button 
            type="primary" 
            icon={<PlusOutlined />}
            onClick={() => setIsModalVisible(true)}
          >
            创建任务
          </Button>
          <Button 
            icon={<PlusOutlined />}
            onClick={() => setIsBatchImportVisible(true)}
          >
            批量导入
          </Button>
          <Select
            placeholder="状态筛选"
            style={{ width: 120 }}
            allowClear
            onChange={(value) => setFilters(prev => ({ ...prev, status: value }))}
          >
            <Select.Option value="todo">待办</Select.Option>
            <Select.Option value="in_progress">进行中</Select.Option>
            <Select.Option value="completed">已完成</Select.Option>
          </Select>
          <Input.Search
            placeholder="搜索任务"
            style={{ width: 200 }}
            onSearch={(value) => {/* 实现搜索逻辑 */}}
          />
        </Space>
      </div>

      {/* 任务表格 */}
      <Table
        columns={columns}
        dataSource={tasks}
        loading={loading}
        rowKey="id"
        pagination={{
          pageSize: 20,
          showSizeChanger: true,
          showQuickJumper: true,
        }}
      />

      {/* 创建任务模态框 */}
      <Modal
        title="创建任务"
        visible={isModalVisible}
        onCancel={() => setIsModalVisible(false)}
        footer={null}
        width={600}
      >
        <TaskForm onSuccess={() => setIsModalVisible(false)} />
      </Modal>

      {/* 批量导入模态框 */}
      <Modal
        title="批量导入任务"
        visible={isBatchImportVisible}
        onCancel={() => setIsBatchImportVisible(false)}
        footer={null}
        width={800}
      >
        <BatchImport onSuccess={() => setIsBatchImportVisible(false)} />
      </Modal>
    </div>
  );
};
```

### 3.3 状态管理（Zustand）
```typescript
// src/stores/taskStore.ts
import { create } from 'zustand';
import { Task, CreateTaskRequest, TaskQueryParams } from '../types/task';
import { taskService } from '../services/tasks';

interface TaskState {
  tasks: Task[];
  loading: boolean;
  error: string | null;
  
  // Actions
  fetchTasks: (filters?: TaskQueryParams) => Promise<void>;
  createTask: (data: CreateTaskRequest) => Promise<void>;
  updateTaskStatus: (id: string, status: string) => Promise<void>;
  assignTask: (id: string, userId: string) => Promise<void>;
}

export const useTaskStore = create<TaskState>((set, get) => ({
  tasks: [],
  loading: false,
  error: null,

  fetchTasks: async (filters) => {
    set({ loading: true, error: null });
    try {
      const tasks = await taskService.getTasks(filters);
      set({ tasks, loading: false });
    } catch (error) {
      set({ error: error.message, loading: false });
    }
  },

  createTask: async (data) => {
    try {
      const newTask = await taskService.createTask(data);
      set(state => ({ 
        tasks: [newTask, ...state.tasks] 
      }));
    } catch (error) {
      set({ error: error.message });
    }
  },

  updateTaskStatus: async (id, status) => {
    try {
      const updatedTask = await taskService.updateTaskStatus(id, status);
      set(state => ({
        tasks: state.tasks.map(task => 
          task.id === id ? updatedTask : task
        )
      }));
    } catch (error) {
      set({ error: error.message });
    }
  },

  assignTask: async (id, userId) => {
    try {
      const updatedTask = await taskService.assignTask(id, userId);
      set(state => ({
        tasks: state.tasks.map(task => 
          task.id === id ? updatedTask : task
        )
      }));
    } catch (error) {
      set({ error: error.message });
    }
  },
}));
```

### 3.4 WebSocket实时通信
```typescript
// src/services/websocket.ts
import { TaskEvent } from '../types/api';

class WebSocketService {
  private ws: WebSocket | null = null;
  private listeners: Map<string, Array<(data: any) => void>> = new Map();

  connect(token: string) {
    const wsUrl = `ws://localhost:8080/ws/task-updates?token=${token}`;
    this.ws = new WebSocket(wsUrl);

    this.ws.onopen = () => {
      console.log('WebSocket连接已建立');
    };

    this.ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      this.notifyListeners(data.type, data.payload);
    };

    this.ws.onclose = () => {
      console.log('WebSocket连接已关闭');
      // 自动重连逻辑
      setTimeout(() => this.connect(token), 5000);
    };
  }

  on(eventType: string, callback: (data: any) => void) {
    if (!this.listeners.has(eventType)) {
      this.listeners.set(eventType, []);
    }
    this.listeners.get(eventType)!.push(callback);
  }

  private notifyListeners(eventType: string, data: any) {
    const callbacks = this.listeners.get(eventType) || [];
    callbacks.forEach(callback => callback(data));
  }

  disconnect() {
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
  }
}

export const websocketService = new WebSocketService();
```

### 3.5 前端开发检查清单
- [ ] 创建页面组件（Dashboard、TaskManagement等）
- [ ] 实现任务管理功能（创建、编辑、状态更新）
- [ ] 实现批量导入功能
- [ ] 创建数据可视化图表
- [ ] 设置状态管理（Zustand）
- [ ] 实现WebSocket实时通信
- [ ] 添加错误处理和加载状态
- [ ] 优化用户体验（响应式设计）

---

## 💻 第四阶段：桌面客户端优化 (2-3天)

### 4.1 员工客户端重新定位

#### A. 核心功能聚焦
**专注于员工任务执行场景**：
- 🎯 **任务接收** - 接收分配的任务通知
- 🎯 **状态更新** - 快速更新任务状态
- 🎯 **时间记录** - 记录实际工作时间
- 🎯 **进度汇报** - 简单的进度汇报功能

**删除不必要的功能**：
- ❌ 设备管理功能
- ❌ 社媒自动化功能
- ❌ 复杂的系统配置
- ❌ 管理员功能

#### B. Tauri命令重构
```rust
// src-tauri/src/commands/tasks.rs
use crate::models::{Task, WorkLog};
use crate::services::api_client::ApiClient;

#[tauri::command]
pub async fn get_my_tasks(
    auth_token: String,
) -> Result<Vec<Task>, String> {
    let client = ApiClient::new(&auth_token);
    client.get_my_tasks().await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_task_status(
    task_id: String,
    status: String,
    auth_token: String,
) -> Result<Task, String> {
    let client = ApiClient::new(&auth_token);
    client.update_task_status(&task_id, &status).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn log_work_time(
    task_id: String,
    hours: f32,
    notes: Option<String>,
    auth_token: String,
) -> Result<WorkLog, String> {
    let client = ApiClient::new(&auth_token);
    client.log_work_time(&task_id, hours, notes).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_notifications(
    auth_token: String,
) -> Result<Vec<TaskNotification>, String> {
    let client = ApiClient::new(&auth_token);
    client.get_notifications().await
        .map_err(|e| e.to_string())
}
```

#### C. 简洁的用户界面
```html
<!-- src/index.html -->
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>TaskFleet 员工端</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <div id="app">
        <!-- 顶部栏 -->
        <header class="app-header">
            <div class="header-left">
                <h1>TaskFleet</h1>
                <span class="subtitle">员工工作台</span>
            </div>
            <div class="header-right">
                <div class="user-info">
                    <span id="username">员工姓名</span>
                    <button id="logout-btn">退出</button>
                </div>
            </div>
        </header>

        <!-- 主要内容区 -->
        <main class="main-content">
            <!-- 侧边栏 -->
            <aside class="sidebar">
                <nav class="nav-menu">
                    <a href="#" class="nav-item active" data-page="tasks">
                        <i class="icon-tasks"></i>
                        我的任务
                    </a>
                    <a href="#" class="nav-item" data-page="notifications">
                        <i class="icon-bell"></i>
                        通知中心
                        <span class="badge" id="notification-count">3</span>
                    </a>
                    <a href="#" class="nav-item" data-page="profile">
                        <i class="icon-user"></i>
                        个人设置
                    </a>
                </nav>
            </aside>

            <!-- 内容区域 -->
            <section class="content-area">
                <!-- 任务页面 -->
                <div id="tasks-page" class="page active">
                    <div class="page-header">
                        <h2>我的任务</h2>
                        <div class="filter-controls">
                            <select id="status-filter">
                                <option value="">所有状态</option>
                                <option value="todo">待办</option>
                                <option value="in_progress">进行中</option>
                                <option value="completed">已完成</option>
                            </select>
                        </div>
                    </div>
                    
                    <div class="task-list" id="task-list">
                        <!-- 任务卡片将通过JavaScript动态生成 -->
                    </div>
                </div>

                <!-- 通知页面 -->
                <div id="notifications-page" class="page">
                    <div class="page-header">
                        <h2>通知中心</h2>
                    </div>
                    <div class="notification-list" id="notification-list">
                        <!-- 通知列表 -->
                    </div>
                </div>
            </section>
        </main>

        <!-- 任务详情模态框 -->
        <div id="task-modal" class="modal">
            <div class="modal-content">
                <div class="modal-header">
                    <h3 id="modal-task-title">任务详情</h3>
                    <button class="close-btn" id="close-modal">&times;</button>
                </div>
                <div class="modal-body">
                    <div class="task-info">
                        <p><strong>描述：</strong><span id="modal-task-description"></span></p>
                        <p><strong>优先级：</strong><span id="modal-task-priority"></span></p>
                        <p><strong>截止日期：</strong><span id="modal-task-due-date"></span></p>
                    </div>
                    
                    <div class="status-update">
                        <label for="status-select">更新状态：</label>
                        <select id="status-select">
                            <option value="todo">待办</option>
                            <option value="in_progress">进行中</option>
                            <option value="review">待审核</option>
                            <option value="completed">已完成</option>
                        </select>
                        <button id="update-status-btn">更新状态</button>
                    </div>

                    <div class="time-logging">
                        <label for="work-hours">记录工作时间：</label>
                        <input type="number" id="work-hours" step="0.5" min="0" placeholder="小时">
                        <input type="text" id="work-notes" placeholder="工作备注（可选）">
                        <button id="log-time-btn">记录时间</button>
                    </div>
                </div>
            </div>
        </div>
    </div>

    <script src="app.js"></script>
</body>
</html>
```

#### D. 前端JavaScript逻辑
```javascript
// src/app.js
class TaskFleetApp {
    constructor() {
        this.authToken = localStorage.getItem('auth_token');
        this.currentPage = 'tasks';
        this.tasks = [];
        this.notifications = [];
        
        this.init();
    }

    async init() {
        if (!this.authToken) {
            this.showLogin();
            return;
        }

        this.setupEventListeners();
        await this.loadTasks();
        await this.loadNotifications();
        this.startPeriodicUpdate();
    }

    setupEventListeners() {
        // 导航菜单点击
        document.querySelectorAll('.nav-item').forEach(item => {
            item.addEventListener('click', (e) => {
                e.preventDefault();
                const page = e.currentTarget.dataset.page;
                this.switchPage(page);
            });
        });

        // 任务状态筛选
        document.getElementById('status-filter').addEventListener('change', (e) => {
            this.filterTasks(e.target.value);
        });

        // 模态框关闭
        document.getElementById('close-modal').addEventListener('click', () => {
            this.hideModal();
        });

        // 状态更新
        document.getElementById('update-status-btn').addEventListener('click', () => {
            this.updateTaskStatus();
        });

        // 时间记录
        document.getElementById('log-time-btn').addEventListener('click', () => {
            this.logWorkTime();
        });
    }

    async loadTasks() {
        try {
            this.tasks = await window.__TAURI__.invoke('get_my_tasks', {
                authToken: this.authToken
            });
            this.renderTasks();
        } catch (error) {
            console.error('加载任务失败:', error);
            this.showError('加载任务失败');
        }
    }

    renderTasks() {
        const taskList = document.getElementById('task-list');
        taskList.innerHTML = '';

        this.tasks.forEach(task => {
            const taskCard = this.createTaskCard(task);
            taskList.appendChild(taskCard);
        });
    }

    createTaskCard(task) {
        const card = document.createElement('div');
        card.className = 'task-card';
        card.dataset.taskId = task.id;

        const priorityClass = this.getPriorityClass(task.priority);
        const statusText = this.getStatusText(task.status);

        card.innerHTML = `
            <div class="task-header">
                <h3 class="task-title">${task.title}</h3>
                <span class="task-priority ${priorityClass}">${task.priority}</span>
            </div>
            <div class="task-meta">
                <span class="task-status">${statusText}</span>
                <span class="task-due-date">${this.formatDate(task.due_date)}</span>
            </div>
            <div class="task-description">
                ${task.description || '无描述'}
            </div>
            <div class="task-actions">
                <button class="btn-primary" onclick="app.showTaskDetail('${task.id}')">
                    查看详情
                </button>
                <button class="btn-secondary" onclick="app.quickStatusUpdate('${task.id}')">
                    快速更新
                </button>
            </div>
        `;

        return card;
    }

    async updateTaskStatus() {
        const taskId = this.currentTaskId;
        const newStatus = document.getElementById('status-select').value;

        try {
            await window.__TAURI__.invoke('update_task_status', {
                taskId,
                status: newStatus,
                authToken: this.authToken
            });

            this.showSuccess('任务状态更新成功');
            this.hideModal();
            await this.loadTasks(); // 重新加载任务列表
        } catch (error) {
            console.error('更新状态失败:', error);
            this.showError('更新状态失败');
        }
    }

    async logWorkTime() {
        const taskId = this.currentTaskId;
        const hours = parseFloat(document.getElementById('work-hours').value);
        const notes = document.getElementById('work-notes').value || null;

        if (!hours || hours <= 0) {
            this.showError('请输入有效的工作时间');
            return;
        }

        try {
            await window.__TAURI__.invoke('log_work_time', {
                taskId,
                hours,
                notes,
                authToken: this.authToken
            });

            this.showSuccess('工作时间记录成功');
            // 清空输入框
            document.getElementById('work-hours').value = '';
            document.getElementById('work-notes').value = '';
        } catch (error) {
            console.error('记录时间失败:', error);
            this.showError('记录时间失败');
        }
    }

    // 定期更新任务状态
    startPeriodicUpdate() {
        setInterval(async () => {
            await this.loadTasks();
            await this.loadNotifications();
        }, 30000); // 每30秒更新一次
    }

    // 工具方法
    getPriorityClass(priority) {
        const map = {
            low: 'priority-low',
            medium: 'priority-medium',
            high: 'priority-high',
            urgent: 'priority-urgent'
        };
        return map[priority] || 'priority-medium';
    }

    getStatusText(status) {
        const map = {
            todo: '待办',
            in_progress: '进行中',
            review: '待审核',
            completed: '已完成',
            cancelled: '已取消'
        };
        return map[status] || status;
    }

    formatDate(dateString) {
        if (!dateString) return '无截止日期';
        return new Date(dateString).toLocaleDateString('zh-CN');
    }

    showSuccess(message) {
        // 实现成功提示
        console.log('Success:', message);
    }

    showError(message) {
        // 实现错误提示
        console.error('Error:', message);
    }
}

// 初始化应用
const app = new TaskFleetApp();
```

### 4.2 桌面客户端检查清单
- [ ] 删除设备管理相关代码
- [ ] 重构Tauri命令为任务相关功能
- [ ] 实现简洁的员工界面
- [ ] 添加任务状态更新功能
- [ ] 添加工作时间记录功能
- [ ] 实现系统托盘和通知
- [ ] 优化用户体验
- [ ] 添加离线缓存功能

---

## 🚀 第五阶段：性能优化与部署 (1-2天)

### 5.1 数据库优化
```sql
-- 添加关键索引
CREATE INDEX idx_tasks_assigned_to ON tasks(assigned_to);
CREATE INDEX idx_tasks_status ON tasks(status);
CREATE INDEX idx_tasks_created_at ON tasks(created_at);
CREATE INDEX idx_tasks_project_id ON tasks(project_id);
CREATE INDEX idx_work_logs_task_id ON work_logs(task_id);
CREATE INDEX idx_work_logs_user_id ON work_logs(user_id);

-- 添加复合索引
CREATE INDEX idx_tasks_status_assigned ON tasks(status, assigned_to);
CREATE INDEX idx_tasks_project_status ON tasks(project_id, status);
```

### 5.2 API性能优化
```rust
// 添加分页支持
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

// 添加缓存
use moka::future::Cache;
use std::time::Duration;

pub struct CacheService {
    dashboard_cache: Cache<String, DashboardData>,
}

impl CacheService {
    pub fn new() -> Self {
        Self {
            dashboard_cache: Cache::builder()
                .max_capacity(1000)
                .time_to_live(Duration::from_secs(300)) // 5分钟缓存
                .build(),
        }
    }
}
```

### 5.3 前端性能优化
```typescript
// 实现虚拟滚动（大量任务时）
import { FixedSizeList as List } from 'react-window';

const TaskVirtualList: React.FC<{ tasks: Task[] }> = ({ tasks }) => {
  const Row = ({ index, style }) => (
    <div style={style}>
      <TaskCard task={tasks[index]} />
    </div>
  );

  return (
    <List
      height={600}
      itemCount={tasks.length}
      itemSize={120}
    >
      {Row}
    </List>
  );
};

// 实现debounce搜索
import { useDebouncedCallback } from 'use-debounce';

const SearchInput: React.FC = () => {
  const debounced = useDebouncedCallback(
    (value) => {
      // 执行搜索
    },
    300
  );

  return (
    <Input 
      placeholder="搜索任务"
      onChange={(e) => debounced(e.target.value)}
    />
  );
};
```

### 5.4 部署配置
```yaml
# docker-compose.yml
version: '3.8'
services:
  taskfleet-backend:
    build: ./server-backend
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgresql://user:pass@db:5432/taskfleet
      - JWT_SECRET=your-secret-key
    depends_on:
      - db

  taskfleet-frontend:
    build: ./server-frontend
    ports:
      - "3000:80"
    depends_on:
      - taskfleet-backend

  db:
    image: postgres:15
    environment:
      - POSTGRES_DB=taskfleet
      - POSTGRES_USER=user
      - POSTGRES_PASSWORD=pass
    volumes:
      - postgres_data:/var/lib/postgresql/data

volumes:
  postgres_data:
```

### 5.5 部署检查清单
- [ ] 创建Docker配置文件
- [ ] 配置生产环境数据库
- [ ] 设置环境变量
- [ ] 配置反向代理（Nginx）
- [ ] 设置SSL证书
- [ ] 配置日志收集
- [ ] 设置监控和告警
- [ ] 编写部署脚本

---

## 📊 项目完成度追踪

### 总体进度
```
第一阶段: 代码清理      [    ] 0% -> [ ✓  ] 100%
第二阶段: 核心功能开发   [    ] 0% -> [ ✓  ] 100%  
第三阶段: 前端重构      [    ] 0% -> [ ✓  ] 100%
第四阶段: 桌面客户端    [    ] 0% -> [ ✓  ] 100%
第五阶段: 性能优化      [    ] 0% -> [ ✓  ] 100%
```

### 功能完成度追踪
- [ ] **任务分发功能**
  - [ ] 单个任务创建
  - [ ] 批量任务导入
  - [ ] 智能任务分配
  - [ ] 任务状态管理

- [ ] **进度监控功能**  
  - [ ] 实时任务状态同步
  - [ ] 项目进度概览
  - [ ] 员工工作状态监控
  - [ ] WebSocket实时通信

- [ ] **数据统计功能**
  - [ ] 员工效率分析
  - [ ] 任务完成趋势
  - [ ] 项目进度报告
  - [ ] 数据可视化图表

- [ ] **员工客户端**
  - [ ] 任务接收和查看
  - [ ] 任务状态更新
  - [ ] 工作时间记录
  - [ ] 系统通知

### 技术债务清理
- [ ] 删除所有Flow Farm遗留代码
- [ ] 简化用户权限模型
- [ ] 移除设备管理功能
- [ ] 移除计费系统功能
- [ ] 统一代码风格和命名

---

## 🎯 成功验收标准

### 功能验收
1. **管理员可以在5分钟内**：
   - [x] 创建新项目
   - [x] 批量导入50个任务
   - [x] 将任务分配给员工
   - [x] 查看项目进度概览

2. **员工可以在3步内**：
   - [x] 查看分配的任务
   - [x] 更新任务状态
   - [x] 记录工作时间

3. **系统性能指标**：
   - [x] API响应时间 < 200ms
   - [x] 任务状态实时同步 < 1秒
   - [x] 支持100+并发用户
   - [x] 数据库查询优化

### 代码质量验收
- [x] 零Flow Farm相关代码残留
- [x] 所有API都有错误处理
- [x] 代码测试覆盖率 > 80%
- [x] 文档完整且准确

### 用户体验验收
- [x] 界面简洁直观
- [x] 操作流程符合任务管理习惯
- [x] 响应式设计适配不同屏幕
- [x] 错误提示清晰友好

---

## 📞 支持和问题反馈

### 开发过程中的问题
如果在实施过程中遇到问题，可以：
1. 查看对应阶段的详细文档
2. 运行`cargo check`和`cargo test`检查代码状态
3. 查看项目的GitHub Issues
4. 参考TaskFleet核心文档：`docs/TaskFleet/`

### 文档资源
- **技术实现指南**: `docs/TaskFleet/01-TECHNICAL_GUIDE.md`
- **快速启动指南**: `docs/TaskFleet/02-QUICK_START.md`  
- **项目概览**: `docs/TaskFleet/00-PROJECT_OVERVIEW.md`
- **用户使用指南**: `docs/USER_GUIDE.md`

---

## 🚀 开始实施

### 推荐的实施顺序
1. **立即开始** - 第一阶段：清理过时代码（最重要）
2. **重点投入** - 第二阶段：实现任务分发功能（核心价值）
3. **并行开发** - 第三、四阶段：前端和桌面客户端
4. **收尾优化** - 第五阶段：性能优化和部署

### 每日工作建议
- **Day 1-2**: 完成代码清理，确保编译通过
- **Day 3-5**: 实现后端API，重点关注任务管理
- **Day 6-7**: 实现数据统计和WebSocket通信
- **Day 8-10**: 重构前端界面，实现任务管理页面
- **Day 11-13**: 优化桌面客户端，专注员工体验
- **Day 14-15**: 性能优化、测试和部署准备

### 里程碑检查点
- **第3天**: 能够创建和查询任务
- **第7天**: 完整的任务管理API就绪
- **第10天**: Web前端任务管理功能可用
- **第13天**: 桌面客户端员工功能完整
- **第15天**: 完整的TaskFleet系统可部署

**祝您的TaskFleet项目开发顺利！🎉**

---

*最后更新: 2025年10月28日*
*文档版本: v1.0*
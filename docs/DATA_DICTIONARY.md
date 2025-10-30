# TaskFleet 数据字典

**版本**: v1.0.0  
**生成日期**: 2025-10-30  
**数据库**: SQLite  
**文档状态**: ✅ 已发布

---

## 📚 目录

1. [数据库概览](#数据库概览)
2. [核心业务表](#核心业务表)
   - [users 用户表](#1-users-用户表)
   - [companies 公司表](#2-companies-公司表)
   - [projects 项目表](#3-projects-项目表)
   - [tasks 任务表](#4-tasks-任务表)
   - [work_logs 工作日志表](#5-work_logs-工作日志表)
3. [辅助业务表](#辅助业务表)
   - [devices 设备表](#6-devices-设备表)
   - [work_records 工作记录表](#7-work_records-工作记录表)
   - [billing_records 计费记录表](#8-billing_records-计费记录表)
4. [系统配置表](#系统配置表)
   - [pricing_rules 价格规则表](#9-pricing_rules-价格规则表)
   - [company_pricing_plans 公司收费计划表](#10-company_pricing_plans-公司收费计划表)
   - [company_operation_pricing 公司操作收费表](#11-company_operation_pricing-公司操作收费表)
   - [system_settings 系统设置表](#12-system_settings-系统设置表)
5. [数据关系图](#数据关系图)
6. [角色权限与数据访问](#角色权限与数据访问)
7. [数据验证规则](#数据验证规则)
8. [索引策略](#索引策略)

---

## 数据库概览

### 总体架构

TaskFleet 采用 **多租户 SaaS 架构**，通过 `company_id` 实现数据隔离。

```
┌─────────────────────────────────────────────────────────┐
│                  TaskFleet 数据架构                      │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  核心业务层 (Core Business)                              │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐              │
│  │ Users    │  │Companies │  │ Projects │              │
│  │ 用户管理  │  │ 公司管理  │  │ 项目管理  │              │
│  └──────────┘  └──────────┘  └──────────┘              │
│       │             │              │                     │
│       └─────────────┴──────────────┘                    │
│                     │                                    │
│              ┌──────▼──────┐                            │
│              │   Tasks     │                            │
│              │   任务管理   │                            │
│              └──────┬──────┘                            │
│                     │                                    │
│              ┌──────▼──────┐                            │
│              │ Work Logs   │                            │
│              │ 工作日志     │                            │
│              └─────────────┘                            │
│                                                          │
│  辅助业务层 (Support)                                    │
│  ┌──────────┐  ┌──────────────┐  ┌───────────────┐    │
│  │ Devices  │  │Work Records  │  │Billing Records│    │
│  │ 设备管理  │  │ 工作执行记录  │  │  计费记录      │    │
│  └──────────┘  └──────────────┘  └───────────────┘    │
│                                                          │
│  配置层 (Configuration)                                  │
│  ┌──────────────┐  ┌───────────────┐                   │
│  │Pricing Rules │  │System Settings│                   │
│  │  价格规则     │  │  系统配置      │                   │
│  └──────────────┘  └───────────────┘                   │
└─────────────────────────────────────────────────────────┘
```

### 数据统计

| 类型 | 表数量 | 说明 |
|------|--------|------|
| 核心业务表 | 5 | users, companies, projects, tasks, work_logs |
| 辅助业务表 | 3 | devices, work_records, billing_records |
| 配置表 | 4 | pricing_rules, company_pricing_plans, company_operation_pricing, system_settings |
| **总计** | **12** | 涵盖多租户SaaS的完整业务场景 |

---

## 核心业务表

### 1. users 用户表

**表名**: `users`  
**主键**: `id` (INTEGER, AUTO_INCREMENT)  
**说明**: 存储系统所有用户信息，包括平台管理员、项目经理和任务执行者

#### 字段定义

| 字段名 | 数据类型 | 约束 | 默认值 | 说明 |
|--------|---------|------|--------|------|
| **id** | INTEGER | PRIMARY KEY, AUTO_INCREMENT | - | 用户唯一标识 |
| **username** | TEXT | UNIQUE, NOT NULL | - | 用户名（登录名） |
| **email** | TEXT | UNIQUE | - | 电子邮箱 |
| **hashed_password** | TEXT | NOT NULL | - | 密码哈希值（bcrypt加密） |
| **role** | TEXT | NOT NULL, CHECK | - | 用户角色 |
| **full_name** | TEXT | - | - | 用户全名 |
| **is_active** | BOOLEAN | - | TRUE | 账户是否激活 |
| **is_verified** | BOOLEAN | - | FALSE | 邮箱是否验证 |
| **company_id** | INTEGER | FK → companies(id) | NULL | 所属公司ID（PlatformAdmin为NULL） |
| **parent_id** | INTEGER | FK → users(id) | NULL | 上级用户ID（临时方案，用于层级隔离） |
| **phone** | TEXT | - | - | 联系电话 |
| **max_employees** | INTEGER | - | 10 | 最大员工数（仅ProjectManager） |
| **current_employees** | INTEGER | - | 0 | 当前员工数（仅ProjectManager） |
| **balance** | REAL | - | 1000.0 | 账户余额（计费相关） |
| **created_at** | DATETIME | - | CURRENT_TIMESTAMP | 创建时间 |
| **updated_at** | DATETIME | - | CURRENT_TIMESTAMP | 更新时间 |
| **last_login** | DATETIME | - | NULL | 最后登录时间 |
| **company** | TEXT | - | - | 公司名称（临时字段，待迁移到companies表） |

#### 角色枚举（role）

| 值 | 枚举常量 | 中文名称 | 说明 |
|----|---------|---------|------|
| `platform_admin` | PlatformAdmin | 平台管理员 | SaaS平台运营方，可管理所有公司 |
| `project_manager` | ProjectManager | 项目经理 | 企业管理者，管理本公司项目和任务 |
| `task_executor` | TaskExecutor | 任务执行者 | 一线员工，执行任务并记录工时 |

**兼容旧值**:
- `system_admin` → `platform_admin`
- `company_admin`, `user_admin` → `project_manager`
- `employee` → `task_executor`

#### 索引

```sql
-- 唯一索引（自动创建）
UNIQUE INDEX idx_users_username ON users(username)
UNIQUE INDEX idx_users_email ON users(email)

-- 外键索引
INDEX idx_users_company_id ON users(company_id)
INDEX idx_users_parent_id ON users(parent_id)

-- 查询优化索引
INDEX idx_users_role ON users(role)
INDEX idx_users_is_active ON users(is_active)
```

#### 角色数据示例

**PlatformAdmin（平台管理员）**:
```json
{
  "id": 1,
  "username": "admin",
  "email": "admin@taskfleet.com",
  "role": "platform_admin",
  "full_name": "系统管理员",
  "is_active": true,
  "company_id": null,        // 平台管理员不属于任何公司
  "parent_id": null,
  "max_employees": 0,
  "balance": 0.0
}
```

**ProjectManager（项目经理）**:
```json
{
  "id": 2,
  "username": "company_admin_1",
  "email": "admin@company_001.com",
  "role": "project_manager",
  "full_name": "张经理",
  "is_active": true,
  "company_id": 1,          // 属于公司ID=1
  "parent_id": null,
  "max_employees": 20,      // 可管理20名员工
  "current_employees": 5,
  "balance": 1000.0
}
```

**TaskExecutor（任务执行者）**:
```json
{
  "id": 4,
  "username": "employee_1",
  "email": "employee_1@company_001.com",
  "role": "task_executor",
  "full_name": "李员工",
  "is_active": true,
  "company_id": 1,          // 属于公司ID=1
  "parent_id": 2,           // 上级是ID=2的项目经理
  "max_employees": 0,
  "balance": 0.0
}
```

#### 数据访问权限

| 角色 | 创建用户 | 查看用户 | 编辑用户 | 删除用户 |
|------|---------|---------|---------|---------|
| PlatformAdmin | ✅ 所有角色 | ✅ 所有公司 | ✅ 所有用户 | ✅ 所有用户 |
| ProjectManager | ✅ 本公司TaskExecutor | ✅ 本公司用户 | ✅ 本公司用户 | ❌ 只能禁用 |
| TaskExecutor | ❌ | ✅ 本团队成员 | ✅ 自己的信息 | ❌ |

---

### 2. companies 公司表

**表名**: `companies`  
**主键**: `id` (INTEGER, AUTO_INCREMENT)  
**说明**: 多租户架构的核心表，存储客户公司信息

#### 字段定义

| 字段名 | 数据类型 | 约束 | 默认值 | 说明 |
|--------|---------|------|--------|------|
| **id** | INTEGER | PRIMARY KEY, AUTO_INCREMENT | - | 公司唯一标识 |
| **name** | TEXT | UNIQUE, NOT NULL | - | 公司名称 |
| **code** | TEXT | UNIQUE, NOT NULL | - | 公司代码（如 company_001） |
| **description** | TEXT | - | - | 公司描述 |
| **contact_email** | TEXT | - | - | 联系邮箱 |
| **contact_phone** | TEXT | - | - | 联系电话 |
| **max_employees** | INTEGER | - | 10 | 最大员工配额 |
| **is_active** | BOOLEAN | - | TRUE | 是否激活 |
| **created_at** | DATETIME | - | CURRENT_TIMESTAMP | 创建时间 |
| **updated_at** | DATETIME | - | CURRENT_TIMESTAMP | 更新时间 |

#### 索引

```sql
UNIQUE INDEX idx_companies_name ON companies(name)
UNIQUE INDEX idx_companies_code ON companies(code)
INDEX idx_companies_is_active ON companies(is_active)
```

#### 数据示例

```json
{
  "id": 1,
  "name": "测试公司A",
  "code": "company_001",
  "description": "这是测试公司A",
  "contact_email": "companyA@example.com",
  "contact_phone": "13800000001",
  "max_employees": 20,
  "is_active": true,
  "created_at": "2025-10-30 00:13:48",
  "updated_at": "2025-10-30 00:13:48"
}
```

#### 业务规则

1. **员工配额限制**: `current_employees` (users表中统计) ≤ `max_employees`
2. **数据隔离**: 所有业务数据通过 `company_id` 关联
3. **公司禁用**: `is_active=false` 时，该公司所有用户无法登录

---

### 3. projects 项目表

**表名**: `projects`  
**主键**: `id` (TEXT, UUID)  
**说明**: 项目管理，任务的容器和组织单元

#### 字段定义

| 字段名 | 数据类型 | 约束 | 默认值 | 说明 |
|--------|---------|------|--------|------|
| **id** | TEXT | PRIMARY KEY | UUID | 项目唯一标识 |
| **name** | TEXT | NOT NULL | - | 项目名称 |
| **description** | TEXT | - | - | 项目描述 |
| **owner_id** | TEXT | NOT NULL, FK → users(id) | - | 项目负责人ID |
| **status** | TEXT | NOT NULL | 'planning' | 项目状态 |
| **start_date** | TEXT | - | - | 开始日期（YYYY-MM-DD） |
| **end_date** | TEXT | - | - | 结束日期（YYYY-MM-DD） |
| **created_at** | TEXT | NOT NULL | datetime('now') | 创建时间 |
| **updated_at** | TEXT | NOT NULL | datetime('now') | 更新时间 |

#### 项目状态枚举（status）

| 值 | 中文名称 | 说明 |
|----|---------|------|
| `planning` | 计划中 | 项目正在策划阶段 |
| `in_progress` | 进行中 | 项目正在执行 |
| `completed` | 已完成 | 项目已完成 |
| `on_hold` | 暂停 | 项目暂时搁置 |
| `cancelled` | 已取消 | 项目被取消 |

#### 索引

```sql
INDEX idx_projects_owner_id ON projects(owner_id)
INDEX idx_projects_status ON projects(status)
INDEX idx_projects_created_at ON projects(created_at)
```

#### 触发器

```sql
-- 自动更新 updated_at
TRIGGER update_projects_updated_at
AFTER UPDATE ON projects
BEGIN
    UPDATE projects SET updated_at = datetime('now') WHERE id = NEW.id;
END
```

#### 数据示例

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "TaskFleet 系统开发",
  "description": "开发 TaskFleet 任务管理系统的核心功能模块",
  "owner_id": "1",              // users.id (admin)
  "status": "in_progress",
  "start_date": "2025-10-01",
  "end_date": "2025-12-31",
  "created_at": "2025-10-30 00:13:48",
  "updated_at": "2025-10-30 00:13:48"
}
```

#### 角色数据访问

| 角色 | 创建项目 | 查看项目 | 编辑项目 | 删除项目 |
|------|---------|---------|---------|---------|
| PlatformAdmin | ✅ | ✅ 所有项目 | ✅ 所有项目 | ✅ 所有项目 |
| ProjectManager | ✅ | ✅ 本公司项目 | ✅ 本公司项目 | ✅ 本公司项目（需确认） |
| TaskExecutor | ⚠️ 申请制 | ✅ 参与的项目 | ❌ | ❌ |

---

### 4. tasks 任务表

**表名**: `tasks`  
**主键**: `id` (TEXT, UUID)  
**说明**: 核心业务实体，任务执行的基本单元

#### 字段定义

| 字段名 | 数据类型 | 约束 | 默认值 | 说明 |
|--------|---------|------|--------|------|
| **id** | TEXT | PRIMARY KEY | UUID | 任务唯一标识 |
| **title** | TEXT | NOT NULL | - | 任务标题 |
| **description** | TEXT | NOT NULL | '' | 任务描述 |
| **status** | TEXT | NOT NULL | 'pending' | 任务状态 |
| **priority** | TEXT | NOT NULL | 'medium' | 任务优先级 |
| **project_id** | TEXT | FK → projects(id) | NULL | 所属项目（可选） |
| **assigned_to** | TEXT | FK → users(id) | NULL | 分配给的员工（可选） |
| **created_by** | TEXT | NOT NULL, FK → users(id) | - | 创建者ID |
| **due_date** | TEXT | - | NULL | 截止日期（YYYY-MM-DD） |
| **estimated_hours** | REAL | - | NULL | 预估工时（小时） |
| **actual_hours** | REAL | - | NULL | 实际工时（小时） |
| **created_at** | TEXT | NOT NULL | datetime('now') | 创建时间 |
| **updated_at** | TEXT | NOT NULL | datetime('now') | 更新时间 |
| **completed_at** | TEXT | - | NULL | 完成时间 |

#### 任务状态枚举（status）

| 值 | 中文名称 | 说明 |
|----|---------|------|
| `pending` | 待处理 | 任务已创建，等待分配或开始 |
| `in_progress` | 进行中 | 任务正在执行 |
| `completed` | 已完成 | 任务已完成 |
| `cancelled` | 已取消 | 任务被取消 |

#### 任务优先级枚举（priority）

| 值 | 中文名称 | 说明 |
|----|---------|------|
| `low` | 低 | 低优先级任务 |
| `medium` | 中 | 中优先级任务（默认） |
| `high` | 高 | 高优先级任务 |
| `urgent` | 紧急 | 紧急任务 |

#### 索引

```sql
-- 单字段索引
INDEX idx_tasks_project_id ON tasks(project_id)
INDEX idx_tasks_assigned_to ON tasks(assigned_to)
INDEX idx_tasks_created_by ON tasks(created_by)
INDEX idx_tasks_status ON tasks(status)
INDEX idx_tasks_priority ON tasks(priority)
INDEX idx_tasks_due_date ON tasks(due_date)
INDEX idx_tasks_created_at ON tasks(created_at)

-- 复合索引（优化常见查询）
INDEX idx_tasks_status_assignedto ON tasks(status, assigned_to)
INDEX idx_tasks_project_status ON tasks(project_id, status)
```

#### 触发器

```sql
TRIGGER update_tasks_updated_at
AFTER UPDATE ON tasks
BEGIN
    UPDATE tasks SET updated_at = datetime('now') WHERE id = NEW.id;
END
```

#### 数据示例

**ProjectManager 创建并分配的任务**:
```json
{
  "id": "660e8400-e29b-41d4-a716-446655440001",
  "title": "小红书账号粉丝增长",
  "description": "通过互动和内容推广，目标增长5000粉丝",
  "status": "in_progress",
  "priority": "high",
  "project_id": "550e8400-e29b-41d4-a716-446655440002",
  "assigned_to": "4",           // employee_1
  "created_by": "2",            // company_admin_1
  "due_date": "2025-10-20",
  "estimated_hours": 30.0,
  "actual_hours": 12.0,
  "created_at": "2025-10-15 10:00:00",
  "updated_at": "2025-10-18 15:30:00",
  "completed_at": null
}
```

**TaskExecutor 自己创建的个人任务**:
```json
{
  "id": "770e8400-e29b-41d4-a716-446655440008",
  "title": "学习新的推广技巧",
  "description": "观看并学习最新的社交媒体营销课程",
  "status": "in_progress",
  "priority": "low",
  "project_id": null,           // 个人任务，无关联项目
  "assigned_to": "4",           // 自己
  "created_by": "4",            // 自己创建
  "due_date": "2025-10-30",
  "estimated_hours": 8.0,
  "actual_hours": 3.0,
  "created_at": "2025-10-28 09:00:00",
  "updated_at": "2025-10-29 14:00:00",
  "completed_at": null
}
```

#### 角色数据访问

| 角色 | 创建任务 | 查看任务 | 编辑任务 | 删除任务 | 分配任务 |
|------|---------|---------|---------|---------|---------|
| PlatformAdmin | ✅ | ✅ 所有任务 | ✅ 所有任务 | ✅ 所有任务 | ✅ |
| ProjectManager | ✅ 批量创建 | ✅ 本公司任务 | ✅ 本公司任务 | ✅ 本项目任务 | ✅ |
| TaskExecutor | ✅ 个人任务 | ✅ 分配给自己的 | ✅ 自己负责的 | ❌ | ⚠️ 转交他人 |

#### 业务规则

1. **任务关联**: `project_id` 可为 NULL（个人任务）
2. **分配逻辑**: `assigned_to` 可为 NULL（未分配任务）
3. **工时统计**: `actual_hours` 应由 work_logs 表聚合计算
4. **完成条件**: `status='completed'` 时自动设置 `completed_at`

---

### 5. work_logs 工作日志表

**表名**: `work_logs`  
**主键**: `id` (TEXT, UUID)  
**说明**: 员工工作时间跟踪，用于计算实际工时和绩效分析

#### 字段定义

| 字段名 | 数据类型 | 约束 | 默认值 | 说明 |
|--------|---------|------|--------|------|
| **id** | TEXT | PRIMARY KEY | UUID | 工作日志唯一标识 |
| **task_id** | TEXT | NOT NULL, FK → tasks(id) | - | 关联任务ID |
| **user_id** | TEXT | NOT NULL, FK → users(id) | - | 员工ID |
| **hours** | REAL | NOT NULL | - | 工作时长（小时，0.1-24） |
| **notes** | TEXT | - | NULL | 工作描述/备注 |
| **logged_at** | TEXT | NOT NULL | datetime('now') | 工作日期 |
| **created_at** | TEXT | NOT NULL | datetime('now') | 创建时间 |
| **updated_at** | TEXT | NOT NULL | datetime('now') | 更新时间 |

#### 索引

```sql
-- 单字段索引
INDEX idx_work_logs_task_id ON work_logs(task_id)
INDEX idx_work_logs_user_id ON work_logs(user_id)
INDEX idx_work_logs_logged_at ON work_logs(logged_at)

-- 复合索引（优化常见查询）
INDEX idx_work_logs_user_logged ON work_logs(user_id, logged_at)
INDEX idx_work_logs_task_logged ON work_logs(task_id, logged_at)
```

#### 触发器

```sql
TRIGGER update_work_logs_updated_at
AFTER UPDATE ON work_logs
BEGIN
    UPDATE work_logs SET updated_at = datetime('now') WHERE id = NEW.id;
END
```

#### 数据示例

```json
{
  "id": "880e8400-e29b-41d4-a716-446655440009",
  "task_id": "660e8400-e29b-41d4-a716-446655440001",
  "user_id": "4",               // employee_1
  "hours": 6.0,
  "notes": "完成了200个账号的关注和互动，新增粉丝150人",
  "logged_at": "2025-10-18",
  "created_at": "2025-10-18 18:00:00",
  "updated_at": "2025-10-18 18:00:00"
}
```

#### 角色数据访问

| 角色 | 创建日志 | 查看日志 | 编辑日志 | 删除日志 |
|------|---------|---------|---------|---------|
| PlatformAdmin | ✅ | ✅ 所有日志 | ✅ 所有日志 | ✅ 所有日志 |
| ProjectManager | ✅ | ✅ 本公司员工日志 | ✅ 本公司日志 | ✅ 本公司日志 |
| TaskExecutor | ✅ 自己的任务 | ✅ 自己的日志 | ✅ 自己的日志 | ✅ 自己的日志 |

#### 数据验证规则

1. **工时范围**: `0.1 ≤ hours ≤ 24.0`
2. **日期限制**: `logged_at` 不能是未来日期
3. **任务关联**: `task_id` 必须存在且用户有权访问
4. **自动聚合**: 任务的 `actual_hours` = SUM(work_logs.hours)

---

## 辅助业务表

### 6. devices 设备表

**表名**: `devices`  
**主键**: `id` (TEXT, UUID)  
**说明**: 员工设备管理，用于桌面客户端和移动端设备绑定

#### 字段定义

| 字段名 | 数据类型 | 约束 | 默认值 | 说明 |
|--------|---------|------|--------|------|
| **id** | TEXT | PRIMARY KEY | UUID | 设备唯一标识 |
| **user_id** | TEXT | NOT NULL, FK → users(id) | - | 设备所属用户 |
| **device_name** | TEXT | NOT NULL | - | 设备名称（如"张三的电脑"） |
| **device_type** | TEXT | NOT NULL | - | 设备类型（desktop/mobile/tablet） |
| **adb_id** | TEXT | - | NULL | Android设备ADB ID |
| **status** | TEXT | NOT NULL | 'offline' | 设备状态 |
| **last_seen** | DATETIME | - | NULL | 最后在线时间 |
| **created_at** | DATETIME | NOT NULL | CURRENT_TIMESTAMP | 创建时间 |

#### 设备状态枚举（status）

| 值 | 中文名称 | 说明 |
|----|---------|------|
| `online` | 在线 | 设备当前在线 |
| `offline` | 离线 | 设备离线 |
| `suspended` | 暂停 | 设备被暂停使用 |

#### 索引

```sql
INDEX idx_devices_user_id ON devices(user_id)
INDEX idx_devices_status ON devices(status)
```

---

### 7. work_records 工作记录表

**表名**: `work_records`  
**主键**: `id` (TEXT, UUID)  
**说明**: 员工在各平台的操作执行记录（小红书、抖音等）

#### 字段定义

| 字段名 | 数据类型 | 约束 | 默认值 | 说明 |
|--------|---------|------|--------|------|
| **id** | TEXT | PRIMARY KEY | UUID | 记录唯一标识 |
| **user_id** | TEXT | NOT NULL, FK → users(id) | - | 执行用户ID |
| **device_id** | TEXT | NOT NULL | - | 执行设备ID |
| **platform** | TEXT | NOT NULL | - | 平台名称（xiaohongshu/douyin） |
| **action_type** | TEXT | NOT NULL | - | 操作类型（follow/like/comment） |
| **target_count** | INTEGER | NOT NULL | 0 | 目标数量 |
| **completed_count** | INTEGER | NOT NULL | 0 | 完成数量 |
| **status** | TEXT | NOT NULL | 'pending' | 记录状态 |
| **created_at** | DATETIME | NOT NULL | CURRENT_TIMESTAMP | 创建时间 |
| **updated_at** | DATETIME | NOT NULL | CURRENT_TIMESTAMP | 更新时间 |

#### 平台枚举（platform）

| 值 | 中文名称 |
|----|---------|
| `xiaohongshu` | 小红书 |
| `douyin` | 抖音 |
| `weibo` | 微博 |

#### 操作类型（action_type）

| 值 | 中文名称 |
|----|---------|
| `follow` | 关注 |
| `like` | 点赞 |
| `favorite` | 收藏 |
| `comment` | 评论 |
| `share` | 分享 |

---

### 8. billing_records 计费记录表

**表名**: `billing_records`  
**主键**: `id` (TEXT, UUID)  
**说明**: 用户操作的计费流水记录

#### 字段定义

| 字段名 | 数据类型 | 约束 | 默认值 | 说明 |
|--------|---------|------|--------|------|
| **id** | TEXT | PRIMARY KEY | UUID | 计费记录唯一标识 |
| **user_id** | TEXT | NOT NULL, FK → users(id) | - | 用户ID |
| **amount** | REAL | NOT NULL | - | 计费金额 |
| **billing_type** | TEXT | NOT NULL | - | 计费类型 |
| **description** | TEXT | - | NULL | 计费描述 |
| **created_at** | DATETIME | NOT NULL | CURRENT_TIMESTAMP | 创建时间 |

#### 计费类型（billing_type）

| 值 | 说明 |
|----|------|
| `monthly_fee` | 月费 |
| `operation_fee` | 操作费 |
| `storage_fee` | 存储费 |
| `recharge` | 充值 |

---

## 系统配置表

### 9. pricing_rules 价格规则表

**表名**: `pricing_rules`  
**主键**: `id` (INTEGER, AUTO_INCREMENT)  
**说明**: 系统级别的定价规则

#### 字段定义

| 字段名 | 数据类型 | 约束 | 默认值 | 说明 |
|--------|---------|------|--------|------|
| **id** | INTEGER | PRIMARY KEY, AUTO_INCREMENT | - | 规则ID |
| **rule_name** | TEXT | NOT NULL | - | 规则名称 |
| **billing_type** | TEXT | NOT NULL | - | 计费类型 |
| **unit_price** | REAL | NOT NULL | - | 单价 |
| **is_active** | BOOLEAN | - | TRUE | 是否启用 |
| **created_at** | DATETIME | - | CURRENT_TIMESTAMP | 创建时间 |
| **updated_at** | DATETIME | - | CURRENT_TIMESTAMP | 更新时间 |

---

### 10. company_pricing_plans 公司收费计划表

**表名**: `company_pricing_plans`  
**主键**: `id` (INTEGER, AUTO_INCREMENT)  
**说明**: 公司级别的定价计划

#### 字段定义

| 字段名 | 数据类型 | 约束 | 默认值 | 说明 |
|--------|---------|------|--------|------|
| **id** | INTEGER | PRIMARY KEY, AUTO_INCREMENT | - | 计划ID |
| **company_name** | TEXT | NOT NULL, UNIQUE | - | 公司名称 |
| **plan_name** | TEXT | NOT NULL | - | 计划名称 |
| **employee_monthly_fee** | REAL | NOT NULL | 50.0 | 员工月费 |
| **is_active** | BOOLEAN | - | TRUE | 是否启用 |
| **created_at** | DATETIME | - | CURRENT_TIMESTAMP | 创建时间 |
| **updated_at** | DATETIME | - | CURRENT_TIMESTAMP | 更新时间 |

#### 数据示例

```json
{
  "id": 1,
  "company_name": "company_001",
  "plan_name": "标准计划",
  "employee_monthly_fee": 50.0,
  "is_active": true
}
```

---

### 11. company_operation_pricing 公司操作收费表

**表名**: `company_operation_pricing`  
**主键**: `id` (INTEGER, AUTO_INCREMENT)  
**说明**: 公司在不同平台的操作单价

#### 字段定义

| 字段名 | 数据类型 | 约束 | 默认值 | 说明 |
|--------|---------|------|--------|------|
| **id** | INTEGER | PRIMARY KEY, AUTO_INCREMENT | - | 记录ID |
| **company_name** | TEXT | NOT NULL | - | 公司名称 |
| **platform** | TEXT | NOT NULL | - | 平台名称 |
| **operation_type** | TEXT | NOT NULL | - | 操作类型 |
| **unit_price** | REAL | NOT NULL | - | 单价 |
| **is_active** | BOOLEAN | - | TRUE | 是否启用 |
| **created_at** | DATETIME | - | CURRENT_TIMESTAMP | 创建时间 |
| **updated_at** | DATETIME | - | CURRENT_TIMESTAMP | 更新时间 |

#### 唯一约束

```sql
UNIQUE(company_name, platform, operation_type)
```

#### 数据示例

```json
{
  "company_name": "company_001",
  "platform": "xiaohongshu",
  "operation_type": "follow",
  "unit_price": 0.08
}
```

---

### 12. system_settings 系统设置表

**表名**: `system_settings`  
**主键**: `key` (TEXT)  
**说明**: 系统配置键值对

#### 字段定义

| 字段名 | 数据类型 | 约束 | 默认值 | 说明 |
|--------|---------|------|--------|------|
| **key** | TEXT | PRIMARY KEY | - | 配置键 |
| **value** | TEXT | NOT NULL | - | 配置值 |
| **description** | TEXT | - | NULL | 配置说明 |
| **updated_at** | DATETIME | NOT NULL | CURRENT_TIMESTAMP | 更新时间 |

---

## 数据关系图

### 核心业务数据流

```
┌─────────────┐
│  Companies  │ (多租户核心)
│   公司表     │
└──────┬──────┘
       │ 1:N
       ▼
┌─────────────┐         ┌─────────────┐
│    Users    │ ────┬──▶│  Projects   │
│   用户表     │ 1:N │   │   项目表     │
└──────┬──────┘     │   └──────┬──────┘
       │            │          │ 1:N
       │ 1:N        │          ▼
       │            │   ┌─────────────┐
       │            └──▶│    Tasks    │ (核心实体)
       │                │   任务表     │
       │                └──────┬──────┘
       │                       │ 1:N
       │                       ▼
       │                ┌─────────────┐
       └───────────────▶│ Work Logs   │
         1:N            │ 工作日志表   │
                        └─────────────┘
```

### 外键关系总览

| 从表 | 外键字段 | 关联主表 | 关联字段 | 删除策略 |
|------|---------|---------|---------|---------|
| users | company_id | companies | id | - |
| users | parent_id | users | id | - |
| projects | owner_id | users | id | CASCADE |
| tasks | project_id | projects | id | CASCADE |
| tasks | assigned_to | users | id | SET NULL |
| tasks | created_by | users | id | CASCADE |
| work_logs | task_id | tasks | id | CASCADE |
| work_logs | user_id | users | id | CASCADE |
| devices | user_id | users | id | - |
| work_records | user_id | users | id | - |
| billing_records | user_id | users | id | - |

---

## 角色权限与数据访问

### PlatformAdmin（平台管理员）

**角色定位**: SaaS 平台运营方

| 数据表 | 查看 | 创建 | 编辑 | 删除 | 备注 |
|--------|------|------|------|------|------|
| companies | ✅ 所有 | ✅ | ✅ | ✅ | 管理所有客户公司 |
| users | ✅ 所有 | ✅ | ✅ | ✅ | 管理所有用户 |
| projects | ✅ 所有 | ✅ | ✅ | ✅ | 查看所有项目 |
| tasks | ✅ 所有 | ✅ | ✅ | ✅ | 查看所有任务 |
| work_logs | ✅ 所有 | ✅ | ✅ | ✅ | 查看所有工时 |
| pricing_rules | ✅ | ✅ | ✅ | ✅ | 管理定价规则 |
| system_settings | ✅ | ✅ | ✅ | ✅ | 管理系统配置 |

**数据过滤**: 无过滤，可访问所有数据

---

### ProjectManager（项目经理）

**角色定位**: 企业内部管理者，管理本公司项目和任务

| 数据表 | 查看 | 创建 | 编辑 | 删除 | 备注 |
|--------|------|------|------|------|------|
| companies | ✅ 本公司 | ❌ | ❌ | ❌ | 只读本公司信息 |
| users | ✅ 本公司 | ✅ TaskExecutor | ✅ 本公司 | ⚠️ 禁用 | 有员工配额限制 |
| projects | ✅ 本公司 | ✅ | ✅ 本公司 | ✅ 需确认 | 管理本公司项目 |
| tasks | ✅ 本公司 | ✅ | ✅ 本公司 | ✅ 本项目 | 管理本公司任务 |
| work_logs | ✅ 本公司 | ✅ | ✅ 本公司 | ✅ 本公司 | 查看员工工时 |

**数据过滤**: 
```sql
WHERE company_id = <当前用户的company_id>
```

---

### TaskExecutor（任务执行者）

**角色定位**: 一线员工，执行任务并记录工时

| 数据表 | 查看 | 创建 | 编辑 | 删除 | 备注 |
|--------|------|------|------|------|------|
| companies | ❌ | ❌ | ❌ | ❌ | - |
| users | ✅ 团队成员 | ❌ | ✅ 自己 | ❌ | 查看协作成员 |
| projects | ✅ 参与的 | ❌ | ❌ | ❌ | 只读项目信息 |
| tasks | ✅ 分配的 | ✅ 个人任务 | ✅ 自己的 | ❌ | 不能删除任务 |
| work_logs | ✅ 自己的 | ✅ 自己的 | ✅ 自己的 | ✅ 自己的 | 记录工作时间 |

**数据过滤**:
```sql
-- 任务查询
WHERE assigned_to = <当前用户ID> 
   OR created_by = <当前用户ID>

-- 工作日志查询
WHERE user_id = <当前用户ID>
```

---

## 数据验证规则

### users 表验证

| 字段 | 规则 | 错误提示 |
|------|------|---------|
| username | 长度: 3-50, 唯一 | "用户名长度必须在3-50字符之间" |
| email | 邮箱格式, 唯一 | "请输入有效的邮箱地址" |
| password | 最小长度: 6 | "密码长度至少6个字符" |
| role | 枚举值 | "角色必须是: platform_admin, project_manager, task_executor" |

### tasks 表验证

| 字段 | 规则 | 错误提示 |
|------|------|---------|
| title | 长度: 1-200 | "任务标题不能为空且不超过200字符" |
| status | 枚举值 | "任务状态无效" |
| priority | 枚举值 | "任务优先级无效" |
| estimated_hours | 范围: 0.1-1000 | "预估工时必须在0.1-1000小时之间" |

### work_logs 表验证

| 字段 | 规则 | 错误提示 |
|------|------|---------|
| hours | 范围: 0.1-24 | "工作时长必须在0.1-24小时之间" |
| notes | 最大长度: 500 | "工作描述不能超过500个字符" |
| logged_at | 不能是未来日期 | "工作日期不能是未来日期" |

---

## 索引策略

### 查询优化索引

**高频查询场景**:

1. **按公司查询用户** (`company_id`)
   ```sql
   SELECT * FROM users WHERE company_id = ? AND is_active = TRUE
   ```

2. **查询用户的待办任务** (`assigned_to`, `status`)
   ```sql
   SELECT * FROM tasks 
   WHERE assigned_to = ? AND status = 'pending'
   ORDER BY due_date ASC
   ```

3. **项目任务统计** (`project_id`, `status`)
   ```sql
   SELECT status, COUNT(*) 
   FROM tasks 
   WHERE project_id = ? 
   GROUP BY status
   ```

4. **员工工时统计** (`user_id`, `logged_at`)
   ```sql
   SELECT SUM(hours) 
   FROM work_logs 
   WHERE user_id = ? 
     AND logged_at >= '2025-10-01' 
     AND logged_at <= '2025-10-31'
   ```

### 索引总览

| 表名 | 索引数量 | 单字段索引 | 复合索引 |
|------|---------|-----------|---------|
| users | 4 | username, email, company_id, role | - |
| projects | 3 | owner_id, status, created_at | - |
| tasks | 9 | 7个单字段 | 2个复合 |
| work_logs | 5 | 3个单字段 | 2个复合 |

---

## 附录

### A. 测试数据概览

参考文档: [`TEST_DATA_SUMMARY.md`](./TEST_DATA_SUMMARY.md)

- **项目**: 3 个
- **任务**: 10 个
- **工作日志**: 8 条
- **用户**: 6 个
- **公司**: 2 个

### B. 角色权限矩阵

参考文档: [`PERMISSION_MATRIX_OPTIMIZED.md`](./PERMISSION_MATRIX_OPTIMIZED.md)

### C. 系统架构分析

参考文档: [`ROLE_SYSTEM_ANALYSIS_AND_OPTIMIZATION.md`](./ROLE_SYSTEM_ANALYSIS_AND_OPTIMIZATION.md)

---

## 版本历史

| 版本 | 日期 | 变更内容 |
|------|------|---------|
| v1.0.0 | 2025-10-30 | 初始版本，包含所有核心业务表和配置表 |

---

**文档维护**: TaskFleet 开发团队  
**最后更新**: 2025-10-30  
**反馈邮箱**: support@taskfleet.com

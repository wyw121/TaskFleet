# TaskFleet 项目架构说明文档

> **生成时间**: 2025年10月28日  
> **版本**: v1.0 (重构后架构)

---

## 📋 概述

TaskFleet 是一个**任务管理和执行专家系统**，采用**前后端分离架构**：

- **后端**: Rust + Axum + SQLite (端口 8000)
- **前端**: React + TypeScript + Vite + Ant Design (开发端口 3000)
- **桌面客户端**: Tauri + Rust (员工角色使用)

---

## 🗄️ 数据库结构

### 当前已创建的表

```sql
-- 用户表 (已创建)
users
├── id INTEGER PRIMARY KEY AUTOINCREMENT
├── username TEXT UNIQUE NOT NULL
├── email TEXT UNIQUE
├── hashed_password TEXT NOT NULL
├── role TEXT (system_admin | user_admin | employee)
├── is_active BOOLEAN
├── parent_id INTEGER
├── company TEXT
└── created_at, updated_at, last_login

-- 其他业务表
work_records          -- 工作记录
devices              -- 设备管理
billing_records      -- 计费记录
pricing_rules        -- 价格规则
company_pricing_plans -- 公司收费计划
company_operation_pricing -- 公司操作定价
system_settings      -- 系统配置
```

### ⚠️ 尚未迁移的表

```sql
-- 待迁移：需要创建或执行 SQL 迁移文件
tasks      -- 任务管理（迁移文件：migrations/003_create_tasks_table.sql）
projects   -- 项目管理（迁移文件：migrations/002_create_projects_table.sql）
```

**状态说明**:
- 这两个表的迁移 SQL 文件已存在于 `server-backend/migrations/` 目录
- 但**尚未执行到数据库**中（当前数据库迁移逻辑在 `database.rs::migrate()` 中手动编写）
- **临时方案**: 后端创建了 `tasks_temp.rs` handler 返回空数组，避免前端 404 错误

---

## 👥 系统账户

### 当前数据库中的用户

| ID | 用户名 | 邮箱 | 角色 | 密码 |
|----|--------|------|------|------|
| 1 | admin | admin@flowfarm.com | **system_admin** (系统管理员) | admin123 |
| 2 | company_admin_1 | company_admin_1@example.com | **user_admin** (公司管理员) | admin123 |
| 3 | company_admin_2 | company_admin_2@example.com | **user_admin** (公司管理员) | admin123 |
| 4 | employee_1 | employee_1@company_001.com | **employee** (员工) | admin123 |
| 5 | employee_2 | employee_2@company_001.com | **employee** (员工) | admin123 |
| 6 | employee_3 | employee_3@company_002.com | **employee** (员工) | admin123 |

### 角色权限说明

1. **system_admin** (系统管理员)
   - 最高权限
   - 可管理所有公司和用户
   - 可访问系统设置和全局统计

2. **user_admin** (公司管理员)
   - 管理本公司员工
   - 分配任务给员工
   - 查看公司级统计数据

3. **employee** (普通员工)
   - 查看分配给自己的任务
   - 更新任务状态
   - 使用桌面客户端执行任务

---

## 🔄 数据流向 (请求路径)

### 典型API调用流程

```
前端浏览器 (localhost:3000)
    ↓
【1. 前端代码发起请求】
    fetch('/api/v1/tasks')
    ↓
【2. Vite 开发服务器代理】
    vite.config.ts: proxy: { '/api': 'http://localhost:8000' }
    ↓
    转发到 → http://localhost:8000/api/v1/tasks
    ↓
【3. 后端 Axum 路由匹配】
    server.rs: .route("/api/v1/tasks", get(handlers::tasks_temp::list_tasks))
    ↓
【4. Handler 处理请求】
    handlers/tasks_temp.rs: list_tasks() → 返回 JSON
    ↓
【5. 中间件处理】
    - AuthLayer: 验证 JWT token
    - CORS: 允许跨域
    - Compression: 压缩响应
    ↓
【6. 数据库查询 (如果需要)】
    sqlx::query("SELECT * FROM tasks") → SQLite (data/taskfleet.db)
    ↓
【7. 返回响应】
    JSON { success: true, data: [...] }
    ↓
前端接收并渲染
```

---

## 🛣️ API 端点状态

### ✅ 已实现且可用

| 端点 | 方法 | 功能 | 状态 |
|------|------|------|------|
| `/api/v1/auth/login` | POST | 登录 | ✅ 正常 |
| `/api/v1/auth/register` | POST | 注册 | ✅ 正常 |
| `/api/v1/auth/refresh` | POST | 刷新token | ✅ 正常 |
| `/api/v1/users` | GET | 用户列表 | ✅ 正常 |
| `/api/v1/users/:id` | GET/PUT/DELETE | 用户CRUD | ✅ 正常 |
| `/api/v1/statistics/tasks` | GET | 任务统计 | ✅ 返回0值 |
| `/api/v1/statistics/projects` | GET | 项目统计 | ✅ 返回0值 |

### 🚧 临时实现 (返回空数据)

| 端点 | 方法 | 功能 | 状态 |
|------|------|------|------|
| `/api/v1/tasks` | GET | 任务列表 | 🚧 返回空数组 `[]` |
| `/api/v1/tasks/:id` | GET | 任务详情 | 🚧 返回404 |
| `/api/v1/tasks` | POST | 创建任务 | 🚧 返回400 |
| `/api/v1/tasks/:id` | PUT | 更新任务 | 🚧 返回400 |
| `/api/v1/tasks/:id` | DELETE | 删除任务 | 🚧 返回400 |

### ❌ 尚未实现

| 端点 | 方法 | 功能 | 原因 |
|------|------|------|------|
| `/api/v1/projects/*` | ALL | 项目管理 | ❌ 路由被注释 |
| `/api/v1/tasks/:id/start` | POST | 开始任务 | ❌ 路由被注释 |
| `/api/v1/tasks/:id/complete` | POST | 完成任务 | ❌ 路由被注释 |
| `/api/v1/statistics/users/workload` | GET | 用户工作量 | ❌ 依赖未迁移 |

---

## 🔧 类型迁移进度

### 已完成迁移 (Uuid → i64)

- ✅ `User.id`: Uuid → i64
- ✅ `UserInfo.id`: Uuid → i64
- ✅ `UserRepository`: 所有方法
- ✅ `UserService`: 所有方法
- ✅ `handlers::users`: 路由参数
- ✅ `services::auth`: register, refresh_token

### 待迁移模块

- ❌ `Task` 模型和 handlers
- ❌ `Project` 模型和 handlers
- ❌ `Statistics` 按用户/项目的端点
- ❌ `WorkRecord` 等业务模型

---

## 📂 核心文件结构

```
server-backend/
├── src/
│   ├── main.rs              # 入口文件
│   ├── server.rs            # 路由配置 ⭐
│   ├── database.rs          # 数据库连接和迁移
│   ├── models.rs            # 数据模型定义
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── auth.rs          # ✅ 认证端点
│   │   ├── users.rs         # ✅ 用户管理
│   │   ├── tasks_temp.rs    # 🚧 临时任务端点
│   │   ├── statistics.rs    # 🚧 统计端点
│   │   └── tasks.rs         # ❌ 完整任务功能（被注释）
│   ├── services/
│   │   ├── auth.rs          # ✅ 认证服务
│   │   ├── user.rs          # ✅ 用户服务
│   │   └── statistics.rs    # 🚧 统计服务
│   ├── repositories/
│   │   └── user_repository.rs  # ✅ 用户数据访问
│   └── middleware/
│       └── auth.rs          # JWT验证中间件
├── data/
│   └── taskfleet.db         # SQLite 数据库文件
└── migrations/
    ├── 002_create_projects_table.sql   # ⚠️ 未执行
    └── 003_create_tasks_table.sql      # ⚠️ 未执行

server-frontend/
├── src/
│   ├── services/
│   │   ├── api.ts           # Axios 客户端 (带Vite代理)
│   │   ├── authService.ts   # ✅ 认证服务
│   │   ├── userService.ts   # ✅ 用户服务
│   │   ├── taskService.ts   # 🚧 任务服务 (调用空API)
│   │   └── analyticsService.ts  # 🚧 统计服务
│   ├── pages/
│   │   ├── Login.tsx        # ✅ 登录页
│   │   ├── SystemAdminDashboard.tsx  # 🚧 管理员仪表盘
│   │   └── UserAdmin/
│   │       └── TaskManagement.tsx   # 🚧 任务管理页
│   └── store/
│       ├── authSlice.ts     # Redux认证状态
│       └── index.ts
└── vite.config.ts           # ⚠️ 代理配置 (关键)
```

---

## 🐛 当前已知问题

### 1. Tasks/Projects 404 错误

**问题**: 前端调用 `/api/v1/tasks` 返回 404 或空数组  
**原因**:
- 数据库中没有 `tasks` 和 `projects` 表
- 后端路由使用临时 handler 返回空数组

**解决方案**:
```bash
# 方案A: 执行 SQL 迁移文件
cd server-backend
sqlite3 data/taskfleet.db < migrations/003_create_tasks_table.sql
sqlite3 data/taskfleet.db < migrations/002_create_projects_table.sql

# 方案B: 在 database.rs::migrate() 中添加表创建逻辑
```

### 2. 用户管理页面数据问题

**问题**: `/users` 页面可能显示异常  
**原因**: 前端期望的用户字段可能与后端返回的不一致  
**状态**: 需要检查 `UserInfo` 结构与前端 TypeScript 类型定义

### 3. 统计数据全为 0

**问题**: Dashboard 显示所有统计为 0  
**原因**: `tasks` 和 `projects` 表不存在，COUNT 查询返回 0  
**状态**: 预期行为，等待表创建后自动恢复

---

## 🚀 后续开发建议

### 短期 (紧急修复)

1. **执行数据库迁移**
   ```bash
   cd server-backend
   sqlite3 data/taskfleet.db < migrations/002_create_projects_table.sql
   sqlite3 data/taskfleet.db < migrations/003_create_tasks_table.sql
   ```

2. **完成 Task/Project 类型迁移**
   - 将所有 Uuid 字段改为 i64 或 String
   - 更新 handlers 和 services

3. **恢复完整路由**
   - 解除 `server.rs` 中被注释的 tasks/projects 路由
   - 移除 `tasks_temp.rs` 临时方案

### 中期 (功能完善)

1. **数据填充**: 创建测试任务和项目数据
2. **权限控制**: 实现基于角色的访问控制 (RBAC)
3. **前端优化**: 处理空数据状态显示

### 长期 (架构优化)

1. **迁移系统重构**: 使用 sqlx migrations 或 sea-orm
2. **测试覆盖**: 添加单元测试和集成测试
3. **性能优化**: 添加查询缓存和连接池优化

---

## 📞 故障排查清单

遇到问题时按以下顺序检查：

1. ✅ **后端是否运行**: `http://localhost:8000/health` 返回 200
2. ✅ **前端代理配置**: `vite.config.ts` 中 proxy 设置正确
3. ✅ **数据库连接**: 检查 `data/taskfleet.db` 文件存在
4. ✅ **路由注册**: 在 `server.rs` 中查找对应端点
5. ✅ **JWT token**: 登录后 localStorage 中有 `token`
6. ✅ **CORS 配置**: 后端允许 `localhost:3000` 跨域

---

**文档版本**: 1.0  
**最后更新**: 2025-10-28 21:30  
**维护者**: TaskFleet 开发团队

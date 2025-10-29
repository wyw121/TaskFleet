# TaskFleet 项目状态与需求分析报告

**生成时间**: 2025年10月29日  
**项目状态**: Phase 3 已完成 - 多租户 SaaS 系统已就绪  
**版本**: v0.2.0 (多租户版本)

---

## 📊 一、项目当前状态

### 1.1 技术架构现状

```
┌─────────────────────────────────────────────────────────────────┐
│                    TaskFleet 多租户 SaaS 系统                      │
└─────────────────────────────────────────────────────────────────┘

前端层 (React + TypeScript)
├─ Web 管理端 (端口 3000)
│  ├─ 技术栈: React 19.2.0 + TypeScript + Vite + Ant Design 5
│  ├─ 状态管理: Redux Toolkit
│  ├─ 路由: react-router-dom v6 (基于角色的访问控制)
│  ├─ 权限系统: ProtectedRoute + usePermissions Hook
│  └─ 页面模块:
│     ├─ 登录页 (Login.tsx)
│     ├─ 仪表板 (Dashboard.tsx) - 所有角色
│     ├─ 任务管理 (TaskManagement.tsx) - 所有角色
│     ├─ 项目管理 (ProjectManagement.tsx) - 所有角色
│     ├─ 数据分析 (Analytics.tsx) - 管理员
│     ├─ 用户管理 (UserManagement.tsx) - 管理员
│     └─ 公司管理 (待实现) - 系统管理员
│
├─ 桌面客户端 (Tauri 2.0)
│  ├─ 技术栈: Rust + Tauri + HTML/CSS/JS
│  ├─ 目标用户: 员工 (轻量级任务查看和更新)
│  └─ 状态: 架构已规划, 实现中
│
└─────────────────────────────────────────────────────────────────

后端层 (Rust + Axum)
├─ RESTful API 服务 (端口 8000)
│  ├─ 框架: Axum 0.7
│  ├─ 数据库: SQLite (开发) / PostgreSQL (生产)
│  ├─ 认证: JWT (jsonwebtoken)
│  ├─ 中间件: AuthContext (角色解析)
│  └─ 日志: tracing + tracing-subscriber
│
├─ API 端点架构:
│  ├─ /api/v1/auth/* - 认证 (登录/注册/当前用户)
│  ├─ /api/v1/users/* - 用户管理 (CRUD + 多租户隔离)
│  ├─ /api/v1/companies/* - 公司管理 (系统管理员)
│  ├─ /api/v1/projects/* - 项目管理 (带 company_id 过滤)
│  ├─ /api/v1/tasks/* - 任务管理 (带 company_id 过滤)
│  └─ /health - 健康检查
│
└─────────────────────────────────────────────────────────────────

数据层 (SQLite/PostgreSQL)
├─ 核心表结构:
│  ├─ companies (公司主表)
│  │  └─ id, name, contact_email, phone, address, status
│  │
│  ├─ users (用户表 - 多租户)
│  │  └─ id, username, email, password_hash, role, company_id ✓
│  │     parent_id (层级结构), is_active, created_at
│  │
│  ├─ projects (项目表 - 多租户)
│  │  └─ id, name, description, status, start_date, end_date,
│  │     manager_id, company_id ✓, created_at, updated_at
│  │
│  ├─ tasks (任务表 - 多租户)
│  │  └─ id, title, description, status, priority, due_date,
│  │     project_id, assignee_id, company_id ✓, created_at
│  │
│  └─ work_logs (工作日志)
│     └─ id, task_id, user_id, hours_spent, description
│
├─ 多租户隔离机制:
│  ├─ 外键约束: users.company_id → companies.id
│  ├─ 索引优化: idx_users_company_id, idx_tasks_company_id
│  └─ Repository 层自动过滤: list_by_company_id()
│
└─────────────────────────────────────────────────────────────────
```

### 1.2 数据流转模型

```
┌──────────────────────────────────────────────────────────────┐
│                      完整数据流程图                            │
└──────────────────────────────────────────────────────────────┘

【用户登录】
用户输入 → Login.tsx
  ↓ POST /api/v1/auth/login {username, password}
handlers/auth.rs → AuthService::login()
  ↓ 验证密码 (bcrypt)
生成 JWT Token (包含 user_id, role, company_id)
  ↓ Response {token, user_info}
前端存储 localStorage.setItem('token', token)
  ↓ Redux: authSlice.setUser({user, token})
重定向到 /dashboard

─────────────────────────────────────────────────────────────

【查看任务列表 - 多租户隔离】
用户访问 /tasks → TaskManagement.tsx
  ↓ useEffect: 自动调用 API
GET /api/v1/tasks (Header: Authorization: Bearer <token>)
  ↓
后端 AuthContext 中间件拦截
  ↓ 解析 JWT → 提取 user_id, role, company_id
handlers/task.rs → TaskService::list_tasks_by_company(company_id)
  ↓
TaskRepository::list_by_company_id(company_id)
  ↓ SQL: SELECT * FROM tasks WHERE company_id = ? AND ...
数据库返回: [Task1, Task2, ...]
  ↓ 组装 JSON Response
前端接收 → setState(tasks)
  ↓
Table 组件渲染任务列表

─────────────────────────────────────────────────────────────

【创建任务 - 权限控制】
管理员点击 "创建任务" 按钮
  ↓ 前端检查: usePermissions().canCreateTask()
    - SystemAdmin: ✅ 允许
    - CompanyAdmin: ✅ 允许
    - Employee: ❌ 按钮禁用
  ↓
Modal 弹出表单 → 填写 title, description, assignee
  ↓ POST /api/v1/tasks {title, ...}
handlers/task.rs → 检查用户角色
  ↓ if role != SystemAdmin && role != CompanyAdmin
    return AppError::Forbidden
  ↓
TaskService::create_task(company_id, request)
  ↓ 自动注入当前用户的 company_id
TaskRepository::create(task)
  ↓ INSERT INTO tasks (title, company_id, ...) VALUES (...)
返回新创建的 Task
  ↓ Response {success: true, data: task}
前端刷新任务列表

─────────────────────────────────────────────────────────────

【路由权限守卫】
用户尝试访问 /users
  ↓
App.tsx: <ProtectedRoute allowedRoles={[SystemAdmin, CompanyAdmin]}>
  ↓ 从 Redux 获取 user.role
  ↓ 检查: user.role in allowedRoles?
    - SystemAdmin: ✅ 渲染 <UserManagement />
    - CompanyAdmin: ✅ 渲染 <UserManagement />
    - Employee: ❌ 显示 403 页面 "抱歉,您没有权限访问此页面"

─────────────────────────────────────────────────────────────

【菜单动态显示】
AppLayout 渲染 → Sidebar.tsx
  ↓ usePermissions() 获取权限函数
  ↓ useMemo 计算菜单项:
    - canViewAnalytics() → 显示"数据分析"菜单?
    - canManageUsers() → 显示"员工管理"菜单?
    - canManageCompanies() → 显示"公司管理"菜单?
  ↓
Menu 组件渲染:
  - SystemAdmin: 7个菜单项 (含公司管理)
  - CompanyAdmin: 6个菜单项 (无公司管理)
  - Employee: 3个菜单项 (仪表板/任务/项目)
```

---

## 🎯 二、核心需求分析

### 2.1 问题背景 - 为什么需要 TaskFleet?

#### 现实痛点 1: 传统项目管理工具过于复杂

**场景描述**:
```
某市场调研公司,有1个项目经理 + 20名调研员
需求: 将 500 份问卷调查任务分配给员工

使用 Jira/禅道:
❌ 需要配置项目权限、工作流、字段、看板...
❌ 新员工需要2小时培训才能使用
❌ 创建500个任务需要手动点击500次
❌ 项目经理看不到简洁的统计报表 (只有复杂的燃尽图)
```

**TaskFleet 解决方案**:
```
✅ 导入 CSV 文件,自动创建500个任务 (10秒)
✅ 一键均衡分配给20名员工 (考虑负载)
✅ 员工打开桌面端,立即看到分配的任务 (零培训)
✅ 项目经理看到清晰的完成率: 245/500 (49%) ↗️
```

#### 现实痛点 2: 多公司 SaaS 缺乏数据隔离

**场景描述**:
```
TaskFleet 作为 SaaS 平台,服务多家企业:
- 公司A: 某电商的客户拜访团队 (30人)
- 公司B: 某咨询公司的数据录入团队 (50人)
- 公司C: 某教育机构的教学质检团队 (20人)

如果没有多租户隔离:
❌ 公司A的管理员能看到公司B的客户数据 (数据泄露!)
❌ 公司C的员工能修改公司A的任务 (越权操作!)
❌ 系统无法按公司统计数据 (混在一起)
```

**TaskFleet 解决方案 (已实现)**:
```
✅ 每个公司有独立的 company_id
✅ 所有数据表 (users, tasks, projects) 都关联 company_id
✅ Repository 层自动过滤: WHERE company_id = ?
✅ 三级权限:
   - SystemAdmin: 跨公司管理 (平台运营方)
   - CompanyAdmin: 本公司管理 (企业管理员)
   - Employee: 只能看自己的任务 (一线员工)
```

#### 现实痛点 3: 员工端操作繁琐

**场景描述**:
```
调研员小李的一天:
- 8:00 到公司 → 打开浏览器登录系统 → 查看今天的5个任务
- 9:00 完成第1个任务 → 再次打开浏览器 → 点击"标记完成"
- 11:00 完成第2个任务 → 又要打开浏览器...
- 下午依此类推...

问题:
❌ 每次都要打开浏览器 (耗时、容易忘记)
❌ 没有桌面通知 (新任务分配了也不知道)
❌ 不能离线查看任务列表
```

**TaskFleet 解决方案 (规划中)**:
```
✅ 桌面客户端 (Tauri) 开机自启 + 系统托盘
✅ 新任务分配 → 立即弹出通知
✅ 双击托盘图标 → 快速查看和更新任务
✅ 离线缓存,地铁里也能查看任务列表
```

### 2.2 核心需求定义

#### 必要目标 (Must Have)

**目标 1: 极简的任务分发流程**
```
输入: 一个 CSV 文件 (500行任务)
处理: 系统自动分配给员工 (考虑负载均衡)
输出: 每个员工立即看到分配的任务
时间: < 1 分钟
```

**实现状态**: 
- ✅ 数据模型已就绪 (tasks 表, projects 表)
- ⏳ CSV 导入功能待开发
- ⏳ 智能分配算法待开发

**目标 2: 完整的多租户数据隔离**
```
要求:
- 每个公司的数据物理隔离 (company_id 外键)
- 公司A的管理员无法访问公司B的数据
- 系统管理员可以跨公司管理 (平台运维)
```

**实现状态**: ✅ 已完成 (Phase 3)
- ✅ companies 表创建
- ✅ users, tasks, projects 都有 company_id
- ✅ Repository 层自动过滤
- ✅ 三级权限控制 (SystemAdmin/CompanyAdmin/Employee)

**目标 3: 实时的进度监控**
```
项目经理打开系统,立即看到:
- 总任务数: 500
- 已完成: 245 (49%) ↗️ +12 今日
- 进行中: 180 (36%)
- 未开始: 75 (15%)
- 预警: 5个任务即将逾期 ⚠️
```

**实现状态**:
- ✅ 数据模型支持 (task.status, task.due_date)
- ⏳ Dashboard 统计页面待开发
- ⏳ 逾期预警逻辑待开发

#### 重要目标 (Should Have)

**目标 4: 深度数据统计**
```
维度:
- 员工效率: 张三本月完成 80 个任务,李四 65 个
- 时间趋势: 本周完成率 92% ↗️ 上周 85%
- 任务类型: 客户拜访 60%, 数据录入 40%
- 工时统计: 项目A 共耗时 320 小时
```

**实现状态**:
- ✅ work_logs 表已创建 (记录工时)
- ⏳ Analytics 页面待开发
- ⏳ 图表可视化待集成 (ECharts/Recharts)

**目标 5: 桌面客户端 (员工端)**
```
功能:
- 系统托盘常驻,快速访问
- 新任务桌面通知
- 离线查看任务列表
- 一键更新任务状态
```

**实现状态**:
- ✅ Tauri 项目架构已规划
- ⏳ 基础功能开发中
- ⏳ 离线缓存机制待实现

#### 可选目标 (Nice to Have)

**目标 6: 移动端应用**
- 📱 iOS/Android 原生应用
- 🔔 推送通知
- 📸 拍照上传任务附件

**目标 7: 高级自动化**
- 🤖 任务自动分配 (AI 算法)
- 📊 预测项目完成时间
- ⚡ 异常自动提醒 (员工连续3天无进展)

### 2.3 目标用户画像

#### 用户类型 1: 项目经理 (主要用户)

**典型画像**: 王经理,35岁,市场调研公司项目负责人
- 管理 20 名调研员
- 每月执行 5-8 个调研项目
- 每个项目包含 200-500 个任务

**核心需求**:
1. 快速导入任务 (CSV/Excel)
2. 一键分配给员工
3. 实时查看完成进度
4. 导出统计报表 (给客户汇报)

**使用场景**:
```
周一上午:
1. 收到客户需求: 500份问卷调查
2. 打开 TaskFleet Web 端
3. 创建项目 "XX品牌市场调研"
4. 导入 500 个任务 (CSV)
5. 点击"智能分配" → 系统自动分配给20名员工
6. 发送通知给员工

工作日:
- 早上查看昨日完成情况
- 中午检查异常任务 (长时间无进展)
- 下午调整任务分配 (重新分配给其他人)

周五下午:
- 导出本周统计报表
- 发送给客户
```

#### 用户类型 2: 一线员工 (次要用户)

**典型画像**: 小李,25岁,市场调研员
- 每天完成 10-15 个调研任务
- 使用桌面客户端

**核心需求**:
1. 快速查看今天的任务
2. 一键标记"完成"
3. 添加任务备注/照片
4. 不需要复杂功能

**使用场景**:
```
早上 8:00:
- 打开电脑,TaskFleet 自动启动 (系统托盘)
- 双击图标,查看今天分配的 12 个任务

工作中:
- 完成第1个任务 → 打开客户端 → 点击"完成"
- 系统弹出: "已完成 1/12 任务,加油!" 🎉

下午 5:00:
- 查看今日进度: 已完成 11/12
- 剩余1个任务明天处理
```

#### 用户类型 3: 系统管理员 (运营方)

**典型画像**: TaskFleet SaaS 平台运营人员
- 管理所有入驻企业
- 负责平台稳定性和数据安全

**核心需求**:
1. 创建和管理公司账户
2. 查看所有公司数据 (跨租户)
3. 监控系统运行状态
4. 处理客户问题

---

## 🚀 三、项目价值定位

### 3.1 解决的核心问题

```
传统项目管理工具的三大痛点:

❌ 痛点 1: 功能复杂,学习成本高
   → TaskFleet: 专注任务执行,零培训上手

❌ 痛点 2: 任务分配繁琐,效率低
   → TaskFleet: CSV 导入 + 智能分配,10秒完成

❌ 痛点 3: 多公司 SaaS 数据不隔离
   → TaskFleet: 完整的多租户架构,数据安全
```

### 3.2 竞争优势

| 对比维度 | Jira/禅道 | DooTask | TaskFleet |
|---------|----------|---------|-----------|
| **学习成本** | 高 (需培训2小时) | 中等 | 低 (5分钟上手) |
| **任务分配** | 手动逐个点击 | 手动逐个点击 | CSV批量导入+自动分配 |
| **多租户** | 不支持 | 不支持 | ✅ 完整隔离 |
| **桌面客户端** | 无 | 无 | ✅ Tauri (轻量级) |
| **技术栈** | Java/PHP | PHP | Rust (高性能) |
| **开源** | 部分开源 | 开源 | 完全开源 MIT |

### 3.3 目标市场

**主要市场**: B2B SaaS (中小企业任务管理)

**细分场景**:
1. 市场调研公司 (问卷调查、街访)
2. 销售团队 (客户拜访、跟进)
3. 客服团队 (工单处理、质检)
4. 数据录入团队 (批量处理)
5. 现场服务 (维修、巡检)

**市场规模估算**:
- 中国中小企业数量: 4000万+
- 有任务分配需求的企业: 约 10% = 400万
- 付费意愿客户: 1% = 4万企业
- 客单价: 1000元/年/企业
- 潜在市场规模: 4000万元/年

---

## 📈 四、当前开发进度

### 4.1 已完成功能 (Phase 1-3)

✅ **Phase 1: 基础用户系统**
- 用户注册/登录 (JWT 认证)
- 用户 CRUD 操作
- 基础 API 架构

✅ **Phase 2: 公司模型 + 层级隔离**
- companies 表创建
- users.company_id 外键
- parent_id 层级结构

✅ **Phase 3: 多租户完整架构** (刚完成!)
- tasks, projects 表添加 company_id
- Repository 层数据隔离
- CompanyService + 6个 API 端点
- 前端 ProtectedRoute 路由守卫
- usePermissions 权限 Hook
- Sidebar 动态菜单
- 三级角色权限 (SystemAdmin/CompanyAdmin/Employee)

### 4.2 待开发功能 (Roadmap)

#### 短期目标 (1-2周)

🔲 **任务管理核心功能**
- [ ] TaskManagement 页面完善
  - [ ] 任务列表 Table (支持排序、筛选)
  - [ ] 创建任务 Modal
  - [ ] 编辑任务 Modal
  - [ ] 任务详情侧边栏
  - [ ] 批量操作 (批量完成、批量删除)

🔲 **项目管理核心功能**
- [ ] ProjectManagement 页面完善
  - [ ] 项目列表卡片视图
  - [ ] 创建项目向导
  - [ ] 项目详情页 (包含任务列表)
  - [ ] 项目进度可视化

🔲 **公司管理页面**
- [ ] CompanyManagement.tsx (新建)
  - [ ] 公司列表 Table
  - [ ] 创建公司 Modal
  - [ ] 编辑公司信息
  - [ ] 启用/禁用公司

#### 中期目标 (1-2月)

🔲 **CSV 导入导出**
- [ ] 任务批量导入 (CSV/Excel)
- [ ] 任务模板下载
- [ ] 数据验证和预览
- [ ] 导入结果报告

🔲 **智能任务分配**
- [ ] 员工负载计算 (当前任务数)
- [ ] 均衡分配算法
- [ ] 按能力/标签分配
- [ ] 分配预览和调整

🔲 **Dashboard 统计图表**
- [ ] 项目完成率饼图
- [ ] 任务完成趋势折线图
- [ ] 员工效率排行榜
- [ ] 逾期任务预警

🔲 **桌面客户端 (Tauri)**
- [ ] 登录界面
- [ ] 任务列表视图
- [ ] 任务详情和更新
- [ ] 系统托盘集成
- [ ] 桌面通知

#### 长期目标 (3-6月)

🔲 **高级功能**
- [ ] 任务评论和协作
- [ ] 文件附件上传
- [ ] 工作日志详细统计
- [ ] 任务模板系统
- [ ] 自动化规则 (定时任务、自动提醒)

🔲 **移动端**
- [ ] React Native / Flutter 移动应用
- [ ] 推送通知
- [ ] 拍照上传

---

## 🎓 五、技术债务和优化项

### 5.1 当前技术债务

⚠️ **数据迁移脚本缺失**
- 问题: migration 006 执行后,现有 tasks 和 projects 的 company_id 为 NULL
- 影响: 现有数据无法被多租户系统访问
- 解决方案: 编写数据修复脚本
  ```sql
  UPDATE tasks SET company_id = (
    SELECT company_id FROM users WHERE users.id = tasks.created_by
  ) WHERE company_id IS NULL;
  
  UPDATE projects SET company_id = (
    SELECT company_id FROM users WHERE users.id = projects.created_by
  ) WHERE company_id IS NULL;
  ```

⚠️ **前端 Bundle 体积过大**
- 问题: dist/assets/index-D8ZwHqys.js = 2.6MB
- 影响: 首次加载慢
- 解决方案: 
  - 启用代码分割 (React.lazy)
  - 按需加载 Ant Design 组件
  - 配置 Vite manualChunks

⚠️ **缺少单元测试**
- 问题: 0% 测试覆盖率
- 影响: 重构风险高
- 解决方案:
  - 后端: 为 CompanyService, TaskService 编写测试
  - 前端: 为 ProtectedRoute, usePermissions 编写测试

### 5.2 性能优化方向

🚀 **数据库优化**
- 添加复合索引: (company_id, status) 用于任务筛选
- 分页查询优化: LIMIT + OFFSET
- 考虑 Redis 缓存热点数据

🚀 **前端优化**
- 虚拟滚动 (任务列表超过1000条)
- 防抖/节流 (搜索输入框)
- 图片懒加载

---

## 📝 六、总结

### 项目定位一句话
> **TaskFleet 是一个专注于任务执行阶段的多租户 SaaS 系统,为需要管理多人团队的项目经理提供极简的任务分发、实时监控和数据统计解决方案**

### 核心价值
1. **极简体验**: 5分钟上手,零培训成本
2. **批量高效**: CSV 导入 + 智能分配,10秒分发500个任务
3. **数据安全**: 完整的多租户架构,公司数据物理隔离
4. **多端协同**: Web全功能版 + 桌面轻量版

### 技术亮点
- 🦀 Rust + Axum 高性能后端 (比 Node.js 快 10x)
- ⚛️ React 19 + TypeScript 现代化前端
- 🖥️ Tauri 2.0 跨平台桌面应用 (比 Electron 小 90%)
- 🔐 完整的 JWT + RBAC 权限系统
- 🏢 生产级多租户 SaaS 架构

### 下一步行动
1. **立即**: 修复数据迁移问题 (company_id NULL 值)
2. **本周**: 完成 TaskManagement 和 ProjectManagement 核心页面
3. **本月**: 实现 CSV 导入和智能分配算法
4. **下月**: 桌面客户端 MVP 版本发布

---

**报告生成者**: GitHub Copilot  
**最后更新**: 2025年10月29日

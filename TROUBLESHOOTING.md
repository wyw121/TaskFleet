# TaskFleet 快速故障排查手册

## 🚨 问题：前端显示 404 / 数据加载失败

### ✅ 已解决：Tasks/Statistics API 404

**现象**: 
```
GET http://localhost:3000/api/v1/tasks 404 (Not Found)
GET http://localhost:3000/api/v1/statistics/tasks 404
```

**根本原因**:
1. ❌ 数据库中没有 `tasks` 和 `projects` 表
2. ❌ 后端路由被注释（已修复）
3. ✅ Vite 代理配置正确 (`/api` → `http://localhost:8000`)

**当前状态**:
- ✅ `/api/v1/tasks` 返回空数组 `[]` (200 OK)
- ✅ `/api/v1/statistics/tasks` 返回零值统计 (200 OK)
- ✅ `/api/v1/statistics/projects` 返回零值统计 (200 OK)
- ✅ 前端不再报 404 错误

---

## 📊 数据库状态

### 已创建的表
```
✅ users               (用户表)
✅ work_records        (工作记录)
✅ devices             (设备)
✅ billing_records     (计费)
✅ pricing_rules       (价格规则)
✅ company_pricing_plans
✅ company_operation_pricing
✅ system_settings
```

### ⚠️ 未创建的表
```
❌ tasks      (需要执行迁移文件)
❌ projects   (需要执行迁移文件)
```

**临时解决方案**:
- 后端使用 `tasks_temp.rs` handler 返回空数组
- 前端页面不会崩溃，但显示"暂无数据"

**永久解决方案**:
```bash
cd server-backend
sqlite3 data/taskfleet.db < migrations/003_create_tasks_table.sql
sqlite3 data/taskfleet.db < migrations/002_create_projects_table.sql
# 然后重启后端，解除完整 tasks handler 注释
```

---

## 👥 测试账户

| 用户名 | 密码 | 角色 | 权限 |
|--------|------|------|------|
| admin | admin123 | system_admin | 全部权限 |
| company_admin_1 | admin123 | user_admin | 管理公司001 |
| company_admin_2 | admin123 | user_admin | 管理公司002 |
| employee_1 | admin123 | employee | 公司001员工 |
| employee_2 | admin123 | employee | 公司001员工 |
| employee_3 | admin123 | employee | 公司002员工 |

---

## 🔍 数据流检查清单

### 1. 后端健康检查
```bash
curl http://localhost:8000/health
# 期望: {"service":"Flow Farm 服务器后端","status":"healthy"}
```

### 2. 登录测试
```powershell
Invoke-WebRequest -Method POST `
  -Uri "http://localhost:8000/api/v1/auth/login" `
  -ContentType "application/json" `
  -Body '{"username":"admin","password":"admin123"}'
# 期望: 200 OK + JWT token
```

### 3. Tasks API 测试
```powershell
# 先登录获取token
$token = "..." # 从登录响应中获取

# 调用tasks API
Invoke-WebRequest -Method GET `
  -Uri "http://localhost:8000/api/v1/tasks" `
  -Headers @{"Authorization"="Bearer $token"}
# 期望: 200 OK + []
```

### 4. 前端代理验证
检查 `server-frontend/vite.config.ts`:
```typescript
server: {
  proxy: {
    '/api': {
      target: 'http://localhost:8000',
      changeOrigin: true,
    },
  },
}
```

### 5. 浏览器网络面板
1. 打开 F12 开发者工具
2. 切换到 Network 标签
3. 刷新页面
4. 检查 `/api/v1/tasks` 请求:
   - Status: 200 (不是 404)
   - Response: `[]`

---

## 🛠️ 常见问题修复

### Q: Dashboard 显示所有统计为 0

**A**: 正常现象，因为 tasks/projects 表不存在或无数据
```json
{
  "total_tasks": 0,
  "pending_tasks": 0,
  "in_progress_tasks": 0,
  "completed_tasks": 0,
  "completion_rate": 0.0
}
```

### Q: 用户管理页面能打开吗？

**A**: ✅ 可以！`/api/v1/users` 端点已完全实现
- 列表: GET `/api/v1/users`
- 详情: GET `/api/v1/users/:id`
- 创建: POST `/api/v1/users`
- 更新: PUT `/api/v1/users/:id`
- 删除: DELETE `/api/v1/users/:id`

### Q: 任务管理页面能用吗？

**A**: 🚧 部分可用
- ✅ 页面可以加载（不会404）
- ✅ 显示空列表
- ❌ 无法创建/编辑任务（需要数据库表）

---

## 🚀 下一步行动

### 立即可做
1. ✅ 使用 admin 账户登录前端
2. ✅ 访问用户管理页面 (http://localhost:3000/users)
3. ✅ 查看 Dashboard（统计为0是正常的）

### 本周完成
1. 执行数据库迁移（创建 tasks/projects 表）
2. 完成 Task/Project 模型的 Uuid → i64 迁移
3. 解除完整 tasks/projects handler 注释
4. 添加测试数据

### 下周计划
1. 实现任务分配功能
2. 添加工作记录统计
3. 桌面客户端对接

---

**文档更新时间**: 2025-10-28 21:35  
**状态**: ✅ 已修复 404 错误，系统可用但数据为空

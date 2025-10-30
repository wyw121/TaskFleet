# 🎯 TaskFleet 前端登录测试指南

## 📋 问题诊断

### ✅ 已确认正常的部分
- 后端API正常: `/api/v1/auth/login` 返回200
- 后端API正常: `/api/v1/auth/me` 返回200
- 角色序列化正确: 返回 `platform_admin`、`project_manager`、`task_executor`
- 数据库正常: 包含company_id列，所有用户使用新角色名

### ❌ 已修复的问题
- **问题**: 前端路由配置使用旧角色名 (`SystemAdmin`, `CompanyAdmin`, `Employee`)
- **位置**: `server-frontend/src/App.tsx` 第104-149行
- **修复**: 已更新为新角色名 (`PlatformAdmin`, `ProjectManager`, `TaskExecutor`)
- **状态**: ✅ 已修复并重新构建

---

## 🔧 测试步骤

### 步骤1: 清理浏览器缓存

**方式A - 使用清理工具页面** (推荐):
1. 打开: http://localhost:8000/clear-cache.html
2. 点击"清理缓存"按钮
3. 点击"前往登录"

**方式B - 手动清理**:
1. 打开浏览器开发者工具 (F12)
2. 切换到Console标签
3. 执行以下命令:
   ```javascript
   localStorage.clear()
   sessionStorage.clear()
   location.reload()
   ```

### 步骤2: 登录测试

访问登录页面: http://localhost:8000/login

#### 测试账号1: 平台管理员
- **用户名**: `admin`
- **密码**: `admin123`
- **预期角色**: PlatformAdmin
- **预期权限**: 所有页面访问

#### 测试账号2: 项目经理
- **用户名**: `company_admin_1`
- **密码**: `admin123`
- **预期角色**: ProjectManager
- **预期权限**: Dashboard, Tasks, Projects, Analytics, Users

#### 测试账号3: 任务执行者
- **用户名**: `employee_1`
- **密码**: `admin123`
- **预期角色**: TaskExecutor
- **预期权限**: Dashboard, Tasks, Projects

---

## 🔍 验证检查清单

登录成功后，请验证以下内容:

### ✅ 1. 登录成功
- [ ] 没有显示"403 没有权限访问此页面"
- [ ] 成功跳转到Dashboard页面
- [ ] 左侧菜单正常显示

### ✅ 2. 用户信息正确
打开浏览器Console，执行:
```javascript
console.log(JSON.parse(localStorage.getItem('persist:root')).auth)
```

检查输出:
- [ ] `isAuthenticated: true`
- [ ] `user.role` 为 `platform_admin` / `project_manager` / `task_executor`
- [ ] `token` 存在

### ✅ 3. 页面访问权限

#### PlatformAdmin 应该可以访问:
- [ ] /dashboard
- [ ] /tasks
- [ ] /projects
- [ ] /analytics
- [ ] /users

#### ProjectManager 应该可以访问:
- [ ] /dashboard
- [ ] /tasks
- [ ] /projects
- [ ] /analytics
- [ ] /users

#### TaskExecutor 应该可以访问:
- [ ] /dashboard
- [ ] /tasks
- [ ] /projects
- [ ] ❌ /analytics (403)
- [ ] ❌ /users (403)

---

## 🐛 如果仍然出现403错误

### 检查1: 确认前端已重新构建
```powershell
cd d:\repositories\TaskFleet\server-frontend
npm run build
```

### 检查2: 确认后端正在运行
```powershell
# 检查进程
Get-Process | Where-Object {$_.ProcessName -like "*flow-farm-backend*"}

# 如果没有运行，启动后端
cd d:\repositories\TaskFleet\server-backend
./target/release/flow-farm-backend.exe
```

### 检查3: 查看浏览器Console错误
1. 按F12打开开发者工具
2. 切换到Console标签
3. 查找红色错误信息
4. 截图发送给我

### 检查4: 查看Network请求
1. 按F12打开开发者工具
2. 切换到Network标签
3. 刷新页面登录
4. 检查 `/api/v1/auth/login` 和 `/api/v1/auth/me` 的响应:
   - 状态码应该是200
   - 响应中的 `user.role` 应该是 `platform_admin` 等

### 检查5: 验证路由配置
在浏览器Console执行:
```javascript
// 检查当前用户角色
fetch('/api/v1/auth/me', {
  headers: {
    'Authorization': 'Bearer ' + localStorage.getItem('token')
  }
})
.then(r => r.json())
.then(d => console.log('当前用户角色:', d.data.role))
```

---

## 📊 预期的登录流程

```
1. 用户访问 /login
   ↓
2. 输入用户名和密码
   ↓
3. POST /api/v1/auth/login
   ← 返回: { token, user: { role: "platform_admin" } }
   ↓
4. 保存token到localStorage
   ↓
5. Redux更新: isAuthenticated = true, user = {...}
   ↓
6. React Router检查: isAuthenticated = true
   ↓
7. 重定向到 /dashboard
   ↓
8. ProtectedRoute检查:
   - allowedRoles = [PlatformAdmin, ProjectManager, TaskExecutor]
   - user.role = "platform_admin"
   - "platform_admin" === PlatformAdmin ✅
   ↓
9. 渲染Dashboard组件 ✅
```

---

## 🔧 紧急修复命令

如果上述步骤都不起作用，执行以下命令强制重置:

```powershell
# 1. 停止所有进程
Get-Process | Where-Object {$_.ProcessName -like "*flow-farm*"} | Stop-Process -Force

# 2. 清理前端构建
cd d:\repositories\TaskFleet\server-frontend
Remove-Item -Recurse -Force dist
npm run build

# 3. 重启后端
cd d:\repositories\TaskFleet\server-backend
Start-Process -FilePath "./target/release/flow-farm-backend.exe" -NoNewWindow

# 4. 等待5秒
Start-Sleep -Seconds 5

# 5. 打开清理工具
Start-Process "http://localhost:8000/clear-cache.html"
```

---

## 📞 需要帮助？

如果问题仍未解决，请提供:
1. 浏览器Console的完整错误日志
2. Network标签中 `/api/v1/auth/login` 的完整响应
3. Network标签中 `/api/v1/auth/me` 的完整响应
4. localStorage中的所有内容 (执行 `console.log(localStorage)`)

---

**最后更新**: 2025-10-30 00:55:00  
**状态**: 🔧 App.tsx已修复，前端已重新构建

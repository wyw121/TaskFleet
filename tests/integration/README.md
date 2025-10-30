# TaskFleet 集成测试指南

## 测试概览

本目录包含 TaskFleet 系统的集成测试,用于验证系统的关键功能和权限控制的一致性。

## 测试文件

### 1. 权限一致性测试 (`test-permission-consistency.ps1`)

验证 Web 端和桌面端的权限控制完全一致。

**测试覆盖**:
- ✅ 三种角色的登录
- ✅ 公司管理权限(仅平台管理员)
- ✅ 用户管理权限(平台管理员 + 项目经理)
- ✅ 任务创建权限(平台管理员 + 项目经理)
- ✅ 任务查看权限(所有角色)

**运行方法**:

```powershell
# 1. 确保后端服务正在运行
cd server-backend
cargo run

# 2. 在新终端运行测试
cd tests/integration
./test-permission-consistency.ps1
```

**预期输出**:

```
=====================================
  TaskFleet 多端权限一致性测试
=====================================

检查服务器连接...
✅ 服务器连接成功

测试用户: admin (平台管理员)
----------------------------------------
1️⃣  测试登录...
  ✅ PASS: 角色正确

2️⃣  测试公司管理权限...
  ✅ PASS: 平台管理员可以查看所有公司

3️⃣  测试用户管理权限...
  ✅ PASS: 管理员可以查看用户列表
  
... (更多测试)

=====================================
         测试结果汇总
=====================================
总测试数: 18
通过: 18
失败: 0

✅ 所有测试通过! 权限控制一致性验证成功!
```

## 测试数据准备

测试需要以下用户账号:

| 用户名 | 密码 | 角色 | 说明 |
|-------|------|------|------|
| admin | admin123 | platform_admin | 平台管理员 |
| manager | manager123 | project_manager | 项目经理 |
| executor | executor123 | task_executor | 任务执行者 |

**创建测试用户**:

```sql
-- 如果测试用户不存在,可以通过后端API创建
-- 或者使用以下SQL插入(需要调整密码哈希)

-- 示例: 通过API创建
# POST /api/v1/users
{
  "username": "admin",
  "email": "admin@taskfleet.com",
  "password": "admin123",
  "role": "platform_admin",
  "full_name": "系统管理员"
}
```

## 自动化测试

### CI/CD 集成

在 GitHub Actions 或其他 CI 系统中运行测试:

```yaml
# .github/workflows/integration-test.yml
name: Integration Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Start Backend
        run: |
          cd server-backend
          cargo run &
          sleep 10
      
      - name: Run Permission Tests
        run: |
          cd tests/integration
          ./test-permission-consistency.ps1
```

### 本地快速测试

```powershell
# 一键启动测试
cd TaskFleet
./run-integration-tests.ps1
```

## 测试失败排查

### 常见问题

1. **服务器连接失败**
   ```
   ❌ 无法连接到服务器: http://localhost:8000
   ```
   **解决**: 确保后端服务正在运行 `cargo run`

2. **用户不存在**
   ```
   ❌ 登录失败: Invalid credentials
   ```
   **解决**: 创建测试用户或检查用户名密码

3. **权限测试失败**
   ```
   ❌ FAIL: 平台管理员可以查看所有公司
   Expected: 200
   Actual: 403
   ```
   **解决**: 检查后端权限逻辑是否正确实现

## 扩展测试

### 添加新的权限测试

```powershell
# 在 test-permission-consistency.ps1 中添加

# 7. 测试删除权限
Write-Host "`n7️⃣  测试删除权限..." -ForegroundColor Yellow
$deleteTaskResult = Test-API -Method DELETE -Endpoint "/api/v1/tasks/1" -Token $token

if ($User.expected_role -in @("platform_admin", "project_manager")) {
    Assert-Equal "管理员可以删除任务" 200 $deleteTaskResult.status
}
else {
    Assert-Equal "任务执行者不能删除任务" 403 $deleteTaskResult.status
}
```

### 测试桌面端

桌面端使用相同的后端 API,因此通过 API 测试即可验证权限一致性。

如需测试桌面端 UI:

```powershell
# 启动桌面端
cd employee-client
cargo tauri dev

# 手动测试:
# 1. 使用不同角色登录
# 2. 检查显示的功能是否与权限匹配
# 3. 尝试执行操作,验证后端权限检查
```

## 测试最佳实践

1. **每次发布前运行**: 确保权限逻辑没有破坏
2. **PR 审查时运行**: 验证新功能不影响现有权限
3. **定期运行**: 发现潜在的权限漏洞
4. **保持测试更新**: 新增功能时同步更新测试

## 测试报告

测试完成后会生成报告,可以保存用于审计:

```powershell
# 保存测试报告
./test-permission-consistency.ps1 | Tee-Object -FilePath "test-report-$(Get-Date -Format 'yyyyMMdd-HHmmss').txt"
```

## 相关文档

- [多端权限统一实现说明](../../docs/MULTI_PLATFORM_PERMISSION_UNITY.md)
- [角色系统分析](../../docs/ROLE_SYSTEM_ANALYSIS_AND_OPTIMIZATION.md)
- [API 文档](../../docs/API.md)

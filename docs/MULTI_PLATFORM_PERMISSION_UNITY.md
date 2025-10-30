# 多端权限统一实现说明

## 背景

TaskFleet 系统支持 Web 端和桌面端两种访问方式,本文档说明如何保证两端权限控制的完全一致性。

## 核心原则

### 1. 权限由角色决定,不由端类型限制

```
✅ 平台管理员在任何端都有完整权限
✅ 项目经理在任何端都能管理项目和任务  
✅ 任务执行者在任何端都只能操作自己的任务
❌ 不存在"Web端才能做X"或"桌面端不能做Y"
```

### 2. UI 差异 ≠ 功能限制

| 特性 | Web端 | 桌面端 | 说明 |
|-----|-------|--------|------|
| 任务列表 | 详细表格视图 | 精简列表视图 | **权限相同**,仅展示形式不同 |
| 批量操作 | 复选框+批量按钮 | 不提供 UI | 技术限制,非权限限制 |
| 数据图表 | 完整统计图表 | 关键指标卡片 | **权限相同**,UI简化 |
| 离线功能 | 不支持 | 支持本地缓存 | 桌面端特有优势 |

### 3. 所有权限检查在后端完成

```rust
// ❌ 错误: 前端根据端类型限制功能
if platform == "desktop" {
    return Err("桌面端不支持此功能");
}

// ✅ 正确: 后端根据角色检查权限
if user.role != UserRole::PlatformAdmin {
    return Err(AppError::Forbidden);
}
```

## 实现细节

### 后端 (server-backend)

后端 API 不区分调用来源,只验证角色权限:

```rust
// src/services/user.rs
pub async fn create_user(
    &self,
    request: CreateUserRequest,
    current_user: &UserInfo,  // 来自JWT,不管Web还是桌面
) -> Result<UserInfo> {
    // 权限检查
    let (company_id, parent_id) = match current_user.role {
        UserRole::PlatformAdmin => {
            // 平台管理员可以创建任何用户
            (request.company_id, request.parent_id)
        }
        UserRole::ProjectManager => {
            // 项目经理只能创建任务执行者
            if request.role != UserRole::TaskExecutor {
                return Err(anyhow!("权限不足"));
            }
            // ...
        }
        UserRole::TaskExecutor => {
            return Err(anyhow!("权限不足"));
        }
    };
    // ... 业务逻辑
}
```

### Web 前端 (server-frontend)

使用 `usePermissions` Hook 动态控制 UI:

```typescript
// src/hooks/usePermissions.ts
export const usePermissions = () => {
  const { user } = useSelector((state: RootState) => state.auth);

  const isPlatformAdmin = () => {
    return user?.role === UserRole.PlatformAdmin;
  };

  const canManageUsers = () => {
    return isPlatformAdmin() || isProjectManager();
  };

  return {
    isPlatformAdmin,
    isProjectManager,
    isTaskExecutor,
    canManageUsers,
    // ...
  };
};

// 组件中使用
const { canCreateTask } = usePermissions();

{canCreateTask() && (
  <Button onClick={createTask}>创建任务</Button>
)}
```

### 桌面端 (employee-client)

#### 1. 权限检查模块

```rust
// src-tauri/src/permissions.rs
pub struct Permissions {
    role: UserRole,
}

impl Permissions {
    pub fn can_create_task(&self) -> bool {
        self.has_admin_role()
    }

    pub fn get_desktop_features(&self) -> Vec<DesktopFeature> {
        let mut features = vec![
            DesktopFeature::ViewTasks,
            DesktopFeature::UpdateTaskStatus,
        ];

        if self.has_admin_role() {
            features.extend_from_slice(&[
                DesktopFeature::CreateTask,
                DesktopFeature::AssignTask,
            ]);
        }

        features
    }
}
```

#### 2. Tauri Command

```rust
// src-tauri/src/taskfleet_commands.rs
#[tauri::command]
pub async fn get_user_permissions(
    state: State<'_, AppState>
) -> Result<UserPermissionsInfo, String> {
    let user = state.current_user.lock().await;
    let permissions = Permissions::new(user.role);
    
    Ok(UserPermissionsInfo {
        role_display: permissions.get_role_display_name(),
        can_create_task: permissions.can_create_task(),
        available_features: permissions.get_desktop_features(),
    })
}
```

#### 3. 前端调用

```javascript
// src-web/app.js
async function loadPermissions() {
    const perms = await invoke('get_user_permissions');
    
    // 动态显示功能
    const menu = document.getElementById('menu');
    perms.available_features.forEach(feature => {
        menu.innerHTML += `
            <button onclick="navigate('${feature.name}')">
                ${feature.icon} ${feature.name}
            </button>
        `;
    });
    
    // 隐藏无权限的按钮
    if (!perms.can_create_task) {
        document.getElementById('create-task-btn').style.display = 'none';
    }
}
```

## 测试验证

### 1. 角色权限一致性测试

测试相同角色在两端的权限:

| 测试项 | 平台管理员 | 项目经理 | 任务执行者 |
|-------|----------|---------|----------|
| 创建用户 (Web) | ✅ 成功 | ✅ 成功 | ❌ 403 |
| 创建用户 (Desktop) | ✅ 成功 | ✅ 成功 | ❌ 403 |
| 查看所有任务 (Web) | ✅ 全部 | ✅ 本公司 | ✅ 自己的 |
| 查看所有任务 (Desktop) | ✅ 全部 | ✅ 本公司 | ✅ 自己的 |
| 删除项目 (Web) | ✅ 成功 | ✅ 成功 | ❌ 403 |
| 删除项目 (Desktop) | ✅ 成功 | ✅ 成功 | ❌ 403 |

### 2. API 调用测试

确保桌面端所有操作都通过后端 API:

```bash
# 启动后端
cd server-backend
cargo run

# 启动桌面端
cd employee-client  
cargo tauri dev

# 监控 API 请求
# 所有操作应该调用 http://localhost:8000/api/*
# 所有请求应该携带 Authorization: Bearer <token>
```

### 3. UI 差异测试

验证 UI 简化不影响功能:

- ✅ Web端批量分配任务 vs 桌面端逐个分配 → 都能完成
- ✅ Web端详细图表 vs 桌面端关键指标 → 数据来源相同
- ✅ 桌面端离线查看任务 → Web端不支持,但不影响权限

## 常见误区

### ❌ 错误理解

```
"桌面端是给员工用的,所以不需要管理功能"
"Web端才能管理公司,桌面端只能查看任务"
```

### ✅ 正确理解

```
"桌面端和Web端都支持所有角色"
"功能由用户角色决定,不由使用的端决定"
"桌面端UI精简,但不阉割功能"
```

## 开发指南

### 添加新功能时的检查清单

1. ✅ 后端 API 是否只检查角色,不检查来源?
2. ✅ Web 前端是否使用 `usePermissions` Hook?
3. ✅ 桌面端是否更新 `permissions.rs`?
4. ✅ 两端权限判断逻辑是否一致?
5. ✅ 文档是否更新说明?

### 代码审查要点

```rust
// ❌ 禁止: 根据端类型限制
if client_type == "desktop" {
    return Err("不支持");
}

// ✅ 推荐: 根据角色权限
if current_user.role != UserRole::PlatformAdmin {
    return Err(AppError::Forbidden);
}
```

## 总结

- **权限完全一致**: 桌面端与Web端权限控制100%相同
- **UI可以不同**: 精简 UI ≠ 功能阉割
- **后端是真相**: 所有权限检查在后端,前端仅控制显示
- **角色是核心**: 功能由用户角色决定,不由端类型限制


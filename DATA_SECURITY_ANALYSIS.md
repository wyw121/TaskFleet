# TaskFleet 数据权限问题分析与解决方案

## 🔴 严重问题:数据权限隔离缺失

### 当前问题描述

**现象**:

- `admin` 登录看到 6 个用户
- `company_admin_1` 登录也看到 6 个用户
- **所有用户都能看到所有数据,没有数据隔离!**

### 问题根源分析

#### 1. 数据库设计问题

**当前用户表结构** (`database.rs` line 20-45):

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE,
    hashed_password TEXT NOT NULL,
    role TEXT NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    full_name TEXT,
    parent_id INTEGER,        -- ❌ 存在但未使用
    company TEXT,             -- ❌ 存在但未使用
    max_employees INTEGER,    -- ❌ 存在但未使用
    current_employees INTEGER, -- ❌ 存在但未使用
    ...
)
```

**问题**:

- ❌ `parent_id` 字段存在但从未赋值(都是 NULL)
- ❌ `company` 字段存在但从未赋值(都是 NULL)
- ❌ 缺少 `company_id` 外键关联公司表
- ❌ 没有公司(Company)表来组织用户

#### 2. 角色设计问题

**当前角色定义** (`models.rs` line 50-56):

```rust
pub enum UserRole {
    ProjectManager,  // 项目管理员
    Employee,        // 普通员工
}
```

**实际数据库中的角色** (`database.rs` line 27):

```sql
role TEXT NOT NULL CHECK (role IN ('system_admin', 'user_admin', 'employee'))
```

**矛盾**:

- ❌ 代码中只定义了 `ProjectManager` 和 `Employee`
- ❌ 数据库中有 `system_admin`, `user_admin`, `employee`
- ❌ 角色映射混乱:`system_admin` 和 `user_admin` 都映射到 `ProjectManager`

**当前角色映射** (`models.rs` line 102-108):

```rust
pub fn from_str(s: &str) -> Option<Self> {
    match s {
        "project_manager" => Some(UserRole::ProjectManager),
        "system_admin" => Some(UserRole::ProjectManager),  // ❌ 都映射到同一个
        "user_admin" => Some(UserRole::ProjectManager),    // ❌ 都映射到同一个
        "employee" => Some(UserRole::Employee),
        _ => None,
    }
}
```

#### 3. 权限控制问题

**用户列表查询** (`services/user.rs` line 23-35):

```rust
pub async fn list_users(
    &self,
    current_user: &UserInfo,
) -> Result<Vec<UserInfo>> {
    // 只有项目管理员可以查看所有用户列表
    if current_user.role != UserRole::ProjectManager {
        return Err(anyhow!("权限不足"));
    }

    // ❌ 获取所有用户 - 没有按公司过滤!
    let users = self.user_repository.list_all().await?;
    Ok(users.into_iter().map(|user| user.into()).collect())
}
```

**问题**:

- ❌ 只检查角色,不检查公司归属
- ❌ `list_all()` 返回所有用户,不过滤公司
- ❌ `system_admin` 和 `company_admin_1` 看到相同数据

#### 4. 当前数据库实际内容

**用户数据** (推断):

```
ID | username        | role         | parent_id | company | company_id
---|-----------------|--------------|-----------|---------|------------
1  | admin           | system_admin | NULL      | NULL    | NULL
2  | company_admin_1 | user_admin   | NULL      | NULL    | NULL
3  | company_admin_2 | user_admin   | NULL      | NULL    | NULL
4  | employee_1      | employee     | NULL      | NULL    | NULL
5  | employee_2      | employee     | NULL      | NULL    | NULL
6  | employee_3      | employee     | NULL      | NULL    | NULL
```

**没有公司归属关系!所有用户都是"孤儿"数据!**

---

## 🎯 正确的数据权限设计

### 设计原则

#### 1. 三级权限模型

```
系统管理员 (SystemAdmin)
    └─ 可以查看/管理所有公司和用户
    └─ 创建公司管理员账号
    └─ 全局统计和监控

公司管理员 (CompanyAdmin)
    └─ 只能查看/管理自己公司的用户
    └─ 创建本公司员工账号
    └─ 本公司统计数据

普通员工 (Employee)
    └─ 只能查看/更新自己的信息
    └─ 只能查看分配给自己的任务
    └─ 提交工作记录
```

#### 2. 数据隔离层级

```
Level 1: 全局数据 (SystemAdmin 可见)
    ├─ Company A
    │   ├─ CompanyAdmin A (只能看 Company A 数据)
    │   ├─ Employee A1
    │   └─ Employee A2
    └─ Company B
        ├─ CompanyAdmin B (只能看 Company B 数据)
        ├─ Employee B1
        └─ Employee B2
```

---

## 🛠️ 修复方案

### 方案1: 添加公司表和关联 (推荐)

#### Step 1: 创建公司表

```sql
CREATE TABLE companies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    contact_email TEXT,
    contact_phone TEXT,
    max_employees INTEGER DEFAULT 10,
    is_active BOOLEAN DEFAULT TRUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

#### Step 2: 修改用户表

```sql
-- 添加 company_id 外键
ALTER TABLE users ADD COLUMN company_id INTEGER;
ALTER TABLE users ADD CONSTRAINT fk_company
    FOREIGN KEY (company_id) REFERENCES companies(id);

-- 创建索引
CREATE INDEX idx_users_company_id ON users(company_id);
```

#### Step 3: 插入测试公司数据

```sql
-- 创建两家公司
INSERT INTO companies (name, contact_email, max_employees) VALUES
    ('测试公司A', 'companyA@example.com', 20),
    ('测试公司B', 'companyB@example.com', 15);

-- 关联用户到公司
UPDATE users SET company_id = 1 WHERE username IN ('company_admin_1', 'employee_1', 'employee_2');
UPDATE users SET company_id = 2 WHERE username IN ('company_admin_2', 'employee_3');
-- admin 作为系统管理员,company_id 保持 NULL
```

#### Step 4: 修正角色枚举

```rust
// models.rs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    SystemAdmin,    // 系统管理员 - 可以查看所有数据
    CompanyAdmin,   // 公司管理员 - 只能查看本公司数据
    Employee,       // 普通员工 - 只能查看自己的数据
}

impl UserRole {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "system_admin" => Some(UserRole::SystemAdmin),
            "user_admin" => Some(UserRole::CompanyAdmin),
            "company_admin" => Some(UserRole::CompanyAdmin),
            "employee" => Some(UserRole::Employee),
            _ => None,
        }
    }
}
```

#### Step 5: 修改权限检查逻辑

```rust
// services/user.rs
pub async fn list_users(
    &self,
    current_user: &UserInfo,
) -> Result<Vec<UserInfo>> {
    match current_user.role {
        // 系统管理员可以查看所有用户
        UserRole::SystemAdmin => {
            let users = self.user_repository.list_all().await?;
            Ok(users.into_iter().map(|user| user.into()).collect())
        }

        // 公司管理员只能查看本公司用户
        UserRole::CompanyAdmin => {
            let company_id = current_user.company_id
                .ok_or_else(|| anyhow!("公司管理员必须关联公司"))?;

            let users = self.user_repository
                .list_by_company_id(company_id)
                .await?;
            Ok(users.into_iter().map(|user| user.into()).collect())
        }

        // 普通员工不能查看用户列表
        UserRole::Employee => {
            Err(anyhow!("权限不足：员工无法查看用户列表"))
        }
    }
}
```

#### Step 6: 添加 Repository 方法

```rust
// repositories/user_repository.rs
impl UserRepository {
    // 按公司ID查询用户
    pub async fn list_by_company_id(&self, company_id: i64) -> Result<Vec<User>> {
        let users = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE company_id = ? ORDER BY created_at DESC"
        )
        .bind(company_id)
        .fetch_all(&self.database.pool)
        .await?;

        Ok(users)
    }
}
```

---

### 方案2: 使用 parent_id 层级关系 (临时方案)

如果不想创建公司表,可以利用现有的 `parent_id`:

```sql
-- company_admin_1 作为公司A的根管理员
UPDATE users SET parent_id = NULL WHERE username = 'company_admin_1';

-- employee_1, employee_2 归属于 company_admin_1
UPDATE users SET parent_id = (SELECT id FROM users WHERE username = 'company_admin_1')
WHERE username IN ('employee_1', 'employee_2');

-- company_admin_2 作为公司B的根管理员
UPDATE users SET parent_id = NULL WHERE username = 'company_admin_2';

-- employee_3 归属于 company_admin_2
UPDATE users SET parent_id = (SELECT id FROM users WHERE username = 'company_admin_2')
WHERE username = 'employee_3';
```

**权限查询**:

```rust
pub async fn list_users(&self, current_user: &UserInfo) -> Result<Vec<UserInfo>> {
    match current_user.role {
        UserRole::SystemAdmin => {
            // 系统管理员看所有人
            self.user_repository.list_all().await
        }
        UserRole::CompanyAdmin => {
            // 公司管理员看自己和下属
            self.user_repository.list_by_parent_id(current_user.id).await
        }
        UserRole::Employee => {
            // 员工只能看自己
            Err(anyhow!("权限不足"))
        }
    }
}
```

---

## 📊 推荐实施步骤

### 第一阶段:紧急修复 (立即执行)

1. **修正角色枚举** ✅

   - 添加 `SystemAdmin`, `CompanyAdmin`, `Employee`
   - 修复角色映射逻辑
2. **利用 parent_id 实现临时隔离** ✅

   - 设置用户的 parent_id 关系
   - 修改查询逻辑按 parent_id 过滤
3. **添加权限检查** ✅

   - 在所有 Service 层方法中检查权限
   - 区分 SystemAdmin 和 CompanyAdmin

### 第二阶段:完整方案 (1-2天内)

1. **创建公司表** ✅

   - 设计 Company 模型
   - 创建数据库迁移
2. **添加 company_id 外键** ✅

   - 修改用户表结构
   - 迁移现有数据
3. **完善权限控制** ✅

   - 所有查询都按 company_id 过滤
   - 任务、项目也需要关联 company_id

### 第三阶段:完善功能 (后续)

1. **前端权限控制** ✅

   - 根据角色显示/隐藏功能
   - SystemAdmin 专属页面
2. **审计日志** ✅

   - 记录跨公司数据访问
   - 异常权限操作告警

---

## 🔍 当前系统安全风险评估

### 严重风险 🔴

1. **数据泄露风险**

   - 公司A的管理员可以看到公司B的员工信息
   - 员工可以看到其他公司的任务数据
2. **越权操作风险**

   - 公司管理员可以修改其他公司的用户
   - 可以删除不属于自己公司的员工
3. **合规风险**

   - 违反数据隐私保护原则
   - 不符合多租户系统设计规范

### 中等风险 🟡

1. **角色混淆**

   - SystemAdmin 和 CompanyAdmin 权限相同
   - 无法区分全局管理员和租户管理员
2. **数据完整性**

   - parent_id 和 company 字段未使用
   - 孤儿数据(没有公司归属)

---

## 💡 建议

### 立即行动

1. **暂停生产部署** ⚠️

   - 当前版本有严重安全问题
   - 不建议在生产环境使用
2. **实施方案2 (临时)** 🔧

   - 使用 parent_id 快速实现隔离
   - 1-2小时可完成
3. **规划方案1 (正式)** 📋

   - 创建公司表
   - 完整的多租户架构
   - 1-2天完成

### 长期规划

1. **引入 Row Level Security (RLS)**

   - 数据库层面的权限控制
   - 防止 SQL 注入绕过权限
2. **实施 RBAC (基于角色的访问控制)**

   - 细粒度的权限配置
   - 支持自定义角色
3. **添加数据脱敏**

   - 敏感字段加密
   - 查询时自动脱敏

---

## 📝 总结

### 问题本质

**当前系统是"单租户"设计,但被当作"多租户"使用!**

- ❌ 没有公司(Company)概念
- ❌ 所有用户共享同一个数据池
- ❌ 权限检查只看角色,不看归属

### 核心修复

**必须引入"租户隔离"机制:**

1. **数据层**: 添加 company_id 字段
2. **逻辑层**: 所有查询都过滤 company_id
3. **权限层**: SystemAdmin 全局,CompanyAdmin 本公司

### 预期效果修复后

```
admin 登录:
  └─ 看到所有公司的所有用户 (6个)

company_admin_1 登录:
  └─ 只看到公司A的用户 (3个: 自己 + employee_1 + employee_2)

company_admin_2 登录:
  └─ 只看到公司B的用户 (2个: 自己 + employee_3)

employee_1 登录:
  └─ 只能看到自己 (1个)
```

---

**需要我立即实施修复方案吗?** 建议先执行"紧急修复",再规划完整方案。

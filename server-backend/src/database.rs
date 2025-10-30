use anyhow::Result;
use sqlx::{Row, SqlitePool};

#[derive(Debug, Clone)]
pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> Result<()> {
        tracing::info!("🔄 开始数据库迁移");

        // 创建用户表
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                email TEXT UNIQUE,
                hashed_password TEXT NOT NULL,
                role TEXT NOT NULL CHECK (role IN ('platform_admin', 'project_manager', 'task_executor')),
                is_active BOOLEAN DEFAULT TRUE,
                is_verified BOOLEAN DEFAULT FALSE,
                parent_id INTEGER,
                full_name TEXT,
                phone TEXT,
                company TEXT,
                max_employees INTEGER DEFAULT 10,
                current_employees INTEGER DEFAULT 0,
                balance REAL DEFAULT 1000.0,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                last_login DATETIME,
                company_id INTEGER
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // 删除旧的工作记录表（如果存在）并重新创建
        sqlx::query("DROP TABLE IF EXISTS work_records")
            .execute(&self.pool)
            .await?;

        // 创建工作记录表（匹配WorkRecord模型）
        sqlx::query(
            r#"
            CREATE TABLE work_records (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                device_id TEXT NOT NULL,
                platform TEXT NOT NULL,
                action_type TEXT NOT NULL,
                target_count INTEGER NOT NULL DEFAULT 0,
                completed_count INTEGER NOT NULL DEFAULT 0,
                status TEXT NOT NULL DEFAULT 'pending',
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (user_id) REFERENCES users (id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // 创建设备表
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS devices (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                device_name TEXT NOT NULL,
                device_type TEXT NOT NULL,
                adb_id TEXT,
                status TEXT NOT NULL DEFAULT 'offline',
                last_seen DATETIME,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (user_id) REFERENCES users (id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // 删除旧的计费记录表（如果存在）并重新创建
        sqlx::query("DROP TABLE IF EXISTS billing_records")
            .execute(&self.pool)
            .await?;

        // 创建计费记录表
        sqlx::query(
            r#"
            CREATE TABLE billing_records (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                amount REAL NOT NULL,
                billing_type TEXT NOT NULL,
                description TEXT,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (user_id) REFERENCES users (id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // 删除旧的价格规则表（如果存在）并重新创建
        sqlx::query("DROP TABLE IF EXISTS pricing_rules")
            .execute(&self.pool)
            .await?;

        // 创建价格规则表
        sqlx::query(
            r#"
            CREATE TABLE pricing_rules (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                rule_name TEXT NOT NULL,
                billing_type TEXT NOT NULL,
                unit_price REAL NOT NULL,
                is_active BOOLEAN DEFAULT TRUE,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // 创建公司表
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS companies (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT UNIQUE NOT NULL,
                code TEXT UNIQUE NOT NULL,
                description TEXT,
                contact_email TEXT,
                contact_phone TEXT,
                max_employees INTEGER DEFAULT 10,
                is_active BOOLEAN DEFAULT TRUE,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // 创建公司收费计划表
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS company_pricing_plans (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                company_name TEXT NOT NULL UNIQUE,
                plan_name TEXT NOT NULL,
                employee_monthly_fee REAL NOT NULL DEFAULT 50.0,
                is_active BOOLEAN DEFAULT TRUE,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // 创建公司操作收费规则表
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS company_operation_pricing (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                company_name TEXT NOT NULL,
                platform TEXT NOT NULL,
                operation_type TEXT NOT NULL,
                unit_price REAL NOT NULL,
                is_active BOOLEAN DEFAULT TRUE,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(company_name, platform, operation_type)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // 创建系统配置表
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS system_settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                description TEXT,
                updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // 插入默认系统管理员(如果不存在)
        let admin_exists =
            sqlx::query("SELECT COUNT(*) as count FROM users WHERE role = 'platform_admin'")
                .fetch_one(&self.pool)
                .await?
                .get::<i64, _>("count")
                > 0;

        if !admin_exists {
            let password_hash = bcrypt::hash("admin123", 12)?;

            sqlx::query(
                r#"
                INSERT INTO users (username, email, hashed_password, role, is_active)
                VALUES (?, ?, ?, ?, ?)
                "#,
            )
            .bind("admin")
            .bind("admin@flowfarm.com")
            .bind(&password_hash)
            .bind("platform_admin")
            .bind(true)
            .execute(&self.pool)
            .await?;

            tracing::info!("✅ 默认管理员账户已创建 - 用户名: admin, 密码: admin123");
        }

        // 创建测试用户（仅在开发环境）
        self.create_test_users().await?;

        // 创建测试公司数据
        self.create_test_companies().await?;

        // 创建测试价格规则
        self.create_test_pricing_rules().await?;

        // 创建测试公司收费计划
        self.create_test_company_pricing().await?;

        // 创建测试项目和任务数据
        self.create_test_projects_and_tasks().await?;

        tracing::info!("✅ 数据库迁移完成");
        Ok(())
    }

    async fn create_test_pricing_rules(&self) -> Result<()> {
        tracing::info!("🔄 创建测试价格规则数据");

        // 检查是否已存在价格规则
        let rules_count = sqlx::query("SELECT COUNT(*) as count FROM pricing_rules")
            .fetch_one(&self.pool)
            .await?
            .get::<i64, _>("count");

        if rules_count == 0 {
            // 创建默认价格规则
            let rules = vec![
                ("抖音关注", "douyin_follow", 0.05),
                ("抖音点赞", "douyin_like", 0.02),
                ("小红书关注", "xiaohongshu_follow", 0.08),
                ("小红书点赞", "xiaohongshu_like", 0.03),
                ("小红书收藏", "xiaohongshu_favorite", 0.04),
            ];

            for (rule_name, billing_type, unit_price) in rules {
                sqlx::query(
                    r#"
                    INSERT INTO pricing_rules (rule_name, billing_type, unit_price, is_active)
                    VALUES (?, ?, ?, ?)
                    "#,
                )
                .bind(rule_name)
                .bind(billing_type)
                .bind(unit_price)
                .bind(true)
                .execute(&self.pool)
                .await?;
            }

            tracing::info!("✅ 测试价格规则创建完成");
        } else {
            tracing::info!("ℹ️  价格规则已存在，跳过创建");
        }

        Ok(())
    }

    async fn create_test_users(&self) -> Result<()> {
        tracing::info!("🔄 创建测试用户数据");

        let password_hash = bcrypt::hash("admin123", 12)?;

        // 检查是否已存在company_admin_1用户
        let company_admin_exists =
            sqlx::query("SELECT COUNT(*) as count FROM users WHERE username = 'company_admin_1'")
                .fetch_one(&self.pool)
                .await?
                .get::<i64, _>("count")
                > 0;

        if !company_admin_exists {
            // 创建公司管理员1
            sqlx::query(
                r#"
                INSERT INTO users (username, email, hashed_password, role, company, is_active, max_employees)
                VALUES (?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind("company_admin_1")
            .bind("company_admin_1@example.com")
            .bind(&password_hash)
            .bind("project_manager")
            .bind("company_001")
            .bind(true)
            .bind(50)
            .execute(&self.pool)
            .await?;

            // 创建公司管理员2
            sqlx::query(
                r#"
                INSERT INTO users (username, email, hashed_password, role, company, is_active, max_employees)
                VALUES (?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind("company_admin_2")
            .bind("company_admin_2@example.com")
            .bind(&password_hash)
            .bind("project_manager")
            .bind("company_002")
            .bind(true)
            .bind(30)
            .execute(&self.pool)
            .await?;

            // 创建测试员工
            let employees = vec![
                ("employee_1", "employee_1@company_001.com", "company_001"),
                ("employee_2", "employee_2@company_001.com", "company_001"),
                ("employee_3", "employee_3@company_002.com", "company_002"),
            ];

            for (username, email, company) in employees {
                sqlx::query(
                    r#"
                    INSERT INTO users (username, email, hashed_password, role, company, is_active)
                    VALUES (?, ?, ?, ?, ?, ?)
                    "#,
                )
                .bind(username)
                .bind(email)
                .bind(&password_hash)
                .bind("task_executor")
                .bind(company)
                .bind(true)
                .execute(&self.pool)
                .await?;
            }

            tracing::info!("✅ 测试用户创建完成");
            tracing::info!("   - company_admin_1 (密码: admin123)");
            tracing::info!("   - company_admin_2 (密码: admin123)");
            tracing::info!("   - employee_1, employee_2, employee_3 (密码: admin123)");
        } else {
            tracing::info!("ℹ️  测试用户已存在，跳过创建");
        }

        Ok(())
    }

    async fn create_test_companies(&self) -> Result<()> {
        tracing::info!("🔄 创建测试公司数据");

        // 检查是否已存在公司数据
        let companies_count = sqlx::query("SELECT COUNT(*) as count FROM companies")
            .fetch_one(&self.pool)
            .await?
            .get::<i64, _>("count");

        if companies_count == 0 {
            // 创建默认测试公司
            let companies = vec![
                ("测试公司A", "company_001", "这是测试公司A", "companyA@example.com", "13800000001", 20),
                ("测试公司B", "company_002", "这是测试公司B", "companyB@example.com", "13800000002", 15),
            ];

            for (name, code, description, email, phone, max_employees) in companies {
                sqlx::query(
                    r#"
                    INSERT INTO companies (name, code, description, contact_email, contact_phone, max_employees, is_active)
                    VALUES (?, ?, ?, ?, ?, ?, ?)
                    "#,
                )
                .bind(name)
                .bind(code)
                .bind(description)
                .bind(email)
                .bind(phone)
                .bind(max_employees)
                .bind(true)
                .execute(&self.pool)
                .await?;
            }

            tracing::info!("✅ 测试公司数据创建完成");
        } else {
            tracing::info!("ℹ️  公司数据已存在，跳过创建");
        }

        Ok(())
    }

    async fn create_test_company_pricing(&self) -> Result<()> {
        tracing::info!("🔄 创建测试公司收费计划");

        // 检查是否已存在公司收费计划
        let plans_count = sqlx::query("SELECT COUNT(*) as count FROM company_pricing_plans")
            .fetch_one(&self.pool)
            .await?
            .get::<i64, _>("count");

        if plans_count == 0 {
            // 创建默认公司收费计划
            let plans = vec![
                ("company_001", "标准计划", 50.0),
                ("company_002", "高级计划", 80.0),
            ];

            for (company_name, plan_name, monthly_fee) in plans {
                sqlx::query(
                    r#"
                    INSERT INTO company_pricing_plans (company_name, plan_name, employee_monthly_fee, is_active)
                    VALUES (?, ?, ?, ?)
                    "#,
                )
                .bind(company_name)
                .bind(plan_name)
                .bind(monthly_fee)
                .bind(true)
                .execute(&self.pool)
                .await?;
            }

            // 创建默认公司操作收费规则
            let operation_pricing = vec![
                // company_001 的收费标准
                ("company_001", "xiaohongshu", "follow", 0.08),
                ("company_001", "xiaohongshu", "like", 0.03),
                ("company_001", "xiaohongshu", "favorite", 0.04),
                ("company_001", "douyin", "follow", 0.05),
                ("company_001", "douyin", "like", 0.02),
                // company_002 的收费标准 (高级计划，价格更低)
                ("company_002", "xiaohongshu", "follow", 0.06),
                ("company_002", "xiaohongshu", "like", 0.02),
                ("company_002", "xiaohongshu", "favorite", 0.03),
                ("company_002", "douyin", "follow", 0.04),
                ("company_002", "douyin", "like", 0.015),
            ];

            for (company_name, platform, operation_type, unit_price) in operation_pricing {
                sqlx::query(
                    r#"
                    INSERT INTO company_operation_pricing (company_name, platform, operation_type, unit_price, is_active)
                    VALUES (?, ?, ?, ?, ?)
                    "#,
                )
                .bind(company_name)
                .bind(platform)
                .bind(operation_type)
                .bind(unit_price)
                .bind(true)
                .execute(&self.pool)
                .await?;
            }

            tracing::info!("✅ 测试公司收费计划创建完成");
        } else {
            tracing::info!("ℹ️  公司收费计划已存在，跳过创建");
        }

        Ok(())
    }

    async fn create_test_projects_and_tasks(&self) -> Result<()> {
        tracing::info!("🔄 创建测试项目和任务数据");

        // 检查是否已存在项目数据
        let projects_count = sqlx::query("SELECT COUNT(*) as count FROM projects")
            .fetch_one(&self.pool)
            .await?
            .get::<i64, _>("count");

        if projects_count == 0 {
            // 获取用户ID
            let admin_id = sqlx::query("SELECT id FROM users WHERE username = 'admin'")
                .fetch_one(&self.pool)
                .await?
                .get::<i64, _>("id");

            let company_admin_1_id = sqlx::query("SELECT id FROM users WHERE username = 'company_admin_1'")
                .fetch_one(&self.pool)
                .await?
                .get::<i64, _>("id");

            let employee_1_id = sqlx::query("SELECT id FROM users WHERE username = 'employee_1'")
                .fetch_one(&self.pool)
                .await?
                .get::<i64, _>("id");

            let employee_2_id = sqlx::query("SELECT id FROM users WHERE username = 'employee_2'")
                .fetch_one(&self.pool)
                .await?
                .get::<i64, _>("id");

            // 创建项目（使用 UUID 作为 ID）
            use uuid::Uuid;
            
            let projects = vec![
                (
                    Uuid::new_v4().to_string(),
                    "TaskFleet 系统开发",
                    "开发 TaskFleet 任务管理系统的核心功能模块",
                    admin_id.to_string(),
                    "in_progress",
                    "2025-10-01",
                    "2025-12-31",
                ),
                (
                    Uuid::new_v4().to_string(),
                    "电商平台推广项目",
                    "为客户的电商平台进行多渠道社交媒体推广",
                    company_admin_1_id.to_string(),
                    "in_progress",
                    "2025-10-15",
                    "2025-11-30",
                ),
                (
                    Uuid::new_v4().to_string(),
                    "品牌营销活动",
                    "策划并执行品牌在小红书和抖音的营销活动",
                    company_admin_1_id.to_string(),
                    "planning",
                    "2025-11-01",
                    "2025-12-15",
                ),
            ];

            let mut project_ids = Vec::new();
            for (id, name, description, owner_id, status, start_date, end_date) in projects {
                sqlx::query(
                    r#"
                    INSERT INTO projects (id, name, description, owner_id, status, start_date, end_date, created_at, updated_at)
                    VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))
                    "#,
                )
                .bind(&id)
                .bind(name)
                .bind(description)
                .bind(&owner_id)
                .bind(status)
                .bind(start_date)
                .bind(end_date)
                .execute(&self.pool)
                .await?;
                
                project_ids.push((id, owner_id));
            }

            tracing::info!("✅ 测试项目创建完成，共 {} 个项目", project_ids.len());

            // 创建任务
            let tasks = vec![
                // 项目1 (TaskFleet系统开发) - admin创建的任务
                (
                    Uuid::new_v4().to_string(),
                    "设计数据库架构",
                    "设计用户、项目、任务、工作日志等核心表结构",
                    "completed",
                    "high",
                    project_ids[0].0.clone(),
                    Some(admin_id.to_string()),
                    admin_id.to_string(),
                    Some("2025-10-05"),
                    Some(16.0),
                    Some(15.5),
                    Some("2025-10-05 18:00:00"),
                ),
                (
                    Uuid::new_v4().to_string(),
                    "实现用户认证系统",
                    "开发JWT认证、角色权限控制等功能",
                    "completed",
                    "high",
                    project_ids[0].0.clone(),
                    Some(admin_id.to_string()),
                    admin_id.to_string(),
                    Some("2025-10-10"),
                    Some(24.0),
                    Some(26.0),
                    Some("2025-10-11 20:00:00"),
                ),
                (
                    Uuid::new_v4().to_string(),
                    "开发前端界面",
                    "使用 React + TypeScript 开发前端管理界面",
                    "in_progress",
                    "high",
                    project_ids[0].0.clone(),
                    Some(admin_id.to_string()),
                    admin_id.to_string(),
                    Some("2025-10-25"),
                    Some(40.0),
                    Some(18.0),
                    None,
                ),
                
                // 项目2 (电商平台推广) - company_admin_1创建并分配给员工
                (
                    Uuid::new_v4().to_string(),
                    "小红书账号粉丝增长",
                    "通过互动和内容推广，目标增长5000粉丝",
                    "in_progress",
                    "high",
                    project_ids[1].0.clone(),
                    Some(employee_1_id.to_string()),
                    company_admin_1_id.to_string(),
                    Some("2025-10-20"),
                    Some(30.0),
                    Some(12.0),
                    None,
                ),
                (
                    Uuid::new_v4().to_string(),
                    "抖音直播间引流",
                    "为电商直播间引流，目标1000人次在线观看",
                    "pending",
                    "medium",
                    project_ids[1].0.clone(),
                    Some(employee_2_id.to_string()),
                    company_admin_1_id.to_string(),
                    Some("2025-10-22"),
                    Some(20.0),
                    None,
                    None,
                ),
                (
                    Uuid::new_v4().to_string(),
                    "产品笔记创作",
                    "撰写并发布10篇高质量产品测评笔记",
                    "in_progress",
                    "medium",
                    project_ids[1].0.clone(),
                    Some(employee_1_id.to_string()),
                    company_admin_1_id.to_string(),
                    Some("2025-10-25"),
                    Some(15.0),
                    Some(6.0),
                    None,
                ),
                
                // 项目3 (品牌营销活动) - company_admin_1创建的计划中任务
                (
                    Uuid::new_v4().to_string(),
                    "市场调研分析",
                    "分析目标用户群体和竞品策略",
                    "pending",
                    "high",
                    project_ids[2].0.clone(),
                    Some(employee_1_id.to_string()),
                    company_admin_1_id.to_string(),
                    Some("2025-11-05"),
                    Some(16.0),
                    None,
                    None,
                ),
                (
                    Uuid::new_v4().to_string(),
                    "内容创意策划",
                    "策划30天的内容发布计划和创意方案",
                    "pending",
                    "medium",
                    project_ids[2].0.clone(),
                    Some(company_admin_1_id.to_string()),
                    company_admin_1_id.to_string(),
                    Some("2025-11-08"),
                    Some(24.0),
                    None,
                    None,
                ),
                
                // employee_1 自己创建的任务（不关联项目）
                (
                    Uuid::new_v4().to_string(),
                    "学习新的推广技巧",
                    "观看并学习最新的社交媒体营销课程",
                    "in_progress",
                    "low",
                    "".to_string(), // 无项目关联
                    Some(employee_1_id.to_string()),
                    employee_1_id.to_string(),
                    Some("2025-10-30"),
                    Some(8.0),
                    Some(3.0),
                    None,
                ),
                (
                    Uuid::new_v4().to_string(),
                    "整理工作报告",
                    "整理本周的工作成果和数据报告",
                    "pending",
                    "low",
                    "".to_string(),
                    Some(employee_1_id.to_string()),
                    employee_1_id.to_string(),
                    Some("2025-10-31"),
                    Some(4.0),
                    None,
                    None,
                ),
            ];

            let mut task_ids = Vec::new();
            for (id, title, description, status, priority, project_id, assigned_to, created_by, due_date, estimated_hours, actual_hours, completed_at) in tasks {
                let project_id_value = if project_id.is_empty() { None } else { Some(project_id) };
                
                sqlx::query(
                    r#"
                    INSERT INTO tasks (id, title, description, status, priority, project_id, assigned_to, created_by, due_date, estimated_hours, actual_hours, completed_at, created_at, updated_at)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))
                    "#,
                )
                .bind(&id)
                .bind(title)
                .bind(description)
                .bind(status)
                .bind(priority)
                .bind(project_id_value)
                .bind(assigned_to)
                .bind(created_by)
                .bind(due_date)
                .bind(estimated_hours)
                .bind(actual_hours)
                .bind(completed_at)
                .execute(&self.pool)
                .await?;
                
                task_ids.push(id);
            }

            tracing::info!("✅ 测试任务创建完成，共 {} 个任务", task_ids.len());

            // 创建工作日志
            let work_logs = vec![
                // admin 的工作日志
                (
                    Uuid::new_v4().to_string(),
                    task_ids[0].clone(), // 设计数据库架构
                    admin_id.to_string(),
                    "完成了用户表和权限表的设计",
                    8.0,
                    "2025-10-04",
                ),
                (
                    Uuid::new_v4().to_string(),
                    task_ids[0].clone(),
                    admin_id.to_string(),
                    "完成了项目和任务表的设计及关系定义",
                    7.5,
                    "2025-10-05",
                ),
                (
                    Uuid::new_v4().to_string(),
                    task_ids[1].clone(), // 用户认证系统
                    admin_id.to_string(),
                    "实现了JWT token生成和验证逻辑",
                    10.0,
                    "2025-10-09",
                ),
                (
                    Uuid::new_v4().to_string(),
                    task_ids[2].clone(), // 前端界面
                    admin_id.to_string(),
                    "搭建了React项目框架，配置了路由和状态管理",
                    9.0,
                    "2025-10-20",
                ),
                
                // employee_1 的工作日志
                (
                    Uuid::new_v4().to_string(),
                    task_ids[3].clone(), // 小红书粉丝增长
                    employee_1_id.to_string(),
                    "完成了200个账号的关注和互动，新增粉丝150人",
                    6.0,
                    "2025-10-18",
                ),
                (
                    Uuid::new_v4().to_string(),
                    task_ids[3].clone(),
                    employee_1_id.to_string(),
                    "发布了3篇互动内容，点赞收藏共计500次",
                    6.0,
                    "2025-10-19",
                ),
                (
                    Uuid::new_v4().to_string(),
                    task_ids[5].clone(), // 产品笔记创作
                    employee_1_id.to_string(),
                    "完成了2篇产品测评笔记的撰写和发布",
                    6.0,
                    "2025-10-23",
                ),
                (
                    Uuid::new_v4().to_string(),
                    task_ids[8].clone(), // 学习新技巧
                    employee_1_id.to_string(),
                    "学习了短视频创作技巧课程",
                    3.0,
                    "2025-10-29",
                ),
            ];

            let work_log_count = work_logs.len();
            for (id, task_id, user_id, description, hours, work_date) in work_logs {
                sqlx::query(
                    r#"
                    INSERT INTO work_logs (id, task_id, user_id, hours, notes, logged_at, created_at, updated_at)
                    VALUES (?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))
                    "#,
                )
                .bind(id)
                .bind(task_id)
                .bind(user_id)
                .bind(hours)
                .bind(description)
                .bind(work_date)
                .execute(&self.pool)
                .await?;
            }

            tracing::info!("✅ 测试工作日志创建完成，共 {} 条记录", work_log_count);
        } else {
            tracing::info!("ℹ️  测试项目数据已存在，跳过创建");
        }

        Ok(())
    }
}

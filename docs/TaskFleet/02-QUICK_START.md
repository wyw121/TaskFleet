# TaskFleet - 快速启动指南

**目的**: 帮助你在 30 分钟内启动 TaskFleet 开发环境并理解核心功能

**创建日期**: 2025年10月28日  
**更新日期**: 2025年10月28日

---

## 🎯 第一步: 获取项目代码 (5分钟)

### 1. 克隆项目

```bash
# 克隆 TaskFleet 项目
git clone https://github.com/wyw121/TaskFleet.git
cd TaskFleet
```

### 2. 了解项目结构

```
TaskFleet/
├── server-backend/         # Rust 后端服务 (Axum + SQLx)
├── server-frontend/        # React Web 前端 (TypeScript + Ant Design)
├── employee-client/        # Tauri 桌面客户端 (Rust + HTML/CSS/JS)
├── docs/                   # 项目文档
│   └── TaskFleet/         # 核心文档目录
└── README.md              # 项目说明
```

### 3. 环境要求检查

```bash
# 检查 Rust 环境
rustc --version  # 需要 1.70+
cargo --version

# 检查 Node.js 环境  
node --version   # 需要 18+
npm --version

# 检查 Tauri CLI
cargo install tauri-cli --version 2.0
cargo tauri --version
```

---

## 🔧 第二步: 重构后端 (10分钟)

---

## 🖥️ 第二步: 启动后端服务 (5分钟)

### 1. 准备数据库

```bash
# 确保 PostgreSQL 正在运行
# 创建数据库 (如果还没有)
createdb taskfleet

# 配置环境变量
cd server-backend
cp .env.example .env

# 编辑 .env 文件，配置数据库连接
# DATABASE_URL=postgresql://username:password@localhost/taskfleet
```

### 2. 运行数据库迁移

```bash
# 安装 sqlx-cli (如果还没有)
cargo install sqlx-cli

# 运行迁移
sqlx migrate run
```

### 3. 启动后端服务

```bash
# 开发模式启动
cargo run

# 或使用 VS Code 任务
# Ctrl+Shift+P -> 任务: 运行任务 -> "🚀 启动服务器后端"
```

后端服务将在 `http://localhost:8000` 启动。

---

## 🌐 第三步: 启动Web前端 (5分钟)

### 1. 安装依赖

```bash
cd ../server-frontend
npm install
```

### 2. 启动开发服务器

```bash
# 开发模式启动
npm run dev

# 或使用 VS Code 任务  
# Ctrl+Shift+P -> 任务: 运行任务 -> "🌐 启动服务器前端开发"
```

Web前端将在 `http://localhost:3000` 启动。

### 3. 验证Web端功能

1. 打开浏览器访问 `http://localhost:3000`
2. 使用测试账户登录 (如果有的话)
3. 查看项目管理界面
4. 测试任务分发功能

---

## 💻 第四步: 启动桌面客户端 (5分钟)

### 1. 检查Tauri环境

```bash
cd ../employee-client

# 检查Tauri CLI
cargo tauri --version

# 如果没有安装
cargo install tauri-cli --version 2.0
```

### 2. 启动开发模式

```bash
# 开发模式启动
cargo tauri dev

# 或使用 VS Code 任务
# Ctrl+Shift+P -> 任务: 运行任务 -> "💻 启动员工客户端开发"
```

桌面应用将自动启动新窗口。

### 3. 验证桌面端功能

1. 确认桌面应用正常启动
2. 测试员工登录功能
3. 查看任务列表界面
4. 测试任务状态更新

---

## 🧪 第五步: 验证系统功能 (10分钟)

### 1. 创建测试数据

**使用Web端 (项目经理视角)**:
1. 登录Web管理界面
2. 创建新项目 "测试项目"
3. 批量导入任务 (如果支持CSV)
4. 将任务分配给员工

### 2. 测试多端协同

**桌面端 (员工视角)**:
1. 登录员工客户端
2. 查看分配的任务列表
3. 更新任务状态 (开始/完成)
4. 验证Web端实时更新

### 3. 检查核心功能

- [ ] ✅ 用户认证和权限控制
- [ ] ✅ 项目创建和管理
- [ ] ✅ 任务分发和分配
- [ ] ✅ 实时状态同步
- [ ] ✅ 数据统计和报表

---

## 🚀 下一步: 开发和部署

### 开发建议

1. **代码检查**: 经常运行 `cargo check` 和 `npm run build`
2. **测试驱动**: 为核心功能编写测试用例
3. **文档更新**: 及时更新API文档和用户指南
4. **版本控制**: 使用语义化版本号管理

### 部署选项

- **Docker**: 使用 `docker-compose.yml` 一键部署
- **云服务**: 部署到 AWS/Azure/阿里云
- **本地服务器**: 编译为二进制文件直接运行

### 扩展方向

- **移动端**: React Native或Flutter版本
- **API扩展**: GraphQL支持
- **集成**: 与现有OA系统集成
- **插件系统**: 支持第三方扩展

---

## � 相关资源

- **[技术实现指南](./01-TECHNICAL_GUIDE.md)** - 详细技术方案
- **[项目概述](./00-PROJECT_OVERVIEW.md)** - 产品定位和愿景
- **[多平台策略](./03-MULTI_PLATFORM_STRATEGY.md)** - 跨平台开发策略
- **主项目README**: `../README.md`
- **API文档**: 启动后端后访问 `/docs` 端点

**需要帮助?** 
- 查看项目Issues: https://github.com/wyw121/TaskFleet/issues
- 阅读开发文档: `docs/` 目录
- 检查配置文件: `.env.example` 和 `tauri.conf.json`
        </div>
        
        <div id="tasks-page" class="page hidden">
            <h1>我的任务</h1>
            <div id="task-list"></div>
        </div>
    </div>
    
    <script src="app.js"></script>
</body>
</html>
```

---

## 📝 第五步: 更新文档

### 1. 更新 README.md

**README.md**:
```markdown
# TaskFleet - 任务执行专家

![Logo](docs/logo.png)

> 专注于任务分发、进度监控和数据统计的开源项目管理系统

## 特性

- 🚀 **智能任务分发** - 批量导入,自动均衡分配
- 📊 **实时进度监控** - 一目了然的执行状态
- 📈 **深度数据统计** - 员工效率分析,趋势预测
- 💻 **多端协同** - Web管理端 + 桌面员工端
- ⚡ **高性能** - Rust后端,极速响应
- 🎯 **简单易用** - 扁平化权限,开箱即用

## 快速开始

### 后端

```bash
cd backend
cargo run
```

### 前端

```bash
cd frontend
npm install
npm run dev
```

### 桌面客户端

```bash
cd desktop-client
cargo tauri dev
```

## 技术栈

- **后端**: Rust + Axum + PostgreSQL
- **前端**: React + TypeScript + Ant Design
- **桌面**: Tauri + Rust

## 文档

- [项目概述](docs/TaskFleet/00-PROJECT_OVERVIEW.md)
- [技术指南](docs/TaskFleet/01-TECHNICAL_GUIDE.md)
- [API 文档](docs/TaskFleet/API.md)

## 贡献

欢迎贡献!请查看 [CONTRIBUTING.md](CONTRIBUTING.md)

## License

MIT License
```

### 2. 创建 .gitignore

**.gitignore**:
```gitignore
# Rust
target/
Cargo.lock

# Node
node_modules/
dist/
.env.local

# IDE
.vscode/
.idea/

# OS
.DS_Store
Thumbs.db

# Database
*.db
*.sqlite

# Logs
logs/
*.log
```

---

## ✅ 第六步: 提交到 GitHub

### 1. 初始提交

```bash
cd ..  # 回到项目根目录

# 添加所有文件
git add .

# 初始提交
git commit -m "feat: initial commit - TaskFleet v0.1.0

- TaskFleet 任务执行专家系统
- 简化权限架构 (项目经理-员工)  
- 专注任务分发和监控管理
- 多端协同 (Web+桌面) 架构"

# 推送到 GitHub
git branch -M main
git push -u origin main
```

### 2. 创建分支保护

在 GitHub 仓库设置中:
- 启用分支保护
- 要求 PR review
- 启用 CI/CD

---

## 🎯 第七步: 验证和测试

### 1. 启动后端

```bash
cd backend

# 设置环境变量
cp .env.example .env
# 编辑 .env 文件

# 运行迁移
sqlx database create
sqlx migrate run

# 启动服务
cargo run
```

### 2. 启动前端

```bash
cd frontend

# 安装依赖
npm install

# 启动开发服务器
npm run dev
```

### 3. 测试功能

- [ ] 用户注册和登录
- [ ] 创建项目
- [ ] 添加任务
- [ ] 分配任务
- [ ] 更新状态

---

## 📋 检查清单

在完成以上步骤后,确认:

### 代码清理
- [ ] 确认了TaskFleet核心功能完整
- [ ] 确认了用户权限系统简洁
- [ ] 确认了三端架构清晰
- [ ] 确认了数据库模型合理

### 功能验证
- [ ] 后端 API 正常启动
- [ ] 前端页面正常显示
- [ ] 用户认证正常工作
- [ ] 数据库迁移成功

### 文档完善
- [ ] README.md 更新
- [ ] 技术文档创建
- [ ] API 文档编写
- [ ] 贡献指南添加

### Git 管理
- [ ] .gitignore 配置正确
- [ ] 代码已提交到 GitHub
- [ ] 分支保护已设置

---

## 🚀 下一步计划

### 本周 (Week 1)
- [ ] 完成项目管理 API
- [ ] 完成任务管理 API
- [ ] 创建基础 Web 页面

### 下周 (Week 2)
- [ ] 实现任务分发功能
- [ ] 实现进度监控
- [ ] 开发桌面客户端

### 第三周 (Week 3)
- [ ] 添加统计功能
- [ ] 集成 WebSocket 实时推送
- [ ] UI/UX 优化

### 第四周 (Week 4)
- [ ] 完整测试
- [ ] 文档完善
- [ ] 准备 MVP 发布

---

## 💡 重要提示

### 保持专注
- ✅ 只做任务执行相关功能
- ❌ 不要添加即时通讯
- ❌ 不要添加文档协作
- ❌ 不要添加复杂权限

### 快速迭代
- 每周发布一个小版本
- 及时收集用户反馈
- 持续优化核心功能

### 社区建设
- 在开发过程中记录博客
- 在社交媒体分享进度
- 积极回应 GitHub Issues

---

## 📞 需要帮助?

如果在创建过程中遇到问题:

1. 查看详细文档: `docs/TaskFleet/`
2. 查看原项目代码作为参考
3. 搜索相关技术问题
4. 在 GitHub 创建 Issue

---

**祝你顺利创建 TaskFleet! 🚀**

**记住**: 简洁 > 复杂,执行 > 计划,迭代 > 完美


# TaskFleet - 任务执行专家

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![React](https://img.shields.io/badge/react-18+-61dafb.svg)](https://reactjs.org/)
[![Tauri](https://img.shields.io/badge/tauri-2.0-24c8db.svg)](https://tauri.app/)

## 📖 项目简介

TaskFleet 是一个专注于**任务分发、进度监控和数据统计**的开源项目管理系统，为管理多个执行人员的项目提供智能化解决方案。

> **"不是又一个项目管理工具，而是专注于任务执行阶段的专业系统"**

### 核心特性

- 🚀 **智能任务分发** - 批量导入、智能分配、考虑员工负载
- 📊 **实时进度监控** - 一目了然的执行状态、异常自动提醒
- � **深度数据统计** - 员工效率分析、任务完成趋势、可视化图表
- 💻 **多端协同** - Web端与桌面端功能一致，根据场景选择使用
- 👥 **扁平化权限** - 平台管理员-项目经理-任务执行者三级架构，权限清晰
- ⚡ **高性能技术栈** - Rust + Tauri，响应迅速、资源占用少

### 目标用户与典型场景

- 📊 **市场调研团队** - 问卷调查、街访任务分配
- 🏢 **客户拜访管理** - 销售团队、客户成功任务追踪  
- � **数据录入任务** - 批量处理、质量控制
- 🎯 **运营任务管理** - 内容发布、社群维护
- � **现场服务管理** - 维修、巡检任务调度

## 🏗️ 技术架构

```
┌─────────────────────────────────────────────────────────┐
│                   TaskFleet 生态系统                       │
├─────────────────────────────────────────────────────────┤
│                                                           │
│  ┌──────────────────┐  ┌──────────────────┐            │
│  │    Web 前端       │  │    后端服务       │            │
│  │ React + TypeScript│  │  Rust + Axum     │            │
│  │   端口: 3000      │  │   端口: 8000     │            │
│  │  (详细视图+批量)  │  │ (权限控制+API)   │            │
│  └──────────────────┘  └──────────────────┘            │
│           ↓                      ↓                       │
│  ┌─────────────────────────────────────────────────┐   │
│  │         桌面客户端 (精简视图+离线)                 │   │
│  │          Rust + Tauri 2.0                        │   │
│  │     快速操作、系统托盘、离线支持                   │   │
│  │     ⚠️ 功能由角色权限决定,非端类型限制             │   │
│  └─────────────────────────────────────────────────┘   │
│                      ↓                                  │
│  ┌─────────────────────────────────────────────────┐   │
│  │         任务执行层 (根据具体业务)                   │   │
│  │     市场调研 / 客户拜访 / 数据录入等                │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

### 技术栈

| 组件 | 技术 | 说明 |
|------|------|------|
| **后端服务** | Rust + Axum + PostgreSQL | RESTful API 服务、数据库 |
| **Web前端** | React 18 + TypeScript + Ant Design 5 | 详细视图、批量操作 |
| **桌面客户端** | Rust + Tauri 2.0 + HTML/CSS/JS | 精简视图、离线支持 |
| **数据库** | PostgreSQL / SQLite | 数据持久化存储 |

## 📚 完整文档

### 🎯 TaskFleet 核心文档 (推荐优先阅读)

| 文档 | 说明 | 重要程度 |
|-----|------|---------|
| **[项目概述](./docs/TaskFleet/00-PROJECT_OVERVIEW.md)** | 项目愿景、功能特性、架构设计 | ⭐⭐⭐⭐⭐ |
| **[技术实现指南](./docs/TaskFleet/01-TECHNICAL_GUIDE.md)** | 详细技术方案、代码示例 | ⭐⭐⭐⭐⭐ |
| **[快速启动指南](./docs/TaskFleet/02-QUICK_START.md)** | 30分钟快速部署 | ⭐⭐⭐⭐⭐ |
| **[多端协同设计](./docs/TaskFleet/03-MULTI_PLATFORM_STRATEGY.md)** | Web端+桌面端策略 | ⭐⭐⭐⭐⭐ |

### � 传统文档目录 (参考资料)

所有历史文档已整理归档到 **[docs/](./docs/)** 目录：

- **[📊 文档目录树](./docs/DOCUMENT_TREE.md)** - 可视化文档结构
- **[📚 文档中心主页](./docs/README.md)** - 完整的文档索引和导航

## 🚀 快速开始

### 1. 环境要求

- **Rust**: 1.70+ (后端服务 + 桌面客户端)
- **Node.js**: 18+ (Web前端开发)
- **PostgreSQL**: 13+ (生产环境数据库)
- **Git**: 版本控制

### 2. 克隆项目

```bash
git clone https://github.com/wyw121/TaskFleet.git
cd TaskFleet
```

### 3. 快速启动 (推荐)

按照 **[快速启动指南](./docs/TaskFleet/02-QUICK_START.md)** 30分钟完成项目搭建：

```bash
# 1. 创建新项目结构
# 2. 重构后端为 TaskFleet 架构  
# 3. 重构前端为任务管理界面
# 4. 创建简化版桌面客户端
# 5. 验证和测试
```

### 4. 传统启动方式

#### Windows 用户

```bash
# 一键启动开发环境（前后端分离）
dev-start.bat
```

#### Linux/Mac 用户

```bash
# 一键启动开发环境
chmod +x dev-start.sh
./dev-start.sh
```

### 5. 访问系统

- **Web前端界面**: http://localhost:3000
- **后端API服务**: http://localhost:8000
- **API文档**: http://localhost:8000/api-docs

## 📖 推荐阅读顺序

### 🎯 TaskFleet 开发者

1. **[项目概述](./docs/TaskFleet/00-PROJECT_OVERVIEW.md)** - 了解TaskFleet定位和特性
2. **[技术实现指南](./docs/TaskFleet/01-TECHNICAL_GUIDE.md)** - 详细技术架构
3. **[快速启动指南](./docs/TaskFleet/02-QUICK_START.md)** - 动手实践
4. **[多端协同设计](./docs/TaskFleet/03-MULTI_PLATFORM_STRATEGY.md)** - 理解双端策略

### 🏢 参考文档

1. **[完整需求文档](./docs/06-requirements/COMPLETE_REQUIREMENTS.md)** - 深入了解项目需求
2. **[架构可视化](./docs/01-architecture/ARCHITECTURE_VISUALIZATION_2025.md)** - 理解系统架构
3. **[开发指南](./docs/02-development/DEVELOPMENT_GUIDE.md)** - 开发环境配置

## 🛠️ 项目结构

```
TaskFleet/
├── server-backend/          # Rust + Axum 后端服务
│   ├── src/                 # 源代码
│   ├── migrations/          # 数据库迁移
│   ├── tests/               # 测试文件
│   ├── Cargo.toml           # Rust 依赖配置
│   └── .env.example         # 环境配置示例
├── server-frontend/         # React + TypeScript Web前端
│   ├── src/                 # 源代码
│   ├── public/              # 静态资源
│   ├── package.json         # NPM 依赖配置
│   └── ...
├── employee-client/         # Tauri 桌面客户端
│   ├── src-tauri/           # Rust 后端代码
│   ├── src/                 # 前端资源
│   └── ...
├── docs/                    # 📚 完整文档库
│   ├── TaskFleet/           # ⭐ 核心 TaskFleet 文档
│   │   ├── 00-PROJECT_OVERVIEW.md    # 项目概述
│   │   ├── 01-TECHNICAL_GUIDE.md     # 技术指南
│   │   ├── 02-QUICK_START.md         # 快速启动
│   │   └── 03-MULTI_PLATFORM_STRATEGY.md # 多端策略
│   ├── 01-architecture/     # 架构文档
│   ├── 02-development/      # 开发指南
│   ├── 03-deployment/       # 部署文档
│   ├── 04-reports/          # 项目报告
│   ├── 05-user-guides/      # 用户手册
│   ├── 06-requirements/     # 需求文档
│   └── 07-ai-instructions/  # AI 辅助开发
├── TaskFleet.code-workspace # VS Code 工作区配置
├── .env.example             # 环境配置示例
├── .gitignore               # Git 忽略规则
└── README.md                # 本文件
```

## 🤝 贡献指南

我们欢迎所有形式的贡献！请查看以下文档：

- **[技术实现指南](./docs/TaskFleet/01-TECHNICAL_GUIDE.md)** - 开发规范和最佳实践
- **[快速启动指南](./docs/TaskFleet/02-QUICK_START.md)** - 如何参与开发

## 📄 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

## 📞 联系方式

- **项目仓库**: [GitHub - TaskFleet](https://github.com/wyw121/TaskFleet)
- **问题反馈**: [GitHub Issues](https://github.com/wyw121/TaskFleet/issues)
- **文档中心**: [docs/TaskFleet/](./docs/TaskFleet/)

---

**最后更新**: 2025年10月28日 - TaskFleet 项目启动

**核心文档**: 4 份 TaskFleet 专用文档 + 36 份历史参考文档

访问 **[文档中心](./docs/)** 获取完整的项目文档 📚

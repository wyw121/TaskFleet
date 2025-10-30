# TaskFleet 部署和测试指南

> **版本**: v1.1.0  
> **最后更新**: 2025-10-29  
> **适用场景**: 生产环境部署、开发环境搭建、多端权限一致性验证

---

## 📋 目录

1. [部署前准备](#部署前准备)
2. [服务器后端部署](#服务器后端部署)
3. [Web前端部署](#web前端部署)
4. [桌面客户端打包](#桌面客户端打包)
5. [权限一致性测试](#权限一致性测试)
6. [验证检查清单](#验证检查清单)
7. [常见问题](#常见问题)

---

## 部署前准备

### 系统要求

#### 服务器环境

**最低配置**:
- **操作系统**: Ubuntu 20.04+ / Debian 11+ / RHEL 8+
- **CPU**: 2 核
- **内存**: 4GB RAM
- **磁盘**: 20GB 可用空间
- **网络**: 公网 IP 或域名

**推荐配置**:
- **CPU**: 4 核以上
- **内存**: 8GB RAM 以上
- **磁盘**: 50GB SSD
- **网络**: 10Mbps+ 带宽

#### 开发环境

**后端开发**:
```bash
# Rust 工具链
rustc >= 1.70.0
cargo >= 1.70.0

# 数据库
SQLite 3.x (开发)
PostgreSQL 13+ (生产推荐)
```

**前端开发**:
```bash
# Node.js 环境
Node.js >= 18.0.0
npm >= 9.0.0

# 构建工具
Vite 6.x
TypeScript 5.x
```

**桌面端开发**:
```bash
# Rust + Tauri
Rust >= 1.70.0
Tauri CLI >= 2.0.0

# 平台特定依赖
Windows: Visual Studio Build Tools
macOS: Xcode Command Line Tools
Linux: webkit2gtk, libgtk-3-dev
```

### 克隆代码

```bash
# 克隆仓库
git clone https://github.com/wyw121/TaskFleet.git
cd TaskFleet

# 检查分支
git branch -a
git checkout main
```

### 环境变量配置

创建 `.env` 文件:

```bash
# 服务器后端配置
cat > server-backend/.env << EOF
# 数据库配置
DATABASE_URL=sqlite://data/taskfleet.db
# DATABASE_URL=postgresql://user:password@localhost/taskfleet

# JWT 密钥 (生产环境必须修改!)
JWT_SECRET=$(openssl rand -base64 32)

# 服务器配置
SERVER_HOST=0.0.0.0
SERVER_PORT=8000

# CORS 配置
CORS_ORIGINS=http://localhost:3000,http://localhost:5173

# 日志级别
RUST_LOG=info
EOF

# Web 前端配置
cat > server-frontend/.env << EOF
# API 地址 (根据实际情况修改)
VITE_API_URL=http://localhost:8000
EOF
```

---

## 服务器后端部署

### 开发模式运行

```bash
cd server-backend

# 1. 初始化数据库
cargo run --bin init-db

# 2. 运行开发服务器
cargo run

# 或使用项目任务
# 在 VS Code 中: Ctrl+Shift+P -> Tasks: Run Task -> 🚀 启动服务器后端
```

### 生产环境编译

```bash
cd server-backend

# 1. 编译 release 版本
cargo build --release

# 2. 编译产物位置
ls -lh target/release/flow-farm-backend
# 文件大小约 15-20 MB

# 3. 测试运行
./target/release/flow-farm-backend
```

### 使用 systemd 部署 (Linux)

**创建 systemd 服务文件**:

```bash
sudo tee /etc/systemd/system/taskfleet-backend.service << EOF
[Unit]
Description=TaskFleet Backend Service
After=network.target

[Service]
Type=simple
User=taskfleet
WorkingDirectory=/opt/taskfleet/server-backend
Environment="DATABASE_URL=sqlite:///opt/taskfleet/data/taskfleet.db"
Environment="JWT_SECRET=your-secret-key-here"
Environment="RUST_LOG=info"
ExecStart=/opt/taskfleet/server-backend/target/release/flow-farm-backend
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF
```

**部署步骤**:

```bash
# 1. 创建用户和目录
sudo useradd -r -s /bin/false taskfleet
sudo mkdir -p /opt/taskfleet/{server-backend,data}
sudo chown -R taskfleet:taskfleet /opt/taskfleet

# 2. 复制编译产物
sudo cp -r server-backend/target/release/flow-farm-backend /opt/taskfleet/server-backend/
sudo cp -r server-backend/migrations /opt/taskfleet/server-backend/

# 3. 初始化数据库
sudo -u taskfleet /opt/taskfleet/server-backend/flow-farm-backend --init-db

# 4. 启动服务
sudo systemctl daemon-reload
sudo systemctl enable taskfleet-backend
sudo systemctl start taskfleet-backend

# 5. 检查状态
sudo systemctl status taskfleet-backend
sudo journalctl -u taskfleet-backend -f
```

### 使用 Docker 部署

**Dockerfile**:

```dockerfile
# server-backend/Dockerfile
FROM rust:1.70 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/flow-farm-backend .
COPY --from=builder /app/migrations ./migrations

ENV DATABASE_URL=sqlite:///app/data/taskfleet.db
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8000

EXPOSE 8000

CMD ["./flow-farm-backend"]
```

**docker-compose.yml**:

```yaml
version: '3.8'

services:
  backend:
    build: ./server-backend
    ports:
      - "8000:8000"
    environment:
      - DATABASE_URL=postgresql://taskfleet:password@db/taskfleet
      - JWT_SECRET=your-secret-key-here
      - RUST_LOG=info
    depends_on:
      - db
    restart: unless-stopped

  db:
    image: postgres:15-alpine
    environment:
      - POSTGRES_USER=taskfleet
      - POSTGRES_PASSWORD=password
      - POSTGRES_DB=taskfleet
    volumes:
      - postgres_data:/var/lib/postgresql/data
    restart: unless-stopped

  frontend:
    build: ./server-frontend
    ports:
      - "80:80"
    depends_on:
      - backend
    restart: unless-stopped

volumes:
  postgres_data:
```

**部署命令**:

```bash
# 构建并启动所有服务
docker-compose up -d

# 查看日志
docker-compose logs -f backend

# 停止服务
docker-compose down
```

---

## Web前端部署

### 开发模式运行

```bash
cd server-frontend

# 1. 安装依赖
npm install

# 2. 运行开发服务器
npm run dev

# 访问: http://localhost:5173
```

### 生产环境构建

```bash
cd server-frontend

# 1. 构建生产版本
npm run build

# 2. 构建产物位置
ls -lh dist/
# dist/
#   ├── index.html
#   ├── assets/
#   │   ├── index-xxx.css
#   │   └── index-xxx.js

# 3. 预览构建结果
npm run preview
```

### 使用 Nginx 部署

**Nginx 配置**:

```nginx
# /etc/nginx/sites-available/taskfleet

server {
    listen 80;
    server_name taskfleet.example.com;

    # Web 前端
    root /var/www/taskfleet;
    index index.html;

    # 前端路由
    location / {
        try_files $uri $uri/ /index.html;
    }

    # API 代理
    location /api/ {
        proxy_pass http://127.0.0.1:8000/;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # 静态资源缓存
    location ~* \.(css|js|jpg|jpeg|png|gif|ico|svg|woff|woff2|ttf|eot)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # Gzip 压缩
    gzip on;
    gzip_types text/css application/javascript application/json;
    gzip_min_length 1000;
}
```

**部署步骤**:

```bash
# 1. 复制构建产物
sudo mkdir -p /var/www/taskfleet
sudo cp -r server-frontend/dist/* /var/www/taskfleet/

# 2. 配置 Nginx
sudo ln -s /etc/nginx/sites-available/taskfleet /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx

# 3. 配置 SSL (可选)
sudo apt install certbot python3-certbot-nginx
sudo certbot --nginx -d taskfleet.example.com
```

### 使用 Docker 部署

**Dockerfile**:

```dockerfile
# server-frontend/Dockerfile
FROM node:18 as builder

WORKDIR /app
COPY package*.json ./
RUN npm ci

COPY . .
RUN npm run build

FROM nginx:alpine

COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
```

**nginx.conf**:

```nginx
server {
    listen 80;
    root /usr/share/nginx/html;
    index index.html;

    location / {
        try_files $uri $uri/ /index.html;
    }

    location /api/ {
        proxy_pass http://backend:8000/;
    }
}
```

---

## 桌面客户端打包

### Windows 打包

```powershell
cd employee-client

# 1. 安装依赖
cargo install tauri-cli

# 2. 检查代码
cargo check

# 3. 构建应用
cargo tauri build

# 4. 输出位置
# src-tauri/target/release/bundle/msi/TaskFleet_1.0.0_x64_zh-CN.msi
# src-tauri/target/release/bundle/nsis/TaskFleet_1.0.0_x64-setup.exe

# 5. 测试安装包
# 双击运行 .exe 安装程序
```

### macOS 打包

```bash
cd employee-client

# 1. 安装 Tauri CLI
cargo install tauri-cli

# 2. 构建应用
cargo tauri build

# 3. 输出位置
# src-tauri/target/release/bundle/dmg/TaskFleet_1.0.0_x64.dmg
# src-tauri/target/release/bundle/macos/TaskFleet.app

# 4. 代码签名 (可选)
codesign --force --deep --sign "Developer ID" \
  src-tauri/target/release/bundle/macos/TaskFleet.app
```

### Linux 打包

```bash
cd employee-client

# 1. 安装系统依赖
sudo apt install -y \
  libwebkit2gtk-4.1-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev

# 2. 构建应用
cargo tauri build

# 3. 输出位置
# src-tauri/target/release/bundle/deb/taskfleet_1.0.0_amd64.deb
# src-tauri/target/release/bundle/appimage/TaskFleet_1.0.0_amd64.AppImage

# 4. 测试 AppImage
chmod +x src-tauri/target/release/bundle/appimage/TaskFleet*.AppImage
./src-tauri/target/release/bundle/appimage/TaskFleet*.AppImage
```

---

## 权限一致性测试

### 自动化集成测试

我们提供了 PowerShell 集成测试脚本,可自动验证 Web 端和桌面端的权限一致性。

**测试脚本位置**:
```
tests/integration/test-permission-consistency.ps1
```

**使用方法**:

```powershell
# 1. 确保后端服务器运行
cd server-backend
cargo run --release

# 2. 在另一个终端运行测试
cd tests/integration
pwsh ./test-permission-consistency.ps1

# 3. 查看测试报告
# 测试会输出详细的权限检查结果
```

**测试覆盖范围**:

| 角色 | 测试项目 | 预期结果 |
|------|----------|----------|
| **平台管理员** | 管理所有公司 | ✅ 允许 |
| | 创建/删除用户 | ✅ 允许 |
| | 查看所有数据 | ✅ 允许 |
| **项目经理** | 管理本公司用户 | ✅ 允许 |
| | 创建任务 | ✅ 允许 |
| | 跨公司操作 | ❌ 拒绝 |
| **任务执行者** | 查看自己的任务 | ✅ 允许 |
| | 更新任务状态 | ✅ 允许 |
| | 创建任务 | ❌ 拒绝 |

### 手动功能测试

#### 1. 准备测试环境

```bash
# 启动后端
cd server-backend
cargo run --release

# 启动 Web 前端
cd server-frontend
npm run dev

# 启动桌面客户端
cd employee-client
cargo tauri dev
```

#### 2. 创建测试用户

使用 SQL 或 API 创建三个测试用户:

```sql
-- 平台管理员
INSERT INTO users (username, password, email, full_name, role) VALUES
('admin', '$argon2...', 'admin@test.com', '测试管理员', 'platform_admin');

-- 项目经理
INSERT INTO users (username, password, email, full_name, role, company_id) VALUES
('manager', '$argon2...', 'manager@test.com', '测试经理', 'project_manager', 1);

-- 任务执行者
INSERT INTO users (username, password, email, full_name, role, company_id) VALUES
('executor', '$argon2...', 'executor@test.com', '测试执行者', 'task_executor', 1);
```

或使用初始化脚本:

```bash
cd server-backend
cargo run --bin create-test-users
```

#### 3. Web 端功能测试

**平台管理员测试**:

```
1. 登录 Web 端: http://localhost:5173
   - 用户名: admin
   - 密码: admin123

2. 验证菜单项:
   ✅ 公司管理
   ✅ 用户管理
   ✅ 任务管理
   ✅ 数据统计

3. 测试功能:
   - 创建公司 ✅
   - 创建用户 (所有角色) ✅
   - 查看所有任务 ✅
   - 删除用户 ✅

4. 测试越权:
   - 尝试修改其他管理员 (应该允许)
   - 尝试删除自己 (应该拒绝)
```

**项目经理测试**:

```
1. 登录 Web 端
   - 用户名: manager
   - 密码: manager123

2. 验证菜单项:
   ✅ 任务管理
   ✅ 团队管理
   ✅ 项目统计
   ❌ 公司管理 (不应显示)

3. 测试功能:
   - 创建任务 ✅
   - 分配任务给本公司成员 ✅
   - 查看本公司任务 ✅
   - 创建本公司用户 ✅

4. 测试越权:
   - 尝试查看其他公司任务 (应该拒绝/看不到)
   - 尝试创建其他公司用户 (应该拒绝)
   - 尝试修改其他公司数据 (应该拒绝)
```

**任务执行者测试**:

```
1. 登录 Web 端
   - 用户名: executor
   - 密码: executor123

2. 验证菜单项:
   ✅ 我的任务
   ✅ 工作记录
   ✅ 个人统计
   ❌ 任务管理 (不应显示)
   ❌ 用户管理 (不应显示)

3. 测试功能:
   - 查看分配给自己的任务 ✅
   - 更新任务状态 ✅
   - 添加工作备注 ✅

4. 测试越权:
   - 尝试查看其他人任务 (应该拒绝/看不到)
   - 尝试创建任务 (应该拒绝/无按钮)
   - 尝试修改其他人任务 (应该拒绝)
```

#### 4. 桌面端功能测试

对每个角色重复上述测试流程,确保:

**一致性检查**:
```
1. 菜单项完全一致
   - 平台管理员: 所有功能可见
   - 项目经理: 公司管理不可见
   - 任务执行者: 只有个人功能可见

2. 功能权限完全一致
   - 所有 API 调用使用相同的权限检查
   - 创建/修改/删除操作结果一致

3. 错误提示一致
   - 越权操作返回相同错误码 (403)
   - 错误消息一致
```

### 性能测试

**并发测试**:

```bash
# 使用 Apache Bench 测试
ab -n 1000 -c 10 -H "Authorization: Bearer $TOKEN" \
  http://localhost:8000/api/tasks

# 预期结果:
# Requests per second: > 100
# Time per request: < 100ms
# Failed requests: 0
```

**压力测试**:

```bash
# 使用 wrk 测试
wrk -t4 -c100 -d30s -H "Authorization: Bearer $TOKEN" \
  http://localhost:8000/api/tasks

# 预期结果:
# Latency: p50 < 50ms, p99 < 200ms
# Errors: 0%
```

---

## 验证检查清单

### 部署验证

**后端服务**:

- [ ] 服务正常启动 (`systemctl status taskfleet-backend`)
- [ ] 数据库连接成功 (检查日志)
- [ ] API 响应正常 (`curl http://localhost:8000/health`)
- [ ] JWT 认证工作 (登录获取 token)
- [ ] CORS 配置正确 (前端能访问 API)

**前端服务**:

- [ ] 构建无错误 (`npm run build`)
- [ ] 静态文件部署正确 (能访问 index.html)
- [ ] 路由工作正常 (刷新页面不报错)
- [ ] API 调用成功 (检查浏览器 Network)
- [ ] 生产环境配置正确 (VITE_API_URL)

**桌面客户端**:

- [ ] 打包成功 (生成安装包)
- [ ] 安装无错误
- [ ] 启动正常
- [ ] 服务器连接成功
- [ ] 登录功能正常

### 功能验证

**平台管理员**:

- [ ] Web 端: 管理所有公司 ✅
- [ ] Web 端: 创建所有角色用户 ✅
- [ ] Web 端: 查看全局数据 ✅
- [ ] 桌面端: 管理所有公司 ✅
- [ ] 桌面端: 创建所有角色用户 ✅
- [ ] 桌面端: 快速查看关键指标 ✅

**项目经理**:

- [ ] Web 端: 创建和分配任务 ✅
- [ ] Web 端: 管理本公司用户 ✅
- [ ] Web 端: 查看团队数据 ✅
- [ ] 桌面端: 创建和分配任务 ✅
- [ ] 桌面端: 管理本公司用户 ✅
- [ ] 桌面端: 快速查看团队进度 ✅
- [ ] 越权测试: 无法查看其他公司数据 ❌

**任务执行者**:

- [ ] Web 端: 查看自己的任务 ✅
- [ ] Web 端: 更新任务状态 ✅
- [ ] Web 端: 查看个人统计 ✅
- [ ] 桌面端: 查看自己的任务 ✅
- [ ] 桌面端: 更新任务状态 ✅
- [ ] 桌面端: 快速操作任务 ✅
- [ ] 越权测试: 无法创建任务 ❌
- [ ] 越权测试: 无法查看其他人任务 ❌

### 安全验证

- [ ] JWT token 过期机制 (2小时过期)
- [ ] 密码哈希存储 (Argon2)
- [ ] HTTPS 加密传输 (生产环境)
- [ ] CORS 白名单限制
- [ ] SQL 注入防护 (使用参数化查询)
- [ ] XSS 防护 (React 自动转义)
- [ ] CSRF 防护 (SameSite cookies)

### 性能验证

- [ ] 首页加载时间 < 2秒
- [ ] API 响应时间 < 100ms (p95)
- [ ] 数据库查询优化 (有索引)
- [ ] 静态资源 Gzip 压缩
- [ ] 前端代码分割和懒加载
- [ ] 并发 100 用户无错误

---

## 常见问题

### 1. 后端无法启动

**问题**: `cargo run` 报错

**解决**:

```bash
# 检查 Rust 版本
rustc --version
# 应该 >= 1.70.0

# 更新 Rust
rustup update stable

# 清理并重新编译
cargo clean
cargo build

# 检查数据库文件权限
ls -l data/taskfleet.db
chmod 644 data/taskfleet.db
```

### 2. 前端 API 调用失败

**问题**: CORS 错误

**解决**:

```bash
# 检查后端 CORS 配置
# server-backend/src/main.rs
# 确保前端地址在白名单中

# 检查前端 API 地址
# server-frontend/.env
VITE_API_URL=http://localhost:8000
```

### 3. 桌面端无法连接服务器

**问题**: "Connection refused"

**解决**:

```bash
# 1. 检查服务器地址配置
# 桌面端设置 -> 服务器地址

# 2. 检查后端是否监听 0.0.0.0
# server-backend/.env
SERVER_HOST=0.0.0.0

# 3. 检查防火墙
sudo ufw allow 8000/tcp

# 4. 测试网络连通性
curl http://your-server:8000/health
```

### 4. 权限检查不一致

**问题**: Web 端能操作,桌面端拒绝

**解决**:

```bash
# 1. 检查 token 是否相同
# 两端应该使用相同的 JWT secret

# 2. 检查用户角色
# 数据库中查询用户 role 字段

# 3. 运行集成测试
pwsh tests/integration/test-permission-consistency.ps1

# 4. 检查后端日志
sudo journalctl -u taskfleet-backend -n 100
```

### 5. 数据库迁移失败

**问题**: "Migration failed"

**解决**:

```bash
# 1. 检查迁移文件
ls server-backend/migrations/

# 2. 手动运行迁移
cd server-backend
cargo sqlx migrate run

# 3. 回滚并重试
cargo sqlx migrate revert
cargo sqlx migrate run

# 4. 查看数据库状态
sqlite3 data/taskfleet.db ".schema"
```

### 6. 打包失败

**问题**: Tauri build 错误

**解决**:

```powershell
# Windows
# 安装 Visual Studio Build Tools
# https://visualstudio.microsoft.com/downloads/

# 检查 WebView2
# 应该已安装 Edge WebView2 Runtime

# macOS
xcode-select --install

# Linux
sudo apt install libwebkit2gtk-4.1-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev
```

---

## 下一步

**生产环境优化**:

1. **性能优化**:
   - 启用数据库连接池
   - 配置 Redis 缓存
   - 使用 CDN 加速静态资源

2. **监控告警**:
   - 集成 Prometheus + Grafana
   - 配置日志聚合 (ELK Stack)
   - 设置错误告警 (Sentry)

3. **备份策略**:
   - 定时数据库备份
   - 配置异地备份
   - 测试恢复流程

4. **扩展性**:
   - 使用负载均衡器
   - 配置多实例部署
   - 数据库主从复制

**参考文档**:

- [系统架构文档](ARCHITECTURE.md)
- [API 接口文档](API.md)
- [用户使用指南](USER_GUIDE.md)
- [权限系统详解](ROLE_SYSTEM_ANALYSIS_AND_OPTIMIZATION.md)
- [多端权限一致性](MULTI_PLATFORM_PERMISSION_UNITY.md)

---

**祝部署顺利!** 🚀

如有问题,请提交 [GitHub Issue](https://github.com/wyw121/TaskFleet/issues)。

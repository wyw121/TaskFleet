# TaskFleet Desktop Client - GitHub Copilot Instructions

## Project Overview

TaskFleet Desktop Client 是一个现代化桌面 GUI 应用程序，使用 **Rust** 和 **Tauri 框架** 构建。这是 TaskFleet 任务执行专家系统的桌面客户端，提供以下功能：

- 多角色身份验证和授权（平台管理员、项目经理、任务执行者）
- 任务查看和状态更新
- 与TaskFleet服务器实时数据同步
- 系统托盘常驻和通知
- 离线工作支持
- 高效的任务管理界面

**重要提醒**: 
1. 本项目使用 **Tauri 原生 GUI（Rust + HTML/CSS/JS）**，而不是 React.js 或纯 Web 框架。
2. **权限完全一致**: 桌面端与Web端权限控制完全相同，功能由用户角色决定，非端类型限制。
3. **UI简化原则**: UI精简但功能不阉割，所有API调用都经过后端权限验证。

## 核心模块架构

### 认证模块

- 用户登录和会话管理
- JWT令牌处理
- 安全的凭证存储

### 任务管理模块（核心）

1. **任务查看**：
   - 获取分配的任务列表
   - 任务详情查看
   - 任务搜索和过滤
   - 任务排序（按优先级、截止日期等）

2. **任务执行**：
   - 任务状态更新（开始、暂停、完成）
   - 添加任务备注和评论
   - 上传任务相关文件
   - 记录工作时间

### 数据同步模块

- 与服务器实时通信
- 任务状态自动同步
- 离线数据缓存
- 网络重连机制

### 通知系统

- 新任务分配通知
- 任务截止日期提醒
- 系统托盘集成
- 桌面通知显示

## Technology Stack

### Core Technologies

- **Language**: Rust (Edition 2021)
- **GUI Framework**: Tauri 2.0 (Native desktop application)
- **Frontend**: HTML/CSS/JavaScript (minimal, for UI only)
- **Build System**: Cargo + Tauri CLI
- **Platform**: Windows (primary), with cross-platform support
- **Database**: SQLx for local storage
- **API Communication**: reqwest for server communication

### Key Dependencies

- `tauri`: 2.0 (Main GUI framework)
- `serde`: JSON serialization
- `tokio`: Async runtime
- `reqwest`: HTTP client
- `sqlx`: Database operations
- `uuid`: Unique identifiers
- `chrono`: Date/time handling

## Project Structure

```
employee-client/
├── src-tauri/              # Rust backend code
│   ├── src/
│   │   ├── main.rs        # Application entry point
│   │   ├── api.rs         # API communication with server
│   │   ├── auth.rs        # Authentication service
│   │   ├── models.rs      # Data models and structures
│   │   └── utils.rs       # Utility functions
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
├── src/                   # Frontend assets (HTML/CSS/JS)
│   ├── index.html         # Main application UI
│   ├── styles.css         # Application styling
│   └── components/        # UI components
├── logs/                  # Application logs
├── public/                # Static assets
└── target/                # Build artifacts (excluded)
```

## Build and Development Instructions

### Environment Setup

1. **Install Rust**: Use rustup to install latest stable Rust
2. **Install Tauri CLI**: `cargo install tauri-cli`
3. **Verify Installation**: `cargo tauri --version`

### Development Commands

```bash
# Development mode (hot reload)
cargo tauri dev

# Check code
cargo check

# Run tests
cargo test

# Code formatting
cargo fmt

# Code linting
cargo clippy --all-targets --all-features

# Production build
cargo tauri build
```

### Important Build Notes

- **Always run `cargo check` before making changes**
- **Use `cargo tauri dev` for development with hot reload**
- **Production builds require: `cargo tauri build`**
- **Target platform**: Windows (x86_64-pc-windows-msvc)

## Build and Development Instructions

### Environment Setup

1. **Install Rust**: Use rustup to install latest stable Rust
2. **Install Tauri CLI**: `cargo install tauri-cli`
3. **Verify Installation**: `cargo tauri --version`

### Development Commands

```bash
# Development mode (hot reload)
cargo tauri dev

# Check code
cargo check

# Run tests
cargo test

# Code formatting
cargo fmt

# Code linting
cargo clippy --all-targets --all-features

# Production build
cargo tauri build
```

### Important Build Notes

- **Always run `cargo check` before making changes**
- **Use `cargo tauri dev` for development with hot reload**
- **Production builds require: `cargo tauri build`**
- **Target platform**: Windows (x86_64-pc-windows-msvc)

## Code Standards and Conventions

### Rust Code Style

- Follow standard Rust conventions (rustfmt)
- Use `snake_case` for functions and variables
- Use `PascalCase` for types and structs
- Maximum line length: 100 characters
- Always use explicit error handling with `Result<T, E>`

### Project-Specific Guidelines

- **Async/Await**: Use tokio for all async operations
- **Error Handling**: Create custom error types using `thiserror`
- **Configuration**: Store app config in `src-tauri/tauri.conf.json`
- **API Communication**: Use `reqwest` with proper error handling
- **Database**: Use SQLx with compile-time checked queries

### File Organization

- Keep business logic in `src-tauri/src/`
- Frontend assets in `src/` (minimal HTML/CSS/JS)
- Tests alongside source files (`#[cfg(test)]` modules)
- Documentation in `README.md` and inline comments

## Key Architectural Patterns

### Tauri Architecture

- **Frontend**: Minimal HTML/CSS/JS for UI rendering
- **Backend**: Rust code handles all business logic
- **Communication**: Tauri commands and events bridge frontend/backend
- **Security**: Tauri provides secure API access and sandboxing

### Data Flow

1. UI interactions trigger Tauri commands
2. Rust backend processes requests
3. API calls to TaskFleet server
4. Database operations for local storage
5. Events update UI state

## Testing and Validation

### Required Tests

- Unit tests for all business logic functions
- Integration tests for API communication
- Error handling tests

### Validation Steps

1. Run `cargo test` - All tests must pass
2. Run `cargo clippy` - No warnings allowed
3. Run `cargo fmt --check` - Code must be formatted
4. Build succeeds: `cargo tauri build`
5. Manual testing of GUI functionality

## Development Workflow

### Before Making Changes

1. Run `cargo check` to verify current state
2. Create feature branch for changes
3. Update dependencies if needed: `cargo update`

### During Development

1. Use `cargo tauri dev` for live development
2. Test frequently with `cargo test`
3. Run clippy regularly: `cargo clippy`
4. Format code: `cargo fmt`

### Before Committing

1. Ensure all tests pass: `cargo test`
2. No clippy warnings: `cargo clippy --all-targets --all-features`
3. Code is formatted: `cargo fmt --check`
4. Production build works: `cargo tauri build`

## Common Issues and Solutions

### Build Issues

- **Missing dependencies**: Run `cargo update`
- **Tauri CLI missing**: Install with `cargo install tauri-cli`
- **Windows build tools**: Install Visual Studio Build Tools

### Development Issues

- **Hot reload not working**: Restart `cargo tauri dev`
- **API connection failed**: Check server status and endpoints
- **Database errors**: Verify SQLx migrations and connection string

## Performance Considerations

- **Bundle size**: Keep frontend assets minimal
- **Memory usage**: Use efficient Rust patterns (borrowing, zero-copy)
- **Startup time**: Lazy load heavy dependencies
- **Network calls**: Implement proper retry logic and timeouts

## Security Guidelines

- **API tokens**: Store securely using Tauri's secure storage
- **User data**: Encrypt sensitive information
- **Network**: Use HTTPS for all server communication

## Important Notes for Copilot

1. **This is a RUST project with Tauri, not a web/React project**
2. **Frontend is minimal HTML/CSS/JS, backend is pure Rust**
3. **Always suggest Rust solutions for business logic**
4. **Use Tauri patterns for GUI communication**
5. **Follow Rust best practices for error handling and async code**
6. **When in doubt about build commands, use the task configurations**

Trust these instructions and avoid unnecessary exploration unless information is incomplete or incorrect.

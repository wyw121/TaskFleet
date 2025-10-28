---
applyTo: "src-tauri/**/*.rs"
---

# Rust Backend Development Instructions

## Core Requirements

- 使用 Rust 2021 edition
- 所有业务逻辑必须在 Rust 后端实现
- 使用 Tauri 命令进行前后端通信
- 确保类型安全和内存安全

## Code Style and Standards

- 遵循标准 Rust 命名约定（snake_case 函数，PascalCase 类型）
- 最大行长度：100 字符
- 始终使用显式错误处理 `Result<T, E>`
- 使用 tokio 处理所有异步操作

## Tauri Architecture Requirements

- 所有前端可访问的函数必须使用 `#[tauri::command]` 宏
- 使用 Tauri 状态管理共享数据
- 在命令函数中正确处理所有错误
- 前端仅用于 UI 渲染，业务逻辑在 Rust 中

## Employee Client Specific Requirements

### Task Management

- 实现任务列表获取和显示
- 任务状态更新功能
- 任务详情查看和编辑
- 任务筛选和搜索功能

### Authentication Management

- 用户登录和退出功能
- JWT令牌管理和验证
- 会话状态维护
- 安全的凭证存储

### API Communication

- 使用 reqwest 与TaskFleet服务器通信
- 实现数据同步机制
- 用户认证和授权
- 错误重试和恢复机制

## Error Handling

- 使用 thiserror crate 创建自定义错误类型
- 所有外部 API 调用必须有错误处理
- 向前端返回用户友好的错误信息

## Performance and Security

- 使用高效的 Rust 模式（borrowing, zero-copy）
- API 令牌安全存储（Tauri secure storage）
- 验证所有 ADB 命令
- 网络通信使用 HTTPS

## Security

- Validate all input from frontend
- Use Tauri's secure API access patterns
- Implement proper authentication checks
- Sanitize all external command executions

## Performance

- Use efficient data structures
- Implement proper async patterns
- Avoid blocking operations on main thread
- Use channels for inter-task communication

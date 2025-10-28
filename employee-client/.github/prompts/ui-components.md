# TaskFleet Employee Client - UI Components

## Modern Desktop Interface Design for Task Management

### Main Dashboard
```prompt
创建任务管理主界面，包含：
1. 任务列表显示
2. 任务状态筛选
3. 任务搜索功能
4. 任务详情查看
5. 状态更新操作

HTML 结构：
```html
<div class="task-dashboard">
    <h2>我的任务 (My Tasks)</h2>
    <div class="task-filters">
        <select id="status-filter">
            <option value="all">全部任务</option>
            <option value="todo">待开始</option>
            <option value="in-progress">进行中</option>
            <option value="completed">已完成</option>
        </select>
        <input type="search" id="task-search" placeholder="搜索任务...">
    </div>
    <div class="task-list">
        <!-- 动态生成任务项 -->
    </div>
</div>
```

CSS 样式要求：
- 现代卡片设计
- 清晰的任务状态指示
- 响应式布局
```

### Task Card Component
```prompt
设计任务卡片组件：
1. 任务标题和描述
2. 优先级标识
3. 截止日期显示
4. 状态更新按钮
5. 详情查看链接

布局结构：
```html
<div class="task-card">
    <div class="task-header">
        <h3 class="task-title"></h3>
        <span class="task-priority"></span>
    </div>
    <div class="task-content">
        <p class="task-description"></p>
        <div class="task-meta">
            <span class="due-date"></span>
            <span class="task-status"></span>
        </div>
    </div>
    <div class="task-actions">
        <button class="btn-start">开始任务</button>
        <button class="btn-complete">完成任务</button>
        <button class="btn-details">查看详情</button>
    </div>
</div>
```
```
### Login Component
```prompt
创建登录界面组件：
1. 用户名和密码输入
2. 记住登录状态选项
3. 登录状态反馈
4. 错误信息显示
5. 服务器连接状态

HTML 结构：
```html
<div class="login-container">
    <div class="login-form">
        <h2>TaskFleet 员工客户端</h2>
        <form id="login-form">
            <div class="input-group">
                <input type="text" id="username" placeholder="用户名" required>
            </div>
            <div class="input-group">
                <input type="password" id="password" placeholder="密码" required>
            </div>
            <div class="checkbox-group">
                <input type="checkbox" id="remember-me">
                <label for="remember-me">记住登录状态</label>
            </div>
            <button type="submit" class="login-btn">登录</button>
        </form>
        <div class="server-status">
            <span id="connection-status">检查服务器连接...</span>
        </div>
    </div>
</div>
```
```

## Interactive Components

### Task Status Update Component
```prompt
创建任务状态更新组件：
1. 状态选择器
2. 备注添加功能
3. 文件附件上传
4. 工作时间记录
5. 确认提交按钮

JavaScript 实现：
```javascript
class TaskStatusUpdater {
    constructor(taskId) {
        this.taskId = taskId;
        this.init();
    }

    async updateTaskStatus(status, notes, attachments) {
        try {
            const result = await window.__TAURI__.invoke('update_task_status', {
                taskId: this.taskId,
                status: status,
                notes: notes,
                attachments: attachments
            });
            this.handleSuccess(result);
        } catch (error) {
            this.handleError(error);
        }
    }

    async uploadAttachment(file) {
        try {
            const result = await window.__TAURI__.invoke('upload_attachment', {
                taskId: this.taskId,
                filePath: file.path
            });
            return result;
        } catch (error) {
            throw error;
        }
    }
}
```
```

### Notification Component
```prompt
实现通知组件：
1. 新任务通知
2. 截止日期提醒
3. 状态更新确认
4. 错误信息显示
5. 系统托盘集成

HTML 结构：
```html
<div class="notification-center">
    <div class="notification-header">
        <h3>通知中心</h3>
        <button id="mark-all-read">全部已读</button>
    </div>
    <div class="notification-list">
        <!-- 动态生成通知项 -->
    </div>
</div>

<template id="notification-template">
    <div class="notification-item">
        <div class="notification-icon"></div>
        <div class="notification-content">
            <h4 class="notification-title"></h4>
            <p class="notification-message"></p>
            <span class="notification-time"></span>
        </div>
        <button class="notification-close">×</button>
    </div>
</template>
```
```

## Responsive Layout Patterns

### Grid Layout System
```prompt
创建响应式网格布局：
```css
.app-container {
    display: grid;
    grid-template-areas:
        "sidebar main-content"
        "sidebar status-bar";
    grid-template-columns: 300px 1fr;
    grid-template-rows: 1fr auto;
    height: 100vh;
    gap: 1rem;
}

.sidebar {
    grid-area: sidebar;
    background: #f5f5f5;
    padding: 1rem;
    overflow-y: auto;
}

.main-content {
    grid-area: main-content;
    padding: 1rem;
    overflow-y: auto;
}

.status-bar {
    grid-area: status-bar;
    background: #e0e0e0;
    padding: 0.5rem 1rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

@media (max-width: 768px) {
    .app-container {
        grid-template-areas:
            "sidebar"
            "main-content"
            "status-bar";
        grid-template-columns: 1fr;
        grid-template-rows: auto 1fr auto;
    }
}
```
```

## Event Handling Patterns

### Tauri Integration
```prompt
实现 Tauri 前后端通信：
```javascript
class TaskFleetService {
    // 调用 Rust 命令
    async invokeCommand(command, payload = {}) {
        try {
            const result = await window.__TAURI__.invoke(command, payload);
            return { success: true, data: result };
        } catch (error) {
            console.error(`Command ${command} failed:`, error);
            return { success: false, error: error };
        }
    }

    // 监听后端事件
    async listen(event, callback) {
        const unlisten = await window.__TAURI__.event.listen(event, callback);
        return unlisten;
    }

    // 发送事件到后端
    async emit(event, payload) {
        await window.__TAURI__.event.emit(event, payload);
    }

    // 任务相关操作
    async getTasks(filter = {}) {
        return this.invokeCommand('get_tasks', filter);
    }

    async updateTask(taskId, updates) {
        return this.invokeCommand('update_task', { taskId, updates });
    }

    async authenticate(username, password) {
        return this.invokeCommand('authenticate', { username, password });
    }
}

// 使用示例
const service = new TaskFleetService();

// 获取任务列表
service.getTasks({ status: 'todo' }).then(result => {
    if (result.success) {
        displayTasks(result.data);
    }
});

// 监听任务更新事件
service.listen('task-updated', (event) => {
    refreshTaskDisplay(event.payload);
});
```
```

## Performance Optimization

### Virtual Scrolling for Large Lists
```prompt
为大量任务列表实现虚拟滚动：
```javascript
class VirtualTaskList {
    constructor(container, itemHeight = 80) {
        this.container = container;
        this.itemHeight = itemHeight;
        this.visibleItems = Math.ceil(container.clientHeight / itemHeight) + 2;
        this.scrollTop = 0;
        this.init();
    }

    init() {
        this.viewport = document.createElement('div');
        this.viewport.style.height = '100%';
        this.viewport.style.overflow = 'auto';
        
        this.content = document.createElement('div');
        this.viewport.appendChild(this.content);
        this.container.appendChild(this.viewport);
        
        this.viewport.addEventListener('scroll', this.handleScroll.bind(this));
    }

    render(tasks) {
        this.tasks = tasks;
        this.content.style.height = `${tasks.length * this.itemHeight}px`;
        this.renderVisibleItems();
    }

    renderVisibleItems() {
        const startIndex = Math.floor(this.scrollTop / this.itemHeight);
        const endIndex = Math.min(startIndex + this.visibleItems, this.tasks.length);
        
        this.content.innerHTML = '';
        
        for (let i = startIndex; i < endIndex; i++) {
            const item = this.createTaskItem(this.tasks[i], i);
            this.content.appendChild(item);
        }
    }
}
```
```
const tauriService = new TauriService();

// 获取设备列表
async function getDevices() {
    const result = await tauriService.invokeCommand('get_devices');
    if (result.success) {
        updateDeviceList(result.data);
    } else {
        showError(result.error);
    }
}

// 监听设备状态变化
tauriService.listen('device-status-changed', (event) => {
    updateDeviceStatus(event.payload);
});
```
```

// TaskFleet Employee Client - JavaScript应用逻辑
// 使用Tauri的invoke API与Rust后端通信

const { invoke } = window.__TAURI__.core;

// ==================== 应用状态 ====================
const app = {
    currentUser: null,
    permissions: null,  // 用户权限信息
    tasks: [],
    workLogs: [],
    activeSession: null,
    filters: {
        status: '',
        priority: ''
    }
};

// ==================== 页面导航 ====================
function showPage(pageName) {
    document.querySelectorAll('.page').forEach(page => {
        page.classList.add('hidden');
    });
    document.getElementById(`${pageName}-page`).classList.remove('hidden');
}

function switchTab(tabName) {
    // 更新tab按钮状态
    document.querySelectorAll('.tab-btn').forEach(btn => {
        btn.classList.remove('active');
    });
    document.querySelector(`[data-tab="${tabName}"]`)?.classList.add('active');

    // 显示对应的tab内容
    document.querySelectorAll('.tab-content').forEach(content => {
        content.classList.remove('active');
    });
    document.getElementById(`${tabName}-tab`)?.classList.add('active');

    // 加载对应tab的数据
    if (tabName === 'tasks') {
        loadTasks();
    } else if (tabName === 'work-logs') {
        loadWorkLogs();
    } else if (tabName === 'settings') {
        loadSettings();
    }
}

// ==================== 认证相关 ====================
async function handleLogin(event) {
    event.preventDefault();

    const username = document.getElementById('username').value;
    const password = document.getElementById('password').value;
    const errorDiv = document.getElementById('login-error');

    try {
        errorDiv.style.display = 'none';

        const response = await invoke('login', { username, password });

        console.log('登录成功:', response);
        app.currentUser = response.user;

        // 保存登录状态到localStorage
        localStorage.setItem('auth_token', response.token);
        localStorage.setItem('current_user', JSON.stringify(response.user));

        // 显示主应用界面
        showPage('app');
        initializeApp();
    } catch (error) {
        console.error('登录失败:', error);
        errorDiv.textContent = error;
        errorDiv.style.display = 'block';
    }
}

async function handleLogout() {
    try {
        await invoke('logout');

        // 清除本地存储
        localStorage.removeItem('auth_token');
        localStorage.removeItem('current_user');

        // 重置应用状态
        app.currentUser = null;
        app.tasks = [];
        app.workLogs = [];
        app.activeSession = null;

        // 显示登录页面
        showPage('login');
    } catch (error) {
        console.error('退出登录失败:', error);
        alert('退出登录失败: ' + error);
    }
}

async function initializeApp() {
    // 加载用户权限信息
    try {
        app.permissions = await invoke('get_user_permissions');
        console.log('用户权限:', app.permissions);
        
        // 显示用户信息和角色
        document.getElementById('user-name').textContent = app.currentUser.full_name;
        
        // 显示角色徽章
        const roleBadge = document.getElementById('user-role-badge');
        roleBadge.textContent = app.permissions.role_display;
        roleBadge.className = `role-badge role-${app.permissions.role_color}`;
        roleBadge.title = `当前角色: ${app.permissions.role_display}`;
        
        // 根据权限动态显示/隐藏功能
        updateUIByPermissions();
        
    } catch (error) {
        console.error('加载权限失败:', error);
    }

    // 加载初始数据
    loadTasks();
    checkActiveSession();

    // 开始定期刷新
    startAutoRefresh();
}

// 根据权限更新UI显示
function updateUIByPermissions() {
    const perms = app.permissions;
    if (!perms) return;

    // 动态添加管理功能标签页(如果有权限)
    const tabsContainer = document.querySelector('.tabs');
    
    // 清除可能已存在的管理标签
    document.querySelectorAll('.admin-tab').forEach(el => el.remove());
    
    // 如果可以创建任务,添加创建按钮
    if (perms.can_create_task) {
        const tasksTab = document.getElementById('tasks-tab');
        const header = tasksTab.querySelector('.content-header');
        
        // 检查是否已存在创建按钮
        if (!header.querySelector('#create-task-btn')) {
            const createBtn = document.createElement('button');
            createBtn.id = 'create-task-btn';
            createBtn.className = 'btn btn-primary';
            createBtn.textContent = '➕ 创建任务';
            createBtn.onclick = () => alert('创建任务功能(待实现)');
            header.appendChild(createBtn);
        }
    }
    
    // 如果可以查看分析,添加分析标签
    if (perms.can_view_analytics) {
        const analyticsTab = document.createElement('button');
        analyticsTab.className = 'tab-btn admin-tab';
        analyticsTab.setAttribute('data-tab', 'analytics');
        analyticsTab.textContent = '📈 数据分析';
        analyticsTab.onclick = () => {
            switchTab('analytics');
            alert('数据分析功能(待实现)');
        };
        tabsContainer.appendChild(analyticsTab);
    }
    
    // 如果可以管理用户,添加用户管理标签
    if (perms.can_manage_users) {
        const usersTab = document.createElement('button');
        usersTab.className = 'tab-btn admin-tab';
        usersTab.setAttribute('data-tab', 'users');
        usersTab.textContent = '👥 用户管理';
        usersTab.onclick = () => {
            switchTab('users');
            alert('用户管理功能(待实现)');
        };
        tabsContainer.appendChild(usersTab);
    }
    
    console.log('可用功能:', perms.available_features);
}

// ==================== 任务管理 ====================
async function loadTasks() {
    const taskList = document.getElementById('task-list');
    taskList.innerHTML = '<div class="loading">加载中...</div>';

    try {
        let tasks = await invoke('get_my_tasks');

        // 应用筛选
        if (app.filters.status) {
            tasks = tasks.filter(t => t.status === app.filters.status);
        }
        if (app.filters.priority) {
            tasks = tasks.filter(t => t.priority === app.filters.priority);
        }

        app.tasks = tasks;
        renderTaskList();
    } catch (error) {
        console.error('加载任务失败:', error);
        taskList.innerHTML = `<div class="empty-state">
            <div class="empty-state-icon">⚠️</div>
            <div class="empty-state-text">加载任务失败: ${error}</div>
        </div>`;
    }
}

function renderTaskList() {
    const taskList = document.getElementById('task-list');

    if (app.tasks.length === 0) {
        taskList.innerHTML = `<div class="empty-state">
            <div class="empty-state-icon">📋</div>
            <div class="empty-state-text">暂无任务</div>
        </div>`;
        return;
    }

    taskList.innerHTML = app.tasks.map(task => `
        <div class="task-card priority-${task.priority}">
            <div class="task-header">
                <div>
                    <h4 class="task-title">${escapeHtml(task.title)}</h4>
                </div>
                <div class="task-badges">
                    <span class="badge badge-status-${task.status}">
                        ${getStatusText(task.status)}
                    </span>
                </div>
            </div>
            ${task.description ? `<p class="task-description">${escapeHtml(task.description)}</p>` : ''}
            <div class="task-meta">
                <span class="task-meta-item">
                    <span>📌</span>
                    <span>${getPriorityText(task.priority)}</span>
                </span>
                ${task.due_date ? `
                <span class="task-meta-item">
                    <span>📅</span>
                    <span>${formatDate(task.due_date)}</span>
                </span>` : ''}
                ${task.estimated_hours ? `
                <span class="task-meta-item">
                    <span>⏱️</span>
                    <span>${task.estimated_hours}h</span>
                </span>` : ''}
            </div>
            <div class="task-actions">
                ${task.status === 'pending' ? `
                    <button class="btn btn-primary" onclick="startTask(${task.id})">
                        开始任务
                    </button>
                ` : ''}
                ${task.status === 'inprogress' ? `
                    <button class="btn btn-primary" onclick="completeTask(${task.id})">
                        完成任务
                    </button>
                ` : ''}
                <button class="btn btn-secondary" onclick="showTaskDetail(${task.id})">
                    查看详情
                </button>
            </div>
        </div>
    `).join('');
}

async function startTask(taskId) {
    try {
        await invoke('start_task', { taskId });
        await loadTasks();
        await checkActiveSession();
        showNotification('任务已开始', 'success');
    } catch (error) {
        console.error('开始任务失败:', error);
        alert('开始任务失败: ' + error);
    }
}

async function completeTask(taskId) {
    try {
        await invoke('complete_task', { taskId });
        await loadTasks();
        await checkActiveSession();
        showNotification('任务已完成', 'success');
    } catch (error) {
        console.error('完成任务失败:', error);
        alert('完成任务失败: ' + error);
    }
}

async function showTaskDetail(taskId) {
    try {
        const task = await invoke('get_task', { taskId });

        document.getElementById('task-detail-title').textContent = task.title;
        document.getElementById('task-detail-status').textContent = getStatusText(task.status);
        document.getElementById('task-detail-priority').textContent = getPriorityText(task.priority);
        document.getElementById('task-detail-due-date').textContent = task.due_date ? formatDateTime(task.due_date) : '无';
        document.getElementById('task-detail-description').textContent = task.description || '无描述';

        // 配置操作按钮
        const actionBtn = document.getElementById('task-action-btn');
        if (task.status === 'pending') {
            actionBtn.textContent = '开始任务';
            actionBtn.onclick = () => {
                hideModal('task-detail-modal');
                startTask(task.id);
            };
        } else if (task.status === 'inprogress') {
            actionBtn.textContent = '完成任务';
            actionBtn.onclick = () => {
                hideModal('task-detail-modal');
                completeTask(task.id);
            };
        } else {
            actionBtn.style.display = 'none';
        }

        showModal('task-detail-modal');
    } catch (error) {
        console.error('获取任务详情失败:', error);
        alert('获取任务详情失败: ' + error);
    }
}

// ==================== 活动会话管理 ====================
async function checkActiveSession() {
    try {
        const session = await invoke('get_active_work_session');
        app.activeSession = session;

        const banner = document.getElementById('active-session-banner');
        if (session) {
            document.getElementById('session-task-title').textContent = session.task_title;
            updateSessionDuration(session.started_at);
            banner.classList.remove('hidden');

            // 开始更新会话时长
            if (app.sessionTimer) {
                clearInterval(app.sessionTimer);
            }
            app.sessionTimer = setInterval(() => {
                updateSessionDuration(session.started_at);
            }, 1000);
        } else {
            banner.classList.add('hidden');
            if (app.sessionTimer) {
                clearInterval(app.sessionTimer);
            }
        }
    } catch (error) {
        console.error('检查活动会话失败:', error);
    }
}

function updateSessionDuration(startedAt) {
    const start = new Date(startedAt);
    const now = new Date();
    const duration = Math.floor((now - start) / 1000);

    const hours = Math.floor(duration / 3600);
    const minutes = Math.floor((duration % 3600) / 60);
    const seconds = duration % 60;

    document.getElementById('session-duration').textContent =
        `已工作: ${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
}

// 未完待续...继续下一部分
// TaskFleet Employee Client - JavaScript应用逻辑 (第2部分)

// ==================== 工作记录管理 ====================
async function loadWorkLogs() {
    const workLogList = document.getElementById('work-log-list');
    workLogList.innerHTML = '<div class="loading">加载中...</div>';

    try {
        const logs = await invoke('get_my_work_logs');
        app.workLogs = logs;
        renderWorkLogList();
    } catch (error) {
        console.error('加载工作记录失败:', error);
        workLogList.innerHTML = `<div class="empty-state">
            <div class="empty-state-icon">⚠️</div>
            <div class="empty-state-text">加载工作记录失败: ${error}</div>
        </div>`;
    }
}

function renderWorkLogList() {
    const workLogList = document.getElementById('work-log-list');

    if (app.workLogs.length === 0) {
        workLogList.innerHTML = `<div class="empty-state">
            <div class="empty-state-icon">📊</div>
            <div class="empty-state-text">暂无工作记录</div>
        </div>`;
        return;
    }

    workLogList.innerHTML = app.workLogs.map(log => `
        <div class="work-log-item">
            <div class="work-log-header">
                <span class="work-log-task">任务 #${log.task_id}</span>
                <span class="work-log-hours">${log.hours} 小时</span>
            </div>
            ${log.notes ? `<div class="work-log-notes">${escapeHtml(log.notes)}</div>` : ''}
            <div class="work-log-date">${formatDateTime(log.created_at)}</div>
        </div>
    `).join('');
}

async function showAddWorkLogModal() {
    // 填充任务下拉列表
    const taskSelect = document.getElementById('work-log-task');
    taskSelect.innerHTML = app.tasks
        .filter(t => t.status !== 'cancelled')
        .map(t => `<option value="${t.id}">${t.title}</option>`)
        .join('');

    document.getElementById('work-log-hours').value = '';
    document.getElementById('work-log-notes').value = '';

    showModal('add-work-log-modal');
}

async function submitWorkLog() {
    const taskId = parseInt(document.getElementById('work-log-task').value);
    const hours = parseFloat(document.getElementById('work-log-hours').value);
    const notes = document.getElementById('work-log-notes').value || null;

    try {
        await invoke('create_work_log', { taskId, hours, notes });
        hideModal('add-work-log-modal');
        showNotification('工作记录已添加', 'success');
        await loadWorkLogs();
    } catch (error) {
        console.error('添加工作记录失败:', error);
        alert('添加工作记录失败: ' + error);
    }
}

// ==================== 设置页面 ====================
async function loadSettings() {
    try {
        const version = await invoke('get_app_version');
        document.getElementById('app-version').textContent = version;
    } catch (error) {
        console.error('加载设置失败:', error);
    }
}

// ==================== 模态框管理 ====================
function showModal(modalId) {
    document.getElementById(modalId).classList.remove('hidden');
}

function hideModal(modalId) {
    document.getElementById(modalId).classList.add('hidden');
}

// ==================== 筛选器 ====================
function applyFilter(type, value) {
    app.filters[type] = value;
    renderTaskList();
}

// ==================== 通知 ====================
function showNotification(message, type = 'info') {
    // 这里可以使用Tauri的通知API或简单的alert
    console.log(`[${type}] ${message}`);

    // 简单的浮动提示
    const notification = document.createElement('div');
    notification.className = `notification notification-${type}`;
    notification.textContent = message;
    notification.style.cssText = `
        position: fixed;
        top: 20px;
        right: 20px;
        padding: 12px 20px;
        background: ${type === 'success' ? '#52c41a' : '#1890ff'};
        color: white;
        border-radius: 6px;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
        z-index: 2000;
        animation: slideIn 0.3s ease;
    `;

    document.body.appendChild(notification);

    setTimeout(() => {
        notification.style.animation = 'slideOut 0.3s ease';
        setTimeout(() => notification.remove(), 300);
    }, 3000);
}

// ==================== 自动刷新 ====================
function startAutoRefresh() {
    // 每30秒刷新一次任务列表
    setInterval(() => {
        if (document.querySelector('.tab-btn[data-tab="tasks"]')?.classList.contains('active')) {
            loadTasks();
        }
    }, 30000);

    // 每5秒检查一次活动会话
    setInterval(() => {
        checkActiveSession();
    }, 5000);
}

// ==================== 工具函数 ====================
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

function formatDate(dateString) {
    const date = new Date(dateString);
    return date.toLocaleDateString('zh-CN', {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit'
    });
}

function formatDateTime(dateString) {
    const date = new Date(dateString);
    return date.toLocaleString('zh-CN', {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit'
    });
}

function getStatusText(status) {
    const statusMap = {
        'pending': '待处理',
        'inprogress': '进行中',
        'completed': '已完成',
        'cancelled': '已取消'
    };
    return statusMap[status] || status;
}

function getPriorityText(priority) {
    const priorityMap = {
        'low': '低',
        'medium': '中',
        'high': '高',
        'urgent': '紧急'
    };
    return priorityMap[priority] || priority;
}

// ==================== 事件监听器 ====================
document.addEventListener('DOMContentLoaded', () => {
    // 登录表单
    document.getElementById('login-form').addEventListener('submit', handleLogin);

    // 退出登录按钮
    document.getElementById('logout-btn').addEventListener('click', handleLogout);

    // 刷新按钮
    document.getElementById('refresh-btn').addEventListener('click', () => {
        loadTasks();
    });

    // 标签页切换
    document.querySelectorAll('.tab-btn').forEach(btn => {
        btn.addEventListener('click', () => {
            switchTab(btn.dataset.tab);
        });
    });

    // 筛选器
    document.getElementById('status-filter').addEventListener('change', (e) => {
        applyFilter('status', e.target.value);
    });

    document.getElementById('priority-filter').addEventListener('change', (e) => {
        applyFilter('priority', e.target.value);
    });

    // 添加工作记录
    document.getElementById('add-work-log-btn').addEventListener('click', showAddWorkLogModal);
    document.getElementById('submit-work-log-btn').addEventListener('click', submitWorkLog);

    // 模态框关闭按钮
    document.querySelectorAll('.modal-close').forEach(btn => {
        btn.addEventListener('click', () => {
            btn.closest('.modal').classList.add('hidden');
        });
    });

    // 点击模态框背景关闭
    document.querySelectorAll('.modal').forEach(modal => {
        modal.addEventListener('click', (e) => {
            if (e.target === modal) {
                modal.classList.add('hidden');
            }
        });
    });

    // 检查是否已登录
    const savedUser = localStorage.getItem('current_user');
    if (savedUser) {
        app.currentUser = JSON.parse(savedUser);
        showPage('app');
        initializeApp();
    } else {
        showPage('login');
    }
});

// 添加CSS动画
const style = document.createElement('style');
style.textContent = `
    @keyframes slideIn {
        from {
            transform: translateX(100%);
            opacity: 0;
        }
        to {
            transform: translateX(0);
            opacity: 1;
        }
    }

    @keyframes slideOut {
        from {
            transform: translateX(0);
            opacity: 1;
        }
        to {
            transform: translateX(100%);
            opacity: 0;
        }
    }
`;
document.head.appendChild(style);

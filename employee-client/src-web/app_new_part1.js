// TaskFleet Employee Client - JavaScript应用逻辑
// 使用Tauri的invoke API与Rust后端通信

const { invoke } = window.__TAURI__.core;

// ==================== 应用状态 ====================
const app = {
    currentUser: null,
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

function initializeApp() {
    // 显示用户信息
    document.getElementById('user-name').textContent = app.currentUser.full_name;

    // 加载初始数据
    loadTasks();
    checkActiveSession();

    // 开始定期刷新
    startAutoRefresh();
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

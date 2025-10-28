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

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// TaskFleet Employee Client
// 专注于员工任务管理的桌面客户端

mod taskfleet_models;
mod taskfleet_api;
mod taskfleet_commands;

use taskfleet_commands::*;

fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 服务器URL - 可以从配置文件读取或环境变量
    let server_url = std::env::var("TASKFLEET_SERVER_URL")
        .unwrap_or_else(|_| "http://localhost:8000".to_string());

    // 创建应用状态
    let app_state = AppState::new(server_url);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            // 认证命令
            login,
            logout,
            get_current_user,
            // 任务命令
            get_my_tasks,
            get_task,
            start_task,
            complete_task,
            cancel_task,
            // 工作记录命令
            get_active_work_session,
            create_work_log,
            get_my_work_logs,
            // 系统命令
            get_app_version,
            check_server_connection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

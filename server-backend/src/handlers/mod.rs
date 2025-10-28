pub mod auth;
pub mod docs;
pub mod health;
pub mod users;
// TODO: 暂时注释,等待类型迁移完成
// pub mod tasks;
pub mod tasks_temp;  // 临时任务端点（返回空数组，避免404）
// pub mod projects;
pub mod projects_temp;  // 临时项目端点（返回空数组，避免404）
pub mod statistics;
pub mod websocket;

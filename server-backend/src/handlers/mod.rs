pub mod auth;
pub mod docs;
pub mod health;
pub mod users;
pub mod company;
pub mod tasks;  // ✅ 启用真实的tasks handler
pub mod tasks_temp;  // 临时任务端点(返回空数组,避免404)
// pub mod projects;
pub mod projects_temp;  // 临时项目端点(返回空数组,避免404)
pub mod statistics;
pub mod websocket;


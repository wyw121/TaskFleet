use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
    Extension,
};
use futures::{stream::StreamExt, SinkExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::database::Database;
use crate::models::{Task, User};
use crate::Config;

type AppState = (Database, Config);

/// WebSocket事件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TaskEvent {
    /// 任务创建事件
    TaskCreated {
        task: Task,
    },
    /// 任务更新事件
    TaskUpdated {
        task: Task,
    },
    /// 任务分配事件
    TaskAssigned {
        task_id: Uuid,
        assigned_to: Uuid,
        assigned_to_name: String,
    },
    /// 任务完成事件
    TaskCompleted {
        task_id: Uuid,
        completed_by: Uuid,
        completed_by_name: String,
    },
    /// 任务取消事件
    TaskCancelled {
        task_id: Uuid,
        cancelled_by: Uuid,
    },
    /// 心跳消息
    Ping,
    /// 心跳响应
    Pong,
}

/// 全局事件广播器
pub type EventBroadcaster = Arc<broadcast::Sender<TaskEvent>>;

/// 创建事件广播器
pub fn create_event_broadcaster() -> EventBroadcaster {
    let (tx, _rx) = broadcast::channel(1000);
    Arc::new(tx)
}

/// WebSocket连接处理
/// GET /ws/task-updates
pub async fn task_updates_websocket(
    ws: WebSocketUpgrade,
    Extension(user): Extension<User>,
    State((db, _config)): State<AppState>,
    Extension(broadcaster): Extension<EventBroadcaster>,
) -> Response {
    tracing::info!("User {} connecting to WebSocket", user.username);
    
    ws.on_upgrade(move |socket| handle_socket(socket, user, db, broadcaster))
}

/// 处理WebSocket连接
async fn handle_socket(
    socket: WebSocket,
    user: User,
    _db: Database,
    broadcaster: EventBroadcaster,
) {
    let (mut sender, mut receiver) = socket.split();
    
    // 订阅广播频道
    let mut rx = broadcaster.subscribe();
    
    // 发送连接成功消息
    let welcome = TaskEvent::Ping;
    if let Ok(msg) = serde_json::to_string(&welcome) {
        let _ = sender.send(Message::Text(msg)).await;
    }
    
    // 接收任务:监听广播事件并发送给客户端
    let mut send_task = tokio::spawn(async move {
        while let Ok(event) = rx.recv().await {
            // 序列化事件并发送
            if let Ok(json) = serde_json::to_string(&event) {
                if sender.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        }
    });
    
    // 接收任务:处理客户端消息
    let user_id = user.id;
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    // 处理客户端发送的文本消息
                    tracing::debug!("Received message from user {}: {}", user_id, text);
                    
                    // 可以解析客户端消息并处理
                    if let Ok(event) = serde_json::from_str::<TaskEvent>(&text) {
                        match event {
                            TaskEvent::Ping => {
                                // 响应心跳
                                tracing::debug!("Ping received from user {}", user_id);
                            }
                            _ => {
                                tracing::debug!("Received event: {:?}", event);
                            }
                        }
                    }
                }
                Message::Close(_) => {
                    tracing::info!("User {} closed WebSocket connection", user_id);
                    break;
                }
                _ => {}
            }
        }
    });
    
    // 等待任一任务完成
    tokio::select! {
        _ = (&mut send_task) => {
            tracing::info!("Send task completed for user {}", user_id);
            recv_task.abort();
        },
        _ = (&mut recv_task) => {
            tracing::info!("Receive task completed for user {}", user_id);
            send_task.abort();
        },
    }
    
    tracing::info!("WebSocket connection closed for user {}", user_id);
}

/// 广播任务事件
pub async fn broadcast_event(broadcaster: &EventBroadcaster, event: TaskEvent) {
    if let Err(e) = broadcaster.send(event.clone()) {
        tracing::warn!("Failed to broadcast event: {:?}, error: {}", event, e);
    } else {
        tracing::debug!("Broadcasted event: {:?}", event);
    }
}

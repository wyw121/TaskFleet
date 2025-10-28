/**
 * TaskFleet - WebSocket实时通信服务
 */

/**
 * WebSocket事件类型
 */
export enum WebSocketEventType {
  TaskCreated = 'task_created',
  TaskUpdated = 'task_updated',
  TaskAssigned = 'task_assigned',
  TaskCompleted = 'task_completed',
  TaskCancelled = 'task_cancelled',
  Ping = 'ping',
  Pong = 'pong',
}

/**
 * WebSocket事件
 */
export interface WebSocketEvent {
  type: WebSocketEventType;
  task?: any;
  task_id?: string;
  assigned_to?: string;
  assigned_to_name?: string;
  completed_by?: string;
  completed_by_name?: string;
  cancelled_by?: string;
}

/**
 * WebSocket事件监听器
 */
type EventListener = (event: WebSocketEvent) => void;

/**
 * WebSocket服务类
 */
class WebSocketService {
  private ws: WebSocket | null = null;
  private listeners: Map<WebSocketEventType, EventListener[]> = new Map();
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private reconnectDelay = 3000;
  private heartbeatInterval: number | null = null;

  /**
   * 连接到WebSocket服务器
   */
  connect(token: string): void {
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const host = window.location.host;
    const url = `${protocol}//${host}/ws/task-updates?token=${token}`;

    try {
      this.ws = new WebSocket(url);

      this.ws.onopen = this.handleOpen.bind(this);
      this.ws.onmessage = this.handleMessage.bind(this);
      this.ws.onerror = this.handleError.bind(this);
      this.ws.onclose = this.handleClose.bind(this);
    } catch (error) {
      console.error('WebSocket connection error:', error);
      this.handleReconnect();
    }
  }

  /**
   * 断开连接
   */
  disconnect(): void {
    if (this.heartbeatInterval) {
      clearInterval(this.heartbeatInterval);
      this.heartbeatInterval = null;
    }

    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }

    this.listeners.clear();
  }

  /**
   * 监听事件
   */
  on(eventType: WebSocketEventType, listener: EventListener): void {
    if (!this.listeners.has(eventType)) {
      this.listeners.set(eventType, []);
    }
    this.listeners.get(eventType)!.push(listener);
  }

  /**
   * 取消监听事件
   */
  off(eventType: WebSocketEventType, listener: EventListener): void {
    const listeners = this.listeners.get(eventType);
    if (listeners) {
      const index = listeners.indexOf(listener);
      if (index > -1) {
        listeners.splice(index, 1);
      }
    }
  }

  /**
   * 发送消息
   */
  private send(event: WebSocketEvent): void {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(event));
    }
  }

  /**
   * 处理连接打开
   */
  private handleOpen(): void {
    console.log('WebSocket connected');
    this.reconnectAttempts = 0;

    // 启动心跳
    this.startHeartbeat();
  }

  /**
   * 处理消息接收
   */
  private handleMessage(event: MessageEvent): void {
    try {
      const data: WebSocketEvent = JSON.parse(event.data);
      this.notifyListeners(data);
    } catch (error) {
      console.error('Failed to parse WebSocket message:', error);
    }
  }

  /**
   * 处理错误
   */
  private handleError(error: Event): void {
    console.error('WebSocket error:', error);
  }

  /**
   * 处理连接关闭
   */
  private handleClose(): void {
    console.log('WebSocket disconnected');
    
    if (this.heartbeatInterval) {
      clearInterval(this.heartbeatInterval);
      this.heartbeatInterval = null;
    }

    this.handleReconnect();
  }

  /**
   * 处理重连
   */
  private handleReconnect(): void {
    if (this.reconnectAttempts < this.maxReconnectAttempts) {
      this.reconnectAttempts++;
      console.log(`Attempting to reconnect (${this.reconnectAttempts}/${this.maxReconnectAttempts})...`);
      
      setTimeout(() => {
        const token = localStorage.getItem('token');
        if (token) {
          this.connect(token);
        }
      }, this.reconnectDelay);
    } else {
      console.error('Max reconnect attempts reached');
    }
  }

  /**
   * 启动心跳
   */
  private startHeartbeat(): void {
    this.heartbeatInterval = window.setInterval(() => {
      this.send({ type: WebSocketEventType.Ping });
    }, 30000); // 每30秒发送一次心跳
  }

  /**
   * 通知监听器
   */
  private notifyListeners(event: WebSocketEvent): void {
    const listeners = this.listeners.get(event.type);
    if (listeners) {
      listeners.forEach(listener => listener(event));
    }
  }
}

export default new WebSocketService();

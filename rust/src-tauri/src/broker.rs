use dashmap::DashMap;
use serde::Serialize;
use tokio::sync::mpsc;
use uuid::Uuid;

/// SSE 事件消息
#[derive(Debug, Clone)]
pub struct SseEvent {
    pub event: String,
    pub data: String,
}

impl SseEvent {
    pub fn new(event: impl Into<String>, data: impl Serialize) -> Self {
        Self {
            event: event.into(),
            data: serde_json::to_string(&data).unwrap_or_else(|_| "{}".into()),
        }
    }
}

/// 轻量级 SSE 广播服务
pub struct SSEBroker {
    clients: DashMap<String, mpsc::UnboundedSender<SseEvent>>,
}

impl SSEBroker {
    pub fn new() -> Self {
        Self {
            clients: DashMap::new(),
        }
    }

    /// 注册新客户端，返回 (client_id, receiver)
    pub fn register(&self) -> (String, mpsc::UnboundedReceiver<SseEvent>) {
        let client_id = Uuid::new_v4().to_string();
        let (tx, rx) = mpsc::unbounded_channel();

        // 发送初始 hello 事件
        let hello = SseEvent::new(
            "hello",
            serde_json::json!({
                "timestamp": chrono::Utc::now().timestamp_millis() as f64 / 1000.0,
                "client_id": &client_id
            }),
        );
        let _ = tx.send(hello);

        self.clients.insert(client_id.clone(), tx);
        (client_id, rx)
    }

    /// 注销客户端
    pub fn unregister(&self, client_id: &str) {
        self.clients.remove(client_id);
    }

    /// 广播事件给所有客户端
    pub fn broadcast(&self, event: &str, data: impl Serialize) {
        let message = SseEvent::new(event, data);
        let mut stale = Vec::new();
        for entry in self.clients.iter() {
            let (id, tx) = entry.pair();
            if tx.send(message.clone()).is_err() {
                stale.push(id.clone());
            }
        }
        for id in stale {
            self.clients.remove(id.as_str());
        }
    }

    /// 发送事件给指定客户端
    pub fn send_to(&self, client_id: &str, event: &str, data: impl Serialize) -> bool {
        let message = SseEvent::new(event, data);
        if let Some(entry) = self.clients.get(client_id) {
            let tx = entry.value();
            if tx.send(message).is_err() {
                drop(entry);
                self.clients.remove(client_id);
                return false;
            }
            true
        } else {
            false
        }
    }

    /// 检查客户端是否仍在连接
    pub fn has_client(&self, client_id: &str) -> bool {
        self.clients.contains_key(client_id)
    }
}

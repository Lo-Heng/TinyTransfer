use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde::Serialize;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

use crate::utils::parse_user_agent;

/// 设备心跳超时时间（秒）
const DEVICE_INACTIVE_TIMEOUT: Duration = Duration::from_secs(15);
/// 独立清理任务扫描间隔（秒）
const CLEANUP_INTERVAL: Duration = Duration::from_secs(5);

#[derive(Debug, Clone)]
pub struct DeviceEntry {
    pub ip: String,
    pub sid: Option<String>,
    pub info: crate::utils::DeviceInfo,
    pub user_agent: String,
    pub last_ping: Instant,
}

impl DeviceEntry {
    pub fn new(ip: String, user_agent: String) -> Self {
        Self {
            ip: ip.clone(),
            sid: None,
            info: parse_user_agent(&user_agent),
            user_agent,
            last_ping: Instant::now(),
        }
    }

    pub fn update_sid(&mut self, sid: String) {
        self.sid = Some(sid);
    }

    pub fn update_ping(&mut self) {
        self.last_ping = Instant::now();
    }

    pub fn is_inactive(&self) -> bool {
        Instant::now().duration_since(self.last_ping) > DEVICE_INACTIVE_TIMEOUT
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DeviceSnapshot {
    pub ip: String,
    #[serde(rename = "type")]
    pub device_type: String,
    pub detail: String,
    pub model: String,
}

pub struct DeviceTracker {
    devices: RwLock<HashMap<String, DeviceEntry>>,
    on_change: Mutex<Option<Box<dyn Fn() + Send + Sync>>>,
    cleanup_handle: Mutex<Option<JoinHandle<()>>>,
}

impl DeviceTracker {
    pub fn new() -> Self {
        Self {
            devices: RwLock::new(HashMap::new()),
            on_change: Mutex::new(None),
            cleanup_handle: Mutex::new(None),
        }
    }

    pub fn set_on_change(&self, callback: Box<dyn Fn() + Send + Sync>) {
        let mut cb = self.on_change.lock().unwrap();
        *cb = Some(callback);
    }

    fn notify_change(&self) {
        let cb = self.on_change.lock().unwrap();
        if let Some(f) = cb.as_ref() {
            f();
        }
    }

    /// 启动后台清理任务；需通过 Arc<DeviceTracker> 调用。
    pub fn spawn_cleanup(self: Arc<Self>) {
        let mut handle = self.cleanup_handle.lock().unwrap();
        if handle.is_none() {
            let tracker = Arc::clone(&self);
            *handle = Some(tokio::spawn(async move {
                let mut interval = tokio::time::interval(CLEANUP_INTERVAL);
                interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
                loop {
                    interval.tick().await;
                    tracker.cleanup_inactive().await;
                }
            }));
        }
    }

    pub async fn register(&self, sid: &str, ip: &str, user_agent: &str) {
        {
            let mut devices = self.devices.write().await;
            let mut device = DeviceEntry::new(ip.to_string(), user_agent.to_string());
            device.update_sid(sid.to_string());
            devices.insert(sid.to_string(), device);
        }
        self.notify_change();
    }

    pub async fn unregister(&self, sid: &str) {
        let removed = {
            let mut devices = self.devices.write().await;
            devices.remove(sid).is_some()
        };
        if removed {
            self.notify_change();
        }
    }

    pub async fn update_ping(&self, sid: &str) {
        let mut devices = self.devices.write().await;
        if let Some(device) = devices.get_mut(sid) {
            device.update_ping();
        }
    }

    pub async fn get_devices_info(&self) -> Vec<DeviceSnapshot> {
        let devices = self.devices.read().await;
        let mut seen_ips = std::collections::HashSet::new();
        let mut result = Vec::new();

        for device in devices.values() {
            if seen_ips.insert(device.ip.clone()) {
                result.push(DeviceSnapshot {
                    ip: device.ip.clone(),
                    device_type: device.info.device_type.clone(),
                    detail: device.info.detail.clone(),
                    model: device.info.model.clone(),
                });
            }
        }

        result
    }

    pub async fn is_any_remote_connected(&self) -> bool {
        let devices = self.devices.read().await;
        devices
            .values()
            .any(|d| d.ip != "127.0.0.1" && d.ip != "::1")
    }

    pub async fn cleanup_inactive(&self) {
        let removed = {
            let mut devices = self.devices.write().await;
            let inactive_sids: Vec<String> = devices
                .iter()
                .filter(|(_, d)| d.is_inactive())
                .map(|(sid, _)| sid.clone())
                .collect();

            for sid in &inactive_sids {
                if let Some(device) = devices.remove(sid) {
                    println!("\n设备超时移除: {} - {}", device.ip, device.info.device_type);
                }
            }

            !inactive_sids.is_empty()
        };

        if removed {
            self.notify_change();
        }
    }
}

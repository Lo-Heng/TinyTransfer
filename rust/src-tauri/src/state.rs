use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::broker::SSEBroker;
use crate::device_tracker::DeviceTracker;
use crate::file_manager::FileManager;
use crate::security::LoginRateLimiter;

/// 全局共享状态
pub struct AppState {
    pub file_manager: Arc<FileManager>,
    pub device_tracker: Arc<DeviceTracker>,
    pub broker: Arc<SSEBroker>,
    pub upload_folder: PathBuf,
    pub share_folder: PathBuf,
    pub server_port: u16,
    pub access_password: Option<String>,
    /// 已认证的令牌集合（仅当设置了访问密码时有效）
    pub auth_tokens: Mutex<HashSet<String>>,
    pub rate_limiter: Arc<LoginRateLimiter>,
}

impl AppState {
    pub fn new() -> Self {
        let exe_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

        let upload_folder = exe_dir.join("uploads");
        let share_folder = exe_dir.join("share");

        let file_manager = Arc::new(FileManager::new(
            upload_folder.to_string_lossy().to_string(),
            share_folder.to_string_lossy().to_string(),
        ));

        let broker = Arc::new(SSEBroker::new());
        let device_tracker = Arc::new(DeviceTracker::new());
        let rate_limiter = Arc::new(LoginRateLimiter::new());

        Self {
            file_manager,
            device_tracker,
            broker,
            upload_folder,
            share_folder,
            server_port: 5000,
            access_password: None,
            auth_tokens: Mutex::new(HashSet::new()),
            rate_limiter,
        }
    }
}

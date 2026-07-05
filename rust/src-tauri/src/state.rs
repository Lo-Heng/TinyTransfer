use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use tokio::sync::watch;

use crate::broker::SseBroker;
use crate::device_tracker::DeviceTracker;
use crate::file_manager::FileManager;
use crate::security::LoginRateLimiter;

/// 全局共享状态
pub struct AppState {
    pub file_manager: Arc<FileManager>,
    pub device_tracker: Arc<DeviceTracker>,
    pub broker: Arc<SseBroker>,
    pub uploads_folder: PathBuf,
    pub server_port: u16,
    pub access_password: Option<String>,
    /// 已认证的令牌集合（仅当设置了访问密码时有效）
    pub auth_tokens: Mutex<HashSet<String>>,
    pub rate_limiter: Arc<LoginRateLimiter>,
    /// 标题栏颜色设置通道（true=深色, false=浅色）
    pub titlebar_color_tx: watch::Sender<Option<bool>>,
}

impl AppState {
    pub fn new() -> Self {
        let exe_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

        let uploads_folder = exe_dir.join("uploads");

        let file_manager = Arc::new(FileManager::new(
            uploads_folder.to_string_lossy().to_string(),
        ));

        let broker = Arc::new(SseBroker::new());
        let device_tracker = Arc::new(DeviceTracker::new());
        let rate_limiter = Arc::new(LoginRateLimiter::new());
        let (titlebar_color_tx, _) = watch::channel(None);

        Self {
            file_manager,
            device_tracker,
            broker,
            uploads_folder,
            server_port: 5000,
            access_password: None,
            auth_tokens: Mutex::new(HashSet::new()),
            rate_limiter,
            titlebar_color_tx,
        }
    }
}

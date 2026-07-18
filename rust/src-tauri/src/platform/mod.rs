// 平台抽象层：隔离 Windows/macOS/Linux 差异

pub mod config;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
mod unix;

/// 平台操作 trait —— 跨平台行为收敛于此
pub trait PlatformOps: Send + Sync {
    /// 设置窗口标题栏颜色（Windows 用 DWM API，其他平台 no-op）
    fn set_titlebar_color(&self, window: &tauri::WebviewWindow, is_dark: bool) -> Result<(), String>;

    /// 确保单实例运行。返回 true=可继续，false=已有实例应退出
    fn ensure_single_instance(&self) -> bool;
}

#[cfg(target_os = "windows")]
pub fn current() -> Box<dyn PlatformOps> {
    Box::new(windows::WindowsPlatform)
}

#[cfg(target_os = "macos")]
pub fn current() -> Box<dyn PlatformOps> {
    Box::new(macos::MacosPlatform)
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn current() -> Box<dyn PlatformOps> {
    Box::new(unix::UnixPlatform)
}

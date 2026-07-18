// Linux/其他平台实现：no-op fallback

use tauri::WebviewWindow;

pub struct UnixPlatform;

impl super::PlatformOps for UnixPlatform {
    fn set_titlebar_color(&self, _window: &WebviewWindow, _is_dark: bool) -> Result<(), String> {
        Ok(())
    }

    fn ensure_single_instance(&self) -> bool {
        true
    }
}

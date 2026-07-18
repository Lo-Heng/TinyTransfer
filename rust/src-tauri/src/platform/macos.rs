// macOS 平台实现：标题栏颜色由系统主题管理，单实例由应用激活机制处理

use tauri::WebviewWindow;

pub struct MacosPlatform;

impl super::PlatformOps for MacosPlatform {
    fn set_titlebar_color(&self, _window: &WebviewWindow, _is_dark: bool) -> Result<(), String> {
        // macOS 标题栏由系统主题自动管理，无需手动设置
        Ok(())
    }

    fn ensure_single_instance(&self) -> bool {
        // macOS 单实例由应用激活机制（NSApplication delegate）处理
        true
    }
}

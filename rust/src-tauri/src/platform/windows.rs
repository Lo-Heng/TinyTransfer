// Windows 平台实现：DWM 标题栏颜色 + CreateMutexW 单实例

use crate::platform::config::WINDOW_CONFIG;
use raw_window_handle::HasWindowHandle;
use tauri::WebviewWindow;

pub struct WindowsPlatform;

impl super::PlatformOps for WindowsPlatform {
    fn set_titlebar_color(&self, window: &WebviewWindow, is_dark: bool) -> Result<(), String> {
        use windows_sys::Win32::Foundation::HWND;
        use windows_sys::Win32::Graphics::Dwm::{
            DwmSetWindowAttribute, DWMWA_CAPTION_COLOR, DWMWA_TEXT_COLOR,
        };

        let handle = window
            .window_handle()
            .map_err(|e| format!("获取窗口句柄失败: {e}"))?;
        let raw_window_handle::RawWindowHandle::Win32(win_handle) = handle.as_ref() else {
            return Err("非 Win32 窗口句柄".into());
        };
        let hwnd: HWND = win_handle.hwnd.get() as isize as _;

        let caption_color: u32 = if is_dark {
            WINDOW_CONFIG.dark_caption_color
        } else {
            WINDOW_CONFIG.light_caption_color
        };
        let text_color: u32 = if is_dark {
            WINDOW_CONFIG.dark_text_color
        } else {
            WINDOW_CONFIG.light_text_color
        };

        unsafe {
            let _ = DwmSetWindowAttribute(
                hwnd,
                DWMWA_CAPTION_COLOR as u32,
                &caption_color as *const _ as _,
                std::mem::size_of::<u32>() as u32,
            );
            let _ = DwmSetWindowAttribute(
                hwnd,
                DWMWA_TEXT_COLOR as u32,
                &text_color as *const _ as _,
                std::mem::size_of::<u32>() as u32,
            );
        }
        Ok(())
    }

    fn ensure_single_instance(&self) -> bool {
        use windows_sys::Win32::Foundation::GetLastError;
        use windows_sys::Win32::System::Threading::CreateMutexW;

        // ERROR_ALREADY_EXISTS = 183
        const ERROR_ALREADY_EXISTS: u32 = 183;
        let mutex_name: Vec<u16> = "TinyTransfer_SingleInstance\0"
            .encode_utf16()
            .collect();

        unsafe {
            let handle = CreateMutexW(std::ptr::null(), 1, mutex_name.as_ptr());
            if handle.is_null() {
                return false;
            }
            if GetLastError() == ERROR_ALREADY_EXISTS {
                return false;
            }
            // handle 是 isize（Copy 类型），无需 forget。
            // mutex 由 OS 持有，进程退出前不会释放。
            let _ = handle;
        }
        true
    }
}

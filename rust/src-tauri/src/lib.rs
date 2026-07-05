#[cfg_attr(mobile, tauri::mobile_entry_point)]

mod broker;
mod device_tracker;
mod file_manager;
mod routes;
mod security;
mod server;
mod state;
mod utils;

use std::sync::Arc;

use open;
use raw_window_handle::HasWindowHandle;
use serde_json::json;
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn save_file(path: String, data: Vec<u8>) -> Result<(), String> {
    let path = std::path::Path::new(&path);
    println!("[save_file] 开始保存文件: {:?}", path);
    println!("[save_file] 数据大小: {} bytes", data.len());
    
    // 检查并创建父目录
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            println!("[save_file] 创建父目录: {:?}", parent);
            std::fs::create_dir_all(parent).map_err(|e| {
                let err = format!("创建目录失败: {} (路径: {:?})", e, parent);
                println!("[save_file] ERROR: {}", err);
                err
            })?;
        }
    }
    
    // 写入文件
    std::fs::write(path, &data).map_err(|e| {
        let err = format!("写入文件失败: {} (路径: {:?})", e, path);
        println!("[save_file] ERROR: {}", err);
        err
    })?;
    
    // 验证文件确实写入成功
    let metadata = std::fs::metadata(path).map_err(|e| {
        let err = format!("写入后验证失败，无法读取文件元数据: {} (路径: {:?})", e, path);
        println!("[save_file] ERROR: {}", err);
        err
    })?;
    
    println!("[save_file] 文件保存成功! 路径: {:?}, 实际大小: {} bytes", path, metadata.len());
    
    if metadata.len() != data.len() as u64 {
        let err = format!("文件大小不匹配: 预期 {} bytes, 实际 {} bytes", data.len(), metadata.len());
        println!("[save_file] WARNING: {}", err);
    }
    
    Ok(())
}

#[tauri::command]
async fn save_file_dialog(
    window: tauri::WebviewWindow,
    filename: String,
    data: Vec<u8>,
) -> Result<Option<String>, String> {
    println!("[save_file_dialog] =========================================");
    println!("[save_file_dialog] 命令被调用");
    println!("[save_file_dialog] 文件名: {}", filename);
    println!("[save_file_dialog] 数据大小: {} bytes", data.len());
    
    // 使用 channel 等待对话框结果
    let (tx, rx) = tokio::sync::oneshot::channel::<Option<std::path::PathBuf>>();
    
    println!("[save_file_dialog] 准备弹出保存对话框...");
    
    let dialog = window.dialog();
    dialog
        .file()
        .set_title("保存文件")
        .set_file_name(&filename)
        .save_file(move |path| {
            println!("[save_file_dialog] 对话框回调触发，path.is_some: {}", path.is_some());
            let path_buf = path.and_then(|p| p.into_path().ok());
            println!("[save_file_dialog] 转换后路径: {:?}", path_buf);
            let send_result = tx.send(path_buf);
            println!("[save_file_dialog] 发送到channel结果: {:?}", send_result.is_ok());
        });
    
    println!("[save_file_dialog] 对话框已弹出，等待用户选择...");
    
    let save_path = rx.await.map_err(|e| {
        let err = format!("对话框等待失败: {}", e);
        println!("[save_file_dialog] ERROR: {}", err);
        err
    })?;
    
    let save_path = match save_path {
        Some(p) => p,
        None => {
            println!("[save_file_dialog] 用户取消保存或未选择路径");
            return Ok(None);
        }
    };
    
    let path_str = save_path.to_string_lossy().to_string();
    println!("[save_file_dialog] 用户选择路径: {}", path_str);
    
    // 检查并创建父目录
    if let Some(parent) = save_path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            println!("[save_file_dialog] 创建父目录: {:?}", parent);
            std::fs::create_dir_all(parent).map_err(|e| {
                let err = format!("创建目录失败: {}", e);
                println!("[save_file_dialog] ERROR: {}", err);
                err
            })?;
        }
    }
    
    println!("[save_file_dialog] 开始写入文件...");
    
    // 写入文件
    std::fs::write(&save_path, &data).map_err(|e| {
        let err = format!("写入文件失败: {}", e);
        println!("[save_file_dialog] ERROR: {}", err);
        err
    })?;
    
    println!("[save_file_dialog] 文件写入完成");
    
    // 验证文件
    match std::fs::metadata(&save_path) {
        Ok(meta) => {
            println!("[save_file_dialog] ✓ 文件保存成功! 大小: {} bytes", meta.len());
            if meta.len() != data.len() as u64 {
                println!("[save_file_dialog] WARNING: 文件大小不匹配");
            }
        }
        Err(e) => {
            println!("[save_file_dialog] WARNING: 无法验证文件: {}", e);
        }
    }
    
    println!("[save_file_dialog] =========================================");
    Ok(Some(path_str))
}

#[tauri::command]
fn open_containing_folder(file_path: String) -> Result<(), String> {
    let path = std::path::Path::new(&file_path);
    let folder = if path.is_dir() {
        path
    } else {
        path.parent().unwrap_or(path)
    };
    println!("[open_containing_folder] 打开文件夹: {:?}", folder);
    open::that(folder).map_err(|e| format!("打开文件夹失败: {}", e))
}

/// 用系统默认播放器打开文件（绕过浏览器无法播放的限制）
#[tauri::command]
fn open_file_with_player(filename: String) -> Result<(), String> {
    // uploads 文件夹在 exe 同级目录下
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")));
    let upload_dir = exe_dir.join("uploads");
    let path = upload_dir.join(&filename);
    
    if !path.exists() {
        return Err(format!("找不到文件: {}", filename));
    }
    println!("[open_file_with_player] 用系统播放器打开: {:?}", path);
    open::that(&path).map_err(|e| format!("打开失败: {}", e))
}

#[cfg(target_os = "windows")]
#[tauri::command]
fn set_titlebar_color(window: tauri::WebviewWindow, is_dark: bool) -> Result<(), String> {
    use windows_sys::Win32::Graphics::Dwm::{
        DwmSetWindowAttribute, DWMWA_CAPTION_COLOR, DWMWA_TEXT_COLOR,
    };
    use windows_sys::Win32::Foundation::HWND;

    if let Ok(handle) = window.window_handle() {
        if let raw_window_handle::RawWindowHandle::Win32(win_handle) = handle.as_ref() {
            let hwnd: HWND = win_handle.hwnd.get() as isize as _;

            // 与页面底色保持一致：白天 #FBFBFA，黑夜 #0F0F0F
            let caption_color: u32 = if is_dark {
                0x000F0F0F  // #0F0F0F 黑暗模式背景
            } else {
                0x00FAFBFB  // #FBFBFA 明亮模式背景
            };

            // Text color: COLORREF format - light text for dark bg, dark text for light bg
            let text_color: u32 = if is_dark {
                0x00FFFFFF  // White text
            } else {
                0x00000000  // Black text
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

            println!("[set_titlebar_color] Set to {} mode", if is_dark { "dark" } else { "light" });
            return Ok(());
        }
    }
    Err("无法获取窗口句柄".to_string())
}

#[cfg(not(target_os = "windows"))]
#[tauri::command]
fn set_titlebar_color(_window: tauri::WebviewWindow, _is_dark: bool) -> Result<(), String> {
    // Non-Windows platforms: titlebar color is handled by system/theme
    Ok(())
}

/// Tauri 命令：从共享文件夹读取文件，弹出保存对话框让用户选择保存位置
/// 文件数据直接在 Rust 侧读写，不经过 WebView，适合大文件
#[tauri::command]
async fn download_file_dialog(
    app: tauri::AppHandle,
    window: tauri::WebviewWindow,
    filename: String,
) -> Result<Option<String>, String> {
    let state = app.state::<Arc<state::AppState>>();
    let file_path = state.file_manager.find_file_path(&filename)
        .ok_or_else(|| format!("文件不存在: {}", filename))?;

    let data = std::fs::read(&file_path)
        .map_err(|e| format!("读取文件失败: {}", e))?;

    println!("[download_file_dialog] 读取文件: {} ({} bytes)", filename, data.len());

    // 弹出保存对话框
    let (tx, rx) = tokio::sync::oneshot::channel::<Option<std::path::PathBuf>>();
    window.dialog()
        .file()
        .set_title("保存文件")
        .set_file_name(&filename)
        .save_file(move |path| {
            let _ = tx.send(path.and_then(|p| p.into_path().ok()));
        });

    let save_path = rx.await.map_err(|e| format!("对话框等待失败: {}", e))?;
    let save_path = match save_path {
        Some(p) => p,
        None => {
            println!("[download_file_dialog] 用户取消保存");
            return Ok(None);
        }
    };

    let path_str = save_path.to_string_lossy().to_string();
    println!("[download_file_dialog] 保存路径: {}", path_str);

    // 确保父目录存在
    if let Some(parent) = save_path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("创建目录失败: {}", e))?;
        }
    }

    // 写文件
    std::fs::write(&save_path, &data)
        .map_err(|e| format!("写入文件失败: {}", e))?;

    println!("[download_file_dialog] ✓ 保存成功: {}", path_str);
    Ok(Some(path_str))
}

#[tokio::main]
pub async fn run() {
    let state = Arc::new(state::AppState::new());

    // 设备列表变化时广播 device_list 事件
    {
        let broker = Arc::clone(&state.broker);
        let device_tracker = Arc::clone(&state.device_tracker);
        state.device_tracker.set_on_change(Box::new(move || {
            let broker = Arc::clone(&broker);
            let tracker = Arc::clone(&device_tracker);
            tokio::spawn(async move {
                let devices = tracker.get_devices_info().await;
                broker.broadcast(
                    "device_list",
                    json!({
                        "connected": !devices.is_empty(),
                        "devices": devices,
                    }),
                );
            });
        }));
    }

    // 启动设备超时清理任务
    Arc::clone(&state.device_tracker).spawn_cleanup();

    // 优雅关闭通道
    let (shutdown_tx, shutdown_rx) = tokio::sync::broadcast::channel(1);

    // 在独立 tokio 任务中启动 HTTP 服务
    let server_state = Arc::clone(&state);
    let server_handle = tokio::spawn(async move {
        if let Err(e) = server::start_server(server_state, shutdown_rx).await {
            eprintln!("[HTTP] Server error: {e}");
        }
    });

    // 等待 HTTP 服务器就绪（最多 3 秒）
    {
        let addr: std::net::SocketAddr = "127.0.0.1:5000".parse().unwrap();
        for _ in 0..30 {
            if std::net::TcpStream::connect(addr).is_ok() {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }

    // 加载高分辨率窗口图标（用于任务栏 + 窗口左上角）
    let window_icon = {
        let png_bytes = include_bytes!("../icons/128x128@2x.png");
        match image::load_from_memory(png_bytes) {
            Ok(img) => {
                let rgba = img.to_rgba8();
                let (w, h) = (rgba.width(), rgba.height());
                Some(tauri::image::Image::new_owned(rgba.into_raw(), w, h))
            }
            Err(e) => {
                eprintln!("[Icon] Failed to load window icon: {e}");
                None
            }
        }
    };

    // 构建 Tauri 应用（窗口由 tauri.conf.json 的 app.windows 配置自动创建）
    let state_clone = Arc::clone(&state);
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
            }
        }))
        .invoke_handler(tauri::generate_handler![greet, save_file, save_file_dialog, open_containing_folder, set_titlebar_color, open_file_with_player, download_file_dialog])
        .setup(move |app| {
            // 将 AppState 注册为 Tauri 托管状态，供 Tauri 命令访问
            app.manage(Arc::clone(&state_clone));
            // 拿到自动创建的窗口，设置 URL、图标并显示
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.navigate("http://127.0.0.1:5000/".parse::<url::Url>().unwrap());
                if let Some(ref icon) = window_icon {
                    let _ = window.set_icon(icon.clone());
                }
                let _ = window.show();
                
                // 启动标题栏颜色监听任务
                let window_clone = window.clone();
                let mut rx = state_clone.titlebar_color_tx.subscribe();
                tokio::spawn(async move {
                    while rx.changed().await.is_ok() {
                        if let Some(is_dark) = *rx.borrow() {
                            println!("[titlebar] 收到标题栏颜色设置: is_dark={}", is_dark);
                            let win = window_clone.clone();
                            // 在 Windows 上通过 DWM API 设置标题栏颜色
                            #[cfg(target_os = "windows")]
                            {
                                use raw_window_handle::HasWindowHandle;
                                use windows_sys::Win32::Graphics::Dwm::{
                                    DwmSetWindowAttribute, DWMWA_CAPTION_COLOR, DWMWA_TEXT_COLOR,
                                };
                                use windows_sys::Win32::Foundation::HWND;

                                if let Ok(handle) = win.window_handle() {
                                    if let raw_window_handle::RawWindowHandle::Win32(win_handle) = handle.as_ref() {
                                        let hwnd: HWND = win_handle.hwnd.get() as isize as _;
                                        let caption_color: u32 = if is_dark {
                                            0x000F0F0F
                                        } else {
                                            0x00FAFBFB
                                        };
                                        let text_color: u32 = if is_dark {
                                            0x00FFFFFF
                                        } else {
                                            0x00000000
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
                                        println!("[titlebar] 标题栏颜色已设置: is_dark={}", is_dark);
                                    }
                                }
                            }
                        }
                    }
                });
                
                // 调试信息：页面内有调试面板，右下角🔧按钮可查看
                println!("[DEV] 程序启动，页面右下角有调试面板按钮");
            }
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    // 运行 Tauri 应用（阻塞直到应用退出）
    app.run(move |_app_handle, event| {
        if let tauri::RunEvent::Exit = event {
            let _ = shutdown_tx.send(());
        }
    });

    if let Err(e) = server_handle.await {
        eprintln!("[HTTP] Server task panicked: {e}");
    }
}

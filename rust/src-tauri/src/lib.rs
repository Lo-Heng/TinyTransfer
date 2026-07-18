#[cfg_attr(mobile, tauri::mobile_entry_point)]

mod broker;
mod commands;
mod device_tracker;
mod file_manager;
mod platform;
mod routes;
mod security;
mod server;
mod state;
mod utils;

use std::sync::Arc;

use serde_json::json;
use tauri::Manager;

/// 调试日志宏：仅在 debug 模式输出到 stderr，release 不执行
macro_rules! debug_log {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        eprintln!($($arg)*)
    };
}
#[allow(unused_imports)]
pub(crate) use debug_log;

#[tokio::main]
pub async fn run() {
    // 单实例检测：已有实例运行则立即退出
    if !platform::current().ensure_single_instance() {
        debug_log!("[init] 已有实例运行，退出");
        std::process::exit(0);
    }

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
        let png_bytes = include_bytes!("../icons/windows/128x128@2x.png");
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
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::save_file_dialog,
            commands::open_containing_folder,
            commands::open_file_with_player,
            commands::download_file_dialog,
        ])
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
                            debug_log!("[titlebar] 收到标题栏颜色设置: is_dark={}", is_dark);
                            let _ = platform::current().set_titlebar_color(&window_clone, is_dark);
                        }
                    }
                });

                debug_log!("[DEV] 程序启动，页面右下角有调试面板按钮");
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

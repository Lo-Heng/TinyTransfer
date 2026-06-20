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

use serde_json::json;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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

    // 构建 Tauri 应用
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
            }
        }))
        .invoke_handler(tauri::generate_handler![greet])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    // 服务器就绪后，创建窗口并加载 HTTP URL
    let window = tauri::WebviewWindowBuilder::new(
        &app,
        "main",
        tauri::WebviewUrl::External("http://127.0.0.1:5000/".parse().unwrap()),
    )
    .title("SlimTransfer")
    .inner_size(1100.0, 780.0)
    .min_inner_size(800.0, 600.0)
    .build()
    .expect("failed to create window");

    // 可选：打开 devtools（调试用）
    // window.open_devtools();

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

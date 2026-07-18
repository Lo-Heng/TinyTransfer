// Tauri 自定义命令：文件保存对话框、打开文件夹、系统播放器打开
// 从 lib.rs 抽离，lib.rs 只保留 run() 主流程

use std::sync::Arc;

use tauri::Manager;
use tauri_plugin_dialog::DialogExt;

use crate::debug_log;
use crate::state;

/// 弹出保存对话框，让用户选择保存位置后写入文件
#[tauri::command]
pub async fn save_file_dialog(
    window: tauri::WebviewWindow,
    filename: String,
    data: Vec<u8>,
) -> Result<Option<String>, String> {
    debug_log!("[save_file_dialog] 文件名: {}, 数据: {} bytes", filename, data.len());

    let (tx, rx) = tokio::sync::oneshot::channel::<Option<std::path::PathBuf>>();

    window
        .dialog()
        .file()
        .set_title("保存文件")
        .set_file_name(&filename)
        .save_file(move |path| {
            let _ = tx.send(path.and_then(|p| p.into_path().ok()));
        });

    let save_path = rx.await.map_err(|e| format!("对话框等待失败: {e}"))?;
    let save_path = match save_path {
        Some(p) => p,
        None => {
            debug_log!("[save_file_dialog] 用户取消保存");
            return Ok(None);
        }
    };

    let path_str = save_path.to_string_lossy().to_string();
    debug_log!("[save_file_dialog] 保存路径: {}", path_str);

    if let Some(parent) = save_path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            std::fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {e}"))?;
        }
    }

    std::fs::write(&save_path, &data).map_err(|e| format!("写入文件失败: {e}"))?;
    debug_log!("[save_file_dialog] 保存成功: {}", path_str);
    Ok(Some(path_str))
}

/// 在系统文件管理器中打开指定文件夹
#[tauri::command]
pub fn open_containing_folder(file_path: String) -> Result<(), String> {
    let path = std::path::Path::new(&file_path);
    let folder = if path.is_dir() {
        path
    } else {
        path.parent().unwrap_or(path)
    };
    debug_log!("[open_containing_folder] 打开文件夹: {:?}", folder);
    open::that(folder).map_err(|e| format!("打开文件夹失败: {e}"))
}

/// 用系统默认播放器打开文件（绕过浏览器无法播放的限制）
#[tauri::command]
pub fn open_file_with_player(filename: String) -> Result<(), String> {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| {
            std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
        });
    let upload_dir = exe_dir.join("uploads");
    let path = upload_dir.join(&filename);

    if !path.exists() {
        return Err(format!("找不到文件: {filename}"));
    }
    debug_log!("[open_file_with_player] 用系统播放器打开: {:?}", path);
    open::that(&path).map_err(|e| format!("打开失败: {e}"))
}

/// 从共享文件夹读取文件，弹出保存对话框让用户选择保存位置
/// 文件数据直接在 Rust 侧读写，不经过 WebView，适合大文件
#[tauri::command]
pub async fn download_file_dialog(
    app: tauri::AppHandle,
    window: tauri::WebviewWindow,
    filename: String,
) -> Result<Option<String>, String> {
    let state = app.state::<Arc<state::AppState>>();
    let file_path = state
        .file_manager
        .find_file_path(&filename)
        .ok_or_else(|| format!("文件不存在: {filename}"))?;

    let data = std::fs::read(&file_path).map_err(|e| format!("读取文件失败: {e}"))?;

    debug_log!(
        "[download_file_dialog] 读取文件: {} ({} bytes)",
        filename,
        data.len()
    );

    let (tx, rx) = tokio::sync::oneshot::channel::<Option<std::path::PathBuf>>();
    window
        .dialog()
        .file()
        .set_title("保存文件")
        .set_file_name(&filename)
        .save_file(move |path| {
            let _ = tx.send(path.and_then(|p| p.into_path().ok()));
        });

    let save_path = rx.await.map_err(|e| format!("对话框等待失败: {e}"))?;
    let save_path = match save_path {
        Some(p) => p,
        None => {
            debug_log!("[download_file_dialog] 用户取消保存");
            return Ok(None);
        }
    };

    let path_str = save_path.to_string_lossy().to_string();
    debug_log!("[download_file_dialog] 保存路径: {}", path_str);

    if let Some(parent) = save_path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            std::fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {e}"))?;
        }
    }

    std::fs::write(&save_path, &data).map_err(|e| format!("写入文件失败: {e}"))?;
    debug_log!("[download_file_dialog] 保存成功: {}", path_str);
    Ok(Some(path_str))
}

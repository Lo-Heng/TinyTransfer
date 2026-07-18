use std::convert::Infallible;
use std::io::Write;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use axum::{
    body::Body,
    extract::{ConnectInfo, Form, Multipart, Path, Query, State},
    http::{header, HeaderMap, HeaderValue, Request, StatusCode},
    response::{Html, IntoResponse, Response},
    Json,
};
use axum::response::sse::{Event, KeepAlive, Sse};
use futures::stream::{self, Stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::time;
use tokio_stream::wrappers::{IntervalStream, UnboundedReceiverStream};
use tower::ServiceExt;
use tower_http::services::fs::ServeFile;

use crate::state::AppState;

// ============================================================
// 请求 / 响应结构体
// ============================================================

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CheckAuthResponse {
    pub authenticated: bool,
    pub has_password: bool,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct DevicesResponse {
    pub connected: bool,
    pub devices: Vec<crate::device_tracker::DeviceSnapshot>,
}

#[derive(Debug, Serialize)]
pub struct IpResponse {
    pub ip: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteFilesRequest {
    pub filenames: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct DeleteFilesResponse {
    pub success: bool,
    pub deleted: usize,
}

#[derive(Debug, Deserialize)]
pub struct DownloadZipRequest {
    pub filenames: Vec<String>,
}

#[derive(Deserialize)]
pub struct SetTitlebarRequest {
    pub is_dark: bool,
}

#[derive(Debug, Serialize)]
pub struct OpenFolderResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EventsQuery {
    #[serde(rename = "ua")]
    pub user_agent: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PingForm {
    pub client_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PingResponse {
    pub ok: bool,
}

// ============================================================
// 认证
// ============================================================

pub async fn check_auth(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let ip = addr.ip().to_string();
    let local_ip = crate::utils::get_local_ip(300);
    let is_host = ip == "127.0.0.1" || ip == "::1" || ip == local_ip;

    let has_password = state.access_password.is_some();
    let authenticated = if !has_password {
        true
    } else {
        headers
            .get("X-Auth-Token")
            .and_then(|v| v.to_str().ok())
            .map(|token| state.auth_tokens.lock().unwrap().contains(token))
            .unwrap_or(false)
    };

    Json(CheckAuthResponse {
        authenticated,
        has_password,
        role: if is_host { "host".into() } else { "guest".into() },
    })
}

pub async fn post_auth(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(req): Json<AuthRequest>,
) -> impl IntoResponse {
    let ip = addr.ip().to_string();

    if state.rate_limiter.is_locked(&ip) {
        let remaining = state.rate_limiter.get_remaining_lockout(&ip);
        return (
            StatusCode::TOO_MANY_REQUESTS,
            Json(AuthResponse {
                success: false,
                token: None,
                error: Some(format!(
                    "尝试次数过多，请在 {remaining} 秒后重试"
                )),
            }),
        );
    }

    let stored = state.access_password.as_deref().unwrap_or("");
    let valid = if state.access_password.is_none() {
        true
    } else {
        use subtle::ConstantTimeEq;
        req.password.as_bytes().ct_eq(stored.as_bytes()).unwrap_u8() == 1
    };

    if valid {
        state.rate_limiter.record_success(&ip);
        let token = generate_token();
        state.auth_tokens.lock().unwrap().insert(token.clone());
        (
            StatusCode::OK,
            Json(AuthResponse {
                success: true,
                token: Some(token),
                error: None,
            }),
        )
    } else {
        state.rate_limiter.record_failure(&ip);
        (
            StatusCode::UNAUTHORIZED,
            Json(AuthResponse {
                success: false,
                token: None,
                error: Some("密码错误".into()),
            }),
        )
    }
}

fn generate_token() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    (0..32)
        .map(|_| CHARSET[rng.gen_range(0..CHARSET.len())] as char)
        .collect()
}

// ============================================================
// 设备
// ============================================================

pub async fn get_devices(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let devices = state.device_tracker.get_devices_info().await;
    Json(DevicesResponse {
        connected: !devices.is_empty(),
        devices,
    })
}

pub async fn get_ip(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let ip = crate::utils::get_local_ip(300);
    Json(IpResponse {
        ip: ip.clone(),
        url: format!("http://{ip}:{}", state.server_port),
    })
}

// ============================================================
// 文件列表
// ============================================================

pub async fn list_files(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(state.file_manager.list_files())
}

pub async fn list_uploaded_files(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(state.file_manager.list_files())
}

pub async fn list_all_files(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(state.file_manager.list_files())
}

// ============================================================
// 文件下载 / 上传 / 删除
// ============================================================

pub async fn download_file(
    State(state): State<Arc<AppState>>,
    Path(filename): Path<String>,
) -> Result<Response, (StatusCode, String)> {
    match state.file_manager.find_file_path(&filename) {
        Some(path) => {
            let response = ServeFile::new(path)
                .oneshot(Request::new(Body::empty()))
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let mut response = response;
            response
                .headers_mut()
                .insert(header::CONTENT_DISPOSITION, content_disposition_attachment(&filename));
            Ok(response.into_response())
        }
        None => Err((StatusCode::NOT_FOUND, "File not found".into())),
    }
}

fn content_disposition_attachment(filename: &str) -> HeaderValue {
    if filename.is_ascii() {
        HeaderValue::from_str(&format!(r#"attachment; filename="{}""#, filename)).unwrap()
    } else {
        let encoded: String = filename
            .bytes()
            .map(|b| {
                if b.is_ascii_alphanumeric() || b == b'-' || b == b'.' || b == b'_' {
                    char::from(b).to_string()
                } else {
                    format!("%{:02X}", b)
                }
            })
            .collect();
        HeaderValue::from_str(&format!("attachment; filename*=UTF-8''{}", encoded)).unwrap()
    }
}

pub async fn upload_file(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let ua = headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let device_type = crate::utils::parse_user_agent(ua).device_type;

    while let Ok(Some(mut field)) = multipart.next_field().await {
        if field.name() == Some("file") {
            if let Some(filename) = field.file_name().map(|s| s.to_string()) {
                let safe = crate::utils::secure_filename(&filename);
                // 测速文件不落盘
                if safe.starts_with("speed-test") {
                    return Json(json!({"success": true, "filename": safe}));
                }
                let file_path = state.file_manager.unique_filepath(&safe);
                let saved_name = file_path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| safe.clone());

                // 流式写入磁盘，不把整个文件读入内存
                let mut file = match tokio::fs::File::create(&file_path).await {
                    Ok(f) => f,
                    Err(e) => return Json(json!({"success": false, "error": format!("创建文件失败: {e}")})),
                };
                loop {
                    match field.next().await {
                        Some(Ok(chunk)) => {
                            if let Err(e) = tokio::io::AsyncWriteExt::write_all(&mut file, &chunk).await {
                                let _ = tokio::fs::remove_file(&file_path).await;
                                return Json(json!({"success": false, "error": format!("写入失败: {e}")}));
                            }
                        }
                        Some(Err(e)) => {
                            let _ = tokio::fs::remove_file(&file_path).await;
                            return Json(json!({"success": false, "error": format!("读取数据失败: {e}")}));
                        }
                        None => break,
                    }
                }
                if let Err(e) = file.sync_all().await {
                    let _ = tokio::fs::remove_file(&file_path).await;
                    return Json(json!({"success": false, "error": format!("同步文件失败: {e}")}));
                }
                state.file_manager.save_upload_meta(&saved_name, &device_type);
                // 通知所有端文件列表已更新
                state.broker.broadcast("file_list_updated", json!({"action": "upload"}));
                return Json(json!({"success": true, "filename": saved_name}));
            }
        }
    }

    Json(json!({
        "success": false,
        "error": "未提供文件"
    }))
}

pub async fn upload_chunk(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut file_id: Option<String> = None;
    let mut chunk_index: Option<usize> = None;
    let mut total_chunks: Option<usize> = None;
    let mut filename: Option<String> = None;
    let mut chunk_data: Option<Vec<u8>> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        match field.name() {
            Some("fileId") => file_id = field.text().await.ok(),
            Some("chunkIndex") => {
                chunk_index = field.text().await.ok().and_then(|s| s.parse().ok())
            }
            Some("totalChunks") => {
                total_chunks = field.text().await.ok().and_then(|s| s.parse().ok())
            }
            Some("filename") => filename = field.text().await.ok(),
            Some("chunk") => chunk_data = field.bytes().await.ok().map(|b| b.to_vec()),
            _ => {}
        }
    }

    match (file_id, chunk_index, total_chunks, filename, chunk_data) {
        (Some(file_id), Some(idx), Some(total), Some(filename), Some(data)) => {
            let (completed, uploaded_chunks, saved_name) =
                state.file_manager.save_chunk(&file_id, idx, total, &filename, &data);

            if completed {
                if let Some(ref saved) = saved_name {
                    let ua = headers
                        .get(header::USER_AGENT)
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("");
                    let device_type = crate::utils::parse_user_agent(ua).device_type;
                    state.file_manager.save_upload_meta(saved, &device_type);
                    // 通知所有端文件列表已更新
                    state.broker.broadcast("file_list_updated", json!({"action": "upload"}));
                }

                Json(json!({
                    "success": true,
                    "completed": true,
                    "filename": saved_name
                }))
            } else {
                Json(json!({
                    "success": true,
                    "completed": false,
                    "uploaded": uploaded_chunks,
                    "total": total
                }))
            }
        }
        _ => Json(json!({
            "success": false,
            "error": "缺少必要的分片字段"
        })),
    }
}

pub async fn delete_files(
    State(state): State<Arc<AppState>>,
    Json(req): Json<DeleteFilesRequest>,
) -> impl IntoResponse {
    let deleted = state.file_manager.delete_files(&req.filenames);
    // 通知所有端文件列表已更新
    state.broker.broadcast("file_list_updated", json!({"action": "delete"}));
    Json(DeleteFilesResponse {
        success: true,
        deleted,
    })
}

pub async fn download_zip(
    State(state): State<Arc<AppState>>,
    Json(req): Json<DownloadZipRequest>,
) -> impl IntoResponse {
    if req.filenames.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "No filenames provided" })),
        )
            .into_response();
    }

    let mut buf = Vec::new();
    let mut added = 0;
    {
        let cursor = std::io::Cursor::new(&mut buf);
        let mut writer = zip::ZipWriter::new(cursor);
        let options =
            zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        for filename in &req.filenames {
            if let Some(path) = state.file_manager.find_file_path(filename) {
                if let Ok(data) = std::fs::read(&path) {
                    if writer.start_file(filename, options).is_ok() {
                        if writer.write_all(&data).is_ok() {
                            added += 1;
                        }
                    }
                }
            }
        }

        if writer.finish().is_err() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "创建压缩包失败" })),
            )
                .into_response();
        }
    }

    if added == 0 {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "No files found" })),
        )
            .into_response();
    }

    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let zip_name = format!("TinyTransfer_{}.zip", timestamp);

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/zip")
        .header(
            header::CONTENT_DISPOSITION,
            format!(r#"attachment; filename="{}""#, zip_name),
        )
        .body(Body::from(buf))
        .unwrap()
        .into_response()
}

pub async fn set_titlebar_color(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SetTitlebarRequest>,
) -> impl IntoResponse {
    let _ = state.titlebar_color_tx.send(Some(req.is_dark));
    Json(json!({"success": true}))
}

// ============================================================
// 磁盘 / 文件夹
// ============================================================

pub async fn disk_info(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.file_manager.get_disk_info() {
        Some(info) => Json(info).into_response(),
        None => Response::builder()
            .status(500)
            .body(Body::from("Unable to get disk info"))
            .unwrap(),
    }
}

pub async fn open_folder_default(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let path = state.file_manager.upload_folder.clone();
    let result = tokio::task::spawn_blocking(move || open_folder_os(std::path::Path::new(&path)))
        .await
        .unwrap_or(Err("任务执行失败".into()));
    match result {
        Ok(_) => Json(OpenFolderResponse { success: true, error: None }),
        Err(e) => Json(OpenFolderResponse { success: false, error: Some(e) }),
    }
}

pub async fn open_folder(
    State(state): State<Arc<AppState>>,
    Path(folder_type): Path<String>,
) -> impl IntoResponse {
    let path = match folder_type.as_str() {
        "uploads" => state.file_manager.upload_folder.clone(),
        _ => state.file_manager.upload_folder.clone(),
    };

    let result = tokio::task::spawn_blocking(move || open_folder_os(std::path::Path::new(&path)))
        .await
        .unwrap_or(Err("任务执行失败".into()));

    match result {
        Ok(_) => Json(OpenFolderResponse {
            success: true,
            error: None,
        }),
        Err(e) => Json(OpenFolderResponse {
            success: false,
            error: Some(e),
        }),
    }
}

fn open_folder_os(path: &std::path::Path) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        std::process::Command::new("explorer")
            .arg(path)
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

// ============================================================
// SSE / Ping
// ============================================================

pub async fn sse_events(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(query): Query<EventsQuery>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let (client_id, rx) = state.broker.register();
    let ua = query.user_agent.unwrap_or_default();
    let ip = addr.ip().to_string();

    // 注册到设备追踪器（优先使用 URL 参数传入的 UA）
    state
        .device_tracker
        .register(&client_id, &ip, &ua)
        .await;

    // 消息流
    let msg_stream = UnboundedReceiverStream::new(rx).map(move |event| {
        Ok::<_, Infallible>(Event::default().event(event.event).data(event.data))
    });

    // 每 10 秒发送一次 ping，同时更新设备活跃时间
    let tracker = Arc::clone(&state.device_tracker);
    let ping_client_id = client_id.clone();
    let ping_stream = IntervalStream::new(time::interval(Duration::from_secs(10))).map(
        move |_| {
            let tracker = Arc::clone(&tracker);
            let cid = ping_client_id.clone();
            tokio::spawn(async move {
                tracker.update_ping(&cid).await;
            });

            let data = json!({ "timestamp": chrono::Utc::now().timestamp_millis() as f64 / 1000.0 });
            Ok::<_, Infallible>(Event::default().event("ping").data(data.to_string()))
        },
    );

    let combined = stream::select(msg_stream, ping_stream);

    Sse::new(combined).keep_alive(KeepAlive::default())
}

pub async fn ping_device(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    form: Option<Form<PingForm>>,
) -> impl IntoResponse {
    let client_id = headers
        .get("X-Client-Id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .or_else(|| form.and_then(|f| f.client_id.clone()))
        .unwrap_or_else(|| "anonymous".into());

    state.device_tracker.update_ping(&client_id).await;
    Json(PingResponse { ok: true })
}

// ============================================================
// 主页面
// ============================================================

pub async fn index() -> Html<&'static str> {
    Html(include_str!("../dist/index.html"))
}

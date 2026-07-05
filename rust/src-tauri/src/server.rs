use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    body::Body,
    extract::Request,
    http::{header::CONTENT_TYPE, StatusCode},
    response::Response,
    routing::{get, post},
    Router,
};
use include_dir::{include_dir, Dir};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::routes;
use crate::state::AppState;

/// 内嵌静态资源目录（构建时打包，作为磁盘 static/ 的 fallback）
static EMBEDDED_STATIC: Dir = include_dir!("$CARGO_MANIFEST_DIR/dist/static");

pub async fn start_server(
    state: Arc<AppState>,
    mut shutdown_rx: tokio::sync::broadcast::Receiver<()>,
) -> Result<(), Box<dyn std::error::Error>> {
    let app = build_router(state);

    let addr: SocketAddr = "0.0.0.0:5000".parse()?;
    println!("[HTTP] Listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(async move {
        let _ = shutdown_rx.recv().await;
        println!("[HTTP] Graceful shutdown signal received");
    })
    .await?;

    println!("[HTTP] Server stopped");
    Ok(())
}

/// 从磁盘查找 static 文件，按优先级查找
fn find_static_file(path: &str) -> Option<std::path::PathBuf> {
    let candidates = vec![
        std::env::current_dir().ok().map(|p| p.join("dist/static").join(path)),
        std::env::current_dir().ok().map(|p| p.join("src-tauri/dist/static").join(path)),
        std::env::current_exe().ok().and_then(|p| p.parent().map(|p| p.join("dist/static").join(path))),
    ];
    
    for candidate in candidates.iter().flatten() {
        if candidate.exists() && candidate.is_file() {
            return Some(candidate.clone());
        }
    }
    None
}

/// 从嵌入的 static 目录提供文件（作为 ServeDir 的 fallback）
/// 优先读取磁盘 dist/static/ 目录（开发时实时生效），找不到再回退到嵌入版本
async fn embedded_static(request: Request) -> Response {
    let path = request
        .uri()
        .path()
        .trim_start_matches("/static/")
        .trim_start_matches('/');

    // 优先尝试磁盘
    if let Some(disk_path) = find_static_file(path) {
        if let Ok(content) = std::fs::read(&disk_path) {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            return Response::builder()
                .status(StatusCode::OK)
                .header(CONTENT_TYPE, mime.as_ref())
                .body(Body::from(content))
                .unwrap();
        }
    }

    // 回退到嵌入版本
    match EMBEDDED_STATIC.get_file(path) {
        Some(file) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            Response::builder()
                .status(StatusCode::OK)
                .header(CONTENT_TYPE, mime.as_ref())
                .body(Body::from(file.contents()))
                .unwrap()
        }
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not found"))
            .unwrap(),
    }
}

pub fn build_router(state: Arc<AppState>) -> Router {
    // API 路由
    let api_router = Router::new()
        .route("/devices", get(routes::get_devices))
        .route("/ip", get(routes::get_ip))
        .route("/check-auth", get(routes::check_auth))
        .route("/auth", post(routes::post_auth))
        .route("/files", get(routes::list_files))
        .route("/download/:filename", get(routes::download_file))
        .route("/upload", post(routes::upload_file))
        .route("/upload-chunk", post(routes::upload_chunk))
        .route("/delete-files", post(routes::delete_files))
        .route("/download-zip", get(routes::download_zip).post(routes::download_zip))
        .route("/disk-info", get(routes::disk_info))
        .route("/open-folder", get(routes::open_folder_default))
        .route("/open-folder/:folder_type", get(routes::open_folder))
        .route("/events", get(routes::sse_events))
        .route("/ping", post(routes::ping_device))
        .route("/set-titlebar-color", post(routes::set_titlebar_color));

    // 静态文件服务：优先磁盘 static/ 目录，未命中时回退到内嵌资源
    let static_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.join("static")))
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| ".".into()).join("static"));

    let static_service = tower_http::services::ServeDir::new(&static_dir)
        .fallback(get(embedded_static));

    Router::new()
        .route("/", get(routes::index))
        .nest("/api", api_router)
        .nest_service("/static", static_service)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

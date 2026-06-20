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

/// 从嵌入的 static 目录提供文件（作为 ServeDir 的 fallback）
async fn embedded_static(request: Request) -> Response {
    let path = request
        .uri()
        .path()
        .trim_start_matches("/static/")
        .trim_start_matches('/');

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
        .route("/uploaded-files", get(routes::list_uploaded_files))
        .route("/all-files", get(routes::list_all_files))
        .route("/download/:filename", get(routes::download_file))
        .route("/upload", post(routes::upload_file))
        .route("/upload-chunk", post(routes::upload_chunk))
        .route("/delete-files", post(routes::delete_files))
        .route("/download-zip", post(routes::download_zip))
        .route("/disk-info", get(routes::disk_info))
        .route("/open-folder/:folder_type", get(routes::open_folder))
        .route("/events", get(routes::sse_events))
        .route("/ping", post(routes::ping_device));

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

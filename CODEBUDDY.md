# CODEBUDDY.md

Project context for WorkBuddy / CodeBuddy when working with this repository.

## Project Summary

**TinyTransfer** — 极速局域网文件传输工具，面向内容创作者（UP主）。
iPhone Safari 扫码即用，电脑和手机在同一 WiFi 下直连传输，无需第三方服务器。

- 后端：Rust (Axum + Tokio + Tauri v2)
- 前端：原生 HTML + JavaScript（单文件 `dist/index.html`）
- 端口：**5000**（硬编码在 `server.rs`，`state.rs` 中 `server_port` 字段默认也是 5000）

## Quick Start

```bash
cd rust/src-tauri
cargo build --release
cargo run --release
# 输出: target/release/slim-transfer.exe
```

## Architecture

### Module Structure (`rust/src-tauri/src/`)

| File | Responsibility |
|------|-----------------|
| `main.rs` | Windows 无控制台窗口入口，调用 `tiny_transfer_lib::run()` |
| `lib.rs` | 初始化 `AppState`，启动 HTTP Server（tokio 任务），运行 Tauri 桌面壳 |
| `server.rs` | Axum HTTP 服务器，`build_router()` 组装路由，内嵌静态资源 `include_dir!` |
| `routes.rs` | 所有 API 路由处理函数（Axum handlers） |
| `state.rs` | `AppState` 全局共享状态：FileManager、DeviceTracker、SSEBroker、密码/令牌等 |
| `file_manager.rs` | 文件操作：列举、上传、分片合并、下载、删除、ZIP 打包 |
| `device_tracker.rs` | 设备连接追踪，超时自动清理，变化时触发广播 |
| `security.rs` | 密码认证、Token 管理、登录速率限制（`LoginRateLimiter`） |
| `broker.rs` | SSE（Server-Sent Events）消息广播器 |
| `utils.rs` | 工具函数 |

### Frontend

- 主页面：`rust/src-tauri/dist/index.html`（单文件，内联 CSS + JS）
- 静态资源：`rust/src-tauri/dist/static/`（磁盘目录优先，未命中回退到 `include_dir` 嵌入的资源）
- QR 码：前端 JavaScript 生成

### API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/` | GET | 前端页面 |
| `/api/devices` | GET | 已连接设备列表 |
| `/api/ip` | GET | 获取本机 IP |
| `/api/check-auth` | GET | 检查认证状态 |
| `/api/auth` | POST | 密码认证，返回 token |
| `/api/files` | GET | 列出共享文件夹文件 |
| `/api/uploaded-files` | GET | 列出上传文件夹文件 |
| `/api/all-files` | GET | 列出所有文件（共享 + 上传）|
| `/api/download/:filename` | GET | 下载文件 |
| `/api/upload` | POST | 上传单文件 |
| `/api/upload-chunk` | POST | 上传分片（大文件） |
| `/api/delete-files` | POST | 删除文件 |
| `/api/download-zip` | POST | 批量下载（ZIP 打包）|
| `/api/disk-info` | GET | 磁盘空间信息 |
| `/api/open-folder/:folder_type` | GET | 在系统文件管理器中打开文件夹 |
| `/api/events` | GET | SSE 事件流（设备连接状态）|
| `/api/ping` | POST | 心跳保活 |

### Key Technical Details

- **端口**：硬编码 `0.0.0.0:5000`（`server.rs` 第28行），`state.rs` 中 `server_port` 默认也是 5000
- **必须使用 HTTP**：iOS Safari 不信任自签名 HTTPS 证书
- **分片上传**：大文件按 5MB 分片，临时存储在 `uploads/.temp_{fileId}/`，全部分片到齐后合并
- **认证机制**：可选密码保护，认证后返回 token，客户端通过 token 访问受保护接口
- **CORS**：使用 `tower_http::CorsLayer` 允许所有跨域请求
- **SSE 实时通信**：通过 `broker.rs` SSEBroker 广播设备列表变化
- **静态文件**：优先磁盘 `static/` 目录，未命中时回退到编译时嵌入的资源（`include_dir`）
- **Tauri v2**：桌面壳，单一实例插件（`tauri_plugin_single_instance`），聚焦已打开窗口
- **优雅关闭**：Tauri `RunEvent::Exit` 时发送 shutdown 信号给 HTTP Server

## Build & Package

```bash
cd rust/src-tauri

# 开发模式
cargo run

# 发布模式（优化 + 单文件）
cargo build --release
# 输出: target/release/tiny-transfer.exe

# 打包安装包（NSIS 安装程序）
cargo tauri build
# 或一键脚本：../../build-tauri.bat
# 输出: output/

# release 优化参数 (Cargo.toml [profile.release])
# codegen-units = 1
# lto = true
# opt-level = "z"
# panic = "abort"
# strip = true
```

## Project Directory Structure

```
SlimTransfer/
├── rust/                        # Rust 核心代码
│   └── src-tauri/             # Tauri 桌面应用 + Axum 服务端
│       ├── src/
│       │   ├── main.rs         # 入口
│       │   ├── lib.rs          # 初始化 + Tauri 运行
│       │   ├── server.rs       # HTTP 服务器
│       │   ├── routes.rs       # API 路由 handlers
│       │   ├── state.rs        # 应用状态
│       │   ├── file_manager.rs # 文件管理
│       │   ├── device_tracker.rs # 设备追踪
│       │   ├── security.rs    # 安全/认证
│       │   ├── broker.rs      # SSE 广播
│       │   └── utils.rs       # 工具函数
│       ├── dist/               # 前端静态文件
│       │   ├── index.html      # 主页面（单文件）
│       │   └── static/        # 静态资源（可选，回退到嵌入资源）
│       ├── Cargo.toml         # Rust 项目配置
│       └── build.rs           # Tauri build script
├── share/                      # 默认共享文件夹（自动创建）
├── uploads/                    # 默认上传文件夹（自动创建）
├── build-tauri.bat             # 一键打包脚本
├── README.md                   # 项目说明
├── CLAUDE.md                   # Claude Code 上下文
└── CODEBUDDY.md               # 本文件（WorkBuddy/CodeBuddy 上下文）
```

## Important Notes

- 必须使用 HTTP（iOS Safari 不信任自签名 HTTPS 证书）
- Axum 运行在 `0.0.0.0:5000` 以允许局域网访问
- 文件名处理需注意中文字符编码
- Rust 版本使用 `include_dir` 将静态资源编译进二进制，便于单文件分发
- 开发时可直接访问 `rust/src-tauri/dist/` 下的前端文件进行调试
- `Cargo.toml` 中 `crate-type = ["lib", "cdylib", "staticlib"]` 支持 Tauri 桌面壳
- 单一实例：通过 `tauri_plugin_single_instance` 插件，重复启动时聚焦已有窗口

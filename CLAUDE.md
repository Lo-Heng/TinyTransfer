# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Quick Start

```bash
# 进入 Rust 项目目录
cd rust/src-tauri

# 构建
cargo build --release

# 运行
cargo run --release
```

## Architecture

局域网文件传输工具，iPhone Safari 扫码即用，电脑和手机在同一 WiFi 下直连传输。

### Rust 版本（当前主力）

- **rust/src-tauri/src/lib.rs**: 初始化 `AppState`，启动 HTTP Server（tokio 任务），运行 Tauri 桌面壳
- **rust/src-tauri/src/server.rs**: Axum HTTP 服务器，`build_router()` 组装路由，内嵌静态资源 `include_dir!`
- **rust/src-tauri/src/routes.rs**: 所有 API 路由处理函数（Axum handlers）
- **rust/src-tauri/src/state.rs**: `AppState` 全局共享状态：FileManager、DeviceTracker、SSEBroker、密码/令牌等
- **rust/src-tauri/src/file_manager.rs**: 文件操作：列举、上传、分片合并、下载、删除、ZIP 打包
- **rust/src-tauri/src/device_tracker.rs**: 设备连接追踪，超时自动清理，变化时触发广播
- **rust/src-tauri/src/security.rs**: 密码认证、Token 管理、登录速率限制（`LoginRateLimiter`）
- **rust/src-tauri/src/broker.rs**: SSE（Server-Sent Events）消息广播器
- **rust/src-tauri/src/utils.rs**: 工具函数
- **rust/src-tauri/dist/index.html**: 前端页面（单文件，内联 CSS + JS）
- **share/**: 电脑端只读共享文件夹，手机可浏览下载
- **uploads/**: 手机上传文件存放目录

### 关键技术点

- **SSE 实时通信**: 通过 `broker.rs` SSEBroker 实现设备列表变化广播（`/api/events`）
- **分片上传**: 大文件按 5MB 分片，临时存储在 `uploads/.temp_{fileId}/`，全部分片到齐后合并
- **认证机制**: 可选密码保护，认证后返回 token，客户端通过 token 访问受保护接口
- **静态文件服务**: 优先磁盘 `static/` 目录，未命中时回退到编译时嵌入的资源（`include_dir`）
- **CORS**: 使用 `tower_http::CorsLayer` 允许跨域请求

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

### SSE Events（通过 `/api/events` 推送）

| Event | Direction | Description |
|-------|-----------|-------------|
| `hello` | Server→Client | 客户端注册时发送，包含 `timestamp` 和 `client_id` |
| `device_list` | Server→Client | 设备列表变化时广播，包含 `connected` 和 `devices` |

## Build & Package

```bash
# 开发模式
cd rust/src-tauri
cargo run

# 发布模式（优化 + 单文件）
cargo build --release
# 输出: target/release/tiny-transfer.exe

# 打包安装包（NSIS 安装程序）
cd rust/src-tauri
cargo tauri build
# 或一键脚本：../../build-tauri.bat
# 输出: output/
```

## Project Structure

```
SlimTransfer/
├── rust/                        # Rust 核心代码
│   └── src-tauri/             # Tauri 桌面应用 + Axum 服务端
│       ├── src/
│       │   ├── main.rs         # 入口（Windows 无控制台）
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
├── README.md                  # 项目说明
├── CLAUDE.md                  # 本文件（Claude Code 上下文）
└── CODEBUDDY.md              # WorkBuddy/CodeBuddy 上下文
```

## Notes

- 必须使用 HTTP（iOS Safari 不信任自签名 HTTPS 证书）
- Axum 运行在 `0.0.0.0:5000` 以允许局域网访问（端口硬编码在 `server.rs` 第28行）
- 文件名处理需注意中文字符编码
- Rust 版本使用 `include_dir` 将静态资源编译进二进制，便于单文件分发
- 开发时可直接访问 `rust/src-tauri/dist/` 下的前端文件进行调试
- Tauri v2 桌面壳，单一实例插件（`tauri_plugin_single_instance`）
- SSE 实时通信：通过 `broker.rs` SSEBroker 广播设备列表变化
- 认证机制：可选密码保护，认证后返回 token（`security.rs`）
- 优雅关闭：Tauri `RunEvent::Exit` 时发送 shutdown 信号给 HTTP Server

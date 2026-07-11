# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Quick Start

```bash
# 开发模式
cd rust/src-tauri
cargo run

# 发布模式（优化 + 单文件）
cargo build --release

# 打包安装包（NSIS 安装程序）
cd rust/src-tauri
cargo tauri build
# 或一键脚本：../../build-tauri.bat
# 输出: ../../output/
```

## Architecture

局域网文件传输工具，iPhone Safari 扫码即用，电脑和手机在同一 WiFi 下直连传输。

Rust + Tauri v2 桌面应用，内嵌 Axum HTTP 服务器。双击 exe 弹出一体化窗口（WebView2），无需外部浏览器。

### 核心模块

- **rust/src-tauri/src/lib.rs**: 初始化 `AppState`，启动 HTTP Server（tokio 任务），运行 Tauri 桌面壳
- **rust/src-tauri/src/server.rs**: Axum HTTP 服务器，`build_router()` 组装路由，内嵌静态资源 `include_dir!`
- **rust/src-tauri/src/routes.rs**: 所有 API 路由处理函数（Axum handlers）
- **rust/src-tauri/src/state.rs**: `AppState` 全局共享状态：FileManager、DeviceTracker、SSEBroker、密码/令牌等
- **rust/src-tauri/src/file_manager.rs**: 文件操作：列举、上传、分片合并、下载、删除、ZIP 打包
- **rust/src-tauri/src/device_tracker.rs**: 设备连接追踪，超时自动清理，变化时触发广播
- **rust/src-tauri/src/security.rs**: 密码认证、Token 管理、登录速率限制（`LoginRateLimiter`）
- **rust/src-tauri/src/broker.rs**: SSE（Server-Sent Events）消息广播器
- **rust/src-tauri/src/utils.rs**: 工具函数
- **rust/src-tauri/dist/index.html**: 前端页面（单文件，3126 行，内联 CSS + JS）
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

## Project Structure

```
Slim_Transfer_2/
├── rust/                        # Rust 核心代码
│   ├── Cargo.toml              # Workspace 配置（release 优化：LTO + strip + opt-z）
│   └── src-tauri/              # Tauri 桌面应用 + Axum 服务端
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
│       │   ├── index.html      # 主页面（单文件，内联 CSS + JS）
│       │   └── static/         # 静态资源（qrcode.min.js）
│       ├── icons/              # 应用图标（闪电图标，多尺寸 PNG + ICO）
│       │   └── icon.ico        # Windows exe 图标（6 尺寸：16/32/48/64/128/256）
│       ├── capabilities/      # Tauri v2 权限配置
│       ├── permissions/       # Tauri v2 自定义命令权限
│       ├── resources/         # 运行时依赖（WebView2Loader.dll）
│       ├── Cargo.toml          # Rust 项目配置 + 依赖
│       ├── build.rs            # Tauri build script
│       └── tauri.conf.json     # Tauri 配置（targets: app，便携 exe）
├── build-tauri.bat             # 一键打包脚本
├── CLAUDE.md                   # 本文件（Claude Code 上下文）
└── LICENSE                     # 开源协议
```

## Build & Package

```bash
# 开发模式
cd rust/src-tauri
cargo run

# ⚠️ 注意：cargo build --release 不会嵌入图标！
# 若需图标生效，必须用 cargo tauri build（它会执行 Patching 步骤将图标写入 exe 资源段）
# 发布模式（无图标需求时可用）
cargo build --release
# 输出: rust/target/release/tiny-transfer.exe

# ✅ 打包 exe（含图标嵌入 + UPX 压缩）—— 用这个！
cd rust/src-tauri
cargo tauri build
# 或一键脚本：../../build-tauri.bat
# 输出: output/tiny-transfer.exe
```

### UPX 压缩

打包脚本会自动使用 UPX 压缩可执行文件，显著减小体积：

| 项目 | 大小 |
|------|------|
| 压缩前 | ~4.8 MB |
| 压缩后 | ~1.8 MB |
| 压缩率 | ~62% |

**要求**：需将 `upx.exe` 放到 `tools/upx/upx.exe`

- 下载地址：https://github.com/upx/upx/releases
- 如果未找到 UPX，脚本会自动跳过压缩步骤

### Windows 图标缓存
Windows 资源管理器会缓存 exe 图标，替换后可能仍显示旧图标。
清缓存方法（PowerShell）：
```powershell
Stop-Process -Name explorer -Force -ErrorAction SilentlyContinue
Remove-Item "$env:LOCALAPPDATA\IconCache.db" -Force -ErrorAction SilentlyContinue
Remove-Item "$env:LOCALAPPDATA\Microsoft\Windows\Explorer\iconcache_*.db" -Force -ErrorAction SilentlyContinue
Start-Process explorer.exe
```

### 图标清晰度（高 DPI）
- `@2x` 后缀的 PNG 实际像素必须是 base 尺寸的 2 倍（如 `32x32@2x.png` = 64x64px）
- 窗口图标需在代码中显式设置高分辨率版本（`WebviewWindowBuilder::icon()`）
- ICO 文件应包含多尺寸（16/32/48/64/128/256），从高分辨率源用 LANCZOS 重采样生成

Release 优化配置（`rust/Cargo.toml`）：
- `lto = true` — 链接时优化
- `opt-level = "z"` — 最小体积
- `strip = true` — 去除符号
- `codegen-units = 1` — 最大优化
- `panic = "abort"` — 减小体积

## Notes

- 必须使用 HTTP（iOS Safari 不信任自签名 HTTPS 证书）
- Axum 运行在 `0.0.0.0:5000` 以允许局域网访问（端口硬编码在 `server.rs`）
- 文件名处理需注意中文字符编码
- Rust 版本使用 `include_dir` 将静态资源编译进二进制，便于单文件分发
- 开发时可直接访问 `rust/src-tauri/dist/` 下的前端文件进行调试
- SSE 实时通信：通过 `broker.rs` SSEBroker 广播设备列表变化
- 认证机制：可选密码保护，认证后返回 token（`security.rs`）
- 优雅关闭：Tauri `RunEvent::Exit` 时发送 shutdown 信号给 HTTP Server

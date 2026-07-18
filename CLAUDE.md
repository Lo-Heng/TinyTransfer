# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Quick Start

```bash
# 首次构建需安装前端依赖
cd frontend && npm install && cd ..

# 开发模式（Vite dev server + Tauri）
cd rust/src-tauri
cargo run

# 发布模式（优化 + 单文件，无图标嵌入）
cargo build --release

# 打包 exe（含图标嵌入）—— 用这个！
cd rust/src-tauri
cargo tauri build
# 或一键脚本：../../build-tauri.bat
# 输出: ../../output/
```

## Architecture

局域网文件传输工具，iPhone Safari 扫码即用，电脑和手机在同一 WiFi 下直连传输。

Rust + Tauri v2 桌面应用，内嵌 Axum HTTP 服务器。双击 exe 弹出一体化窗口（WebView2），无需外部浏览器。

前端采用 Vue 3 + Vite + Pinia，源码在 `frontend/`，构建产物输出到 `rust/src-tauri/dist/`，通过 `include_dir!` 编译进二进制。

### 核心模块

- **rust/src-tauri/src/lib.rs**: 初始化 `AppState`，启动 HTTP Server（tokio 任务），运行 Tauri 桌面壳，`debug_log!` 宏定义
- **rust/src-tauri/src/server.rs**: Axum HTTP 服务器，`build_router()` 组装路由，内嵌静态资源 `include_dir!`
- **rust/src-tauri/src/routes.rs**: 所有 API 路由处理函数（Axum handlers）
- **rust/src-tauri/src/state.rs**: `AppState` 全局共享状态：FileManager、DeviceTracker、SSEBroker、密码/令牌等
- **rust/src-tauri/src/file_manager.rs**: 文件操作：列举、上传、分片合并、下载、删除、ZIP 打包
- **rust/src-tauri/src/device_tracker.rs**: 设备连接追踪，超时自动清理，变化时触发广播
- **rust/src-tauri/src/security.rs**: 密码认证、Token 管理、登录速率限制（`LoginRateLimiter`）
- **rust/src-tauri/src/broker.rs**: SSE（Server-Sent Events）消息广播器
- **rust/src-tauri/src/commands.rs**: Tauri 命令（文件保存对话框、打开文件夹、下载对话框）
- **rust/src-tauri/src/utils.rs**: 工具函数
- **rust/src-tauri/src/platform/**: 跨平台抽象层
  - `mod.rs`: `PlatformOps` trait + `current()` 工厂函数（`cfg(target_os)` 分发）
  - `config.rs`: `WindowConfig` 常量（标题栏色值、窗口尺寸、图标路径）
  - `windows.rs`: Windows 实现（DWM 标题栏 + `CreateMutexW` 单实例）
  - `macos.rs`: macOS no-op 实现
  - `unix.rs`: Linux/其他 no-op 实现
- **rust/src-tauri/dist/**: Vite 构建产物（`npm run build` 生成，`include_dir!` 编译进二进制）
- **frontend/**: Vue 3 + Vite + Pinia 前端源码
- **share/**: 电脑端只读共享文件夹，手机可浏览下载
- **uploads/**: 手机上传文件存放目录

### 关键技术点

- **SSE 实时通信**: 通过 `broker.rs` SSEBroker 实现设备列表变化广播（`/api/events`）
- **分片上传**: 大文件按 5MB 分片，临时存储在 `uploads/.temp_{fileId}/`，全部分片到齐后合并
- **认证机制**: 可选密码保护，认证后返回 token，客户端通过 token 访问受保护接口
- **静态文件服务**: 优先磁盘 `static/` 目录，未命中时回退到编译时嵌入的资源（`include_dir`）
- **CORS**: 使用 `tower_http::CorsLayer` 允许跨域请求
- **跨平台抽象**: `PlatformOps` trait 隔离 Windows/macOS/Linux 差异，标题栏颜色与单实例检测统一入口
- **前端构建链路**: `tauri.conf.json` 的 `beforeBuildCommand` 在 `cargo tauri build` 时自动触发 `npm run build`，产物输出到 `rust/src-tauri/dist/`

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
│       │   ├── lib.rs          # 初始化 + Tauri 运行 + debug_log 宏
│       │   ├── server.rs       # HTTP 服务器
│       │   ├── routes.rs       # API 路由 handlers
│       │   ├── state.rs        # 应用状态
│       │   ├── file_manager.rs # 文件管理
│       │   ├── device_tracker.rs # 设备追踪
│       │   ├── security.rs    # 安全/认证
│       │   ├── broker.rs      # SSE 广播
│       │   ├── commands.rs    # Tauri 命令（文件对话框/打开文件夹）
│       │   ├── utils.rs       # 工具函数
│       │   └── platform/      # 跨平台抽象层
│       │       ├── mod.rs     # PlatformOps trait + current() 工厂
│       │       ├── config.rs  # WindowConfig 常量（标题栏色值/窗口尺寸）
│       │       ├── windows.rs # Windows: DWM 标题栏 + CreateMutexW 单实例
│       │       ├── macos.rs   # macOS: no-op
│       │       └── unix.rs    # Linux/其他: no-op
│       ├── dist/               # Vite 构建产物（npm run build 生成）
│       │   ├── index.html      # 入口 HTML
│       │   └── assets/         # 打包后的 CSS/JS（文件名含 hash）
│       ├── icons/              # 应用图标
│       │   ├── source.png      # 唯一源文件（1024×1024）
│       │   ├── icon.ico        # Windows exe 图标（由脚本生成）
│       │   └── *.png           # 各尺寸 PNG（由脚本生成）
│       ├── capabilities/      # Tauri v2 权限配置
│       ├── permissions/       # Tauri v2 自定义命令权限
│       ├── resources/         # 运行时依赖（WebView2Loader.dll）
│       ├── Cargo.toml          # Rust 项目配置 + 依赖
│       ├── build.rs            # Tauri build script
│       └── tauri.conf.json     # Tauri 配置（Vite 构建链路 + 便携 exe）
├── frontend/                   # Vue 3 + Vite + Pinia 前端源码
│   ├── src/
│   │   ├── App.vue             # 根组件
│   │   ├── main.ts             # 入口（挂载 Pinia + import CSS）
│   │   ├── components/         # Vue SFC 组件（layout/files/modals/ui/icons）
│   │   ├── composables/        # 组合式函数（SSE/上传/下载/拖拽等）
│   │   ├── stores/             # Pinia 状态（auth/devices/files/theme/ui/upload）
│   │   ├── api/                # HTTP API 封装（client/auth/devices/files/system）
│   │   ├── styles/             # CSS（tokens/base/components/layout/files/modals）
│   │   └── utils/              # 工具函数
│   ├── public/static/          # 静态资源（payment-qr.jpeg）
│   ├── index.html              # Vite HTML 模板
│   ├── vite.config.ts          # Vite 配置（outDir → ../rust/src-tauri/dist）
│   └── package.json            # 依赖（vue@3.5 + pinia@2.2 + vite@5.4）
├── scripts/
│   └── generate-icons.bat      # 图标生成脚本（source.png → 全平台图标）
├── build-tauri.bat             # 一键打包脚本（GNU 工具链 + 前端构建）
├── CLAUDE.md                   # 本文件
└── LICENSE                     # 开源协议
```

## Build & Package

```bash
# 首次构建需安装前端依赖
cd frontend && npm install && cd ..

# 开发模式（Vite dev server + Tauri 热更新）
cd rust/src-tauri
cargo run

# 发布模式（无图标嵌入，调试用）
cargo build --release
# 输出: rust/target/release/tiny-transfer.exe

# ✅ 打包 exe（含图标嵌入）—— 用这个！
cd rust/src-tauri
cargo tauri build
# 或一键脚本：../../build-tauri.bat
# 输出: output/tiny-transfer.exe
```

`cargo tauri build` 通过 `tauri.conf.json` 的 `beforeBuildCommand` 自动执行 `npm run build`，无需手动构建前端。一键脚本 `build-tauri.bat` 会额外执行 `npm install` 确保依赖就绪。

### 换图标流程

1. 替换 `rust/src-tauri/icons/source.png`（建议 1024×1024 PNG）
2. 运行 `scripts/generate-icons.bat`
   - 自动生成：32x32.png、128x128.png、128x128@2x.png、icon.ico、icon.icns、Square*.png、android/、ios/
3. 重新 `cargo tauri build`（图标嵌入 exe 资源段）

**注意**：`cargo build --release` 不会嵌入图标，必须用 `cargo tauri build`。

### 改标题栏颜色

标题栏色值集中在 `rust/src-tauri/src/platform/config.rs` 的 `WINDOW_CONFIG` 常量：

| 字段 | 说明 | 当前值 |
|------|------|--------|
| `light_caption_color` | 明亮模式背景 | `0x00FAFBFB` (#FBFBFA) |
| `light_text_color` | 明亮模式文字 | `0x00000000` (黑) |
| `dark_caption_color` | 黑暗模式背景 | `0x000F0F0F` (#0F0F0F) |
| `dark_text_color` | 黑暗模式文字 | `0x00FFFFFF` (白) |

色值格式为 COLORREF（`0x00BBGGRR`，注意 BGR 顺序）。修改后 `cargo run` 即可看到效果。

仅 Windows 生效（DWM API），macOS/Linux 为 no-op。

### Windows 图标缓存
Windows 资源管理器会缓存 exe 图标，替换后可能仍显示旧图标。
清缓存方法（PowerShell）：
```powershell
Stop-Process -Name explorer -Force -ErrorAction SilentlyContinue
Remove-Item "$env:LOCALAPPDATA\IconCache.db" -Force -ErrorAction SilentlyContinue
Remove-Item "$env:LOCALAPPDATA\Microsoft\Windows\Explorer\iconcache_*.db" -Force -ErrorAction SilentlyContinue
Start-Process explorer.exe
```

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
- 开发时前端源码在 `frontend/`，`npm run dev` 启动 Vite 热更新（端口 5173）
- SSE 实时通信：通过 `broker.rs` SSEBroker 广播设备列表变化
- 认证机制：可选密码保护，认证后返回 token（`security.rs`）
- 优雅关闭：Tauri `RunEvent::Exit` 时发送 shutdown 信号给 HTTP Server
- 单实例检测：Windows 用 `CreateMutexW` + `ERROR_ALREADY_EXISTS`（`platform/windows.rs`），非 Tauri 插件
- GNU 工具链：Windows 构建使用 `stable-x86_64-pc-windows-gnu` + 项目自带 MinGW（`tools/upx/mingw64/bin`）

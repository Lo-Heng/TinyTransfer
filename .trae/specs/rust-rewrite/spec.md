# Rust + Tauri 重写 Spec

## Why

当前 Python + Flask + pywebview + PyInstaller 版本已将单文件 exe 从约 25MB 优化到 14MB 左右，启动速度也有所提升，但继续压小的空间非常有限：Python 解释器（python314.DLL 约 2.7MB）、OpenSSL（约 2.1MB）、ucrtbase 等运行时依赖已经占了近 10MB，且 PyInstaller 单文件解压到临时目录本身也会带来启动开销。

采用 **Rust + Tauri** 重写，可以把 GUI 壳子和 HTTP 后端都编译成单一原生二进制，利用系统自带的 WebView2（Windows）/ WebKit（macOS）/ WebKitGTK（Linux）渲染界面，预期能把安装包降到 **2–4MB**，冷启动时间降到 **1 秒以内**，同时为后续跨平台发布打下基础。

**约束**：原 Python 代码保留在 `f:\workspace\Slim_Transfer_2` 不动，Rust 版本在新建的 `rust/` 子目录中独立开发、独立打包。

## What Changes

### 1. 新建 Rust 工作区（不动原代码）
- 在项目根目录新建 `rust/` 文件夹，与现有 Python 代码完全隔离。
- `rust/` 内使用 Cargo 管理依赖，Tauri 作为桌面壳。
- 原 `src/`、`templates/`、`static/`、`gui.py`、`SlimTransfer.spec` 等全部保留，作为底档。

### 2. 复用现有前端
- 将 `templates/index.html`、`static/` 复制或软链接到 `rust/src-tauri/` 的 Tauri 前端目录。
- 前端 JS 中 SSE 连接地址改为 Tauri 提供的本地 server 地址（默认仍是 `http://127.0.0.1:5000`）。
- QR 码、设备列表、文件上传/下载等 UI 逻辑尽量原样保留。

### 3. Rust 后端（HTTP + SSE）
- 使用轻量 Web 框架（推荐 `axum` 或 `tokio` + `axum`）实现：
  - `GET /`：返回主页面。
  - `GET /api/events`：SSE 设备列表实时推送。
  - `POST /api/ping`：客户端心跳。
  - `GET /api/devices`：设备列表快照。
  - `GET /api/ip`：本机 IP 和连接 URL。
  - 文件上传（multipart）、文件下载、文件列表相关端点。
  - 可选：`/api/check-auth` 等兼容端点，保证前端 loading 页面可用。
- 设备追踪器 `DeviceTracker` 用 Rust 实现：注册、心跳超时清理、SSE 广播。
- 文件管理器 `FileManager` 用 Rust 实现：上传目录、分享目录、临时清理。

### 4. Tauri 桌面壳
- `src-tauri/src/main.rs`：启动 Rust HTTP server，同时创建 Tauri 窗口加载本地前端。
- 窗口配置：标题 `Slim Transfer`、尺寸 1100×780、最小 800×600。
- 打包配置：`tauri.conf.json` 关闭开发工具、启用单实例、配置图标和元数据。

### 5. 打包输出
- 使用 `cargo tauri build` 生成单文件 `SlimTransfer.exe`（Windows）/ `.app`（macOS）。
- 目标体积：**< 5MB**，理想情况 **2–4MB**。
- 目标启动速度：**< 1 秒** 出现主窗口。

## Impact

- **影响的 specs**：`arch-optimize-plan-c` 作为底档保留，不再继续迭代；新增 `rust-rewrite` 规格。
- **影响的代码**：仅 `rust/` 目录下新增代码；原 Python 项目文件不改动。
- **用户体验**：界面和交互与当前版本保持一致，安装包显著变小，启动更快。

## ADDED Requirements

### Requirement: Rust 项目结构
系统 SHALL 在 `f:\workspace\Slim_Transfer_2/rust/` 目录下建立独立的 Rust + Tauri 项目，且不删除或修改原 Python 代码。

#### Scenario: 目录隔离
- **WHEN** 开发者查看项目根目录
- **THEN** 同时存在 Python 原项目文件和 `rust/` 目录
- **AND** Python 原项目仍可正常打包运行

### Requirement: HTTP + SSE 后端
系统 SHALL 使用 Rust 实现与当前 Python 版本兼容的 REST + SSE 接口。

#### Scenario: 桌面端与手机端连接
- **WHEN** Rust 版本启动后
- **THEN** 本机页面通过 SSE 收到 `hello` 和 `device_list` 事件
- **AND** 手机扫码后同样能注册设备并显示在设备列表中

### Requirement: 文件传输
系统 SHALL 支持通过网页上传、下载、浏览文件，行为与当前 Python 版本一致。

#### Scenario: 手机向电脑传文件
- **WHEN** 手机端选择文件并上传
- **THEN** 文件保存到 Rust 后端的 uploads 目录
- **AND** 电脑端页面显示新增文件并可下载

### Requirement: 设备类型识别
系统 SHALL 根据客户端 User-Agent 识别并显示设备类型（Windows PC、iPhone、Android 等）。

#### Scenario: 多设备连接
- **WHEN** 多台设备接入 SSE
- **THEN** 设备列表显示正确的类型和 IP

### Requirement: Tauri 桌面窗口
系统 SHALL 使用 Tauri 创建原生桌面窗口，内嵌前端页面。

#### Scenario: 双击启动
- **WHEN** 用户双击 SlimTransfer.exe
- **THEN** 出现标题为 "Slim Transfer" 的桌面窗口
- **AND** 窗口加载完成后显示主界面

## MODIFIED Requirements

### Requirement: 打包产物
**修改前**：PyInstaller 生成单文件 exe，约 14MB。
**修改后**：Tauri 生成单文件 exe，目标 < 5MB。

#### Scenario: Windows 打包
- **WHEN** 执行 `cargo tauri build`
- **THEN** 在 `rust/src-tauri/target/release/` 下生成 `SlimTransfer.exe`
- **AND** exe 大小不超过 5MB

## REMOVED Requirements

### Requirement: Python 运行时依赖
**原因**：Rust 编译为原生二进制，不再需要 Python 解释器、Flask、pywebview、PyInstaller。
**迁移**：原 Python 项目保留在根目录作为底档，仅 Rust 版本进入日常开发和发布流程。

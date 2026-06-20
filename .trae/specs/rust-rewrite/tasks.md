# Rust + Tauri 重写 Tasks

## Task 1: 初始化 Rust + Tauri 工作区
- [x] 1.1 在项目根目录创建 `rust/` 文件夹，并初始化 Cargo 工作区
- [x] 1.2 安装并配置 Tauri CLI 与依赖（`tauri`、`tauri-build`）
- [x] 1.3 创建 `rust/src-tauri/` 标准目录结构（`src/main.rs`、`tauri.conf.json`、`Cargo.toml`、`icons/`）
- [x] 1.4 将 `templates/index.html` 和 `static/` 引入 `src-tauri` 作为前端资源
- [ ] 1.5 验证 `cargo tauri dev` 能打开空白窗口并加载本地页面（当前环境 MSVC 工具链无法 spawn 子进程，无法验证）

## Task 2: 搭建 Rust HTTP 后端骨架
- [x] 2.1 引入 `axum`、`tokio`、`tower`、`tower-http` 依赖
- [x] 2.2 实现基础 HTTP server，监听 `0.0.0.0:5000`
- [x] 2.3 实现 `GET /` 返回主页面 HTML
- [x] 2.4 配置静态资源服务（CSS/JS/字体）
- [x] 2.5 添加 CORS 头，允许 loading 页面跨域健康检查

## Task 3: 实现 SSE 与设备追踪
- [x] 3.1 实现 SSE broadcast 模块（`src/broker.rs`），支持注册/注销客户端和广播消息
- [x] 3.2 实现 `GET /api/events`，返回 `text/event-stream` 并发送 `hello` 事件
- [x] 3.3 实现 `DeviceTracker`（`src/device_tracker.rs`），管理设备注册、心跳、超时清理
- [x] 3.4 实现 `POST /api/ping` 接收心跳
- [x] 3.5 实现 `GET /api/devices` 返回设备列表快照
- [x] 3.6 实现 `GET /api/ip` 返回本机 IP 和连接 URL
- [x] 3.7 集成定时心跳，SSE 每 10 秒发送一次 `ping` 事件并更新设备活跃时间

## Task 4: 实现文件传输接口
- [x] 4.1 实现 `FileManager`（`src/file_manager.rs`），管理 uploads、share、temp 目录
- [x] 4.2 实现 multipart 文件上传端点
- [x] 4.3 实现文件列表查询端点
- [x] 4.4 实现文件下载端点
- [x] 4.5 实现批量打包下载与删除接口
- [x] 4.6 集成临时目录清理逻辑

## Task 5: 前端适配与复用
- [x] 5.1 调整 `index.html` 中 API 基础 URL，适配 Tauri 本地 server（相对路径已可用）
- [x] 5.2 将 QR 码库从 CDN 改为本地 `static/qrcode.min.js`
- [ ] 5.3 验证 QR 码生成、设备列表、连接状态指示器正常显示（需编译后验证）
- [ ] 5.4 验证文件上传、下载、删除流程与后端正常交互（需编译后验证）
- [x] 5.5 页面可见性变化时的心跳和 SSE 重连逻辑保留原前端实现

## Task 6: Tauri 窗口与打包配置
- [x] 6.1 配置 `tauri.conf.json`：窗口标题、尺寸、最小尺寸
- [x] 6.2 配置应用图标、应用名称、版本号
- [x] 6.3 集成 `tauri-plugin-single-instance` 实现单实例
- [x] 6.4 关闭窗口时优雅退出 HTTP server 和清理线程
- [ ] 6.5 运行 `cargo tauri build` 生成 release exe（当前环境无法编译）

## Task 7: 验证与优化
- [ ] 7.1 验证 exe 大小是否 < 5MB（需编译后验证）
- [ ] 7.2 验证冷启动时间 < 1 秒（需编译后验证）
- [ ] 7.3 验证手机扫码后设备列表正确显示（需编译后验证）
- [ ] 7.4 验证文件上传下载功能完整（需编译后验证）
- [x] 7.5 与原 Python 版本并行动行，确认原项目未被改动

# Task Dependencies

- Task 2 依赖 Task 1
- Task 3 依赖 Task 2
- Task 4 依赖 Task 2
- Task 5 依赖 Task 3 和 Task 4
- Task 6 依赖 Task 1 和 Task 5
- Task 7 依赖 Task 6

# 可并行工作

- Task 3（SSE/设备追踪）和 Task 4（文件传输）可在 Task 2 完成后并行开发。

# 当前阻塞

本机 MSVC Rust 工具链在执行 `std::process::Command::spawn` 时 panic（`Os { code: 0 }`），导致任何带 build script 的 crate 都无法编译。GNU 工具链可编译纯 Rust，但 Tauri 依赖的 `dlltool` 同样受该环境限制无法 spawn 子进程。因此所有需 `cargo build/check` 的验证步骤均无法在本地完成，需要在 Rust 环境正常的机器上执行 `cargo tauri build` 验证。

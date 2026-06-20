# 方案 C：SSE + WebView2 + 依赖精简 Spec

## Why

当前打包体积 ~25MB，启动速度偏慢，主要由以下问题导致：

1. **pythonnet / clr_loader / cffi**：pywebview 的 WinForms 后端依赖 .NET CLR 桥接，占 3-5MB，且在 Python 3.13+ 下有兼容性 monkey patch 代码，增加启动开销。
2. **Flask-SocketIO 全家桶**：flask_socketio / python-socketio / engineio / bidict / wsproto / simple-websocket / h11，共 7 个依赖仅用于"设备列表实时推送"这一单一功能，架构冗余。
3. **Pillow + qrcode**：仅用于生成一张二维码 PNG 图片。二维码完全可以在前端用 Canvas 生成。
4. **eventlet**：当前已使用 `async_mode='threading'`，eventlet 是 dead code，但仍然被打进 PYZ 包。
5. **zeroconf / mDNS**：当前用于局域网服务发现，但用户确认不需要此功能，移除可进一步减小体积和启动开销。

**目标：** 把安装包从 ~25MB 降到 ~8-10MB，显著加快启动速度，**同时保留原生桌面窗口体验**（关闭窗口 = 退出程序）。

## What Changes

### 1. 推送层：SocketIO → SSE (Server-Sent Events)
- 移除 `flask_socketio`、`python-socketio`、`engineio`、`bidict`、`wsproto`、`simple-websocket`、`h11` 全部依赖
- `src/slim_transfer/app.py`：移除 engineio monkey patch 代码，`create_app()` 返回 `(app, sse_broker)` 替代 `(app, socketio)`
- `src/slim_transfer/socket/handlers.py` → 重写为 `src/slim_transfer/services/sse_broker.py`：用标准库 `threading` + `queue` 实现轻量事件广播，`@app.route('/api/events')` 使用 `text/event-stream` MIME 类型推送
- `src/slim_transfer/api/devices.py`：新增 `GET /api/devices` 返回当前设备列表快照（供前端首次获取 / SSE 断开重连时拉取）
- `src/run.py`：`socketio.run()` → `app.run()`
- `gui.py`：`socketio.run()` → `app.run()`

### 2. QR 码：后端生成 → 前端 Canvas 生成
- 移除 `src/slim_transfer/api/devices.py` 中的 `import qrcode` 和 `/api/qrcode` endpoint
- `templates/index.html`：用纯 JS 在 Canvas 上生成二维码（使用轻量 qrcode.js 或内联 QR 算法），从 `/api/ip` 获取 URL 后在浏览器端绘制
- 移除 `Pillow`、`qrcode` 依赖

### 3. pywebview：WinForms → EdgeChromium (Windows 专用)
- `gui.py`：在 `import webview` 之前，Windows 平台设置 `os.environ['PYWEBVIEW_GUI'] = 'edgechromium'`
- 其他平台（macOS/Linux）自动使用原生后端（WebKit / GTK+ / Qt），不受影响
- `pythonnet`、`clr_loader`、`cffi`：从 hiddenimports 中移除

### 4. 依赖链精简
- `requirements.txt`：从 6 行精简为 2 行：`Flask>=3.0.0`、`pywebview`
- 移除列表：qrcode, Pillow, flask-socketio, eventlet, zeroconf（及其传递依赖 python-socketio, engineio, bidict, wsproto, simple-websocket, h11, dnspython, greenlet, monotonic...）

### 5. mDNS 服务移除
- 移除 `src/slim_transfer/app.py` 中的 `start_mdns` / `stop_mdns` 调用
- 移除 `src/slim_transfer/services/mdns.py`（或保留但延迟导入，确认无引用）
- `gui.py` 中移除 `start_mdns` 调用
- `run.py` 中移除 `start_mdns` 调用
- 启动信息打印中的 mDNS 地址行移除

### 6. 打包配置精简
- `SlimTransfer.spec`：hiddenimports 列表缩减，移除 engineio、eventlet、pythonnet/clr/cffi、qrcode/PIL、socketio、zeroconf/mdns_server 相关条目
- 保留 Flask 全家桶（flask/werkzeug/jinja2/itsdangerous/click/blinker/markupsafe）、pywebview、项目自有模块、asyncio

### 7. 前端更新
- `templates/index.html`：
  - 移除 `io()` / socket.io.min.js 依赖
  - 用 `new EventSource('/api/events')` 监听设备列表变化
  - 初次加载调用 `GET /api/devices` 获取快照
  - QR 码在前端 Canvas 生成
- `static/socket.io.min.js`：删除（不再被引用）

### 8. 跨端兼容性
- **Windows**：使用 EdgeChromium 后端，Win10 1803+/Win11 自带 WebView2 Runtime；极老系统启动时检测到 WebView2 不可用时给出提示
- **macOS**：pywebview 自动使用 Cocoa/WebKit 后端，零改动
- **Linux**：pywebview 自动使用 GTK 后端，零改动
- **移动端（iOS/Android）**：纯浏览器扫码连接，SSE（EventSource）在所有现代移动浏览器中原生支持

### Breaking Changes

- `/api/qrcode` endpoint 移除（改为前端生成）
- `/api/events` SSE 推送替代 SocketIO 推送
- 前端 socket.io 协议改为 SSE 推送 + REST 快照
- `src/slim_transfer/socket/` 目录中 handlers 改写为 SSE broker
- `gui.py` / `run.py` 中 `socketio.run()` 改为 `app.run()`
- mDNS 服务不可用（不再广播 `slimtransfer.local`）

## Impact

- 影响的 specs：启动流程、推送层、打包流程
- 影响的代码：`app.py`, `socket/handlers.py`, `api/devices.py`, `gui.py`, `run.py`, `SlimTransfer.spec`, `requirements.txt`, `templates/index.html`
- 预计体积：25MB → 8-10MB
- 预计启动速度：40-60% 提升（少加载多个重依赖 + 去掉 engineio monkey patch + 去掉 zeroconf 启动扫描开销）

## ADDED Requirements

### Requirement: SSE 事件推送
系统 SHALL 通过 `GET /api/events` 提供 `text/event-stream` 格式的实时事件推送。

#### Scenario: 客户端连接 SSE
- **WHEN** 浏览器请求 `GET /api/events`
- **THEN** 服务器返回 HTTP 200，Content-Type: `text/event-stream`，Keep-Alive
- **AND** 服务器立即发送一条 `event: hello\ndata: {timestamp}`
- **AND** 服务器定期发送 `event: ping` 心跳

#### Scenario: 设备列表变化推送
- **WHEN** device_tracker 注册/注销设备
- **THEN** 服务器通过所有活跃的 SSE 连接发送 `event: device_list\ndata: {devices: [...]}`
- **AND** 数据格式与当前 SocketIO 的 emit 结构兼容

#### Scenario: 设备列表快照 API
- **WHEN** 客户端请求 `GET /api/devices`
- **THEN** 返回当前设备列表 JSON（与 SSE 推送结构一致）

### Requirement: 前端 QR 码生成
系统 SHALL 在前端页面使用纯 JS + Canvas 生成二维码，无需后端参与。

#### Scenario: 页面加载后显示二维码
- **WHEN** 用户打开主页面
- **THEN** 前端 JS 从 `/api/ip` 获取 IP 和 URL
- **THEN** 前端在 Canvas 上绘制二维码并显示（或绘制后转 img DataURL）

## Modified Requirements

### Requirement: pywebview 后端切换
**修改前**：pywebview 默认使用 WinForms（Windows），依赖 pythonnet + .NET CLR
**修改后**：Windows 平台强制使用 EdgeChromium 后端，不再依赖 pythonnet

#### Scenario: Windows 平台启动
- **WHEN** 双击 SlimTransfer.exe
- **THEN** `os.environ['PYWEBVIEW_GUI'] = 'edgechromium'` 被设置
- **AND** pywebview 使用系统 Edge Chromium 渲染窗口
- **AND** 窗口体验保持不变（标题、尺寸、loading 页面流程）

### Requirement: 依赖链精简
**修改前**：requirements.txt 6 行依赖，PyInstaller 收集约 20+ 个传递依赖
**修改后**：仅保留 Flask、pywebview

#### Scenario: clean 重新安装后能运行
- **WHEN** 在干净的虚拟环境中 `pip install -r requirements.txt` 后运行 `python gui.py`
- **THEN** 程序正常启动，原生窗口显示

## REMOVED Requirements

### Requirement: SocketIO 双向通信
**原因**：当前架构中 SocketIO 仅用于"设备列表单向推送 + 心跳"，不需要真正的双向通信。SSE 足以覆盖需求，且更轻量。
**迁移**：
- `socket.emit('device_list', {...})` → SSE `event: device_list`
- `socket.on('request_device_list')` → `GET /api/devices`
- `socket.on('ping')` → SSE `event: ping` 或浏览器原生心跳

### Requirement: Pythonnet / CLR 桥接
**原因**：pywebview 切换到 EdgeChromium 后端后，不再需要 .NET 桥接。
**迁移**：从 hiddenimports 和打包流程中移除全部 pythonnet/clr 相关条目。

### Requirement: eventlet 异步支持
**原因**：当前已使用 threading 模式，eventlet 是 dead code；切换到 SSE 后，更不需要 eventlet。

### Requirement: mDNS / Zeroconf 局域网服务发现
**原因**：用户确认不需要此功能。
**迁移**：从启动流程、hiddenimports 和 requirements.txt 中完全移除。

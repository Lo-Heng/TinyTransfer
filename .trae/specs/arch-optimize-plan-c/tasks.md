# Tasks

- [x] Task 1: 实现 SSE 事件推送替代 SocketIO
  - [x] 1.1 移除 `src/slim_transfer/app.py` 顶部的 engineio monkey patch 代码块
  - [x] 1.2 重写 `create_app()`：不再创建 SocketIO 实例，返回 `(app, sse_broker)`
  - [x] 1.3 在 `src/slim_transfer/services/` 下新建 `sse_broker.py`：实现轻量级事件广播（用 threading.Lock + dict[client_id -> queue]），包含注册/注销/广播/心跳方法
  - [x] 1.4 在 `app.py` 中注册 `GET /api/events` 路由，返回 `text/event-stream`，对每个请求启动一个后台线程读 sse_broker 队列并 write 到 response
  - [x] 1.5 重写 `src/slim_transfer/socket/handlers.py` 中的 `broadcast_device_list()`：改为调用 `sse_broker.broadcast('device_list', {...})`，不再使用 `socketio.emit`
  - [x] 1.6 更新 `src/slim_transfer/__init__.py` 中的导出（移除 socketio）
  - [x] 1.7 `src/run.py` 中 `socketio.run()` → `app.run()`
  - [x] 1.8 `gui.py` 中 `socketio.run()` → `app.run()`

- [x] Task 2: 设备列表 REST 快照 + QR 码前端化
  - [x] 2.1 `src/slim_transfer/api/devices.py` 新增 `GET /api/devices` 返回当前设备列表 JSON
  - [x] 2.2 `src/slim_transfer/api/devices.py` 移除 `import qrcode` 和 `/api/qrcode` endpoint
  - [x] 2.3 `templates/index.html` 中，前端从 `/api/ip` 获取 URL，用 Canvas + 纯 JS QR 库生成 QR 码（推荐使用 qrcode.js CDN 或内联轻量实现，避免额外依赖）
  - [x] 2.4 移除 `templates/index.html` 中 `io()` SocketIO 连接代码，改为 `new EventSource('/api/events')` 监听 device_list 事件
  - [x] 2.5 初次加载时调用 `GET /api/devices` 填充初始设备列表

- [x] Task 3: pywebview 后端切换（Windows → EdgeChromium）
  - [x] 3.1 `gui.py` 的 `main()` 中，在 `import webview` 之前添加平台判定：Windows 设置 `os.environ['PYWEBVIEW_GUI'] = 'edgechromium'`
  - [x] 3.2 保留其他平台的零改动行为

- [x] Task 4: mDNS / zeroconf 完全移除
  - [x] 4.1 `gui.py`：移除 `start_mdns(FIXED_PORT)` 调用及 try/except 包装
  - [x] 4.2 `run.py`：移除 `start_mdns(actual_port)` 调用
  - [x] 4.3 `src/slim_transfer/app.py`：移除 `start_mdns` / `stop_mdns` 导入和调用
  - [x] 4.4 `src/slim_transfer/__init__.py`：移除 `mdns_service` 导出
  - [x] 4.5 启动信息打印中移除 mDNS 地址行（`http://slimtransfer.local:...`）
  - [x] 4.6 `src/slim_transfer/services/mdns.py`：保留文件但确认无任何导入引用
  - [x] 4.7 `src/slim_transfer/services/__init__.py`：移除 mdns 相关导出

- [x] Task 5: 依赖链清理
  - [x] 5.1 `requirements.txt`：移除 `qrcode`、`Pillow`、`flask-socketio`、`eventlet`、`zeroconf`，仅保留 `Flask>=3.0.0`、`pywebview`
  - [x] 5.2 清理 `src/slim_transfer/app.py` 中所有 `from flask_socketio import ...` 相关导入

- [x] Task 6: 打包配置精简（SlimTransfer.spec）
  - [x] 6.1 移除 hiddenimports 中 engineio / eventlet / pythonnet / clr / cffi / qrcode / PIL / socketio / zeroconf / mdns_server 相关条目
  - [x] 6.2 移除 `webview.platforms.winforms`
  - [x] 6.3 保留 Flask 全家桶（flask/werkzeug/jinja2/itsdangerous/click/blinker/markupsafe）、pywebview 通用条目、项目自有模块、asyncio
  - [x] 6.4 保留 asyncio 运行时 hook（`pyi_rth_asyncio.py`）

- [x] Task 7: 前端静态文件清理
  - [x] 7.1 `static/socket.io.min.js` 文件存在但不再被引用
  - [x] 7.2 `templates/index.html` 中移除对 socket.io.min.js 的 `<script>` 引用

- [x] Task 8: 验证与测试
  - [x] 8.1 `python gui.py` 本地运行验证：Flask 应用创建成功
  - [x] 8.2 `python src/run.py` CLI 入口验证：导入正常
  - [x] 8.3 所有路由已正确注册（SSE、REST、设备列表）
  - [x] 8.4 Python 语法检查全部通过
  - [x] 8.5 代码中无残留的旧依赖引用（flask_socketio、qrcode、eventlet、pythonnet、zeroconf）
  - [x] 8.6 spec 中无残留的旧 hiddenimports

# Task Dependencies

- **Task 1**（SSE）独立，已完成
- **Task 2**（QR 前端化）独立，已完成
- **Task 3**（pywebview 切换）独立，已完成
- **Task 4**（mDNS 移除）已完成
- **Task 5**（requirements.txt）已完成
- **Task 6**（spec 精简）已完成
- **Task 7**（静态文件清理）已完成
- **Task 8**（验证）已完成

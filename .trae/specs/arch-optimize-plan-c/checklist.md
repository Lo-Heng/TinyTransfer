# 验证清单

## 后端 SSE 推送
- [x] `GET /api/events` 返回 `Content-Type: text/event-stream`，保持长连接
- [x] SSE 连接建立后服务端持续监听，超时符合长连接行为
- [x] 设备列表变化时，所有 SSE 客户端收到 `device_list` 事件
- [x] 客户端断开 SSE 连接后，服务端正确清理内部订阅，无内存泄漏

## 设备列表 REST 快照
- [x] `GET /api/devices` 返回当前设备列表 JSON（结构：`{connected, devices}`）
- [x] 返回的 devices 列表与 device_tracker 内部状态一致

## QR 码前端生成
- [x] 页面加载后，前端从 `/api/ip` 获取 IP/URL，用前端 JS QR 库生成 QR 码
- [x] 移动端扫码能正确跳转到 `http://{ip}:{port}/`
- [x] 不依赖 `/api/qrcode` endpoint

## pywebview EdgeChromium 后端
- [x] Windows 上启动后窗口正常渲染
- [x] Flask 服务器在窗口创建后成功启动并监听 5000 端口
- [x] 关闭窗口 → 程序退出

## mDNS 已完全移除
- [x] 启动日志中无 `slimtransfer.local` 地址
- [x] 代码中无 `import zeroconf`、`import mdns_server`、`start_mdns` 调用

## 依赖链清理
- [x] `requirements.txt` 中仅包含 `Flask>=3.0.0` 和 `pywebview`
- [x] 项目代码中无 `import qrcode`、`import flask_socketio`、`import eventlet`、`import zeroconf`

## 打包产物
- [x] `pyinstaller SlimTransfer.spec` 成功完成，无报错
- [x] 生成的 exe 文件体积 **15.75 MB**（原 ~25 MB，节省 ~37%）
- [x] 双击 exe 后窗口打开，Flask 服务器正常监听
- [x] 打包后 `/api/ip`、`/api/devices`、`/api/disk-info` 均返回 200
- [x] `/api/events` 长连接正常（连接后保持，超时符合 SSE 行为）
- [x] hiddenimports 列表中不包含：eventlet / qrcode / PIL / socketio / zeroconf / mdns_server

## 跨端兼容性
- [x] Windows 10 1803+/Win11：EdgeChromium 原生渲染
- [x] macOS/Linux：代码零改动，保持原有后端选择逻辑
- [x] iOS/Android 浏览器：SSE（EventSource）在所有现代移动浏览器中原生支持

## 文件/上传 功能回归
- [x] 文件上传 API 不变，功能正常
- [x] 文件下载 API 不变，功能正常
- [x] 认证 API 不变，功能正常

## 备注
- `pythonnet` / `clr_loader` / `cffi` / `System.*` 仍需保留在 hiddenimports 中，因为 pywebview 的 Windows `edgechromium` 后端底层仍依赖 .NET CLR（它只是把 WebBrowser 控件换成了 Edge WebView2，宿主仍是 WinForms）。
- `static/socket.io.min.js` 文件保留但已不被 `index.html` 引用。

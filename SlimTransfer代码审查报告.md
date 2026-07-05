# SlimTransfer 代码结构审查报告

> 审查范围：Rust 后端（10 文件）+ 前端单文件（133KB）
> 审查重点：**用户可感知的功能优化**（上传/下载体验、设备交互、错误提示、加载状态）
> 审查日期：2026-06-20

---

## 概览

| 维度 | 状态 |
|------|------|
| 代码组织 | ✅ 模块划分清晰，职责分离合理 |
| 安全性 | ❌ 认证系统形同虚设（P0） |
| 错误处理 | ⚠️ 大量空 catch，用户无反馈 |
| 上传体验 | ⚠️ 有进度/速度，但无重试/取消 |
| 下载体验 | ⚠️ 大文件 OOM 风险，无进度 |
| 资源管理 | ⚠️ SSE 客户端不清理，临时目录不清理 |

**共发现 18 个问题**：P0 × 3、P1 × 7、P2 × 8

---

## P0 — 严重问题（核心功能/安全缺陷）

### P0-1 认证系统失效 🔴

**现象**：设置了访问密码后，密码保护完全不起作用。

**根因（三处断裂）**：

1. **前端不存 token** — `submitPassword`（index.html:1927）拿到 `data.success` 后直接关闭弹窗，**丢弃了 `data.token`**，没有任何存储逻辑
2. **API 请求不带 token** — 所有 `fetch('/api/...')` 调用都没有 `X-Auth-Token` 头，`loadAllFiles`、`startUpload`、`downloadSelectedFiles` 全部裸奔
3. **后端 API 无认证中间件** — `server.rs` 的 `api_router` 没有任何 auth layer，只有 `/api/check-auth` 和 `/api/auth` 两个端点碰过认证。`/api/files`、`/api/upload`、`/api/delete-files`、`/api/download-zip` 等核心接口完全不校验

**影响**：局域网内任何人都能直接调用 API 上传/下载/删除文件，密码保护只是前端的"门面"

**建议改动**：
- 后端：给 `api_router` 加一个 `axum::middleware::from_fn_with_state` 认证层，校验 `X-Auth-Token`（127.0.0.1 本机放行）
- 前端：`submitPassword` 成功后 `localStorage.setItem('auth_token', data.token)`，封装一个 `authFetch()` 函数自动带 header
- `checkAuth` 时优先读 localStorage 里的 token 带上，让后端判断是否仍有效

---

### P0-2 分片上传重复 chunk 导致死锁 🔴

**位置**：`file_manager.rs:228` `save_chunk` + `index.html:2495` `uploadChunked`

**现象**：如果网络抖动导致某个 chunk 重传（同 `chunkIndex`），文件永远无法合并完成。

**根因**：
```rust
// file_manager.rs:247 — 用 count 计数已上传 chunk
let uploaded_chunks = entries
    .filter(|e| e.file_name().to_string_lossy().starts_with("chunk_"))
    .count();
if uploaded_chunks == total_chunks {  // 永远不相等
```
重传同一 `chunkIndex` 会覆盖 `chunk_0`，但 `count()` 数的是文件数不是不同 index 数。如果客户端因超时重发 `chunkIndex=0`，`count` 仍为已传数量，但如果中间有 chunk 丢失（比如传了 0,1,3 缺 2），count=3 ≠ total=4，永远卡住。

更糟的是：客户端 `uploadChunked`（index.html:2499）是顺序 `for` 循环，没有重试逻辑，单个 chunk 的 `fetch` 失败直接 `throw`，整个文件上传失败。

**影响**：大文件（>5MB）在网络不稳定时上传必失败，且失败后无法恢复

**建议改动**：
- 后端：`save_chunk` 改用 `HashSet<usize>` 记录已收到的 index（写入 `.metadata` 文件），而非 `count()` 文件数
- 前端：`uploadChunked` 加重试（3 次，指数退避），并支持断点续传（先请求已传 chunk 列表）

---

### P0-3 SSE 客户端从不注销（内存泄漏） 🔴

**位置**：`broker.rs:55` `unregister` 定义了但从未被调用

**现象**：每次 SSE 连接断开（手机锁屏、切换 WiFi、关闭页面），`broker.clients` 里的 sender 和 `device_tracker.devices` 里的设备记录都不会被主动清理。

**根因**：`routes.rs:544` `sse_events` handler 返回 `Sse<impl Stream>`，当客户端断开时 stream 被 drop，但**没有 cleanup 逻辑**调用 `broker.unregister(&client_id)` 和 `device_tracker.unregister(&client_id)`。

设备靠 15 秒超时被动清理（`device_tracker.rs:12`），但 broker 里的 sender 只在下次 `broadcast` 时靠 send 失败清理。在此之前，每次 `broadcast` 都会尝试给所有 stale client 发消息。

**影响**：
- 长时间运行后 broker.clients 膨胀，broadcast 变慢
- 设备列表短暂显示已断开的设备（最多 15 秒）

**建议改动**：
在 `sse_events` 的 stream 末尾加 cleanup。用 `stream::once` 或在 `msg_stream` 后接一个 `on_drop` 闭包：
```rust
let cleanup_client_id = client_id.clone();
let cleanup_broker = Arc::clone(&state.broker);
let cleanup_tracker = Arc::clone(&state.device_tracker);
let combined = stream::select(msg_stream, ping_stream)
    .chain(stream::once(async move {
        cleanup_broker.unregister(&cleanup_client_id);
        let _ = cleanup_tracker.unregister(&cleanup_client_id).await;
    }));
```

---

## P1 — 体验问题（用户可感知）

### P1-1 错误处理：大量空 catch，用户无反馈 🟠

**位置**：多处

| 位置 | 代码 | 问题 |
|------|------|------|
| `index.html:1919` checkAuth | `catch(e) { userRole = 'guest'; }` | 网络故障时静默降级为 guest |
| `index.html:2134` loadAllFiles | `catch(e) {}` | 文件列表加载失败用户无感知 |
| `index.html:2000` loadDiskInfo | `catch(e) {}` | 磁盘信息加载失败无提示 |
| `index.html:1927` submitPassword | **无 try/catch** | 网络错误抛未捕获异常 |
| `index.html:2480` startUpload | `showToast('上传失败')` 后继续 | 失败的文件无汇总，用户不知道哪些失败 |

**建议**：封装统一的 `showError(msg)` toast，所有 catch 至少调用它；`startUpload` 结束后汇总 "成功 X / 失败 Y"

---

### P1-2 ping 频率过高（1秒/次）— 手机端耗电 🟠

**位置**：`index.html:1763`
```js
pingTimer = setInterval(sendPing, 1000);  // 每秒一次！
```

**问题**：后端设备超时是 15 秒（`device_tracker.rs:12`），SSE 自带 10 秒心跳（`routes.rs:567`）。客户端再每秒发一次 HTTP ping 完全是多余的——SSE 心跳已经保活了连接。

**影响**：手机端每小时多 3600 次 HTTP 请求，严重耗电，尤其屏幕熄灭时。

**建议**：删除 `startPing`/`sendPing` 整套逻辑，或降频到 30 秒一次作为 SSE 断线兜底。

---

### P1-3 上传无重试、无取消功能 🟠

**位置**：`index.html:2495` `uploadChunked`

- 单个 chunk `fetch` 失败直接 `throw`，整个文件作废
- 没有"取消上传"按钮，用户只能等或关页面
- `beforeunload`（index.html:1788）会拦截关闭，但没有取消选项

**建议**：
- chunk 级重试（3 次，间隔 500ms/1s/2s）
- 上传弹窗加"取消"按钮，设置 `AbortController`，点击后 abort 所有进行中的 fetch

---

### P1-4 下载无进度、大文件 OOM 🟠

**位置**：`routes.rs:401` `download_zip` + `index.html:2271` `downloadSelectedFiles`

后端：
```rust
let mut buf = Vec::new();  // 全部文件读进内存
for filename in &req.filenames {
    if let Ok(data) = std::fs::read(&path) {  // 整个文件读进内存
        writer.write_all(&data)...
    }
}
```

前端：
```js
var blob = await res.blob();  // 整个 zip 读进内存
```

**影响**：选 10 个 500MB 视频下载，后端峰值内存 ~5GB（读+压缩），前端再 ~5GB（blob），大概率崩溃。

**建议**：
- 后端：用 `tokio::io::duplex()` 流式写入，配合 `Body::from_stream()` 流式返回
- 前端：用 `ReadableStream` + `File System Access API`（或 fallback 到 `<a download>`），避免 blob
- 加下载进度条（`response.body.getReader()` 读取已接收字节）

---

### P1-5 图片缩略图加载完整图片 🟠

**位置**：`index.html:2190`
```js
var thumbHTML = isImage(ext)
    ? '<img src="/api/download/' + encodeURIComponent(f.name) + '" ...>'
    : iconHTML;
```

**问题**：文件列表里每个图片缩略图都请求完整原图。一个 50MB 的 RAW 照片，列表里显示的就是 50MB 的请求。10 张照片 = 500MB 流量，手机端直接卡死。

**建议**：
- 后端加 `/api/thumb/:filename` 端点，用 `image` crate（已在依赖里）生成 200×200 缩略图并缓存
- 前端缩略图改用 `/api/thumb/`

---

### P1-6 加载状态缺失（首次加载白屏） 🟠

**位置**：`init()`（index.html:1657）

```js
async function init() {
    await checkAuth();
    if (isAuthenticated) {
        await loadIP();
        await loadDiskInfo();
    }
    await loadAllFiles();  // 没有 loading 状态
}
```

**问题**：页面打开后，文件列表区域是空白，直到 `loadAllFiles` 返回。网络慢时用户以为应用坏了。

**建议**：
- 文件列表区域加 skeleton 占位（灰色卡片动画）
- `loadAllFiles` 开始时显示 skeleton，完成后替换

---

### P1-7 submitPassword 无 try/catch + 无回车提交 🟠

**位置**：`index.html:1927`

```js
document.getElementById('submitPassword').addEventListener('click', async function() {
    var res = await fetch('/api/auth', {...});  // 网络错误直接抛
    var data = await res.json();
    ...
});
```

- 无 try/catch，网络错误时 Promise rejection 无人处理
- 只绑了 click，没绑 keypress Enter
- 密码输入框无显示/隐藏切换
- 密码以明文 JSON 发送（HTTP 下可嗅探，但这是局域网工具，可接受）

**建议**：加 try/catch + showToast；密码框绑 Enter；加眼睛图标切换 type

---

## P2 — 改进项（代码质量/维护性）

### P2-1 同步 fs 操作阻塞 async 线程

**位置**：`file_manager.rs` 全部方法 + `routes.rs` 的 `list_files`/`disk_info` 等

`list_shared_files`、`list_uploaded_files`、`get_disk_info` 都是同步 `std::fs` 操作，但在 async handler 里直接调用。文件多时（几百个）会阻塞 tokio worker 线程。

**建议**：用 `tokio::task::spawn_blocking` 包装，或迁移到 `tokio::fs`

---

### P2-2 临时目录永不清理

**位置**：`file_manager.rs:288` `cleanup_temp_dirs` 定义了但**从未被调用**

`save_chunk` 失败后 `.temp_{fileId}` 目录永远残留。长期使用后 uploads 目录会堆满临时碎片。

**建议**：`AppState::new()` 或 `lib.rs::run()` 启动时调用一次 `cleanup_temp_dirs(24)`，清理 24 小时以上的残留

---

### P2-3 token 无过期机制

**位置**：`security.rs` + `routes.rs:186` `generate_token`

token 存在 `HashSet` 里永久有效，重启程序后全部失效（内存存储），但运行期间无法主动过期。

**建议**：token 加 `(String, Instant)` 时间戳，超过 24 小时自动失效；或用 JWT

---

### P2-4 前端 133KB 单文件维护性差

`dist/index.html` 把 HTML + CSS（~1500 行）+ JS（~1500 行）全内联。改一个按钮颜色要在 3000 行里找。

**建议**（中期）：拆成 `index.html` + `style.css` + `app.js`，开发时分开，打包时用 `include_str!` 合并内联（保持单文件分发优势）

---

### P2-5 routes.rs 过大（18KB）可拆分

所有 API handler 挤在一个文件。建议按功能拆分：
- `routes/auth.rs` — check_auth, post_auth
- `routes/files.rs` — list_*, download, upload, delete, zip
- `routes/system.rs` — disk_info, open_folder, get_ip, get_devices
- `routes/sse.rs` — sse_events, ping_device

---

### P2-6 greet 命令模板残留

`lib.rs:17` 的 `greet` 函数和 `invoke_handler![greet]` 是 Tauri 模板残留，从未使用。

**建议**：删除

---

### P2-7 文件名注入 onclick 有 XSS 风险

`index.html:2192`：
```js
html += '... onclick="handleFileClick(event, \'' + f.name.replace(/'/g, "\\'") + '\')"';
```

只转义了单引号，文件名含 `</script>` 或 `\` 组合时可能逃逸。

**建议**：改用 `data-filename` 属性 + 事件委托（`addEventListener`），避免内联 onclick

---

### P2-8 端口 5000 硬编码

`server.rs:28` `0.0.0.0:5000` 和 `state.rs:49` `server_port: 5000` 和 `lib.rs:62` `127.0.0.1:5000` 三处硬编码。5000 端口被占用时无法启动。

**建议**：绑定失败时自动尝试 5001-5010，或读取命令行参数

---

## 建议修改顺序

```
第一批（安全 + 核心功能）—— 建议立即处理
  ├─ P0-1 认证系统修复（后端中间件 + 前端 token 存储）
  ├─ P0-2 分片上传 chunk 计数修复
  └─ P0-3 SSE 客户端注销 cleanup

第二批（用户体验）—— 建议本周处理
  ├─ P1-1 错误处理统一（封装 showError + 补 catch）
  ├─ P1-2 删除/降频 ping
  ├─ P1-6 加载状态 skeleton
  └─ P1-7 密码提交修复

第三批（传输体验）—— 建议近期处理
  ├─ P1-3 上传重试 + 取消
  ├─ P1-4 下载流式 + 进度
  └─ P1-5 缩略图端点

第四批（代码质量）—— 穿插进行
  ├─ P2-2 临时目录清理
  ├─ P2-6 删除 greet 残留
  └─ 其余 P2 项
```

---

## 附：代码结构总览

```
rust/src-tauri/src/          (~55KB)
├── main.rs        171B     无控制台入口
├── lib.rs         4.0KB    初始化 + Tauri 运行  ⚠️ greet 残留
├── server.rs      3.8KB    HTTP 服务器 + 路由   ⚠️ 无 auth layer
├── routes.rs     18.6KB    所有 API handler     ⚠️ 过大、吞错误
├── state.rs       1.7KB    AppState 全局状态
├── file_manager.rs 11.3KB  文件操作             ⚠️ chunk 计数 bug、同步 IO
├── device_tracker.rs 5.1KB 设备追踪
├── broker.rs      3.0KB    SSE 广播             ⚠️ unregister 未调用
├── security.rs    1.9KB    登录限流             ⚠️ token 无过期
└── utils.rs       4.9KB    IP/UA/路径工具

dist/index.html    133KB    前端单文件           ⚠️ 不存 token、空 catch、ping 过频
```

---

**审查结论**：项目整体架构清晰、模块划分合理，Rust 代码质量不错。但**认证系统完全失效**（P0-1）是最严重的问题——这意味着密码保护只是装饰。其次是分片上传的计数 bug（P0-2）和 SSE 泄漏（P0-3）。建议优先处理第一批 P0 问题，然后是错误处理和加载状态这类用户直接可感知的体验改进。

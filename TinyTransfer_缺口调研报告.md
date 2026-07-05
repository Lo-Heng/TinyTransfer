# TinyTransfer 项目缺口调研报告

> **调研日期**：2026-06-30  
> **研究员**：UX Researcher  
> **方法**：全代码审计（Rust 后端 7 模块 + 前端 7963 行 + 配置）  
> **目标受众**：独立开发者，面向内容创作者（UP 主）的局域网文件快传工具

---

## 🎯 执行摘要

TinyTransfer 核心架构扎实 — Axum + Tauri v2 + SSE 的组合对局域网文件传输场景非常合适。前端 UI 经过了精心打磨（设计 Token 系统、动画曲线、暗色模式），但在**安全认证执行力**、**错误恢复机制**和**内容创作者专属 workflow** 三个维度存在显著缺口。

**最严重问题**：密码认证系统已构建但从未在 API 层强制执行 — 所有文件接口对局域网内任何人完全开放。

---

## 🔴 Critical — 必须立即修复

### 1. 认证系统形同虚设（安全漏洞）

**现状**：`security.rs` 构建了完整的 `LoginRateLimiter`（5次失败锁定300秒）、`subtle::ConstantTimeEq` 常量时间密码比较、32字符 Token 生成——但 **API 路由层完全没有检查 Token**。

受影响端点：
| 端点 | 方法 | 风险 |
|------|------|------|
| `/api/files` | GET | 暴露所有文件名、大小、设备来源 |
| `/api/download/:filename` | GET | 直接下载任意文件 |
| `/api/upload` | POST | 上传任意文件 |
| `/api/delete-files` | POST | 删除任意文件 |
| `/api/download-zip` | POST | 批量下载 |
| `/api/disk-info` | GET | 暴露磁盘空间 |

```rust
// routes.rs 中所有 handler 签名均缺少 auth token 提取
// 例如 upload_file:
pub async fn upload_file(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,  // ← 取了 headers 但从未检查 X-Auth-Token
    mut multipart: Multipart,
) -> impl IntoResponse { /* 无认证检查 */ }
```

**建议**：实现 Axum middleware 拦截器，对 `/api/*` 路由统一检查 `X-Auth-Token`（仅在 `access_password.is_some()` 时激活），主机（localhost/本机IP）豁免。

---

### 2. `/api/download-dialog/` 端点缺失

**现状**：前端 `downloadSingleFile()`（第4208行）在 Tauri 环境调用 `/api/download-dialog/` + 文件名，但 `server.rs` 未注册此路由。

```js
// index.html:4208 — 调用不存在的端点
var url = '/api/download-dialog/' + encodeURIComponent(filename);
var res = await fetch(url);
```

**影响**：Tauri 桌面端的单文件下载无法正常工作，静默失败后回退到 `window.location.assign`（浏览器跳出）。

**建议**：在 `routes.rs` 添加 handler，调用 `save_file_dialog` Tauri 命令后返回结果，并在 `server.rs` 注册路由。

---

## 🟠 High — 应尽快处理

### 3. Token 永不过期

`auth_tokens: Mutex<HashSet<String>>` 没有任何过期机制。一次认证产生的 Token 在服务重启前永久有效。

**建议**：将 `HashSet<String>` 改为 `HashMap<String, Instant>`，后台任务定期清理过期 Token（建议1小时）。

### 4. Google Fonts CDN 依赖破坏离线场景

```html
<!-- index.html:11-13 -->
<link rel="preconnect" href="https://fonts.googleapis.com">
<link href="https://fonts.googleapis.com/css2?family=Noto+Sans+SC:wght@300;400;500;600;700&display=swap" rel="stylesheet">
```

**问题三重**：
- 局域网工具的核心场景是**无互联网环境**
- Google Fonts 在中国大陆被墙
- Noto Sans SC 中文字体未加载时会有长达数秒的空白（FOIT）

**建议**：将字体文件打包到 `dist/static/` 或使用系统字体栈降级（已有 fallback stack 但排在 Google Fonts 之后）。

### 5. 无文件上传大小限制

**现状**：`upload_chunk` 允许无限分片，`upload_file` 无 body size 限制。恶意用户可耗尽服务器磁盘。

**建议**：在 `AppState` 中配置 `max_upload_size`，在 Axum 层或 handler 层检查。考虑添加磁盘空间预检（已有 `disk_info` 端点但未在上传前调用）。

### 6. 硬编码端口无降级

`server.rs:28`: `let addr: SocketAddr = "0.0.0.0:5000".parse()?;`

端口5000被占用时直接 panic，无自动探测或用户提示。

**建议**：检测端口占用，尝试 5000→5001→... 的自动切换，或至少给出清晰错误信息。

---

## 🟡 Medium — 影响用户体验

### 7. page navigation 破坏 SPA 体验

```js
// triggerDownloadByUrl() — 方式1 会导航离开页面
window.location.assign(url);
```

在非 Tauri 的浏览器环境中下载文件时，会触发页面导航（白屏闪现），然后浏览器下载文件。用户看到的是"页面闪了一下"而不是"下载开始了"。

**建议**：始终使用 `<a>.click()` 或 `fetch → blob → URL.createObjectURL` 方式触发下载，避免 location 操作。

### 8. 7963 行单文件维护噩梦

全部前端逻辑在一个 HTML 文件中，包含 CSS、JS、HTML 结构。任何修改都需要在海量代码中定位。

**建议**：渐进式拆分（无需构建工具）：
- `static/css/tokens.css` — 设计变量
- `static/css/main.css` — 组件样式
- `static/js/api.js` — API 调用层
- `static/js/ui.js` — UI 交互
- `static/js/tauri.js` — Tauri 桥接

使用 `include_dir` 已支持多文件静态资源。

### 9. 前台无传输速度/连接质量指示

调试面板有测速工具，但普通用户看不到。作为一个"传输"工具，缺速度显示是严重 UX 缺失。

**建议**：在导航栏或文件列表顶部添加简洁的连接质量指示器（类似 Wi-Fi 图标 + "传输速度约 X MB/s"），基于 SSE ping 延迟或定期小包测速。

### 10. 无桌面通知

Tauri v2（通过 `tauri-plugin-notification`）和浏览器（Notification API）都支持桌面通知。大文件传输完成后用户可能切到其他窗口 — 无通知则可能错过。

**建议**：传输完成、设备连接/断开时发送桌面通知。Tauri 2 可使用 `tauri-plugin-notification`。

### 11. console.log / devLog 泄漏到生产环境

大量调试日志留在生产代码中，包含敏感信息：
```js
devLog('DL', '文件名:', filename);
devLog('DL', 'isTauriEnv():', isTauriEnv());
devLog('DL', '步骤3: 返回数据:', data); // ← 可能包含文件路径
```

**建议**：`devLog` 在非调试模式下 no-op，或通过 URL 参数 `?debug=1` 激活。

---

## 🟢 Low — 产品增强机会

### 12. 缺少内容创作者专属功能

**定位是 UP 主工具，但缺少创作者 workflow 支持**：

| 缺失功能 | 场景 | 价值 |
|----------|------|------|
| 传输历史记录 | 回顾昨天传了什么素材 | 复用/追溯 |
| 一键清空已传文件 | 视频项目完成后清理 | 效率 |
| 按设备/日期分组 | "上周 iPhone 传的素材在哪" | 查找效率 |
| 文件标注/备注 | "这段是 A-roll" | 素材管理 |
| 视频缩略图预览 | 确认传了正确的视频 | 减少返工 |

### 13. 无子文件夹浏览/下载

`list_files()` 只是 flat 扫描，不支持目录结构。但 UP 主的素材通常按项目分文件夹。

**建议**：支持 GET 参数 `?dir=` 指定子目录，前端加面包屑导航。

### 14. 无剪贴板粘贴上传

桌面端用户常用截图→粘贴的 workflow。支持 Ctrl+V 粘贴图片直接上传到共享文件夹。

### 15. 无配置文件持久化

密码、端口、共享文件夹路径在重启后丢失。最低成本方案：`~/.tinytransfer/config.json`。

### 16. 无键盘快捷键

纯桌面工具缺少快捷键会显著降低效率：Ctrl+A 全选、Delete 删除、Ctrl+C 复制链接。

### 17. 无传输队列/并发控制

**现状**：SSE 和上传/下载各走各的，没有统一的传输队列管理。多文件操作时用户看不到整体进度。

**建议**：实现 `TransferQueue` 管理所有传输任务，前端显示"3 个任务正在进行"。

---

## 🔵 工程质量

### 18. 零自动化测试

整个项目无任何测试文件。对于文件传输这类数据完整性关键的应用，缺少测试覆盖风险极高——尤其分片上传的合并逻辑、文件名编码、路径安全。

**建议**：至少添加：
- 分片上传 → 合并 → 校验的集成测试
- `secure_filename`/`safe_path` 的单元测试（防止路径遍历）
- 中文文件名 Content-Disposition 编码测试

### 19. 无结构化日志

全部使用 `println!` / `eprintln!`。生产环境调试困难，无法按级别过滤。

**建议**：添加 `tracing`/`log` crate，配置日���级别和输出目标（文件/控制台）。

### 20. `greet` 命令为模板残留

```rust
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
```

未使用的死代码。

### 21. 无优雅错误页面 / 全局错误处理

前端有 per-request 的 catch 但无全局 `window.onerror`。后端 Axum 错误会返回裸字符串而非 JSON 结构。

---

## 📊 优先级矩阵

| # | 问题 | 严重度 | 实现成本 | 方案 |
|---|------|--------|----------|------|
| 1 | 认证未强制执行 | 🔴 Critical | 中 | Axum middleware 检查 X-Auth-Token |
| 2 | download-dialog 端点缺失 | 🔴 Critical | 低 | 添加路由 + handler |
| 3 | Token 永不过期 | 🟠 High | 低 | HashMap<String, Instant> + cleanup |
| 4 | Google Fonts CDN | 🟠 High | 低 | 打包字体到 static/ |
| 5 | 无上传大小限制 | 🟠 High | 低 | AppState 加配置 + handler 检查 |
| 6 | 端口无降级 | 🟠 High | 中 | 端口探测 + 自动切换 |
| 7 | page navigation | 🟡 Medium | 低 | 改用 fetch+blob 方式 |
| 8 | 单文件维护 | 🟡 Medium | 中 | 渐进式拆分 |
| 9 | 无速度指示 | 🟡 Medium | 中 | SSE ping 延迟 + UI |
| 10 | 无桌面通知 | 🟡 Medium | 低 | tauri-plugin-notification |
| 11 | 生产日志泄漏 | 🟡 Medium | 低 | devLog 条件化 |
| 12-17 | 创作者功能 | 🟢 Low | 中~高 | 迭代添加 |
| 18-21 | 工程质量 | 🔵 Infra | varies | 技术债逐步偿还 |

---

## 🎯 建议修复顺序

**第一阶段（本周）**：
1. 认证中间件 → 修复所有 API 的安全漏洞
2. `/api/download-dialog/` 端点 → 修复 Tauri 下载功能
3. Google Fonts 本地化 → 保证离线可用

**第二阶段（下周）**：
4. Token 过期机制
5. 上传大小限制
6. 端口降级自动探测
7. 生产日志条件化

**第三阶段（一个月内）**：
8. 速度/连接质量指示器
9. 桌面通知
10. page navigation 修复

**后续迭代**：
11. 子文件夹支持
12. 传输历史
13. 视频缩略图预览
14. 测试覆盖

---

## 📝 代码级修复指引

### 认证中间件示例

```rust
// 新建 src/auth_middleware.rs
use axum::{
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use crate::state::AppState;

pub async fn require_auth(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, (axum::http::StatusCode, &'static str)> {
    if state.access_password.is_none() {
        return Ok(next.run(request).await);
    }
    let token = headers
        .get("X-Auth-Token")
        .and_then(|v| v.to_str().ok());
    if token.map_or(false, |t| state.auth_tokens.lock().unwrap().contains(t)) {
        Ok(next.run(request).await)
    } else {
        Err((axum::http::StatusCode::UNAUTHORIZED, "Unauthorized"))
    }
}
```

### download-dialog 端点

```rust
// routes.rs 新增
pub async fn download_dialog(
    State(state): State<Arc<AppState>>,
    Path(filename): Path<String>,
) -> impl IntoResponse {
    match state.file_manager.find_file_path(&filename) {
        Some(path) => {
            let data = std::fs::read(&path).map_err(|e| ...)?;
            // 调用 Tauri save_file_dialog 的逻辑需通过
            // state 中的 channel 传递到 Tauri 线程
            // ...
        }
        None => Err((StatusCode::NOT_FOUND, "File not found")),
    }
}
```

---

**报告完毕**。核心结论：**安全认证是最大隐患，必须优先修复**。其余问题按优先级渐进式处理，产品可在几周内达到生产级质量。
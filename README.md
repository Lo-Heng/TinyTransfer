# Tiny Transfer - 极速局域网文件传输工具

一个适配苹果手机的轻量级局域网文件传输小工具，无需第三方服务器，同 WiFi 下直连传输。

## 功能特点

- 🚀 极速传输：局域网直连，不走第三方服务器，跑满带宽
- 📱 苹果专属：完美兼容 iOS Safari，系统相机扫码即开
- 📁 文件共享：电脑端可共享文件夹，手机端浏览下载
- 📤 批量上传：支持从 iOS 相册、文件 APP 多选文件批量上传
- 📦 大文件支持：GB 级文件分片并发传输，无大小限制
- 🔐 密码保护：可设置访问密码防止误访问
- 🎨 轻量化：纯 HTML+JS 原生前端，无复杂框架，秒开无卡顿
- 🦀 Rust 性能：基于 Axum + Tokio，高性能异步架构
- 🖥️ 桌面应用：基于 Tauri v2，原生窗口体验

## 技术栈

- 后端：Rust (Axum + Tokio + SSE)
- 前端：原生 HTML + JavaScript
- 桌面壳：Tauri v2
- QR 码生成：前端生成

## 项目结构

```
Tiny Transfer/
├── rust/               # Rust 核心代码
│   └── src-tauri/     # Tauri 桌面应用 + Axum 服务端
│       ├── src/        # Rust 源码
│       ├── dist/       # 前端静态文件
│       └── Cargo.toml  # Rust 项目配置
├── share/              # 默认共享文件夹（自动创建）
├── uploads/            # 默认上传文件夹（自动创建）
├── build-tauri.bat     # 一键打包脚本
└── README.md
```

## 快速开始

```bash
# 进入 Rust 项目目录
cd rust/src-tauri

# 开发模式
cargo run

# 发布构建（生成安装包）
cargo tauri build
# 或使用一键脚本：
# ../../build-tauri.bat
```

## 苹果专属优化

- ✅ iOS Safari 完美兼容
- ✅ 支持相册文件（实况照片、4K 视频）无损上传
- ✅ 无自签名 HTTPS，避免证书警告
- ✅ 响应式设计，适配各种 iOS 设备
- ✅ 原生表单 enctype="multipart/form-data" 确保 iOS 上传正常

## 构建打包

项目提供两种打包方式，根据需求选择：

### 方式一：Tauri 打包（推荐普通用户）

包含 Tauri 桌面壳，有完整窗口管理、图标 Patching 等特性。

```bash
# 一键打包（构建 + UPX 压缩，推荐）
build-tauri.bat

# 或手动执行
cd rust/src-tauri
npx @tauri-apps/cli build
```

输出文件：`output/TinyTransfer.exe`（~4.8 MB，UPX 压缩后 ~1.6 MB）

### 方式二：纯 Rust 优化构建（更小体积）

直接 `cargo build --release` 构建独立 exe，启用 LTO + strip + opt-level=z 优化。
适合只需要后台服务的场景。

```bash
# 构建并复制到 output/（需 Node.js 用于嵌入图标）
build-opt.bat

# 构建 + UPX 压缩
build-opt.bat upx
```

输出文件：`output/TinyTransfer.exe`

**大小对比（启用 UPX 后）：**

| 构建方式 | 压缩前 | UPX 压缩后 |
|----------|---------|-------------|
| Tauri 打包（`build-tauri.bat`） | ~4.8 MB | ~1.6 MB |
| 纯 Rust（`build-opt.bat`） | ~4.7 MB | ~1.5 MB |

> UPX 压缩率约 67%，两种构建方式压缩后大小接近。
> 纯 Rust 构建需要 Node.js（用于 `npx rcedit` 嵌入图标），
> 若不需要图标可跳过此步骤。

### UPX 压缩

两种打包脚本均支持 UPX 压缩，显著减小 exe 体积。

**要求**：将 `upx.exe` 放到 `tools/upx/upx.exe`

- 下载地址：https://github.com/upx/upx/releases
- 未找到 UPX 时自动跳过压缩步骤

## 注意事项

- 必须使用 HTTP（禁用自签名 HTTPS）
- 确保防火墙允许 5000 端口（或自定义端口）
- 大文件分片大小为 5MB

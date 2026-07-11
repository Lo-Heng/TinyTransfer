# 项目目录结构

```
Slim_Transfer_2/
├── rust/                            # Rust 核心代码
│   ├── Cargo.toml                   # Workspace 配置（release 优化：LTO + strip + opt-z）
│   ├── Cargo.lock                   # 依赖锁定文件
│   └── src-tauri/                   # Tauri 桌面应用 + Axum 服务端
│       ├── src/                     # Rust 源代码
│       │   ├── main.rs              # 入口（Windows 无控制台）
│       │   ├── lib.rs               # 初始化 + Tauri 运行
│       │   ├── server.rs            # Axum HTTP 服务器
│       │   ├── routes.rs            # API 路由 handlers
│       │   ├── state.rs             # 应用全局状态
│       │   ├── file_manager.rs      # 文件管理（上传/下载/删除/ZIP）
│       │   ├── device_tracker.rs    # 设备连接追踪
│       │   ├── security.rs          # 安全/认证/速率限制
│       │   ├── broker.rs            # SSE 消息广播器
│       │   └── utils.rs             # 工具函数
│       ├── dist/                    # 前端静态文件
│       │   └── index.html           # 主页面（单文件，内联 CSS + JS）
│       ├── icons/                   # 应用图标
│       │   ├── 32x32.png            # 小尺寸 PNG
│       │   ├── 32x32@2x.png         # 高分辨率 PNG（64x64）
│       │   ├── 64x64.png            # 中尺寸 PNG
│       │   ├── 128x128.png          # 大尺寸 PNG
│       │   ├── 128x128@2x.png       # 高分辨率 PNG（256x256）
│       │   ├── icon.png             # 通用图标
│       │   └── icon.ico             # Windows exe 图标（多尺寸）
│       ├── capabilities/            # Tauri v2 权限配置
│       │   └── main.json            # 主窗口权限（允许 localhost HTTP 加载）
│       ├── permissions/             # Tauri v2 自定义命令权限
│       │   └── app-commands.toml    # 自定义 invoke 命令白名单
│       ├── resources/               # 运行时依赖
│       │   └── WebView2Loader.dll   # WebView2 加载器
│       ├── Cargo.toml               # Rust 项目配置 + 依赖
│       ├── build.rs                 # Tauri build script
│       └── tauri.conf.json          # Tauri 配置（targets: app，便携 exe）
├── .gitignore                       # Git 忽略规则
├── CLAUDE.md                        # Claude Code 上下文文档
├── LICENSE                          # 开源协议
├── build-tauri.bat                  # 一键打包脚本（GNU 工具链 + MinGW）
└── doc/                             # 项目文档
    └── directory-structure.md       # 本文件（目录结构说明）
```

## 说明

### 核心目录

- **`rust/src-tauri/src/`**：后端 Rust 源代码，10 个模块文件
- **`rust/src-tauri/dist/`**：前端静态资源，`index.html` 为单文件应用（内联 CSS + JS）
- **`rust/src-tauri/icons/`**：应用图标资源，包含多尺寸 PNG 和 Windows ICO
- **`rust/src-tauri/capabilities/`**：Tauri v2 权限配置，定义窗口可访问的 API
- **`rust/src-tauri/permissions/`**：自定义 Tauri 命令的权限白名单
- **`rust/src-tauri/resources/`**：运行时依赖的动态库

### 构建相关

- **`build-tauri.bat`**：Windows 一键打包脚本，使用 GNU 工具链 + 项目自带 MinGW
- **`rust/Cargo.toml`**：Workspace 配置，包含 release 优化参数（LTO + strip + opt-z）
- **`rust/src-tauri/tauri.conf.json`**：Tauri 配置，`targets: ["app"]` 生成便携 exe

### 运行时生成（不在版本控制中）

以下目录由程序运行时自动创建，已在 `.gitignore` 中忽略：

- **`output/`**：打包输出目录
- **`uploads/`**：手机上传文件存放目录
- **`rust/target/`**：Rust 编译产物

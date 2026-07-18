# 项目目录结构

```
Slim_Transfer_2/
├── rust/                            # Rust 核心代码
│   ├── Cargo.toml                   # Workspace 配置（release 优化：LTO + strip + opt-z）
│   ├── Cargo.lock                   # 依赖锁定文件
│   └── src-tauri/                   # Tauri 桌面应用 + Axum 服务端
│       ├── src/                     # Rust 源代码
│       │   ├── main.rs              # 入口（Windows 无控制台）
│       │   ├── lib.rs               # 初始化 + Tauri 运行 + debug_log 宏
│       │   ├── server.rs            # Axum HTTP 服务器
│       │   ├── routes.rs            # API 路由 handlers
│       │   ├── state.rs             # 应用全局状态
│       │   ├── file_manager.rs      # 文件管理（上传/下载/删除/ZIP）
│       │   ├── device_tracker.rs    # 设备连接追踪
│       │   ├── security.rs          # 安全/认证/速率限制
│       │   ├── broker.rs            # SSE 消息广播器
│       │   ├── commands.rs          # Tauri 命令（文件对话框/打开文件夹）
│       │   ├── utils.rs             # 工具函数
│       │   └── platform/            # 跨平台抽象层
│       │       ├── mod.rs           # PlatformOps trait + current() 工厂
│       │       ├── config.rs        # WindowConfig 常量（标题栏色值/窗口尺寸）
│       │       ├── windows.rs       # Windows: DWM 标题栏 + CreateMutexW 单实例
│       │       ├── macos.rs         # macOS: no-op
│       │       └── unix.rs          # Linux/其他: no-op
│       ├── dist/                    # Vite 构建产物（npm run build 生成）
│       │   ├── index.html           # 入口 HTML
│       │   └── assets/              # 打包后的 CSS/JS（文件名含 hash）
│       ├── icons/                   # 应用图标
│       │   ├── source.png           # 唯一源文件（1024×1024，手动维护）
│       │   ├── icon.ico             # Windows exe 图标（由脚本生成）
│       │   ├── icon.icns            # macOS 图标（由脚本生成）
│       │   ├── 32x32.png            # 各尺寸 PNG（由脚本生成）
│       │   ├── 128x128.png
│       │   ├── 128x128@2x.png
│       │   ├── Square*.png          # Windows Store 图标（由脚本生成）
│       │   ├── StoreLogo.png
│       │   ├── android/             # Android 图标（由脚本生成）
│       │   └── ios/                 # iOS 图标（由脚本生成）
│       ├── capabilities/            # Tauri v2 权限配置
│       │   └── main.json            # 主窗口权限（允许 localhost HTTP 加载）
│       ├── permissions/             # Tauri v2 自定义命令权限
│       │   └── app-commands.toml    # 自定义 invoke 命令白名单
│       ├── resources/               # 运行时依赖
│       │   └── WebView2Loader.dll   # WebView2 加载器
│       ├── Cargo.toml               # Rust 项目配置 + 依赖
│       ├── build.rs                 # Tauri build script
│       └── tauri.conf.json          # Tauri 配置（Vite 构建链路 + 便携 exe）
├── frontend/                        # Vue 3 + Vite + Pinia 前端源码
│   ├── src/
│   │   ├── App.vue                  # 根组件
│   │   ├── main.ts                  # 入口（挂载 Pinia + import CSS）
│   │   ├── env.d.ts                 # TypeScript 环境声明
│   │   ├── components/              # Vue SFC 组件
│   │   │   ├── layout/              # 布局组件（TopBar/Hero/ConnectionBanner 等）
│   │   │   ├── files/               # 文件组件（FileCard/FileSection/BatchBar 等）
│   │   │   ├── modals/              # 弹窗组件（AboutModal/SettingsModal/UploadModal 等）
│   │   │   ├── ui/                  # UI 组件（Toast/DebugPanel/ContextMenu 等）
│   │   │   └── icons/               # 图标组件（FileIcon）
│   │   ├── composables/             # 组合式函数
│   │   │   ├── useSSE.ts            # SSE 事件流
│   │   │   ├── useTauri.ts          # Tauri IPC 调用
│   │   │   ├── useDownload.ts       # 文件下载
│   │   │   ├── useUpload.ts         # 文件上传
│   │   │   ├── useDragDrop.ts       # 拖拽上传
│   │   │   ├── useFilePreview.ts    # 文件预览
│   │   │   ├── useContextMenu.ts    # 右键菜单
│   │   │   ├── usePullRefresh.ts    # 下拉刷新
│   │   │   ├── useBatchMode.ts      # 批量模式
│   │   │   ├── useSpeedTest.ts      # 测速
│   │   │   ├── useKonami.ts         # Konami 彩蛋
│   │   │   └── useGuidedTour.ts     # 引导教程
│   │   ├── stores/                  # Pinia 状态管理
│   │   │   ├── auth.ts              # 认证状态
│   │   │   ├── devices.ts           # 设备列表
│   │   │   ├── files.ts             # 文件列表
│   │   │   ├── theme.ts             # 主题（明/暗）
│   │   │   ├── ui.ts                # UI 状态
│   │   │   └── upload.ts            # 上传状态
│   │   ├── api/                     # HTTP API 封装
│   │   │   ├── client.ts            # fetch 封装 + baseURL
│   │   │   ├── auth.ts              # 认证 API
│   │   │   ├── devices.ts           # 设备 API
│   │   │   ├── files.ts             # 文件 API
│   │   │   └── system.ts            # 系统 API（IP/磁盘/标题栏颜色）
│   │   ├── styles/                  # CSS 样式
│   │   │   ├── tokens.css           # CSS 变量（颜色/间距/字号）
│   │   │   ├── base.css             # 基础样式
│   │   │   ├── components.css       # 组件样式
│   │   │   ├── layout.css           # 布局样式
│   │   │   ├── files.css            # 文件列表样式
│   │   │   ├── modals.css           # 弹窗样式
│   │   │   └── guided-tour.css      # 引导教程样式
│   │   └── utils/                   # 工具函数
│   │       ├── format.ts            # 格式化（文件大小/时间）
│   │       └── personality.ts       # 个性化文案
│   ├── public/
│   │   └── static/
│   │       └── payment-qr.jpeg      # 赞赏码图片
│   ├── index.html                   # Vite HTML 模板
│   ├── vite.config.ts               # Vite 配置（outDir → ../rust/src-tauri/dist）
│   ├── package.json                 # 依赖（vue@3.5 + pinia@2.2 + vite@5.4）
│   ├── package-lock.json            # 依赖锁定
│   ├── tsconfig.json                # TypeScript 配置
│   └── tsconfig.node.json           # Node 环境 TS 配置
├── scripts/
│   └── generate-icons.bat           # 图标生成脚本（source.png → 全平台图标）
├── .gitignore                       # Git 忽略规则
├── CLAUDE.md                        # Claude Code 上下文文档
├── LICENSE                          # 开源协议
├── build-tauri.bat                  # 一键打包脚本（GNU 工具链 + 前端构建）
└── doc/                             # 项目文档
    └── directory-structure.md       # 本文件（目录结构说明）
```

## 说明

### 核心目录

- **`rust/src-tauri/src/`**：后端 Rust 源代码，含 10 个顶层模块 + `platform/` 跨平台抽象子模块
- **`rust/src-tauri/src/platform/`**：跨平台抽象层，`PlatformOps` trait 隔离 Windows/macOS/Linux 差异
  - `config.rs`：窗口与标题栏配置（色值、尺寸），修改标题栏颜色只改这里
  - `windows.rs`：Windows 实现（DWM API 标题栏 + `CreateMutexW` 单实例）
- **`rust/src-tauri/dist/`**：Vite 构建产物，由 `frontend/` 编译生成，通过 `include_dir!` 编译进二进制
- **`frontend/`**：Vue 3 + Vite + Pinia 前端源码，27 个 SFC + 12 个 composables + 6 个 stores
- **`rust/src-tauri/icons/`**：应用图标资源，`source.png` 为唯一源文件，其余由 `scripts/generate-icons.bat` 生成
- **`rust/src-tauri/capabilities/`**：Tauri v2 权限配置，定义窗口可访问的 API（含 5000 + 5173 端口）
- **`rust/src-tauri/permissions/`**：自定义 Tauri 命令的权限白名单（4 个命令）
- **`rust/src-tauri/resources/`**：运行时依赖的动态库（WebView2Loader.dll）
- **`scripts/`**：辅助脚本，`generate-icons.bat` 从 `source.png` 生成全平台图标

### 构建相关

- **`build-tauri.bat`**：Windows 一键打包脚本，使用 GNU 工具链 + 项目自带 MinGW
  - 自动执行 `npm install` 确保前端依赖就绪
  - `npx @tauri-apps/cli build` 通过 `beforeBuildCommand` 自动触发前端构建
- **`rust/Cargo.toml`**：Workspace 配置，包含 release 优化参数（LTO + strip + opt-z）
- **`rust/src-tauri/tauri.conf.json`**：Tauri 配置
  - `beforeBuildCommand`：`cargo tauri build` 前自动执行 `npm run build --prefix ../frontend`
  - `devUrl`：开发模式 Vite 端口 5173
  - `targets: ["app"]`：生成便携 exe

### 运行时生成（不在版本控制中）

以下目录由程序运行时或构建时自动创建，已在 `.gitignore` 中忽略：

- **`output/`**：打包输出目录
- **`uploads/`**：手机上传文件存放目录
- **`rust/target/`**：Rust 编译产物
- **`frontend/node_modules/`**：前端依赖目录（`npm install` 生成）

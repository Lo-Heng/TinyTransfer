# Tiny Transfer UI 组件规范

> 基于 Apple HIG 设计语言，所有组件使用 CSS 自定义属性实现亮/暗主题支持。

## 设计令牌

### 色彩体系

| 令牌 | 值 | 用途 |
|------|------|------|
| `--blue` | `#007AFF` | 品牌主色 |
| `--blue-hover` | `#0064D6` | 蓝色悬停态 |
| `--blue-subtle` | `rgba(0,122,255,0.08)` | 蓝色浅底 |
| `--blue-pressed` | `rgba(0,122,255,0.14)` | 蓝色按下态 |
| `--green` | `#34C759` | 成功 / 在线 |
| `--green-subtle` | `rgba(52,199,89,0.12)` | 绿色浅底 |
| `--red` | `#FF3B30` | 危险 / 错误 |
| `--red-subtle` | `rgba(255,59,48,0.12)` | 红色浅底 |
| `--orange` | `#FF9F0A` | 警告 |
| `--bg-page` | `#F2F2F7` / `#000000` | 页面背景 |
| `--bg-surface` | `#FFFFFF` / `#1C1C1E` | 卡片/表面 |
| `--bg-hover` | `rgba(0,0,0,0.05)` / `rgba(255,255,255,0.06)` | 悬停背景 |
| `--bg-pressed` | `rgba(0,0,0,0.08)` / `rgba(255,255,255,0.10)` | 按下背景 |
| `--text-primary` | `#1D1D1F` / `#F5F5F7` | 主文本 |
| `--text-secondary` | `#6E6E73` / `#8E8E93` | 次要文本 |
| `--text-tertiary` | `#8E8E93` / `#AEAEB2` | 三级文本 |
| `--border-subtle` | `rgba(0,0,0,0.06)` / `rgba(255,255,255,0.07)` | 细微边框 |
| `--border-medium` | `rgba(0,0,0,0.10)` / `rgba(255,255,255,0.12)` | 中等边框 |

### 圆角

| 令牌 | 值 | 用途 |
|------|------|------|
| `--r-sm` | `6px` | 小元素（菜单项、checkbox） |
| `--r-md` | `10px` | 中等（输入框、URL行） |
| `--r-lg` | `14px` | 大（文件卡片、弹出菜单） |
| `--r-xl` | `18px` | 超大（预览容器、QR卡片） |
| `--r-2xl` | `22px` | 模态框 |
| `--r-full` | `9999px` | 全圆（胶囊按钮、标签） |

### 阴影

| 令牌 | 值 |
|------|------|
| `--shadow-sm` | `0 1px 2px rgba(0,0,0,0.04)` |
| `--shadow-md` | `0 2px 8px rgba(0,0,0,0.06)` |
| `--shadow-lg` | `0 8px 24px rgba(0,0,0,0.08)` |
| `--shadow-modal` | `0 12px 40px rgba(0,0,0,0.12)` |

### 动效

| 令牌 | 值 | 用途 |
|------|------|------|
| `--ease-out` | `cubic-bezier(0.16, 1, 0.3, 1)` | iOS 强减速 |
| `--ease-standard` | `cubic-bezier(0.42, 0, 0.58, 1)` | 对称 ease-in-out |
| `--ease-spring` | `cubic-bezier(0.34, 1.56, 0.64, 1)` | 带回弹 spring |
| `--dur-fast` | `200ms` | 快速交互 |
| `--dur-normal` | `300ms` | 标准过渡 |
| `--dur-slow` | `450ms` | 慢速/大元素 |

### 字体

- **主字体**: `"Noto Sans SC", "PingFang SC", "HarmonyOS Sans", -apple-system, ...`
- **等宽字体**: `'SF Mono', 'JetBrains Mono', 'Menlo', 'Consolas', ...`
- **字号阶梯**: 11 / 13 / 14 / 16 / 18 / 22 / 28 / 34px

---

## 组件目录

共 27 个组件，按类别分组：

### 按钮系统（6 种）

| 组件 | 选择器 | 形态 | 圆角 | 字号 | 字重 |
|------|--------|------|------|------|------|
| 主按钮 | `.btn-primary` | 胶囊 | `--r-full` | 13px | 600 |
| 次要按钮 | `.btn-secondary` | 胶囊 | `--r-full` | 13px | 500 |
| 芯片按钮 | `.chip` | 胶囊 | `--r-full` | 13px | 500 |
| 顶栏按钮 | `.topbar-btn` | 32x32 方形 | 8px | 16px icon | — |
| 批量操作按钮 | `.batch-btn` | 胶囊 | `--r-full` | 11px | — |
| 确认弹窗按钮 | `.confirm-btn` | 全宽 | — | 14px | — |

**交互模式**: hover → `translateY(-2px) scale(1.02)` + shadow 扩大; active → `scale(0.95)` + dur 80ms; focus-visible → `box-shadow: 0 0 0 3px rgba(0,122,255,0.25)`

### 卡片与列表

| 组件 | 选择器 | 圆角 | 特性 |
|------|--------|------|------|
| 文件卡片 | `.file-card` | `--r-lg` (14px) | 网格/列表双视图，选中态蓝色边框 |
| 文件预览 | `.file-card-preview` | `--r-md` (10px) | 60px (移动) / 72px (桌面) |
| 设备标签 | `.file-device-tag` | `--r-full` | local/phone/other 三色变体 |
| 已选文件项 | `.sel-item` | `--r-sm` (6px) | hover 右移，删除按钮红色 |

### 弹窗系统（4 种）

| 组件 | 选择器 | z-index | 圆角 | 入场动效 |
|------|--------|---------|------|----------|
| 通用模态框 | `.modal-sheet` | 300 | `--r-2xl` (22px) | scale(0.92) → 1, spring |
| 确认弹窗 | `.confirm-dialog` | 1200 | `--r-xl` (18px) | scale(0.92) → 1, spring |
| 文件预览弹窗 | `.preview-modal` | 1000 | `--r-xl` (18px) | scale(0.9) → 1, spring |
| 图片放大预览 | `.img-preview-overlay` | 1300 | `--r-lg` (14px) | scale(0.92) → 1, spring |

### 反馈组件

| 组件 | 选择器 | 形态 | 动效 |
|------|--------|------|------|
| Toast 通知 | `.toast` | 胶囊，顶部居中 | translateY(-120%) → 0, spring |
| 进度条 | `.progress-fill` | 8px 圆角条 | shimmer 动画 + 宽度过渡 |
| 骨架屏 | `.skeleton-card` | 卡片形态 | shimmer 1.8s infinite |
| 成功庆祝 | `.success-card` | 圆角卡片 | confetti + scale spring |
| 空状态 | `.empty-state` | 居中插图 | pulse 3s infinite |

### 导航与信息

| 组件 | 选择器 | 特性 |
|------|--------|------|
| 顶部导航栏 | `.topbar` | sticky, z-100, 底部边框 |
| 连接指示器 | `.conn-indicator` | 6px 圆点 + 文字，online 变绿 |
| 设备浮出面板 | `.flyout` | 右上角弹出，z-200 |
| QR 码卡片 | `.qr-card` | 180px → 260px expanded |
| 步骤进度 | `.hero-step` | 36px 图标 + 连接线 |
| 磁盘空间条 | `.disk-bar` | 3px 细条，low/critical 变色 |

### 交互组件

| 组件 | 选择器 | 特性 |
|------|--------|------|
| 拖拽区域 | `.drop-zone` | 虚线边框，dragover 变实线蓝 |
| 全局拖拽覆盖 | `.drag-overlay` | 半透明蓝底 + blur |
| 右键菜单 | `.context-menu` | z-1100, scale 入场 |
| 设置菜单 | `.settings-menu` | absolute 定位弹出 |
| FAB 浮动按钮 | `.fab` | 52px 圆形，fixed 右下 |
| 输入框 | `.input-field` | focus 蓝色边框 + 4px 光晕 |
| 引导教程 | `.guide-overlay` | z-1500, 聚光灯 + 卡片 |

### 响应式断点

| 断点 | 适用范围 |
|------|----------|
| `<=480px` | 极小屏幕：缩小 padding、QR 码、网格 |
| `<=639px` | 小屏幕：compact hero 垂直布局 |
| `>=640px` | 平板+：增大 padding、QR 码、网格列宽 |
| `>=1024px` | 桌面：进一步增大 padding |

### 无障碍

- `prefers-reduced-motion`: 动画 duration → 0.01ms
- `:focus-visible`: box-shadow 自定义焦点环
- Toast: `role="status"` + `aria-live="polite"`

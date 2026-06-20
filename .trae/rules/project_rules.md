# Slim Transfer 动画与交互规范

## 设计目标
所有交互动画统一为 iOS 风格：轻快、弹性、克制、优雅。

## 缓动曲线
必须使用 CSS 变量，禁止硬编码 cubic-bezier：
- `--ease-out: cubic-bezier(0.16, 1, 0.3, 1)` — 绝大多数过渡
- `--ease-standard: cubic-bezier(0.42, 0, 0.58, 1)` — 对称缓动
- `--ease-spring: cubic-bezier(0.34, 1.56, 0.64, 1)` — 模态/卡片入场，带轻微回弹
- `--ease-spring-sm: cubic-bezier(0.25, 0.46, 0.45, 0.94)` — 需要弹性但不回弹

## 时长规范
- `--dur-fast: 200ms` — 按钮图标旋转、快速显隐、微小状态切换
- `--dur-normal: 300ms` — 面板/提示/banner/进度条/overlay 显隐
- `--dur-slow: 450ms` — 磁盘条等较慢反馈
- `--dur-hover: 350ms` — 所有 hover 反馈

## Hover 反馈
- 必须平滑渐变（使用 `--dur-hover`）
- 上浮幅度克制：`translateY(-1px)`
- 阴影淡而小
- active 缩放轻微：`scale(0.95–0.98)`

## 显隐/展开/收起
禁止直接使用 `display: none/block` 做交互动画。必须采用以下模式：
- 基础状态：`display: flex/block; visibility: hidden; opacity: 0; transform: scale(0.96) translateY(-8px); pointer-events: none;`
- 激活状态：`.open / .visible { visibility: visible; opacity: 1; transform: scale(1) translateY(0); pointer-events: auto; }`
- 过渡：`transition: opacity var(--dur-fast) var(--ease-out), transform var(--dur-fast) var(--ease-spring), visibility var(--dur-fast) var(--ease-out);`

适用组件：Modal、Dropdown、Popover、Flyout、Batch bar、Drag overlay、Progress section、Files section、Empty state、Download-all 按钮等。

## Modal
- Overlay：`opacity + visibility` 淡入淡出
- Sheet：`scale(0.96) translateY(16px)` → `scale(1) translateY(0)`，使用 `--ease-spring`，`--dur-normal`

## Dropdown / Popover / Flyout
- 从触发点向下展开：`scale(0.96) translateY(-8px)` → `scale(1) translateY(0)`
- `transform-origin` 与触发位置对齐
- 使用 `--ease-spring`

## 状态切换
所有动态状态类（`.selected`、`.online`、`.low`、`.critical`、`.active` 等）必须有过渡：
- 颜色/背景变化：`transition: background var(--dur-hover) var(--ease-out), color var(--dur-hover) var(--ease-out);`
- 宽度变化（进度条）：`transition: width var(--dur-normal) var(--ease-out);`

## 例外
- 引导 tour（guide tour / onboarding）相关动画独立处理，不强制套用本规范
- 系统减少动态效果：`prefers-reduced-motion: reduce` 只限制自动播放的 animation，不限制 hover transition

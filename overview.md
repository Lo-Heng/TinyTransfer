# TinyTransfer UX 优化 — P0 + P2 实施报告

## 改动概览

对 `rust/src-tauri/dist/index.html` 进行了 7 项 UX 优化，涉及 HTML 结构重构、CSS 样式调整和 JS 逻辑更新。

---

## P0 — 关键改动

### 1. 步骤指示器布局重构（扫码连接/拖拽传输/完成）
- **HTML**: 将 `steps-row` 从 `.qr-section` 内部移到 `.hero` 顶层，位于 QR 码上方
- **CSS**: border 从 `border-top` 改为 `border-bottom`，间距从 `margin-top` 改为 `margin-bottom`
- **紧凑模式**: `.hero.qr-compact .steps-row` 以 `opacity: 0; max-height: 0` 平滑隐藏
- **移除**: 旧的 `.qr-section.qr-compact .steps-row { display: none }`

### 2. 上传来源标签统一蓝色
- `.file-card-badge.phone`: 绿色 → 蓝色 `rgba(0,102,255,0.08)`
- `.file-card-badge.other`: 灰色 → 蓝色 `rgba(0,102,255,0.06)`
- 三个来源（本机/手机/其他）统一蓝色系

### 3. 上传文件夹设置增强
- 添加文件夹 SVG 图标到设置菜单项
- 标签改为 `<span id="uploadFolderLabel">` + tooltip 显示完整路径
- `updateSettingsUI()` 仅显示文件夹名（取路径最后一段）

---

## P2 — 辅助改动

### 4. 文件栏目间距缩减
- `.hero` padding: `var(--s-6) 0 var(--s-3)` → `var(--s-5) 0 var(--s-2)`（桌面端同理）
- 移动端 480px 断点的 steps-row 样式同步更新

### 5. "选择" → "多选"
- 移动端选择按钮 `title="选择"` → `title="多选"`
- 与桌面端 batch 按钮文案一致

### 6. 筛选按钮仅图标 — 无需改动（已是 icon-only）

### 7. 视图切换在文件行 — 无需改动（已在 `files-actions` 中）

---

## 技术细节

- 所有改动集中在单文件 `dist/index.html`（CSS + HTML + JS 内联）
- 未修改后端 Rust 代码
- 步骤状态管理（`updateQSSteps` / `compactQR`）通过元素 ID 引用，不受 DOM 结构调整影响
- 导览功能（`GUIDE_STEPS`）目标元素未变

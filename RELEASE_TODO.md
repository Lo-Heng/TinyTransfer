# TinyTransfer 发布清单

## 必做（发布前）

- [ ] **GitHub Actions 自动构建** — 创建 `.github/workflows/release.yml`，打 tag 自动构建发布到 GitHub Release
- [ ] **安全审计** — 检查路径穿越、密码认证、CORS、文件大小限制
- [ ] **依赖漏洞扫描** — `cargo audit` 扫描已知漏洞
- [ ] **版本号统一** — `0.1.0` → `1.0.0`（tauri.conf.json / Cargo.toml / package.json）
- [ ] **隐私政策** — 写一份简洁的隐私政策，放到关于页和 README

## 强烈建议

- [ ] **Tauri Updater 自动更新** — 集成自动更新，GitHub Releases 作为更新源
- [ ] **README 完善** — 加截图、FAQ、未签名说明、使用说明
- [ ] **CHANGELOG.md** — 从 v1.0.0 开始记录变更
- [ ] **CSP 收紧** — tauri.conf.json 当前 `csp: null`，建议收紧
- [ ] **应用版权信息** — tauri.conf.json 补充 `copyright` / `publisher`

## 可选

- [ ] **落地页** — GitHub Pages 免费，一个产品列表页
- [ ] **爱发电** — 开通捐赠渠道
- [ ] **软件著作权登记** — 国内维权用，¥300/个
- [ ] **提交杀毒白名单** — 火绒/360/腾讯误报上报
- [ ] **崩溃日志** — 本地写文件，方便排查问题

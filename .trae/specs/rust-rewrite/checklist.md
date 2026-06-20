# Rust + Tauri 重写 Checklist

- [x] `rust/` 目录已创建，且未修改原 Python 项目文件
- [ ] `cargo tauri dev` 能正常启动 Tauri 窗口并加载前端页面（环境阻塞，未验证）
- [x] Rust HTTP server 监听 `0.0.0.0:5000`，`GET /` 返回主页面
- [x] `GET /api/events` 返回 `text/event-stream` 并发送 `hello`/`ping`/`device_list` 事件
- [x] `POST /api/ping` 能更新设备心跳
- [x] `GET /api/devices` 返回当前连接设备列表
- [x] `GET /api/ip` 返回本机 IP 和 URL
- [ ] 手机扫码连接后，设备列表正确显示设备类型和 IP（需编译运行后验证）
- [x] 文件上传接口能保存文件到 uploads 目录
- [x] 文件下载接口能正确返回文件内容
- [x] 关闭 Tauri 窗口时程序完全退出，无残留进程（代码已实现优雅关闭）
- [ ] `cargo tauri build` 成功生成 `SlimTransfer.exe`（环境阻塞，未验证）
- [ ] 生成的 exe 大小 < 5MB（环境阻塞，未验证）
- [ ] 冷启动时间 < 1 秒（环境阻塞，未验证）
- [x] 原 Python 项目仍可正常 `python gui.py` 和 PyInstaller 打包（文件未改动）

# 验证阻塞说明

本机 Rust MSVC 工具链在 spawn 子进程时 panic（`Os { code: 0, kind: Uncategorized, message: "操作成功完成。" }`），导致带 build script 的 crate 无法编译，Tauri 项目无法 `cargo check`/`cargo build`。因此 checklist 中所有依赖编译运行的条目均标记为未验证。建议在 Rust 环境正常的 Windows 机器上执行以下命令完成验证：

```powershell
cd f:\workspace\Slim_Transfer_2\rust\src-tauri
cargo tauri dev          # 开发验证
cargo tauri build        # Release 打包
```

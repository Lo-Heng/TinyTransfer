// Tauri 环境检测与原生 API 调用
// 原文件: 4448-4468 (getTauriInvoke/getTauriDialog) + 4817-4819 (isTauriEnv) + 5181-5214 (窗口控件)

import { openFolder } from '@/api/system'

interface TauriGlobal {
  invoke?: (...args: any[]) => Promise<any>
  core?: { invoke?: (...args: any[]) => Promise<any> }
  dialog?: any
  plugins?: { dialog?: any }
  window?: { getCurrentWindow?: () => any }
}

function getTauriGlobal(): TauriGlobal | null {
  return (window as any).__TAURI__ || null
}

function getTauriInternals(): any {
  return (window as any).__TAURI_INTERNALS__ || null
}

/** 获取 Tauri invoke 函数（兼容 v2 多路径） */
export function getTauriInvoke(): ((cmd: string, args?: any) => Promise<any>) | null {
  const t = getTauriGlobal()
  if (!t) return null
  if (typeof t.invoke === 'function') return t.invoke as (cmd: string, args?: any) => Promise<any>
  if (t.core && typeof t.core.invoke === 'function') return t.core.invoke as (cmd: string, args?: any) => Promise<any>
  return null
}

/** 获取 Tauri dialog 插件（兼容 v2 多路径） */
export function getTauriDialog(): any {
  const t = getTauriGlobal()
  if (!t) return null
  if (t.dialog) return t.dialog
  if (t.plugins && t.plugins.dialog) return t.plugins.dialog
  return null
}

/** 是否运行在 Tauri 环境中 */
export function isTauriEnv(): boolean {
  return !!(getTauriInternals() || (getTauriGlobal() && (getTauriInvoke() || getTauriDialog())))
}

// 缓存当前 Tauri 窗口实例（原 DOMContentLoaded 中 window._tauriWin 赋值）
let _tauriWin: any = null

/** 获取/初始化 Tauri 窗口引用 */
export function getTauriWindow(): any {
  if (_tauriWin) return _tauriWin
  try {
    const t = getTauriGlobal()
    if (t?.window?.getCurrentWindow) {
      _tauriWin = t.window.getCurrentWindow()
    }
  } catch (e) {
    console.warn('Tauri window API not available:', e)
  }
  return _tauriWin
}

/** 用系统播放器打开视频（绕过浏览器 HEVC 等不支持格式） */
export async function openFileWithPlayer(filename: string): Promise<void> {
  if (!isTauriEnv()) {
    alert('仅支持桌面客户端')
    return
  }
  const invoke = getTauriInvoke()
  if (!invoke) {
    alert('Tauri invoke 不可用')
    return
  }
  await invoke('open_file_with_player', { filename })
}

/** 在系统文件管理器中打开共享文件夹 */
export async function openShareFolder(): Promise<void> {
  await openFolder('uploads')
}

export function useTauri() {
  return {
    isTauriEnv,
    getTauriInvoke,
    getTauriDialog,
    getTauriWindow,
    openFileWithPlayer,
    openShareFolder,
  }
}

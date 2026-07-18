// 文件下载 — 单文件/批量 ZIP，Tauri 对话框与浏览器 URL 下载
// 原文件: 4842-5058 (downloadSingleFile/downloadZip/browserDownloadFile/triggerDownloadByUrl)

import { useFilesStore } from '@/stores/files'
import { useUiStore } from '@/stores/ui'
import { downloadZipBlob, getDownloadUrl } from '@/api/files'
import { isTauriEnv, getTauriInvoke } from './useTauri'

/** 下载完成后打开所在文件夹（Tauri） */
async function tryOpenFolder(filePath: string) {
  if (!filePath) return
  const invoke = getTauriInvoke()
  if (invoke) {
    try {
      await invoke('open_containing_folder', { file_path: filePath })
    } catch (e) {
      console.warn('[tryOpenFolder] 无法打开文件夹:', e)
    }
  }
}

/** 通过 URL 触发下载（WebView2/浏览器原生方式） */
function triggerDownloadByUrl(url: string, filename?: string) {
  // 方式1: window.location.assign（最可能触发 WebView2 下载）
  try {
    window.location.assign(url)
    return
  } catch (e) {
    console.log('[triggerDownloadByUrl] 方式1失败:', e)
  }
  // 方式2: window.open
  try {
    window.open(url, '_blank')
    return
  } catch (e) {
    console.log('[triggerDownloadByUrl] 方式2失败:', e)
  }
  // 方式3: iframe fallback
  try {
    const iframe = document.createElement('iframe')
    iframe.style.display = 'none'
    iframe.src = url
    document.body.appendChild(iframe)
    setTimeout(() => {
      if (iframe.parentNode) iframe.parentNode.removeChild(iframe)
    }, 30000)
  } catch (e) {
    // 方式4: <a> 标签 fallback
    const a = document.createElement('a')
    a.href = url
    if (filename) a.download = filename
    a.style.display = 'none'
    document.body.appendChild(a)
    a.click()
    setTimeout(() => {
      if (a.parentNode) a.parentNode.removeChild(a)
    }, 1000)
  }
}

/** 下载单个文件（Tauri 用对话框直接读磁盘，浏览器用 URL 触发） */
export async function downloadSingleFile(filename: string) {
  const ui = useUiStore()

  // Tauri 环境：调用 Rust 命令直接读磁盘、弹对话框、写入选中路径
  if (isTauriEnv()) {
    const invoke = getTauriInvoke()
    if (invoke) {
      try {
        const savedPath = await invoke('download_file_dialog', { filename })
        if (savedPath) {
          ui.showToast('下载成功', 'success')
          ui.showSuccess('下载完成')
          return
        }
        // 用户取消
        return
      } catch (e: any) {
        const errMsg =
          typeof e === 'string' ? e : (e && e.message) || JSON.stringify(e)
        ui.showToast('下载失败: ' + errMsg, 'error')
        return
      }
    }
  }

  // 浏览器环境：URL 方式下载
  const url = getDownloadUrl(filename)
  triggerDownloadByUrl(url, filename)
  ui.showToast('下载已开始', 'success')
  ui.showSuccess('下载已开始 📥')
}

/** 批量下载 ZIP（Tauri 优先对话框，失败回退 URL） */
export async function downloadZip(filenames: string[]) {
  const ui = useUiStore()

  // Tauri 环境：blob + save_file_dialog
  if (isTauriEnv() && getTauriInvoke()) {
    const invoke = getTauriInvoke()!
    try {
      ui.showToast('正在打包 ' + filenames.length + ' 个文件...')
      const blob = await downloadZipBlob(filenames)
      if (blob.size === 0) throw new Error('打包失败')

      const timestamp =
        new Date().toISOString().slice(0, 10).replace(/-/g, '') +
        '_' +
        new Date().toTimeString().slice(0, 8).replace(/:/g, '')
      const zipName = 'tiny_transfer_' + timestamp + '.zip'

      const buffer = new Uint8Array(await blob.arrayBuffer())
      const result = await invoke('save_file_dialog', {
        filename: zipName,
        data: buffer,
      })

      if (result) {
        tryOpenFolder(result)
        ui.showToast('下载成功', 'success')
        ui.showSuccess('下载完成 📥')
        return
      }
      // 用户取消
      return
    } catch (e: any) {
      const zipErr =
        typeof e === 'string' ? e : (e && e.message) || JSON.stringify(e)
      console.log('[downloadZip] Tauri失败，回退到URL方式:', zipErr)
    }
  }

  // 回退：GET URL 方式
  const data = encodeURIComponent(JSON.stringify(filenames))
  const url = '/api/download-zip?data=' + data
  triggerDownloadByUrl(url, 'tiny_transfer.zip')
  ui.showToast('下载已开始', 'success')
  ui.showSuccess('下载已开始 📥')
}

/** 浏览器环境下载（URL 方式触发） */
export function browserDownloadFile(filename: string) {
  const url = getDownloadUrl(filename)
  triggerDownloadByUrl(url, filename)
}

/** 下载当前选中的文件（批量 ZIP） */
export function downloadSelectedFiles() {
  const files = useFilesStore()
  const ui = useUiStore()
  if (files.selectedFileNames.size === 0) {
    ui.showToast('请先选择文件')
    return
  }
  const filenames = Array.from(files.selectedFileNames)
  downloadZip(filenames)
  files.toggleBatchMode()
}

/** 下载全部文件 */
export function downloadAllFiles() {
  const files = useFilesStore()
  const ui = useUiStore()
  if (!files.allFilesData || files.allFilesData.length === 0) {
    ui.showToast('没有可下载的文件')
    return
  }
  const filenames = files.allFilesData.map((f) => f.name)
  downloadZip(filenames)
}

export function useDownload() {
  return {
    downloadSingleFile,
    downloadZip,
    browserDownloadFile,
    downloadSelectedFiles,
    downloadAllFiles,
  }
}

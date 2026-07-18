// 右键上下文菜单 — 显示/隐藏/菜单项动作
// 原文件: 6481-6652 (showContextMenu/hideContextMenu/downloadSelectedFile/shareSelectedFile/selectFileFromContextMenu/deleteSelectedFile/showFileProperties)

import { reactive } from 'vue'
import { useFilesStore } from '@/stores/files'
import { useUiStore } from '@/stores/ui'
import { downloadSingleFile } from './useDownload'

export interface ContextMenuState {
  visible: boolean
  x: number
  y: number
  filename: string
  isSelected: boolean
  propertiesOpen: boolean
}

// 模块级单例状态（reactive 便于组件直接访问属性，无需 .value）
const contextMenuState = reactive<ContextMenuState>({
  visible: false,
  x: 0,
  y: 0,
  filename: '',
  isSelected: false,
  propertiesOpen: false,
})

const MENU_WIDTH = 180
const MENU_HEIGHT = 120

/** 显示右键菜单 */
export function showContextMenu(x: number, y: number, filename: string): void {
  const files = useFilesStore()
  contextMenuState.visible = true
  contextMenuState.x = Math.min(x, window.innerWidth - MENU_WIDTH - 10)
  contextMenuState.y = Math.min(y, window.innerHeight - MENU_HEIGHT - 10)
  contextMenuState.filename = filename
  contextMenuState.isSelected = files.selectedFileNames.has(filename)
  contextMenuState.propertiesOpen = false
}

/** 隐藏右键菜单 */
export function hideContextMenu(): void {
  contextMenuState.visible = false
}

/** 下载当前菜单指向的文件 */
export function downloadCurrent(): void {
  if (!contextMenuState.filename) return
  hideContextMenu()
  downloadSingleFile(contextMenuState.filename)
}

/** 复制当前文件的分享链接 */
export async function shareCurrent(): Promise<void> {
  const ui = useUiStore()
  const filename = contextMenuState.filename
  if (!filename) return
  const shareUrl =
    window.location.origin + '/api/download/' + encodeURIComponent(filename)
  try {
    if (navigator.clipboard) {
      await navigator.clipboard.writeText(shareUrl)
    } else {
      const input = document.createElement('input')
      input.value = shareUrl
      document.body.appendChild(input)
      input.select()
      document.execCommand('copy')
      document.body.removeChild(input)
    }
    ui.showToast('链接已复制 📋')
  } catch {
    ui.showToast('复制失败', 'error')
  }
  hideContextMenu()
}

/** 切换当前文件的选中状态（自动进入多选模式） */
export function toggleSelectCurrent(): void {
  const files = useFilesStore()
  const filename = contextMenuState.filename
  if (!filename) return
  hideContextMenu()
  if (!files.batchMode) {
    files.toggleBatchMode()
  }
  files.toggleFileSelection(filename)
}

/** 删除当前菜单指向的文件（组件应在调用前弹出确认框） */
export async function deleteCurrent(): Promise<void> {
  const files = useFilesStore()
  const ui = useUiStore()
  const filename = contextMenuState.filename
  if (!filename) return
  hideContextMenu()
  try {
    await files.deleteFiles([filename])
    ui.showToast('已删除 ' + filename, 'success')
  } catch (e: any) {
    ui.showToast('删除失败: ' + (e?.message || e), 'error')
  }
}

/** 显示当前文件属性 */
export function showCurrentProperties(): void {
  const files = useFilesStore()
  const filename = contextMenuState.filename
  if (!filename) return
  hideContextMenu()
  const file = files.allFilesData.find((f) => f.name === filename)
  if (!file) return
  contextMenuState.propertiesOpen = true
}

/** 关闭文件属性弹窗 */
export function closeFileProperties(): void {
  contextMenuState.propertiesOpen = false
}

export function useContextMenu() {
  return {
    contextMenuState,
    showContextMenu,
    hideContextMenu,
    downloadCurrent,
    shareCurrent,
    toggleSelectCurrent,
    deleteCurrent,
    showCurrentProperties,
    closeFileProperties,
  }
}

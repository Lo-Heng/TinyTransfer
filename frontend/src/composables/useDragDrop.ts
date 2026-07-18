// 全局拖拽上传 — 桌面端 host 拖拽文件到页面 + 粘贴上传
// 原文件: 5236-5271 (dragenter/dragleave/dragover/drop)

import { useUiStore } from '@/stores/ui'
import { useAuthStore } from '@/stores/auth'
import { openUploadModalWithFiles } from './useUpload'

let dragCounter = 0
let initialized = false

function onDragEnter(e: DragEvent) {
  const auth = useAuthStore()
  if (auth.userRole !== 'host') return
  e.preventDefault()
  dragCounter++
  if (dragCounter === 1) {
    const ui = useUiStore()
    ui.dragOverlayActive = true
  }
}

function onDragLeave(e: DragEvent) {
  const auth = useAuthStore()
  if (auth.userRole !== 'host') return
  e.preventDefault()
  dragCounter--
  if (dragCounter <= 0) {
    dragCounter = 0
    const ui = useUiStore()
    ui.dragOverlayActive = false
  }
}

function onDragOver(e: DragEvent) {
  const auth = useAuthStore()
  if (auth.userRole !== 'host') return
  e.preventDefault()
}

function onDrop(e: DragEvent) {
  const auth = useAuthStore()
  if (auth.userRole !== 'host') return
  e.preventDefault()
  dragCounter = 0
  const ui = useUiStore()
  ui.dragOverlayActive = false
  const files = e.dataTransfer?.files
  if (files && files.length > 0) {
    openUploadModalWithFiles(Array.from(files))
  }
}

function onPaste(e: ClipboardEvent) {
  const auth = useAuthStore()
  if (auth.userRole !== 'host') return
  const items = e.clipboardData?.items
  if (!items) return
  const files: File[] = []
  for (let i = 0; i < items.length; i++) {
    if (items[i].kind === 'file') {
      const file = items[i].getAsFile()
      if (file) files.push(file)
    }
  }
  if (files.length > 0) {
    openUploadModalWithFiles(files)
  }
}

/** 初始化全局拖拽与粘贴上传监听（仅注册一次） */
export function initDragDrop(): void {
  if (initialized) return
  initialized = true
  document.addEventListener('dragenter', onDragEnter)
  document.addEventListener('dragleave', onDragLeave)
  document.addEventListener('dragover', onDragOver)
  document.addEventListener('drop', onDrop)
  document.addEventListener('paste', onPaste)
}

/** 销毁全局拖拽监听 */
export function destroyDragDrop(): void {
  if (!initialized) return
  initialized = false
  document.removeEventListener('dragenter', onDragEnter)
  document.removeEventListener('dragleave', onDragLeave)
  document.removeEventListener('dragover', onDragOver)
  document.removeEventListener('drop', onDrop)
  document.removeEventListener('paste', onPaste)
}

export function useDragDrop() {
  return { initDragDrop, destroyDragDrop }
}

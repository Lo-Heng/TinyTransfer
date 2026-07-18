// 批量模式 — 多选/全选/批量下载/批量删除
// 原文件: 7013-7100 (toggleBatchMode/selectAllFiles/downloadSelectedFiles/deleteSelectedFiles/downloadAllFiles)

import { useFilesStore } from '@/stores/files'
import { useUiStore } from '@/stores/ui'
import { downloadSelectedFiles as doDownloadSelected } from './useDownload'

/** 切换批量模式 */
export function toggleBatchMode(): void {
  const files = useFilesStore()
  files.toggleBatchMode()
}

/** 全选当前筛选结果 */
export function selectAllFiles(): void {
  const files = useFilesStore()
  files.selectAllFiles()
}

/** 批量下载选中的文件（ZIP） */
export function downloadSelectedFiles(): void {
  doDownloadSelected()
}

/** 批量删除选中的文件（组件应在调用前弹出确认框） */
export async function deleteSelectedFiles(): Promise<void> {
  const files = useFilesStore()
  const ui = useUiStore()
  if (files.selectedFileNames.size === 0) {
    ui.showToast('请先选择文件')
    return
  }
  const filenames = Array.from(files.selectedFileNames)
  try {
    await files.deleteFiles(filenames)
    ui.showToast('已删除 ' + filenames.length + ' 个文件', 'success')
    files.toggleBatchMode()
  } catch (e: any) {
    ui.showToast('删除失败: ' + (e?.message || e), 'error')
  }
}

export function useBatchMode() {
  return {
    toggleBatchMode,
    selectAllFiles,
    downloadSelectedFiles,
    deleteSelectedFiles,
  }
}

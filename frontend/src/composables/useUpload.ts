// 文件上传 — 单文件/分片并发上传，进度/速度/ETA
// 原文件: 7169-7442 (openUploadModal/openUploadModalWithFiles/closeUploadModal/startUpload/uploadChunked)

import { useUploadStore } from '@/stores/upload'
import { useFilesStore } from '@/stores/files'
import { useUiStore } from '@/stores/ui'
import { apiFetch } from '@/api/client'

interface UploadResponse {
  success: boolean
  error?: string
  completed?: boolean
}

let uploadSpeedStartTime = 0

/** 打开上传模态框（清空状态，组件负责触发文件选择器） */
export function openUploadModal() {
  const upload = useUploadStore()
  upload.clearFiles()
  upload.openModal()
}

/** 用已有文件列表打开上传模态框并自动开始上传 */
export function openUploadModalWithFiles(files: File[]) {
  const upload = useUploadStore()
  upload.setFiles(files)
  upload.openModal()
  // 自动开始上传
  setTimeout(() => {
    startUpload()
  }, 300)
}

/** 关闭上传模态框（上传中不可关闭） */
export function closeUploadModal() {
  const upload = useUploadStore()
  upload.closeModal()
}

/** 分片并发上传（1MB 分片，6 并发） */
async function uploadChunked(
  file: File,
  onProgress: (chunkProgress: number) => void,
): Promise<void> {
  const CHUNK_SIZE = 1 * 1024 * 1024 // 1MB（局域网优化）
  const CONCURRENCY = 6 // 6 并发（浏览器每域名上限）
  const totalChunks = Math.ceil(file.size / CHUNK_SIZE)
  const fileId = Date.now().toString(36) + Math.random().toString(36).substr(2)

  let uploadedChunks = 0
  let bytesUploaded = 0
  let nextChunkIndex = 0
  let hasError = false
  let isCompleted = false

  async function worker() {
    while (!isCompleted && nextChunkIndex < totalChunks && !hasError) {
      const i = nextChunkIndex++
      if (i >= totalChunks) return

      const start = i * CHUNK_SIZE
      const end = Math.min(file.size, start + CHUNK_SIZE)
      const chunk = file.slice(start, end)

      const fd = new FormData()
      fd.append('fileId', fileId)
      fd.append('chunkIndex', String(i))
      fd.append('totalChunks', String(totalChunks))
      fd.append('filename', file.name)
      fd.append('chunk', chunk)

      // 上传目录偏好
      const uploadFolder = localStorage.getItem('tiny-upload-folder')
      if (uploadFolder) {
        fd.append('target_folder', uploadFolder)
      }

      try {
        const data = await apiFetch<UploadResponse>('/api/upload-chunk', {
          method: 'POST',
          body: fd,
        })
        if (isCompleted) return
        if (!data.success) {
          hasError = true
          throw new Error(data.error)
        }

        uploadedChunks++
        bytesUploaded += end - start
        onProgress(bytesUploaded / file.size)

        if (data.completed && !isCompleted) {
          isCompleted = true
          return
        }
      } catch (err) {
        if (!isCompleted) {
          hasError = true
          throw err
        }
        return
      }
    }
  }

  const workers = []
  for (let w = 0; w < CONCURRENCY; w++) {
    workers.push(worker())
  }
  await Promise.all(workers)
}

/** 开始上传所有已选文件 */
export async function startUpload() {
  const upload = useUploadStore()
  const files = useFilesStore()
  const ui = useUiStore()

  if (upload.isUploading) return
  if (!upload.hasSelectedFiles) return

  // 记录上传前的文件名集合，用于完成后高亮新文件
  const preUploadFileNames = new Set(
    files.allFilesData.map((f) => f.name),
  )

  upload.isUploading = true
  upload.uploadProgress = 0

  const selectedFiles = upload.selectedFiles
  const totalFiles = selectedFiles.length
  const totalSize = selectedFiles.reduce((s, f) => s + f.size, 0)
  let uploadedSize = 0
  let completed = 0
  let failedCount = 0

  uploadSpeedStartTime = Date.now()

  function updateSpeed(bytesSoFar: number) {
    const elapsed = (Date.now() - uploadSpeedStartTime) / 1000
    if (elapsed < 0.1) return
    const speed = bytesSoFar / elapsed
    upload.uploadSpeed = speed

    const remaining = totalSize - bytesSoFar
    if (speed > 0 && remaining > 0) {
      const eta = remaining / speed
      if (eta < 60) upload.uploadEta = '剩余 ' + Math.ceil(eta) + '秒'
      else if (eta < 3600) upload.uploadEta = '剩余 ' + Math.ceil(eta / 60) + '分钟'
      else upload.uploadEta = '剩余 ' + (eta / 3600).toFixed(1) + '小时'
    } else if (remaining === 0) {
      upload.uploadEta = '完成'
    }
  }

  for (let i = 0; i < selectedFiles.length; i++) {
    const file = selectedFiles[i].file
    try {
      if (file.size > 0.1 * 1024 * 1024) {
        await uploadChunked(file, (chunkProgress) => {
          const currentUploaded = uploadedSize + file.size * chunkProgress
          upload.uploadProgress = Math.round((currentUploaded / totalSize) * 100)
          updateSpeed(currentUploaded)
        })
      } else {
        const formData = new FormData()
        formData.append('file', file)
        const uploadFolder = localStorage.getItem('tiny-upload-folder')
        if (uploadFolder) {
          formData.append('target_folder', uploadFolder)
        }
        const data = await apiFetch<UploadResponse>('/api/upload', {
          method: 'POST',
          body: formData,
        })
        if (!data.success) throw new Error(data.error)
      }
      completed++
      uploadedSize += file.size
      upload.uploadProgress = Math.round((uploadedSize / totalSize) * 100)
      updateSpeed(uploadedSize)
    } catch (err: any) {
      failedCount++
      ui.showToast('上传失败: ' + file.name)
    }
  }

  upload.uploadProgress = 100
  upload.isUploading = false
  updateSpeed(totalSize)
  if (failedCount > 0) {
    ui.showToast(`上传完成，${failedCount} 个文件失败`, 'error')
  } else {
    ui.showToast('上传完成 🚀', 'success')
  }

  // 刷新文件列表（store 内会自动高亮新文件）
  await files.loadAllFiles()
  ui.completeStep(2)

  // 延迟关闭模态框
  setTimeout(() => {
    closeUploadModal()
  }, 1500)
}

export function useUpload() {
  return {
    openUploadModal,
    openUploadModalWithFiles,
    closeUploadModal,
    startUpload,
  }
}

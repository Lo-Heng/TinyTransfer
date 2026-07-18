import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export interface SelectedFile {
  file: File
  name: string
  size: number
}

export const useUploadStore = defineStore('upload', () => {
  const selectedFiles = ref<SelectedFile[]>([])
  const isUploading = ref(false)
  const uploadProgress = ref(0)
  const uploadSpeed = ref(0)
  const uploadEta = ref('-')
  const uploadModalOpen = ref(false)
  const CHUNK_SIZE = 5 * 1024 * 1024 // 5MB 分片

  const hasSelectedFiles = computed(() => selectedFiles.value.length > 0)

  function setFiles(files: File[]) {
    selectedFiles.value = files.map((file) => ({
      file,
      name: file.name,
      size: file.size,
    }))
  }

  function clearFiles() {
    selectedFiles.value = []
    uploadProgress.value = 0
    uploadSpeed.value = 0
    uploadEta.value = '-'
  }

  function openModal() {
    uploadModalOpen.value = true
  }

  function closeModal() {
    if (isUploading.value) return // 上传中禁止关闭
    uploadModalOpen.value = false
    clearFiles()
  }

  return {
    selectedFiles, isUploading, uploadProgress, uploadSpeed, uploadEta,
    uploadModalOpen, CHUNK_SIZE, hasSelectedFiles,
    setFiles, clearFiles, openModal, closeModal,
  }
})

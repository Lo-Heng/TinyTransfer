<script setup lang="ts">
import { ref, watch, onUnmounted } from 'vue'
import { useUploadStore } from '@/stores/upload'
import { useUiStore } from '@/stores/ui'
import { useUpload } from '@/composables/useUpload'
import { formatBytes, formatSpeed } from '@/utils/format'

const upload = useUploadStore()
const ui = useUiStore()
const { closeUploadModal, startUpload } = useUpload()

const fileInput = ref<HTMLInputElement | null>(null)
const isDragOver = ref(false)

function openPicker() {
  fileInput.value?.click()
}

// 选完文件后立即上传,简化用户操作步骤
function onFileChange(e: Event) {
  const input = e.target as HTMLInputElement
  if (input.files && input.files.length > 0) {
    upload.setFiles(Array.from(input.files))
    startUpload()
  }
  input.value = ''
}

function onDrop(e: DragEvent) {
  isDragOver.value = false
  if (e.dataTransfer && e.dataTransfer.files.length > 0) {
    upload.setFiles(Array.from(e.dataTransfer.files))
    startUpload()
  }
}

function removeFile(i: number) {
  upload.selectedFiles.splice(i, 1)
}

function overlayClick() {
  if (!upload.isUploading) closeUploadModal()
}

// Escape 键关闭(上传中不可关闭)
function onKeydown(e: KeyboardEvent) {
  if (e.key === 'Escape' && !upload.isUploading) closeUploadModal()
}

watch(() => upload.uploadModalOpen, (open) => {
  if (open) {
    window.addEventListener('keydown', onKeydown)
  } else {
    window.removeEventListener('keydown', onKeydown)
  }
})

onUnmounted(() => {
  window.removeEventListener('keydown', onKeydown)
})

const speedText = () => {
  const s = formatSpeed(upload.uploadSpeed)
  return s.value + ' ' + s.unit
}
</script>

<template>
  <Transition name="modal">
    <div
      v-if="upload.uploadModalOpen"
      class="modal-overlay"
      role="dialog"
      aria-modal="true"
      aria-label="上传文件"
      @click.self="overlayClick"
    >
      <div class="modal-sheet">
        <div class="modal-header">
          <h3 class="modal-title">上传文件</h3>
          <button class="modal-close" title="关闭" @click="closeUploadModal">
            <svg class="icon icon-sm" viewBox="0 0 24 24"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
          </button>
        </div>
        <div class="drop-zone-shell">
          <div
            class="drop-zone"
            :class="{ dragover: isDragOver }"
            @click="openPicker"
            @dragover.prevent="isDragOver = true"
            @dragleave.prevent="isDragOver = false"
            @drop.prevent="onDrop"
          >
            <div class="drop-zone-icon">
              <svg class="icon" viewBox="0 0 24 24" style="font-size:36px;"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" y1="3" x2="12" y2="15"/></svg>
            </div>
            <div class="drop-zone-text">点击或拖拽文件到此处</div>
            <div class="drop-zone-hint">支持多选 · 并行加速上传</div>
            <input type="file" multiple style="display:none;" accept="*/*" ref="fileInput" @change="onFileChange" />
          </div>
        </div>

        <!-- Selected files list -->
        <div v-if="upload.hasSelectedFiles" class="sel-list">
          <div v-for="(f, i) in upload.selectedFiles" :key="i" class="sel-item">
            <span class="sel-item-name">{{ f.name }}</span>
            <span class="sel-item-size">{{ formatBytes(f.size) }}</span>
            <button v-if="!upload.isUploading" class="sel-item-remove" @click="removeFile(i)">✕</button>
          </div>
        </div>

        <!-- Progress -->
        <div v-if="upload.isUploading || upload.uploadProgress > 0" class="progress-section">
          <div class="progress-track">
            <div class="progress-fill" :style="{ width: upload.uploadProgress + '%' }"></div>
          </div>
          <div class="progress-text">{{ upload.uploadProgress.toFixed(0) }}%</div>
          <div class="progress-speed">
            <span class="progress-speed-value">{{ speedText() }}</span>
            <span>剩余 {{ upload.uploadEta }}</span>
          </div>
        </div>
      </div>
    </div>
  </Transition>
</template>

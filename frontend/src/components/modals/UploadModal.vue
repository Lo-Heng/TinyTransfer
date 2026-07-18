<script setup lang="ts">
import { ref } from 'vue'
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

function onFileChange(e: Event) {
  const input = e.target as HTMLInputElement
  if (input.files && input.files.length > 0) {
    upload.setFiles(Array.from(input.files))
  }
  input.value = ''
}

function onDrop(e: DragEvent) {
  isDragOver.value = false
  if (e.dataTransfer && e.dataTransfer.files.length > 0) {
    upload.setFiles(Array.from(e.dataTransfer.files))
  }
}

function removeFile(i: number) {
  upload.selectedFiles.splice(i, 1)
}

function overlayClick(e: MouseEvent) {
  if (e.target === e.currentTarget && !upload.isUploading) closeUploadModal()
}

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
      @click="overlayClick"
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
        <div class="drop-zone-tip">💡 想要原视频不压缩？选择「浏览」从文件中选取</div>

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

        <button
          v-if="upload.hasSelectedFiles && !upload.isUploading"
          class="btn-upload"
          @click="startUpload"
        >
          开始上传
          <span class="btn-upload-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" y1="3" x2="12" y2="15"/></svg>
          </span>
        </button>
      </div>
    </div>
  </Transition>
</template>

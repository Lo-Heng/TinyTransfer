<script setup lang="ts">
import { computed } from 'vue'
import { useFilePreview } from '@/composables/useFilePreview'
import { useDownload } from '@/composables/useDownload'
import { getDownloadUrl } from '@/api/files'

const { previewState, closePreview } = useFilePreview()
const { downloadSingleFile } = useDownload()

// 假定 previewState 形状：{ visible, filename, type, url, text }
const url = computed(() => previewState.url || (previewState.filename ? getDownloadUrl(previewState.filename) : ''))

function download() {
  if (previewState.filename) downloadSingleFile(previewState.filename)
}
</script>

<template>
  <Transition name="modal">
    <div v-if="previewState.visible" class="preview-modal" @click.self="closePreview">
      <div class="preview-container">
        <div class="preview-header">
          <span class="preview-filename">{{ previewState.filename }}</span>
          <div class="preview-actions">
            <button
              class="btn-secondary"
              style="display:flex;align-items:center;gap:6px;padding:6px 14px;font-size:var(--text-sm);"
              @click="download"
            >
              <svg class="icon icon-sm" viewBox="0 0 24 24"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
              下载
            </button>
            <button
              class="topbar-btn"
              style="width:32px;height:32px;"
              title="关闭预览"
              @click="closePreview"
            >
              <svg class="icon icon-sm" viewBox="0 0 24 24"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
            </button>
          </div>
        </div>
        <div class="preview-content">
          <img v-if="previewState.type === 'image'" :src="url" :alt="previewState.filename" />
          <video v-else-if="previewState.type === 'video'" controls :src="url"></video>
          <audio v-else-if="previewState.type === 'audio'" controls :src="url"></audio>
          <pre v-else-if="previewState.type === 'text'" class="preview-text">{{ previewState.text }}</pre>
          <div v-else class="preview-unsupported">
            <p>该文件类型暂不支持在线预览</p>
            <button class="btn-primary" @click="download">下载文件</button>
          </div>
        </div>
      </div>
    </div>
  </Transition>
</template>

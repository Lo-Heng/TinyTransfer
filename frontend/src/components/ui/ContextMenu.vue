<script setup lang="ts">
import { useContextMenu } from '@/composables/useContextMenu'
import { useDownload } from '@/composables/useDownload'
import { useFilesStore } from '@/stores/files'

const emit = defineEmits<{
  (e: 'show-properties', filename: string): void
  (e: 'confirm-delete', filename: string): void
}>()

const { contextMenuState, hideContextMenu } = useContextMenu()
const { downloadSingleFile } = useDownload()
const files = useFilesStore()

function download() {
  if (contextMenuState.filename) downloadSingleFile(contextMenuState.filename)
  hideContextMenu()
}

async function share() {
  const name = contextMenuState.filename
  if (!name) return
  try {
    if (navigator.share) {
      await navigator.share({ title: name, url: '/api/download/' + encodeURIComponent(name) })
    }
  } catch (e) {
    /* 用户取消等 */
  }
  hideContextMenu()
}

function selectFile() {
  const name = contextMenuState.filename
  if (!name) return
  if (!files.batchMode) files.toggleBatchMode()
  files.toggleFileSelection(name)
  hideContextMenu()
}

function showProperties() {
  if (contextMenuState.filename) emit('show-properties', contextMenuState.filename)
  hideContextMenu()
}

function confirmDelete() {
  if (contextMenuState.filename) emit('confirm-delete', contextMenuState.filename)
  hideContextMenu()
}
</script>

<template>
  <div
    class="context-menu"
    :class="{ open: contextMenuState.visible }"
    :style="{ left: contextMenuState.x + 'px', top: contextMenuState.y + 'px' }"
  >
    <div class="context-menu-item" @click="download">
      <svg class="icon icon-sm" viewBox="0 0 24 24"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
      下载
    </div>
    <div class="context-menu-item" @click="share">
      <svg class="icon icon-sm" viewBox="0 0 24 24"><circle cx="18" cy="5" r="3"/><circle cx="6" cy="12" r="3"/><circle cx="18" cy="19" r="3"/><line x1="8.59" y1="13.51" x2="15.42" y2="17.49"/><line x1="15.41" y1="6.51" x2="8.59" y2="10.49"/></svg>
      分享
    </div>
    <div class="context-menu-divider"></div>
    <div class="context-menu-item" @click="selectFile">
      <svg class="icon icon-sm" viewBox="0 0 24 24"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"/><polyline points="9 11 12 14 22 4"/></svg>
      多选
    </div>
    <div class="context-menu-item" @click="showProperties">
      <svg class="icon icon-sm" viewBox="0 0 24 24"><circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/></svg>
      属性
    </div>
    <div class="context-menu-divider"></div>
    <div class="context-menu-item danger" @click="confirmDelete">
      <svg class="icon icon-sm" viewBox="0 0 24 24"><polyline points="3 6 5 6 21 6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>
      删除
    </div>
  </div>
</template>

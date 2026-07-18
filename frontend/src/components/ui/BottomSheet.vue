<script setup lang="ts">
import { ref } from 'vue'
import { useDownload } from '@/composables/useDownload'

const emit = defineEmits<{
  (e: 'confirm-delete', filename: string): void
}>()

const { downloadSingleFile } = useDownload()

const open = ref(false)
const currentFilename = ref('')

function show(filename: string) {
  currentFilename.value = filename
  open.value = true
}

function close() {
  open.value = false
}

async function action(type: string) {
  const name = currentFilename.value
  close()
  if (!name) return
  if (type === 'download') {
    downloadSingleFile(name)
  } else if (type === 'share') {
    try {
      if (navigator.share) await navigator.share({ title: name, url: '/api/download/' + encodeURIComponent(name) })
    } catch (e) { /* ignored */ }
  } else if (type === 'delete') {
    emit('confirm-delete', name)
  }
}

defineExpose({ show, close })
</script>

<template>
  <Transition name="sheet">
    <div v-if="open" class="bottom-sheet-overlay" @click="close">
      <div class="bottom-sheet" @click.stop>
        <div class="bottom-sheet-handle"></div>
        <div class="bottom-sheet-item" @click="action('download')">
          <svg class="bs-icon" viewBox="0 0 24 24">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
            <polyline points="7 10 12 15 17 10" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
            <line x1="12" y1="15" x2="12" y2="3" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          <span>下载</span>
        </div>
        <div class="bottom-sheet-item" @click="action('share')">
          <svg class="bs-icon" viewBox="0 0 24 24">
            <circle cx="18" cy="5" r="3" fill="none" stroke="currentColor" stroke-width="1.5"/>
            <circle cx="6" cy="12" r="3" fill="none" stroke="currentColor" stroke-width="1.5"/>
            <circle cx="18" cy="19" r="3" fill="none" stroke="currentColor" stroke-width="1.5"/>
            <line x1="8.59" y1="13.51" x2="15.42" y2="17.49" stroke="currentColor" stroke-width="1.5"/>
            <line x1="15.41" y1="6.51" x2="8.59" y2="10.49" stroke="currentColor" stroke-width="1.5"/>
          </svg>
          <span>分享链接</span>
        </div>
        <div class="bottom-sheet-divider"></div>
        <div class="bottom-sheet-item danger" @click="action('delete')">
          <svg class="bs-icon" viewBox="0 0 24 24">
            <polyline points="3 6 5 6 21 6" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
            <path d="M19 6l-2 14a2 2 0 0 1-2 2H9a2 2 0 0 1-2-2L5 6" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
            <path d="M10 11v6M14 11v6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
            <path d="M9 6V4a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v2" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          </svg>
          <span>删除</span>
        </div>
        <div class="bottom-sheet-divider"></div>
        <div class="bottom-sheet-item cancel" @click="close">
          <span>取消</span>
        </div>
      </div>
    </div>
  </Transition>
</template>

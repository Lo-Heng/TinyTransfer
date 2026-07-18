<script setup lang="ts">
import { ref, computed } from 'vue'
import { useFilesStore } from '@/stores/files'
import { formatBytes, formatTime } from '@/utils/format'
import FileIcon from '@/components/icons/FileIcon.vue'

const files = useFilesStore()

const open = ref(false)
const currentFilename = ref('')

const file = computed(() =>
  files.allFilesData.find((f) => f.name === currentFilename.value)
)

const ext = computed(() => {
  const name = currentFilename.value
  const idx = name.lastIndexOf('.')
  return idx > 0 ? name.slice(idx + 1).toUpperCase() : '—'
})

function show(filename: string) {
  currentFilename.value = filename
  open.value = true
}

function close() {
  open.value = false
}

defineExpose({ show, close })
</script>

<template>
  <Transition name="modal">
    <div v-if="open" class="modal-overlay" @click.self="close">
      <div class="modal-sheet" style="max-width: 420px;">
        <div class="modal-header">
          <h3 class="modal-title">文件属性</h3>
          <button class="modal-close" @click="close" title="关闭">✕</button>
        </div>
        <div class="file-prop-content">
          <div class="file-prop-icon">
            <FileIcon :filename="currentFilename" />
          </div>
          <div class="file-prop-name">{{ currentFilename }}</div>
          <div class="file-prop-list">
            <div class="file-prop-row">
              <span class="file-prop-label">类型</span>
              <span class="file-prop-value">{{ ext }}</span>
            </div>
            <div class="file-prop-row">
              <span class="file-prop-label">大小</span>
              <span class="file-prop-value">{{ file ? formatBytes(file.size) : '—' }}</span>
            </div>
            <div class="file-prop-row">
              <span class="file-prop-label">来源</span>
              <span class="file-prop-value">{{ file?.source || '—' }}</span>
            </div>
            <div class="file-prop-row">
              <span class="file-prop-label">修改时间</span>
              <span class="file-prop-value">{{ file ? formatTime(file.modified) : '—' }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useFilesStore } from '@/stores/files'
import { useDownload } from '@/composables/useDownload'

const emit = defineEmits<{ (e: 'delete-selected'): void }>()

const files = useFilesStore()
const { downloadSelectedFiles } = useDownload()

const countText = computed(() => {
  if (files.selectedCount === 0) return '未选择'
  return '已选 ' + files.selectedCount + ' 项'
})

function onDelete() {
  emit('delete-selected')
}
</script>

<template>
  <div v-if="files.batchMode" class="batch-bar">
    <span>{{ countText }}</span>
    <div class="batch-bar-actions">
      <button class="batch-btn" @click="files.selectAllFiles()">全选</button>
      <button class="batch-btn" @click="downloadSelectedFiles">下载</button>
      <button class="batch-btn batch-btn-danger" @click="onDelete">删除</button>
      <button class="batch-btn batch-btn-primary" @click="files.toggleBatchMode()">取消</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import type { FileInfo } from '@/api/files'
import { getDownloadUrl } from '@/api/files'
import { formatBytes, formatDateTime } from '@/utils/format'
import { useFilesStore } from '@/stores/files'
import { useFilePreview } from '@/composables/useFilePreview'
import { useContextMenu } from '@/composables/useContextMenu'
import FileIcon from '@/components/icons/FileIcon.vue'

const props = defineProps<{ file: FileInfo }>()
const emit = defineEmits<{
  (e: 'long-press', filename: string): void
}>()

const files = useFilesStore()
const { openPreview } = useFilePreview()
const { showContextMenu } = useContextMenu()

const name = computed(() => props.file.name || '')
const lastDot = computed(() => name.value.lastIndexOf('.'))
const hasExt = computed(() => lastDot.value > 0)
const baseName = computed(() => (hasExt.value ? name.value.slice(0, lastDot.value) : name.value))
const ext = computed(() => (hasExt.value ? name.value.slice(lastDot.value + 1).toLowerCase() : ''))

const category = computed(() => files.getFileCategory(name.value))
const isImage = computed(() => category.value === 'image')
const isVideo = computed(() => category.value === 'video')
const imgUrl = computed(() => getDownloadUrl(name.value))

const isSelected = computed(() => files.selectedFileNames.has(name.value))
const isNew = computed(() => files.newFileNames.has(name.value))

const tagClassMap: Record<string, string> = { '本机': 'tag-local', '手机': 'tag-phone', '其他': 'tag-other' }
const deviceTag = computed(() => {
  const src = props.file.source || ''
  if (!src) return ''
  const d = src.toLowerCase()
  if (src === '本机' || d === 'pc' || d === 'mac' || d === 'linux') return '本机'
  if (d.includes('iphone') || d.includes('ipad')) return '手机'
  if (d.includes('android')) return '手机'
  return '其他'
})

// 视频缩略图：尝试截取第一帧，失败则回退图标
const videoRef = ref<HTMLVideoElement | null>(null)
const videoOk = ref(true)

onMounted(() => {
  const v = videoRef.value
  if (!v) return
  v.addEventListener('loadeddata', () => {
    try { v.currentTime = Math.min(1, (v.duration || 1) * 0.1) } catch (e) { /* ignore */ }
  })
  v.addEventListener('error', () => { videoOk.value = false })
  setTimeout(() => {
    if (v.readyState === 0) videoOk.value = false
  }, 4000)
})

function handleClick() {
  if (files.batchMode) {
    files.toggleFileSelection(name.value)
  } else {
    openPreview(name.value)
  }
}

function handleMore(e: MouseEvent) {
  e.stopPropagation()
  showContextMenu(e.clientX, e.clientY, name.value)
}

// 长按触发底部操作面板
let pressTimer: number | undefined
function onTouchStart() {
  pressTimer = window.setTimeout(() => {
    emit('long-press', name.value)
  }, 500)
}
function onTouchMove() {
  if (pressTimer) { clearTimeout(pressTimer); pressTimer = undefined }
}
function onTouchEnd() {
  if (pressTimer) { clearTimeout(pressTimer); pressTimer = undefined }
}
</script>

<template>
  <div
    class="file-card"
    :class="{ selected: isSelected, 'new-file': isNew }"
    :data-filename="name"
    @click="handleClick"
    @touchstart="onTouchStart"
    @touchmove="onTouchMove"
    @touchend="onTouchEnd"
    @contextmenu.prevent="handleMore"
  >
    <span
      v-if="deviceTag"
      class="file-device-tag"
      :class="tagClassMap[deviceTag]"
    >{{ deviceTag }}</span>
    <div class="file-card-check"></div>
    <button class="file-card-more-btn" title="更多" @click.stop="handleMore">
      <svg class="icon icon-sm" viewBox="0 0 24 24"><circle cx="12" cy="5" r="2"/><circle cx="12" cy="12" r="2"/><circle cx="12" cy="19" r="2"/></svg>
    </button>
    <div class="file-card-preview">
      <!-- 图片缩略图 -->
      <div
        v-if="isImage"
        class="file-card-thumb"
        :style="{ backgroundImage: `url('${imgUrl}')` }"
      ></div>
      <!-- 视频缩略图 -->
      <template v-else-if="isVideo">
        <div v-if="videoOk" class="video-thumb-wrap">
          <video
            ref="videoRef"
            preload="metadata"
            muted
            playsinline
            crossorigin="anonymous"
            :src="imgUrl"
          ></video>
          <div class="video-play-overlay">
            <svg class="icon" viewBox="0 0 24 24" style="width:24px;height:24px;fill:white;"><polygon points="5 3 19 12 5 21 5 3"/></svg>
          </div>
        </div>
        <FileIcon v-else :filename="name" />
      </template>
      <!-- 其他文件图标 -->
      <FileIcon v-else :filename="name" />
    </div>
    <div class="file-card-name-row">
      <span class="file-card-name" :title="name">{{ baseName }}</span>
    </div>
    <div class="file-card-meta">
      <span>{{ formatBytes(file.size || 0) }}</span>
      <span class="meta-dot"></span>
      <span>{{ formatDateTime(file.modified) }}</span>
    </div>
    <span v-if="hasExt" class="file-card-ext">.{{ ext }}</span>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { useFilesStore } from '@/stores/files'
import { useUiStore } from '@/stores/ui'
import { useUpload } from '@/composables/useUpload'
import { useDownload } from '@/composables/useDownload'
import { useTauri } from '@/composables/useTauri'
import { usePullRefresh } from '@/composables/usePullRefresh'
import { getDiskInfo } from '@/api/system'
import { formatBytes } from '@/utils/format'
import FileCard from './FileCard.vue'
import BatchBar from './BatchBar.vue'
import FilterPopover from './FilterPopover.vue'
import PullRefreshIndicator from './PullRefreshIndicator.vue'

const emit = defineEmits<{
  (e: 'long-press', filename: string): void
  (e: 'delete-selected'): void
}>()

const auth = useAuthStore()
const files = useFilesStore()
const ui = useUiStore()
const { openUploadModal } = useUpload()
const { downloadAllFiles } = useDownload()
const { openShareFolder } = useTauri()
const { pullState, initPullToRefresh, destroyPullToRefresh } = usePullRefresh()

// 磁盘信息
const diskFree = ref('')
const diskPct = ref(0)
const diskLoaded = ref(false)

async function loadDiskInfo() {
  try {
    const data = await getDiskInfo()
    if (data.error) return
    diskFree.value = formatBytes(data.used) + ' / ' + formatBytes(data.total)
    diskPct.value = data.total > 0 ? Math.round((data.used / data.total) * 100) : 0
    diskLoaded.value = true
  } catch (e) { /* ignore */ }
}

const filterNameMap: Record<string, string> = {
  image: '图片', video: '视频', audio: '音频', document: '文档',
}

const emptyTitle = computed(() => {
  if (files.currentFilter !== 'all' && files.allFilesData.length > 0) {
    return '没有' + (filterNameMap[files.currentFilter] || '该类型') + '文件'
  }
  return '暂无文件'
})
const emptyDesc = computed(() => {
  if (files.currentFilter !== 'all' && files.allFilesData.length > 0) {
    const fn = filterNameMap[files.currentFilter] || '该类型'
    return '已有其他文件？点击左上方【' + fn + '】切换筛选类型试试'
  }
  return '上传文件，或等待对方共享'
})

const showDownloadAll = computed(() => auth.userRole === 'guest' && !files.isEmpty)

onMounted(() => {
  if (auth.userRole === 'host') loadDiskInfo()
  // 初始化下拉刷新(内部会判断是否移动端,非移动端为 no-op)
  initPullToRefresh(document.body)
})

onUnmounted(() => {
  destroyPullToRefresh(document.body)
})
</script>

<template>
  <section id="filesSection" class="section">
    <!-- Disk info bar -->
    <div
      v-if="auth.userRole === 'host' && diskLoaded"
      class="disk-bar host-only"
    >
      <span>存储空间</span>
      <span style="font-weight:500;color:var(--text-secondary);">{{ diskFree }}</span>
      <div class="disk-bar-fill">
        <div class="disk-bar-used" :style="{ width: diskPct + '%' }"></div>
      </div>
    </div>

    <div class="files-header">
      <div class="files-title-wrap" style="position:relative;">
        <h2
          class="files-title files-title-clickable"
          @click.stop="ui.toggleFilterPopover()"
        >
          <span>{{ files.currentFilter === 'all' ? '全部' : (filterNameMap[files.currentFilter] || '全部') }}</span>
          <svg class="icon icon-sm files-title-arrow" viewBox="0 0 24 24"><polyline points="6 9 12 15 18 9"/></svg>
        </h2>
        <FilterPopover />
      </div>
      <div class="files-actions">
        <div class="files-secondary" :class="{ open: ui.filesMoreOpen }">
          <button class="chip chip-icon" title="切换视图" @click="files.toggleView()">
            <svg v-if="files.currentView === 'grid'" class="icon icon-sm" viewBox="0 0 24 24"><rect x="3" y="3" width="7" height="7" rx="1"/><rect x="14" y="3" width="7" height="7" rx="1"/><rect x="3" y="14" width="7" height="7" rx="1"/><rect x="14" y="14" width="7" height="7" rx="1"/></svg>
            <svg v-else class="icon icon-sm" viewBox="0 0 24 24"><line x1="8" y1="6" x2="21" y2="6"/><line x1="8" y1="12" x2="21" y2="12"/><line x1="8" y1="18" x2="21" y2="18"/><line x1="3" y1="6" x2="3.01" y2="6"/><line x1="3" y1="12" x2="3.01" y2="12"/><line x1="3" y1="18" x2="3.01" y2="18"/></svg>
            <span class="action-label">视图</span>
          </button>
          <button
            class="chip chip-icon"
            :class="{ active: files.batchMode }"
            title="多选"
            @click="files.toggleBatchMode()"
          >
            <svg class="icon icon-sm" viewBox="0 0 24 24"><polyline points="9 11 12 14 22 4"/><path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11"/></svg>
            <span class="action-label">多选</span>
          </button>
          <button
            v-if="auth.userRole === 'host'"
            class="chip chip-icon host-only"
            title="打开共享文件夹"
            @click="openShareFolder()"
          >
            <svg class="icon icon-sm" viewBox="0 0 24 24"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
            <span class="action-label">打开文件夹</span>
          </button>
        </div>
        <button
          class="chip chip-icon files-more-btn"
          :class="{ active: ui.filesMoreOpen }"
          title="更多操作"
          @click.stop="ui.toggleFilesMore()"
        >
          <svg class="icon icon-sm" viewBox="0 0 24 24"><circle cx="12" cy="5" r="2"/><circle cx="12" cy="12" r="2"/><circle cx="12" cy="19" r="2"/></svg>
        </button>
        <button class="btn-primary" @click="openUploadModal">
          <span class="action-label">上传</span>
          <span class="btn-icon-wrap">
            <svg class="icon icon-sm" viewBox="0 0 24 24"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" y1="3" x2="12" y2="15"/></svg>
          </span>
        </button>
      </div>
    </div>

    <!-- Batch bar -->
    <BatchBar @delete-selected="emit('delete-selected')" />

    <!-- Pull to refresh indicator -->
    <PullRefreshIndicator
      :visible="pullState.distance > 0 || pullState.isRefreshing || pullState.showSuccess"
      :loading="pullState.isRefreshing"
    />

    <!-- Download all (guest) -->
    <div v-if="showDownloadAll" class="files-header visible">
      <button class="btn-secondary" @click="downloadAllFiles">
        <svg class="icon icon-sm" viewBox="0 0 24 24"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
        下载全部
      </button>
    </div>

    <!-- File grid + empty state, wrapped for view mode class -->
    <div
      :class="{ 'file-list-mode': files.currentView === 'list' }"
      style="width:100%;"
    >
      <!-- File grid -->
      <div
        v-if="!files.isEmpty"
        class="file-grid"
        :class="{ 'file-grid-entrance': !files.isEmpty }"
      >
        <div class="group-header">共 {{ files.filteredFiles.length }} 个文件</div>
        <FileCard
          v-for="f in files.filteredFiles"
          :key="f.name"
          :file="f"
          @long-press="emit('long-press', $event)"
        />
      </div>

      <!-- Empty state -->
      <div v-else class="empty-state visible" role="status" aria-live="polite">
      <div class="empty-illustration">
        <svg viewBox="0 0 160 140" fill="none" xmlns="http://www.w3.org/2000/svg">
          <defs>
            <linearGradient id="emptyGrad1" x1="0%" y1="0%" x2="100%" y2="100%">
              <stop offset="0%" style="stop-color:var(--accent);stop-opacity:0.15"/>
              <stop offset="100%" style="stop-color:var(--accent);stop-opacity:0.03"/>
            </linearGradient>
          </defs>
          <ellipse cx="80" cy="120" rx="45" ry="6" fill="var(--bg-hover)"/>
          <g transform="translate(42, 28)">
            <rect x="0" y="20" width="68" height="56" rx="10" fill="var(--bg-surface)" stroke="var(--border-subtle)" stroke-width="1.5"/>
            <rect x="8" y="0" width="52" height="60" rx="10" fill="url(#emptyGrad1)" stroke="var(--accent)" stroke-width="1.5" stroke-opacity="0.4"/>
          </g>
          <g transform="translate(78, 36)">
            <circle cx="0" cy="0" r="18" fill="var(--accent-subtle)"/>
            <path d="M-6 -2 L0 -8 L6 -2" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
            <line x1="0" y1="-8" x2="0" y2="8" stroke="var(--accent)" stroke-width="2" stroke-linecap="round"/>
          </g>
          <circle cx="120" cy="52" r="3" fill="var(--accent)" opacity="0.4"/>
          <circle cx="38" cy="68" r="2" fill="var(--text-tertiary)" opacity="0.3"/>
          <circle cx="112" cy="88" r="2.5" fill="var(--text-tertiary)" opacity="0.25"/>
        </svg>
      </div>
      <div class="empty-title">{{ emptyTitle }}</div>
      <div class="empty-desc">{{ emptyDesc }}</div>
      <button class="btn-primary" style="margin-top: var(--s-4);" aria-label="上传文件到共享文件夹" @click="openUploadModal">
        上传文件
        <span class="btn-icon-wrap">
          <svg class="icon icon-sm" viewBox="0 0 24 24"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" y1="3" x2="12" y2="15"/></svg>
        </span>
      </button>
    </div>
    </div>
  </section>
</template>

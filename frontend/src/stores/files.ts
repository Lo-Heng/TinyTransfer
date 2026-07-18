import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { FileInfo } from '@/api/files'
import * as filesApi from '@/api/files'

export type SortKey = 'time-desc' | 'time-asc' | 'name-asc' | 'size-desc'
export type FilterKey = 'all' | 'image' | 'video' | 'audio' | 'document'
export type ViewMode = 'grid' | 'list'

export const useFilesStore = defineStore('files', () => {
  const allFilesData = ref<FileInfo[]>([])
  const batchMode = ref(false)
  const selectedFileNames = ref<Set<string>>(new Set())
  const currentSort = ref<SortKey>('time-desc')
  const currentFilter = ref<FilterKey>('all')
  const currentView = ref<ViewMode>('grid')
  const newFileNames = ref<Set<string>>(new Set()) // 新增高亮用

  // 筛选 + 排序后的文件列表（替代原 renderAllFiles 逻辑）
  const filteredFiles = computed(() => {
    let result = [...allFilesData.value]
    // 筛选
    if (currentFilter.value !== 'all') {
      result = result.filter((f) => getFileCategory(f.name) === currentFilter.value)
    }
    // 排序
    switch (currentSort.value) {
      case 'time-desc':
        result.sort((a, b) => new Date(b.modified).getTime() - new Date(a.modified).getTime())
        break
      case 'time-asc':
        result.sort((a, b) => new Date(a.modified).getTime() - new Date(b.modified).getTime())
        break
      case 'name-asc':
        result.sort((a, b) => a.name.localeCompare(b.name, 'zh'))
        break
      case 'size-desc':
        result.sort((a, b) => b.size - a.size)
        break
    }
    return result
  })

  const isEmpty = computed(() => filteredFiles.value.length === 0)
  const selectedCount = computed(() => selectedFileNames.value.size)

  function getFileCategory(filename: string): FilterKey {
    const ext = filename.split('.').pop()?.toLowerCase() || ''
    if (['jpg', 'jpeg', 'png', 'gif', 'webp', 'heic', 'heif'].includes(ext)) return 'image'
    if (['mp4', 'mov', 'avi', 'mkv', 'webm', 'hevc'].includes(ext)) return 'video'
    if (['mp3', 'wav', 'flac', 'aac', 'm4a'].includes(ext)) return 'audio'
    if (['pdf', 'doc', 'docx', 'xls', 'xlsx', 'ppt', 'pptx', 'txt'].includes(ext)) return 'document'
    return 'all'
  }

  async function loadAllFiles() {
    try {
      const files = await filesApi.getAllFiles()
      // 新增高亮：找出新增文件
      const oldNames = new Set(allFilesData.value.map((f) => f.name))
      const newNames = new Set<string>()
      for (const f of files) {
        if (!oldNames.has(f.name)) newNames.add(f.name)
      }
      newFileNames.value = newNames
      allFilesData.value = files
      // 3 秒后清除新高亮
      if (newNames.size > 0) {
        setTimeout(() => { newFileNames.value = new Set() }, 3000)
      }
    } catch (e: any) {
      if (e?.status !== 401) console.log('Failed to load files:', e)
    }
  }

  async function deleteFiles(filenames: string[]) {
    await filesApi.deleteFiles(filenames)
    for (const name of filenames) {
      selectedFileNames.value.delete(name)
    }
    await loadAllFiles()
  }

  function toggleBatchMode() {
    batchMode.value = !batchMode.value
    if (!batchMode.value) {
      selectedFileNames.value = new Set()
    }
  }

  function toggleFileSelection(filename: string) {
    if (selectedFileNames.value.has(filename)) {
      selectedFileNames.value.delete(filename)
    } else {
      selectedFileNames.value.add(filename)
    }
    // 触发响应式
    selectedFileNames.value = new Set(selectedFileNames.value)
  }

  function selectAllFiles() {
    selectedFileNames.value = new Set(filteredFiles.value.map((f) => f.name))
  }

  function setFilter(filter: FilterKey) {
    currentFilter.value = filter
  }

  function setSort(sort: SortKey) {
    currentSort.value = sort
  }

  function toggleView() {
    currentView.value = currentView.value === 'grid' ? 'list' : 'grid'
  }

  function getFileIconSVG(filename: string): string {
    const ext = filename.split('.').pop()?.toLowerCase() || ''
    // 图标 SVG 映射（从原 getFileIconSVG 函数迁移）
    const iconMap: Record<string, string> = {
      jpg: '🖼️', jpeg: '🖼️', png: '🖼️', gif: '🖼️', webp: '🖼️', heic: '🖼️',
      mp4: '🎬', mov: '🎬', avi: '🎬', mkv: '🎬', webm: '🎬',
      mp3: '🎵', wav: '🎵', flac: '🎵', aac: '🎵',
      pdf: '📄', doc: '📄', docx: '📄', xls: '📊', xlsx: '📊', ppt: '📊', pptx: '📊',
      txt: '📝', zip: '🗜️', rar: '🗜️', '7z': '🗜️',
    }
    return iconMap[ext] || '📄'
  }

  return {
    allFilesData, batchMode, selectedFileNames, currentSort, currentFilter, currentView,
    newFileNames, filteredFiles, isEmpty, selectedCount,
    loadAllFiles, deleteFiles, toggleBatchMode, toggleFileSelection,
    selectAllFiles, setFilter, setSort, toggleView, getFileCategory, getFileIconSVG,
  }
})

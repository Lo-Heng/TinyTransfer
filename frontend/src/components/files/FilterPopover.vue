<script setup lang="ts">
import { useFilesStore } from '@/stores/files'
import { useUiStore } from '@/stores/ui'

const files = useFilesStore()
const ui = useUiStore()
</script>

<template>
  <Transition name="popover">
    <div v-if="ui.filterPopoverOpen" class="filter-popover">
      <div class="filter-section">
        <div class="filter-section-title">
          <svg class="icon icon-xs" viewBox="0 0 24 24"><polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3"/></svg>
          按类型筛选
        </div>
        <div class="filter-type-grid">
          <div
            class="filter-type-item"
            :class="{ active: files.currentFilter === 'all' }"
            @click="files.setFilter('all')"
          >
            <div class="filter-type-icon">
              <svg class="icon" viewBox="0 0 24 24"><rect x="3" y="3" width="18" height="18" rx="2"/><line x1="3" y1="9" x2="21" y2="9"/><line x1="9" y1="21" x2="9" y2="9"/></svg>
            </div>
            <span>全部</span>
          </div>
          <div
            class="filter-type-item"
            :class="{ active: files.currentFilter === 'image' }"
            @click="files.setFilter('image')"
          >
            <div class="filter-type-icon">
              <svg class="icon" viewBox="0 0 24 24"><rect x="3" y="3" width="18" height="18" rx="2"/><circle cx="8.5" cy="8.5" r="1.5"/><polyline points="21 15 16 10 5 21"/></svg>
            </div>
            <span>图片</span>
          </div>
          <div
            class="filter-type-item"
            :class="{ active: files.currentFilter === 'video' }"
            @click="files.setFilter('video')"
          >
            <div class="filter-type-icon">
              <svg class="icon" viewBox="0 0 24 24"><polygon points="23 7 16 12 23 17 23 7"/><rect x="1" y="5" width="15" height="14" rx="2"/></svg>
            </div>
            <span>视频</span>
          </div>
          <div
            class="filter-type-item"
            :class="{ active: files.currentFilter === 'audio' }"
            @click="files.setFilter('audio')"
          >
            <div class="filter-type-icon">
              <svg class="icon" viewBox="0 0 24 24"><path d="M9 18V5l12-2v13"/><circle cx="6" cy="18" r="3"/><circle cx="18" cy="16" r="3"/></svg>
            </div>
            <span>音频</span>
          </div>
          <div
            class="filter-type-item"
            :class="{ active: files.currentFilter === 'document' }"
            @click="files.setFilter('document')"
          >
            <div class="filter-type-icon">
              <svg class="icon" viewBox="0 0 24 24"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/></svg>
            </div>
            <span>文档</span>
          </div>
        </div>
      </div>
      <div class="filter-divider"></div>
      <div class="filter-section">
        <div class="filter-section-title">
          <svg class="icon icon-xs" viewBox="0 0 24 24"><line x1="6" y1="6" x2="20" y2="6"/><line x1="6" y1="12" x2="20" y2="12"/><line x1="6" y1="18" x2="20" y2="18"/><polyline points="10 10 8 12 10 14"/><polyline points="16 14 18 12 16 10"/></svg>
          排序方式
        </div>
        <div class="filter-sort-list">
          <div
            class="filter-sort-item"
            :class="{ active: files.currentSort === 'time-desc' }"
            @click="files.setSort('time-desc')"
          >
            <span>最新优先</span>
            <svg v-if="files.currentSort === 'time-desc'" class="icon icon-xs check-icon" viewBox="0 0 24 24"><polyline points="20 6 9 17 4 12"/></svg>
          </div>
          <div
            class="filter-sort-item"
            :class="{ active: files.currentSort === 'time-asc' }"
            @click="files.setSort('time-asc')"
          >
            <span>最早优先</span>
            <svg v-if="files.currentSort === 'time-asc'" class="icon icon-xs check-icon" viewBox="0 0 24 24"><polyline points="20 6 9 17 4 12"/></svg>
          </div>
          <div
            class="filter-sort-item"
            :class="{ active: files.currentSort === 'name-asc' }"
            @click="files.setSort('name-asc')"
          >
            <span>名称 A-Z</span>
            <svg v-if="files.currentSort === 'name-asc'" class="icon icon-xs check-icon" viewBox="0 0 24 24"><polyline points="20 6 9 17 4 12"/></svg>
          </div>
          <div
            class="filter-sort-item"
            :class="{ active: files.currentSort === 'size-desc' }"
            @click="files.setSort('size-desc')"
          >
            <span>最大优先</span>
            <svg v-if="files.currentSort === 'size-desc'" class="icon icon-xs check-icon" viewBox="0 0 24 24"><polyline points="20 6 9 17 4 12"/></svg>
          </div>
        </div>
      </div>
    </div>
  </Transition>
</template>

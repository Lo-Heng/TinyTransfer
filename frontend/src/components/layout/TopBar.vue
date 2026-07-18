<script setup lang="ts">
import { computed } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { useDevicesStore } from '@/stores/devices'
import { useFilesStore } from '@/stores/files'
const emit = defineEmits<{ (e: 'open-settings'): void }>()

const auth = useAuthStore()
const devices = useDevicesStore()
const files = useFilesStore()

const connLabel = computed(() => {
  if (!devices.connected) return '等待连接'
  const names = devices.deviceNames.join(', ')
  return names.length > 15 ? names.slice(0, 15) + '...' : names || '本机'
})
</script>

<template>
  <div class="topbar">
    <span class="topbar-brand">
      <span class="topbar-brand-logo">
        <img src="/static/icons/lightning.svg" alt="Tiny Transfer" />
      </span>
      <span class="topbar-brand-text">
        <span class="topbar-brand-name">Tiny Transfer</span>
        <span class="topbar-brand-slogan">小巧、极速、无需上手</span>
      </span>
    </span>
    <div class="topbar-actions">
      <!-- Connection indicator (host) -->
      <div
        v-if="auth.userRole === 'host'"
        class="conn-indicator host-only"
        :class="{ online: devices.connected }"
        title="设备连接状态"
        @click.stop="devices.toggleFlyout()"
      >
        <span class="conn-dot" :class="{ online: devices.connected }"></span>
        <span class="conn-device-name">{{ connLabel }}</span>
      </div>
      <!-- Settings button -->
      <button class="topbar-btn" title="设置" aria-label="设置" @click="emit('open-settings')">
        <svg class="icon icon-sm" viewBox="0 0 24 24"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>
      </button>
      <!-- Refresh button -->
      <button class="topbar-btn" title="刷新文件列表" aria-label="刷新" @click="files.loadAllFiles()">
        <svg class="icon icon-sm" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M23 4v6h-6"/><path d="M1 20v-6h6"/><path d="M3.51 9a9 9 0 0114.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0020.49 15"/></svg>
      </button>
    </div>
  </div>
</template>

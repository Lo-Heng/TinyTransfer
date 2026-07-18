<script setup lang="ts">
import { useAuthStore } from '@/stores/auth'
import { useUiStore } from '@/stores/ui'

const auth = useAuthStore()
const ui = useUiStore()
</script>

<template>
  <div
    v-if="auth.userRole === 'host'"
    class="drag-overlay host-only"
    :class="{ active: ui.dragOverlayActive }"
  >
    <div class="drag-overlay-inner">
      <div class="drag-overlay-icon">
        <svg class="icon" viewBox="0 0 24 24" style="font-size:56px;"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" y1="3" x2="12" y2="15"/></svg>
      </div>
      <div class="drag-overlay-text">松手即可上传</div>
      <div class="drag-overlay-hint">文件将直接上传到共享文件夹</div>
    </div>
  </div>
</template>

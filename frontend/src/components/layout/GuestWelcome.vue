<script setup lang="ts">
import { useAuthStore } from '@/stores/auth'
import { useUpload } from '@/composables/useUpload'

const auth = useAuthStore()
const { openUploadModal } = useUpload()

function scrollToFiles() {
  document.getElementById('filesSection')?.scrollIntoView({ behavior: 'smooth' })
}
</script>

<template>
  <section v-if="auth.userRole === 'guest'" class="section guest-only">
    <div class="guest-welcome">
      <div class="guest-welcome-icon">
        <svg class="icon icon-xl" viewBox="0 0 24 24" style="font-size:48px;"><path d="M14 1L3 13L11 13L9 23L21 11L13 11Z"/></svg>
      </div>
      <h2 class="guest-welcome-title">已连接到电脑</h2>
      <p class="guest-welcome-desc">上传文件到电脑，或浏览下载共享文件</p>
      <div style="display:flex;gap:var(--s-3);justify-content:center;flex-wrap:wrap;">
        <button class="btn-primary" @click="openUploadModal">
          上传文件
          <span class="btn-icon-wrap">
            <svg class="icon icon-sm" viewBox="0 0 24 24"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" y1="3" x2="12" y2="15"/></svg>
          </span>
        </button>
        <button class="btn-secondary" @click="scrollToFiles">浏览共享文件</button>
      </div>
    </div>
  </section>
</template>

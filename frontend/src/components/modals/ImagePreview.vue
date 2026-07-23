<script setup lang="ts">
import { ref } from 'vue'

const open = ref(false)
const src = ref('')
const name = ref('')

function show(imgSrc: string, filename: string) {
  src.value = imgSrc
  name.value = filename || ''
  open.value = true
  document.body.style.overflow = 'hidden'
}

function close() {
  open.value = false
  document.body.style.overflow = ''
}

defineExpose({ show, close })
</script>

<template>
  <Transition name="img-preview">
    <div
      v-if="open"
      class="img-preview-overlay"
      role="dialog"
      aria-modal="true"
      :aria-label="name || '图片预览'"
      @click.self="close"
    >
      <div class="img-preview-wrap">
        <button class="img-preview-close" aria-label="关闭" title="关闭" @click="close">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"><path d="M18 6 6 18"/><path d="M6 6l12 12"/></svg>
        </button>
        <img :src="src" :alt="name" />
        <div class="img-preview-name">{{ name }}</div>
      </div>
    </div>
  </Transition>
</template>

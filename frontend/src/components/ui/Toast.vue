<script setup lang="ts">
import { useUiStore } from '@/stores/ui'

const ui = useUiStore()

const icons = {
  success: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>',
  error: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/></svg>',
  info: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/></svg>',
}
</script>

<template>
  <TransitionGroup tag="div" name="toast" class="toast-container">
    <div
      v-for="t in ui.toasts"
      :key="t.id"
      class="toast"
      :class="['toast-' + t.type]"
    >
      <span class="toast-icon" v-html="t.icon || icons[t.type]"></span>
      <span class="toast-msg">{{ t.msg }}</span>
    </div>
  </TransitionGroup>
</template>

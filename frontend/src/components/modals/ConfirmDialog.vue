<script setup lang="ts">
import { ref } from 'vue'

interface ConfirmOptions {
  title?: string
  message?: string
  confirmLabel?: string
  cancelLabel?: string
  onConfirm?: () => void
  onCancel?: () => void
}

const open = ref(false)
const title = ref('确认删除')
const message = ref('此操作不可撤销')
const confirmLabel = ref('删除')
const cancelLabel = ref('取消')
let confirmCb: (() => void) | null = null
let cancelCb: (() => void) | null = null

function show(opts: ConfirmOptions = {}) {
  title.value = opts.title || '确认删除'
  message.value = opts.message || '此操作不可撤销'
  confirmLabel.value = opts.confirmLabel || '删除'
  cancelLabel.value = opts.cancelLabel || '取消'
  confirmCb = opts.onConfirm || null
  cancelCb = opts.onCancel || null
  open.value = true
}

function close() {
  open.value = false
}

function onConfirm() {
  close()
  if (confirmCb) confirmCb()
}

function onCancel() {
  close()
  if (cancelCb) cancelCb()
}

defineExpose({ show, close })
</script>

<template>
  <Transition name="modal">
    <div v-if="open" class="confirm-overlay">
      <div class="confirm-dialog">
        <div class="confirm-body">
          <div class="confirm-icon-wrap">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
              <path d="M3 6h18"/><path d="M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"/><path d="M10 11v6"/><path d="M14 11v6"/>
            </svg>
          </div>
          <div class="confirm-title">{{ title }}</div>
          <div class="confirm-message">{{ message }}</div>
        </div>
        <div class="confirm-actions">
          <button class="confirm-btn confirm-btn-cancel" @click="onCancel">{{ cancelLabel }}</button>
          <button class="confirm-btn confirm-btn-danger" @click="onConfirm">{{ confirmLabel }}</button>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ref, nextTick } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { useUiStore } from '@/stores/ui'

const auth = useAuthStore()
const ui = useUiStore()

const open = ref(false)
const password = ref('')
const inputRef = ref<HTMLInputElement | null>(null)

function show() {
  password.value = ''
  open.value = true
  nextTick(() => inputRef.value?.focus())
}

function close() {
  open.value = false
}

async function submit() {
  if (!password.value) return
  const ok = await auth.login(password.value)
  if (ok) {
    close()
    ui.showToast('登录成功', 'success')
  } else {
    ui.showToast('密码错误', 'error')
    password.value = ''
  }
}

defineExpose({ show, close })
</script>

<template>
  <Transition name="modal">
    <div
      v-if="open"
      class="modal-overlay"
      role="dialog"
      aria-modal="true"
      aria-label="输入访问密码"
      @click.self="close"
    >
      <div class="modal-sheet">
        <div class="modal-header">
          <h3 class="modal-title">输入访问密码</h3>
          <button class="modal-close" title="关闭" @click="close">
            <svg class="icon icon-sm" viewBox="0 0 24 24"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
          </button>
        </div>
        <input
          ref="inputRef"
          type="password"
          class="input-field"
          placeholder="请输入密码"
          autocomplete="off"
          v-model="password"
          @keyup.enter="submit"
        />
        <button class="btn-primary" style="width:100%;" @click="submit">确认</button>
      </div>
    </div>
  </Transition>
</template>

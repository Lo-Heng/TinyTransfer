<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { useDevicesStore } from '@/stores/devices'

const auth = useAuthStore()
const devices = useDevicesStore()

const flyoutRef = ref<HTMLElement | null>(null)

function isLocal(ip: string) {
  return ip === '127.0.0.1' || ip === '::1'
}

function handleClickOutside(e: MouseEvent) {
  if (!flyoutRef.value) return
  const target = e.target as HTMLElement
  if (flyoutRef.value.contains(target)) return
  if (target.closest('.conn-indicator')) return
  devices.closeFlyout()
}

watch(() => devices.flyoutOpen, (isOpen) => {
  if (isOpen) {
    document.addEventListener('mousedown', handleClickOutside)
  } else {
    document.removeEventListener('mousedown', handleClickOutside)
  }
})

onUnmounted(() => {
  document.removeEventListener('mousedown', handleClickOutside)
})
</script>

<template>
  <div
    ref="flyoutRef"
    v-if="auth.userRole === 'host'"
    class="flyout host-only"
    :class="{ open: devices.flyoutOpen }"
  >
    <div class="flyout-label">已连接设备</div>
    <div>
      <template v-if="devices.remoteDevices.length === 0">
        <div style="text-align:center;color:var(--text-tertiary);padding:8px 4px;font-size:12px;">等待设备连接...</div>
      </template>
      <template v-else>
        <div v-for="(d, i) in devices.remoteDevices" :key="i" class="flyout-device">
          <div class="flyout-device-icon">{{ devices.getDeviceEmoji(d.type) }}</div>
          <div class="flyout-device-info">
            <div class="flyout-device-name">{{ d.model || d.type }}</div>
            <div class="flyout-device-meta">{{ d.detail }} · {{ d.ip }}</div>
          </div>
          <div v-if="isLocal(d.ip)" class="flyout-tag">本机</div>
        </div>
      </template>
    </div>
  </div>
</template>

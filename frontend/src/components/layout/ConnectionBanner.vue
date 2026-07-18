<script setup lang="ts">
import { useAuthStore } from '@/stores/auth'
import { useDevicesStore } from '@/stores/devices'

const auth = useAuthStore()
const devices = useDevicesStore()
</script>

<template>
  <Transition name="banner">
    <div
      v-if="auth.userRole === 'host' && devices.connected"
      class="conn-banner host-only"
    >
      <span class="conn-banner-icon">
        <svg viewBox="0 0 24 24" fill="none" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="5" y="2" width="14" height="20" rx="2" ry="2"/><line x1="12" y1="18" x2="12.01" y2="18"/></svg>
      </span>
      <span>{{ devices.deviceNames.length ? devices.deviceNames[0] + ' 已连接' : '设备已连接' }}</span>
    </div>
  </Transition>
</template>

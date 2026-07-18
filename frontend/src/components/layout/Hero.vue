<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import qrcode from 'qrcode-generator'
import { useAuthStore } from '@/stores/auth'
import { useDevicesStore } from '@/stores/devices'
import { useUiStore } from '@/stores/ui'
import { getIP } from '@/api/system'

const auth = useAuthStore()
const devices = useDevicesStore()
const ui = useUiStore()

const serverURL = ref('加载中...')
const qrSrc = ref('')

const heroTitle = computed(() =>
  devices.connected ? '已连接' : '扫码连接'
)
const heroSubtitle = computed(() =>
  devices.connected
    ? (devices.deviceNames.join('、') || '设备已就绪')
    : '手机扫码即可开始传输'
)

const connStatusText = computed(() => {
  if (!devices.connected) return '已连接'
  const names = devices.deviceNames
  return names.length === 1 ? '已连接 · ' + names[0] : '已连接 · ' + names.length + '台设备'
})

async function loadIPAndQR() {
  try {
    const info = await getIP()
    serverURL.value = info.url
    const qr = qrcode(0, 'L')
    qr.addData(info.url)
    qr.make()
    qrSrc.value = qr.createDataURL(8)
  } catch (e) {
    console.log('Failed to generate QR:', e)
  }
}

async function copyURL() {
  const text = serverURL.value
  try {
    await navigator.clipboard.writeText(text)
  } catch (e) {
    const input = document.createElement('textarea')
    input.value = text
    document.body.appendChild(input)
    input.select()
    document.execCommand('copy')
    document.body.removeChild(input)
  }
  ui.showToast('链接已复制', 'success')
  ui.completeStep(1)
}

function toggleQRSize() {
  // 已连接（紧凑模式）时点击放大：退出紧凑
  if (devices.compactQRActive) devices.compactQRActive = false
}

onMounted(() => {
  if (auth.userRole === 'host') loadIPAndQR()
})

watch(() => auth.userRole, (newRole) => {
  if (newRole === 'host') loadIPAndQR()
})
</script>

<template>
  <section
    v-if="auth.userRole === 'host'"
    class="hero hero-entrance"
    :class="{ 'qr-compact': devices.compactQRActive }"
  >
    <div class="hero-card" :class="{ 'qr-compact': devices.compactQRActive }">
      <div class="hero-heading">
        <h1 class="hero-title">{{ heroTitle }}</h1>
        <p class="hero-subtitle">{{ heroSubtitle }}</p>
      </div>
      <div class="hero-steps">
        <div class="hero-step" :class="{ active: ui.qsCompleted[0] }">
          <div class="step-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
              <rect x="3" y="3" width="7" height="7" rx="1.5"/>
              <rect x="14" y="3" width="7" height="7" rx="1.5"/>
              <rect x="3" y="14" width="7" height="7" rx="1.5"/>
              <rect x="14" y="14" width="3" height="3" rx="0.5"/>
              <rect x="19" y="14" width="2" height="3" rx="0.5"/>
              <rect x="14" y="19" width="3" height="2" rx="0.5"/>
            </svg>
          </div>
          <span class="step-label">扫码连接</span>
        </div>
        <div class="step-connector"></div>
        <div class="hero-step" :class="{ active: ui.qsCompleted[1] }">
          <div class="step-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
              <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z"/>
              <polyline points="14 2 14 8 20 8"/>
              <line x1="12" y1="18" x2="12" y2="12"/>
              <polyline points="9 14 12 11 15 14"/>
            </svg>
          </div>
          <span class="step-label">拖拽传输</span>
        </div>
      </div>

      <!-- QR code area -->
      <div class="qr-card" title="点击放大二维码" @click="toggleQRSize">
        <img class="qr-img" alt="扫描二维码连接" :src="qrSrc" />
        <div class="qr-connected-badge" :class="{ show: devices.connected }">✓</div>
      </div>
      <div class="qr-expand-hint">点击二维码放大</div>
      <div v-show="!devices.connected" class="wifi-hint">确保手机和电脑在同一 WiFi 下</div>
      <!-- Compact mode info container -->
      <div class="compact-info">
        <!-- Connection status bar -->
        <div class="conn-status-bar" :class="{ show: devices.connected }">
          <svg class="icon icon-xs" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>
          <span>{{ connStatusText }}</span>
        </div>

        <!-- URL row -->
        <div class="url-row" style="cursor:pointer;" title="点击复制链接" @click="copyURL">
          <span class="url-text-inner">{{ serverURL }}</span>
          <button class="url-copy-btn" title="复制链接" @click.stop="copyURL">
            <svg class="icon icon-sm" viewBox="0 0 24 24"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>
          </button>
        </div>
      </div>
    </div>
  </section>
</template>

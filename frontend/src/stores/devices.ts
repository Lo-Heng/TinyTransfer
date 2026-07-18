import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { DeviceSnapshot } from '@/api/devices'
import * as devicesApi from '@/api/devices'

export const useDevicesStore = defineStore('devices', () => {
  const remoteDevices = ref<DeviceSnapshot[]>([])
  const prevRemoteCount = ref(0)
  const sseClientId = ref<string | null>(null)
  const compactQRActive = ref(false)
  const hasEverConnected = ref(false)
  const flyoutOpen = ref(false)

  const remoteOnly = computed(() =>
    remoteDevices.value.filter((d) => d.ip !== '127.0.0.1' && d.ip !== '::1')
  )
  const connected = computed(() => remoteOnly.value.length > 0)
  const deviceNames = computed(() =>
    remoteOnly.value.map((d) => d.model || d.type)
  )

  function setDevices(devices: DeviceSnapshot[]) {
    prevRemoteCount.value = remoteDevices.value.length
    remoteDevices.value = devices || []
    const newRemoteOnly = remoteDevices.value.filter((d) => d.ip !== '127.0.0.1' && d.ip !== '::1')
    if (newRemoteOnly.length > 0) {
      compactQRActive.value = true
    } else {
      compactQRActive.value = false
    }
  }

  async function loadDevices() {
    try {
      const data = await devicesApi.getDevices()
      setDevices(data.devices || [])
    } catch (e: any) {
      if (e?.status !== 401) {
        console.log('Failed to load devices:', e)
      }
    }
  }

  function toggleFlyout() {
    flyoutOpen.value = !flyoutOpen.value
  }

  function closeFlyout() {
    flyoutOpen.value = false
  }

  function getDeviceEmoji(type: string): string {
    const map: Record<string, string> = {
      iPhone: '📱', iPad: '📱', Android: '📱', 'Android平板': '📱',
      'Windows PC': '💻', Mac: '💻', Linux: '💻',
    }
    return map[type] || '💻'
  }

  return {
    remoteDevices, prevRemoteCount, sseClientId, compactQRActive,
    hasEverConnected, flyoutOpen, connected, remoteOnly, deviceNames,
    setDevices, loadDevices, toggleFlyout, closeFlyout, getDeviceEmoji,
  }
})

// SSE 实时通信 — 设备列表/文件列表变化推送
// 原文件: 5380-5636 (connectSSE/scheduleReconnect/loadDevices/sendPing/startPing/stopPing/visibilitychange)

import { useDevicesStore } from '@/stores/devices'
import { useFilesStore } from '@/stores/files'
import * as devicesApi from '@/api/devices'

// 模块级单例状态（SSE 连接全局唯一）
let eventSource: EventSource | null = null
let pingTimer: ReturnType<typeof setInterval> | null = null
let reconnectTimer: ReturnType<typeof setTimeout> | null = null
let reconnectDelay = 500
let visibilityHandlerRegistered = false

function sendPing(clientId: string) {
  if (!clientId) return
  devicesApi.ping(clientId).catch((e) => {
    console.log('ping failed:', e)
  })
}

function startPing(clientId: string) {
  stopPing()
  sendPing(clientId)
  pingTimer = setInterval(() => sendPing(clientId), 10000)
}

function stopPing() {
  if (pingTimer) {
    clearInterval(pingTimer)
    pingTimer = null
  }
}

function scheduleReconnect(connectFn: () => void) {
  if (reconnectTimer) return
  reconnectTimer = setTimeout(() => {
    reconnectTimer = null
    connectFn()
  }, reconnectDelay)
  // 指数退避，最多 3000ms
  reconnectDelay = Math.min(reconnectDelay * 2, 3000)
}

/** 建立 SSE 连接，监听设备列表与文件列表变化 */
export function connectSSE() {
  const devices = useDevicesStore()
  const files = useFilesStore()

  if (reconnectTimer) {
    clearTimeout(reconnectTimer)
    reconnectTimer = null
  }

  // 注册 visibilitychange（仅一次）
  if (!visibilityHandlerRegistered) {
    visibilityHandlerRegistered = true
    document.addEventListener('visibilitychange', () => {
      if (document.visibilityState !== 'visible') return
      if (!eventSource || eventSource.readyState === EventSource.CLOSED) {
        reconnectDelay = 100
        connectSSE()
      } else if (devices.sseClientId) {
        sendPing(devices.sseClientId)
      }
    })
  }

  // 通过 URL 传 UA（某些环境请求头不带 UA）
  const sseUrl = '/api/events?ua=' + encodeURIComponent(navigator.userAgent || '')
  const es = new EventSource(sseUrl)
  eventSource = es

  es.addEventListener('hello', (e) => {
    const data = JSON.parse((e as MessageEvent).data)
    devices.sseClientId = data.client_id || null
    console.log('SSE connected, client_id:', devices.sseClientId)
    reconnectDelay = 500
    startPing(devices.sseClientId!)
    // 主动拉一次设备列表，避免错过第一次推送
    devices.loadDevices()
  })

  es.addEventListener('device_list', (e) => {
    const data = JSON.parse((e as MessageEvent).data)
    devices.setDevices(data.devices || [])
  })

  es.addEventListener('ping', () => {
    // 服务端心跳，无需操作
  })

  es.addEventListener('file_list_updated', () => {
    files.loadAllFiles()
  })

  es.onerror = () => {
    console.log('SSE connection lost, reconnecting in', reconnectDelay, 'ms')
    es.close()
    stopPing()
    devices.sseClientId = null
    scheduleReconnect(connectSSE)
  }
}

/** 断开 SSE 连接，停止心跳 */
export function disconnectSSE() {
  stopPing()
  if (reconnectTimer) {
    clearTimeout(reconnectTimer)
    reconnectTimer = null
  }
  if (eventSource) {
    eventSource.close()
    eventSource = null
  }
  const devices = useDevicesStore()
  devices.sseClientId = null
}

export function useSSE() {
  return { connectSSE, disconnectSSE }
}

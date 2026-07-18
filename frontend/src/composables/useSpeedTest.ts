// 局域网测速 — 下载/上传测速，进度与历史记录
// 原文件: 4535-4819 (startDownloadSpeedTest/startUploadSpeedTest/startBothSpeedTest/formatSpeed)

import { ref } from 'vue'
import { formatSpeed } from '@/utils/format'

export interface SpeedTestHistoryItem {
  type: string
  speed: string
  size: string
  time: string
}

// 模块级单例状态
const speedTestHistory = ref<SpeedTestHistoryItem[]>([])
const isSpeedTesting = ref(false)
const speedTestStatus = ref('')
const speedTestProgress = ref({ percent: 0, speedText: '' })
const downloadSpeed = ref({ value: '0', unit: 'B/s' })
const uploadSpeed = ref({ value: '0', unit: 'B/s' })
const showSpeedValues = ref(false)

function setStatus(text: string) {
  speedTestStatus.value = text
}

function setProgress(percent: number, speedText: string) {
  speedTestProgress.value = { percent, speedText }
}

function updateDownloadSpeed(speed: number) {
  downloadSpeed.value = formatSpeed(speed)
}

function updateUploadSpeed(speed: number) {
  uploadSpeed.value = formatSpeed(speed)
}

function addHistory(type: string, speed: number, sizeMB: number) {
  const formatted = formatSpeed(speed)
  speedTestHistory.value.unshift({
    type,
    speed: formatted.value + ' ' + formatted.unit,
    size: sizeMB + ' MB',
    time: new Date().toLocaleTimeString(),
  })
  if (speedTestHistory.value.length > 10) {
    speedTestHistory.value.pop()
  }
}

/** 下载测速 */
export async function startDownloadSpeedTest(sizeMB: number = 10): Promise<void> {
  if (isSpeedTesting.value) return
  isSpeedTesting.value = true

  const totalBytes = sizeMB * 1024 * 1024
  setStatus('正在下载测速...')
  setProgress(0, '')
  showSpeedValues.value = true
  updateDownloadSpeed(0)

  const startTime = Date.now()
  let lastUpdateTime = startTime
  let downloaded = 0

  try {
    const response = await fetch('/api/speedtest/download?size=' + sizeMB)
    if (!response.ok) throw new Error('HTTP ' + response.status)

    const reader = response.body!.getReader()
    // eslint-disable-next-line no-constant-condition
    while (true) {
      const result = await reader.read()
      if (result.done) break

      downloaded += result.value!.length
      const now = Date.now()
      const elapsed = (now - startTime) / 1000
      const speed = downloaded / elapsed

      if (now - lastUpdateTime >= 100) {
        const percent = (downloaded / totalBytes) * 100
        const formatted = formatSpeed(speed)
        setProgress(percent, formatted.value + ' ' + formatted.unit)
        updateDownloadSpeed(speed)
        lastUpdateTime = now
      }
    }

    const totalTime = (Date.now() - startTime) / 1000
    const avgSpeed = downloaded / totalTime
    const formatted = formatSpeed(avgSpeed)
    setProgress(100, formatted.value + ' ' + formatted.unit)
    updateDownloadSpeed(avgSpeed)
    setStatus('下载测速完成 - 平均: ' + formatted.value + ' ' + formatted.unit)
    addHistory('下载', avgSpeed, sizeMB)
  } catch (e: any) {
    setStatus('下载测速失败: ' + e.message)
  }

  isSpeedTesting.value = false
}

/** 上传测速 */
export async function startUploadSpeedTest(sizeMB: number = 10): Promise<void> {
  if (isSpeedTesting.value) return
  isSpeedTesting.value = true

  const totalBytes = sizeMB * 1024 * 1024
  setStatus('正在上传测速...')
  setProgress(0, '')
  showSpeedValues.value = true
  updateUploadSpeed(0)

  try {
    const startTime = Date.now()
    let lastUpdateTime = startTime

    const formData = new FormData()
    const blob = new Blob([new Uint8Array(totalBytes)], {
      type: 'application/octet-stream',
    })
    formData.append('data', blob, 'speedtest.bin')

    const xhr = new XMLHttpRequest()
    xhr.open('POST', '/api/speedtest/upload', true)

    xhr.upload.onprogress = function (e) {
      if (e.lengthComputable) {
        const uploaded = e.loaded
        const now = Date.now()
        const elapsed = (now - startTime) / 1000
        const speed = uploaded / elapsed

        if (now - lastUpdateTime >= 100) {
          const percent = (uploaded / e.total) * 100
          const formatted = formatSpeed(speed)
          setProgress(percent, formatted.value + ' ' + formatted.unit)
          updateUploadSpeed(speed)
          lastUpdateTime = now
        }
      }
    }

    await new Promise<void>((resolve, reject) => {
      xhr.onload = () => {
        if (xhr.status >= 200 && xhr.status < 300) {
          resolve()
        } else {
          reject(new Error('HTTP ' + xhr.status))
        }
      }
      xhr.onerror = () => reject(new Error('网络错误'))
      xhr.send(formData)
    })

    const totalTime = (Date.now() - startTime) / 1000
    const avgSpeed = totalBytes / totalTime
    const formatted = formatSpeed(avgSpeed)
    setProgress(100, formatted.value + ' ' + formatted.unit)
    updateUploadSpeed(avgSpeed)
    setStatus('上传测速完成 - 平均: ' + formatted.value + ' ' + formatted.unit)
    addHistory('上传', avgSpeed, sizeMB)
  } catch (e: any) {
    setStatus('上传测速失败: ' + e.message)
  }

  isSpeedTesting.value = false
}

/** 同时进行下载+上传测速 */
export async function startBothSpeedTest(sizeMB: number = 10): Promise<void> {
  if (isSpeedTesting.value) return
  await startDownloadSpeedTest(sizeMB)
  await new Promise((r) => setTimeout(r, 500))
  await startUploadSpeedTest(sizeMB)
}

/** 清除测速历史 */
export function clearSpeedTestHistory() {
  speedTestHistory.value = []
}

export function useSpeedTest() {
  return {
    speedTestHistory,
    isSpeedTesting,
    speedTestStatus,
    speedTestProgress,
    downloadSpeed,
    uploadSpeed,
    showSpeedValues,
    startDownloadSpeedTest,
    startUploadSpeedTest,
    startBothSpeedTest,
    clearSpeedTestHistory,
  }
}

// 格式化工具函数

export function formatSpeed(bytesPerSecond: number): { value: string; unit: string } {
  if (bytesPerSecond >= 1024 * 1024 * 1024) {
    return { value: (bytesPerSecond / (1024 * 1024 * 1024)).toFixed(2), unit: 'GB/s' }
  } else if (bytesPerSecond >= 1024 * 1024) {
    return { value: (bytesPerSecond / (1024 * 1024)).toFixed(2), unit: 'MB/s' }
  } else if (bytesPerSecond >= 1024) {
    return { value: (bytesPerSecond / 1024).toFixed(2), unit: 'KB/s' }
  } else {
    return { value: bytesPerSecond.toFixed(0), unit: 'B/s' }
  }
}

export function formatBytes(bytes: number): string {
  if (bytes >= 1024 * 1024 * 1024) {
    return (bytes / (1024 * 1024 * 1024)).toFixed(2) + ' GB'
  } else if (bytes >= 1024 * 1024) {
    return (bytes / (1024 * 1024)).toFixed(2) + ' MB'
  } else if (bytes >= 1024) {
    return (bytes / 1024).toFixed(2) + ' KB'
  } else {
    return bytes + ' B'
  }
}

// formatSize 是 formatBytes 的别名
export const formatSize = formatBytes

export function formatTime(dateStr: string): string {
  const d = new Date(dateStr)
  const now = new Date()
  const diff = now.getTime() - d.getTime()
  if (diff < 60 * 1000) return '刚刚'
  if (diff < 60 * 60 * 1000) return Math.floor(diff / (60 * 1000)) + ' 分钟前'
  if (diff < 24 * 60 * 60 * 1000) return Math.floor(diff / (60 * 60 * 1000)) + ' 小时前'
  if (d.getFullYear() === now.getFullYear()) {
    return `${d.getMonth() + 1}月${d.getDate()}日`
  }
  return `${d.getFullYear()}年${d.getMonth() + 1}月${d.getDate()}日`
}

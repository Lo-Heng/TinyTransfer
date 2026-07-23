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

export function formatTime(dateStr: string | number): string {
  // 后端返回秒级 Unix 时间戳(浮点),前端 new Date() 按毫秒解析,需要乘 1000
  // 判定依据:10 位及以下为秒级,13 位为毫秒级
  let timestamp: number
  if (typeof dateStr === 'number') {
    timestamp = dateStr > 1e12 ? dateStr : dateStr * 1000
  } else if (/^\d+(\.\d+)?$/.test(dateStr)) {
    const num = parseFloat(dateStr)
    timestamp = num > 1e12 ? num : num * 1000
  } else {
    // ISO 字符串或其他格式
    const d = new Date(dateStr)
    return formatTimeFromDate(d)
  }
  return formatTimeFromDate(new Date(timestamp))
}

function formatTimeFromDate(d: Date): string {
  if (isNaN(d.getTime())) return '—'
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

/** 完整日期时间:YYYY-MM-DD HH:mm:ss */
export function formatDateTime(dateStr: string | number): string {
  let timestamp: number
  if (typeof dateStr === 'number') {
    timestamp = dateStr > 1e12 ? dateStr : dateStr * 1000
  } else if (/^\d+(\.\d+)?$/.test(dateStr)) {
    const num = parseFloat(dateStr)
    timestamp = num > 1e12 ? num : num * 1000
  } else {
    const d = new Date(dateStr)
    if (isNaN(d.getTime())) return '—'
    timestamp = d.getTime()
  }
  const d = new Date(timestamp)
  if (isNaN(d.getTime())) return '—'
  const pad = (n: number) => n.toString().padStart(2, '0')
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())} ${pad(d.getHours())}:${pad(d.getMinutes())}:${pad(d.getSeconds())}`
}

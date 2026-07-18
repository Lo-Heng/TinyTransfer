// Konami Code 彩蛋 — ↑↑↓↓←→←→BA 彩虹模式
// 原文件: 5119-5179 (konamiSequence/activateRainbowMode/deactivateRainbowMode)

import { ref, onUnmounted } from 'vue'

const KONAMI_CODE = [38, 38, 40, 40, 37, 39, 37, 39, 66, 65] // ↑↑↓↓←→←→BA

// 模块级单例状态
const isRainbow = ref(false)
let konamiSequence: number[] = []
let listenerRegistered = false
let deactivateTimer: ReturnType<typeof setTimeout> | null = null

function activateRainbowMode() {
  if (isRainbow.value) return
  isRainbow.value = true

  // 10 秒后自动关闭
  if (deactivateTimer) clearTimeout(deactivateTimer)
  deactivateTimer = setTimeout(() => {
    deactivateRainbowMode()
  }, 10000)
}

function deactivateRainbowMode() {
  isRainbow.value = false
  if (deactivateTimer) {
    clearTimeout(deactivateTimer)
    deactivateTimer = null
  }
}

function handleKeydown(e: KeyboardEvent) {
  konamiSequence.push(e.keyCode)
  konamiSequence = konamiSequence.slice(-10) // 只保留最后 10 个按键
  if (konamiSequence.join(',') === KONAMI_CODE.join(',')) {
    activateRainbowMode()
  }
}

export function useKonami() {
  if (!listenerRegistered) {
    listenerRegistered = true
    document.addEventListener('keydown', handleKeydown)
  }

  onUnmounted(() => {
    // 彩蛋是全局的，不随组件卸载移除
  })

  return { isRainbow, deactivateRainbowMode }
}

// 分步引导 — 蒙版式新手引导
// 原文件: 7501-7737 (GUIDE_STEPS/showGuideTour/closeGuideTour/renderGuideStep/keyboard)

import { reactive, computed } from 'vue'

export interface GuideStep {
  step: number
  title: string
  icon: string
  text: string
  target: string
  position: 'top' | 'bottom' | 'left' | 'right' | 'bottom-right'
  padding: number
}

export const GUIDE_STEPS: GuideStep[] = [
  {
    step: 0,
    title: '手机扫码连接',
    icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" style="width:22px;height:22px;flex-shrink:0;"><path d="M5 3L3 3L3 7"/><path d="M19 3L21 3L21 7"/><path d="M3 17L3 21L7 21"/><path d="M21 17L21 21L17 21"/><line x1="3" y1="12" x2="21" y2="12"/><circle cx="12" cy="12" r="1.5" fill="currentColor"/></svg>',
    text: '用手机相机扫描这个二维码，浏览器会自动打开。确保手机和电脑连接在同一个 WiFi 网络下。',
    target: '.qr-card',
    position: 'bottom',
    padding: 16,
  },
  {
    step: 1,
    title: '也可以复制地址',
    icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" style="width:22px;height:22px;flex-shrink:0;"><path d="M10 13a5 5 0 007.54.54l3-3a5 5 0 00-7.07-7.07l-1.72 1.71"/><path d="M14 11a5 5 0 00-7.54-.54l-3 3a5 5 0 007.07 7.07l1.71-1.71"/></svg>',
    text: '不方便扫码？点击地址栏自动复制链接，粘贴到手机浏览器同样可以连接。',
    target: '.url-row',
    position: 'bottom',
    padding: 16,
  },
  {
    step: 2,
    title: '拖拽传输文件',
    icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" style="width:22px;height:22px;flex-shrink:0;"><path d="M12 3v13"/><path d="M8 12l4 4 4-4"/><path d="M5 18h14v1a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2v-1z"/></svg>',
    text: '点击「上传」按钮或直接拖拽文件到文件列表，支持多文件批量传输，并行加速上传，速度提升 3-4 倍。',
    target: '#filesSection .files-header',
    position: 'bottom',
    padding: 20,
  },
]

// 模块级单例状态（reactive 便于组件直接访问属性）
const tourState = reactive({
  visible: false,
  currentStep: 0,
  totalSteps: GUIDE_STEPS.length,
  title: '',
  text: '',
})

const currentStep = computed(() => GUIDE_STEPS[tourState.currentStep])

let escListenerRegistered = false

function updateStepInfo() {
  const step = GUIDE_STEPS[tourState.currentStep]
  tourState.title = step?.title || ''
  tourState.text = step?.text || ''
}

function handleEsc(e: KeyboardEvent) {
  if (!tourState.visible) return
  if (e.key === 'Escape') {
    skipTour()
  } else if (e.key === 'Enter' || e.key === ' ') {
    e.preventDefault()
    nextStep()
  }
}

/** 显示引导（首次访问时调用） */
export function showGuideTour(): void {
  tourState.visible = true
  tourState.currentStep = 0
  updateStepInfo()

  if (!escListenerRegistered) {
    escListenerRegistered = true
    document.addEventListener('keydown', handleEsc)
  }
}

/** 下一步（最后一步时关闭） */
export function nextStep(): void {
  if (tourState.currentStep >= GUIDE_STEPS.length - 1) {
    skipTour()
  } else {
    tourState.currentStep++
    updateStepInfo()
  }
}

/** 上一步 */
export function prevStep(): void {
  if (tourState.currentStep > 0) {
    tourState.currentStep--
    updateStepInfo()
  }
}

/** 跳过/完成引导 */
export function skipTour(): void {
  tourState.visible = false
  localStorage.setItem('tiny-guided', '1')
}

export function useGuidedTour() {
  return {
    tourState,
    currentStep,
    GUIDE_STEPS,
    showGuideTour,
    nextStep,
    prevStep,
    skipTour,
  }
}

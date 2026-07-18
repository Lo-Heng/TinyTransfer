// 移动端下拉刷新 — touch 事件驱动
// 原文件: 6835-7011 (initPullToRefresh/handlePullStart/handlePullMove/handlePullEnd/updatePullIndicator/resetPullIndicator/triggerPullRefresh)

import { ref } from 'vue'
import { useFilesStore } from '@/stores/files'

export interface PullState {
  distance: number
  isPulling: boolean
  isRefreshing: boolean
  canRelease: boolean
  text: string
  showSuccess: boolean
}

const PULL_THRESHOLD = 60
const PULL_MAX_DISTANCE = 100

// 模块级单例状态
const pullState = ref<PullState>({
  distance: 0,
  isPulling: false,
  isRefreshing: false,
  canRelease: false,
  text: '下拉刷新',
  showSuccess: false,
})

let pullStartY = 0
let pullCurrentY = 0

function isMobile(): boolean {
  return (
    window.innerWidth <= 640 ||
    /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(
      navigator.userAgent,
    )
  )
}

function isPageAtTop(): boolean {
  return window.scrollY <= 0
}

function updateIndicator(distance: number) {
  pullState.value.distance = distance
  if (distance >= PULL_THRESHOLD) {
    pullState.value.canRelease = true
    pullState.value.text = '释放刷新'
  } else {
    pullState.value.canRelease = false
    pullState.value.text = '下拉刷新'
  }
}

function resetIndicator() {
  pullState.value.distance = 0
  pullState.value.canRelease = false
  pullState.value.isRefreshing = false
  pullState.value.showSuccess = false
  pullState.value.text = '下拉刷新'
}

async function triggerRefresh() {
  const files = useFilesStore()
  pullState.value.isRefreshing = true
  pullState.value.canRelease = false
  pullState.value.text = '刷新中...'

  try {
    await files.loadAllFiles()
    pullState.value.isRefreshing = false
    pullState.value.showSuccess = true
    pullState.value.text = '刷新成功'
    setTimeout(() => {
      resetIndicator()
    }, 800)
  } catch {
    resetIndicator()
  }
}

function handlePullStart(e: TouchEvent) {
  if (!isMobile() || pullState.value.isRefreshing || !isPageAtTop()) return
  const touch = e.touches[0]
  pullStartY = touch.clientY
  pullCurrentY = touch.clientY
  pullState.value.isPulling = true
}

function handlePullMove(e: TouchEvent) {
  if (!isMobile() || !pullState.value.isPulling || pullState.value.isRefreshing) return
  const touch = e.touches[0]
  pullCurrentY = touch.clientY
  const delta = pullCurrentY - pullStartY

  if (delta <= 0) {
    pullState.value.isPulling = false
    resetIndicator()
    return
  }

  e.preventDefault()
  let distance = delta * 0.5
  if (distance > PULL_MAX_DISTANCE) distance = PULL_MAX_DISTANCE
  updateIndicator(distance)
}

function handlePullEnd() {
  if (!isMobile() || !pullState.value.isPulling || pullState.value.isRefreshing) {
    pullState.value.isPulling = false
    return
  }
  pullState.value.isPulling = false
  const delta = pullCurrentY - pullStartY
  const distance = delta * 0.5
  if (distance >= PULL_THRESHOLD) {
    triggerRefresh()
  } else {
    resetIndicator()
  }
}

/** 初始化下拉刷新（绑定到指定元素） */
export function initPullToRefresh(el: HTMLElement): void {
  if (!isMobile() || !el) return
  el.addEventListener('touchstart', handlePullStart, { passive: false })
  el.addEventListener('touchmove', handlePullMove, { passive: false })
  el.addEventListener('touchend', handlePullEnd, { passive: false })
}

/** 销毁下拉刷新监听 */
export function destroyPullToRefresh(el: HTMLElement): void {
  if (!el) return
  el.removeEventListener('touchstart', handlePullStart)
  el.removeEventListener('touchmove', handlePullMove)
  el.removeEventListener('touchend', handlePullEnd)
}

export function usePullRefresh() {
  return {
    pullState,
    initPullToRefresh,
    destroyPullToRefresh,
  }
}

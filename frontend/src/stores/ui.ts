import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface ToastItem {
  id: number
  msg: string
  type: 'success' | 'error' | 'info'
  icon?: string
}

export const useUiStore = defineStore('ui', () => {
  const toasts = ref<ToastItem[]>([])
  const qsCompleted = ref<boolean[]>(
    JSON.parse(localStorage.getItem('tiny-qs') || '[false,false,false]')
  )
  const filterPopoverOpen = ref(false)
  const filesMoreOpen = ref(false)
  const dragOverlayActive = ref(false)
  const showGuideTour = ref(false)
  const showSuccessCelebration = ref(false)
  const successMessage = ref('')

  let toastId = 0

  function showToast(msg: string, type: ToastItem['type'] = 'info', icon?: string, duration = 3000) {
    const id = ++toastId
    toasts.value.push({ id, msg, type, icon })
    setTimeout(() => {
      removeToast(id)
    }, duration)
  }

  function removeToast(id: number) {
    toasts.value = toasts.value.filter((t) => t.id !== id)
  }

  function completeStep(idx: number) {
    if (qsCompleted.value[idx]) return
    qsCompleted.value[idx] = true
    localStorage.setItem('tiny-qs', JSON.stringify(qsCompleted.value))
  }

  function toggleFilterPopover() {
    filterPopoverOpen.value = !filterPopoverOpen.value
  }

  function closeFilterPopover() {
    filterPopoverOpen.value = false
  }

  function toggleFilesMore() {
    filesMoreOpen.value = !filesMoreOpen.value
  }

  function showSuccess(msg: string) {
    successMessage.value = msg
    showSuccessCelebration.value = true
    setTimeout(() => {
      showSuccessCelebration.value = false
    }, 2000)
  }

  return {
    toasts, qsCompleted, filterPopoverOpen, filesMoreOpen,
    dragOverlayActive, showGuideTour, showSuccessCelebration, successMessage,
    showToast, removeToast, completeStep,
    toggleFilterPopover, closeFilterPopover, toggleFilesMore, showSuccess,
  }
})

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, nextTick } from 'vue'
import { useGuidedTour } from '@/composables/useGuidedTour'

const { tourState, currentStep, nextStep, prevStep, skipTour } = useGuidedTour()

const spotlightStyle = ref<Record<string, string>>({})
const cardStyle = ref<Record<string, string>>({})
const cardArrowPosition = ref('')

function updatePosition() {
  if (!tourState.visible) return
  const step = currentStep.value
  if (!step || !step.target) return

  const target = document.querySelector(step.target) as HTMLElement
  if (!target) return

  const rect = target.getBoundingClientRect()
  const padding = step.padding ?? 12

  const spotlightX = rect.left - padding
  const spotlightY = rect.top - padding
  const spotlightW = rect.width + padding * 2
  const spotlightH = rect.height + padding * 2

  spotlightStyle.value = {
    left: spotlightX + 'px',
    top: spotlightY + 'px',
    width: spotlightW + 'px',
    height: spotlightH + 'px',
  }

  nextTick(() => {
    const card = document.querySelector('.guide-card') as HTMLElement
    if (!card) return
    const cardRect = card.getBoundingClientRect()
    const cardW = cardRect.width
    const cardH = cardRect.height

    const gap = 16
    let left = 0
    let top = 0
    let arrowPos = ''

    switch (step.position) {
      case 'bottom':
        left = rect.left + rect.width / 2 - cardW / 2
        top = rect.bottom + gap
        arrowPos = 'top'
        break
      case 'top':
        left = rect.left + rect.width / 2 - cardW / 2
        top = rect.top - cardH - gap
        arrowPos = 'bottom'
        break
      case 'left':
        left = rect.left - cardW - gap
        top = rect.top + rect.height / 2 - cardH / 2
        arrowPos = 'right'
        break
      case 'right':
        left = rect.right + gap
        top = rect.top + rect.height / 2 - cardH / 2
        arrowPos = 'left'
        break
      case 'bottom-right':
        left = rect.right - cardW
        top = rect.bottom + gap
        arrowPos = 'top'
        break
      default:
        left = rect.left + rect.width / 2 - cardW / 2
        top = rect.bottom + gap
        arrowPos = 'top'
    }

    const viewportW = window.innerWidth
    const viewportH = window.innerHeight
    const edgePadding = 16

    if (left < edgePadding) left = edgePadding
    if (left + cardW > viewportW - edgePadding) left = viewportW - cardW - edgePadding
    if (top < edgePadding) top = edgePadding
    if (top + cardH > viewportH - edgePadding) top = viewportH - cardH - edgePadding

    cardStyle.value = {
      left: left + 'px',
      top: top + 'px',
    }
    cardArrowPosition.value = arrowPos
  })
}

let resizeTimer: number | null = null
function handleResize() {
  if (resizeTimer) clearTimeout(resizeTimer)
  resizeTimer = window.setTimeout(() => {
    updatePosition()
  }, 100)
}

watch(
  () => [tourState.visible, tourState.currentStep],
  () => {
    if (tourState.visible) {
      nextTick(() => {
        updatePosition()
        setTimeout(updatePosition, 100)
      })
    }
  },
  { immediate: true }
)

onMounted(() => {
  window.addEventListener('resize', handleResize)
  window.addEventListener('scroll', handleResize, true)
})

onUnmounted(() => {
  window.removeEventListener('resize', handleResize)
  window.removeEventListener('scroll', handleResize, true)
  if (resizeTimer) clearTimeout(resizeTimer)
})
</script>

<template>
  <div v-if="tourState.visible" class="guide-overlay" :class="{ 'center-mode': !currentStep?.target }">
    <div
      v-if="currentStep?.target"
      class="guide-spotlight"
      :style="spotlightStyle"
    ></div>
    <div
      class="guide-card"
      :class="cardArrowPosition"
      :style="cardStyle"
    >
      <div v-if="cardArrowPosition" class="guide-arrow" :class="cardArrowPosition"></div>
      <div class="guide-card-progress">
        <span class="guide-step-indicator">
          {{ (tourState.currentStep ?? 0) + 1 }} / {{ tourState.totalSteps ?? 3 }}
        </span>
        <div class="guide-dots">
          <span
            v-for="i in (tourState.totalSteps ?? 3)"
            :key="i"
            class="guide-dot"
            :class="{ active: (tourState.currentStep ?? 0) === i - 1 }"
          ></span>
        </div>
      </div>
      <div class="guide-card-title">
        <span class="guide-card-icon" v-html="currentStep?.icon || ''"></span>
        <span>{{ tourState.title || '欢迎使用' }}</span>
      </div>
      <div class="guide-card-text">{{ tourState.text || '让我带你快速了解如何使用这个文件传输工具，只需几步即可开始。' }}</div>
      <div class="guide-card-actions">
        <button class="guide-btn-skip" @click="skipTour">跳过</button>
        <button v-if="(tourState.currentStep ?? 0) > 0" class="guide-btn-prev" @click="prevStep">← 上一步</button>
        <button class="guide-btn-next" @click="nextStep">
          {{ (tourState.currentStep ?? 0) + 1 >= (tourState.totalSteps ?? 3) ? '完成' : '下一步 →' }}
        </button>
      </div>
    </div>
  </div>
</template>

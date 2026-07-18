<script setup lang="ts">
import { useUiStore } from '@/stores/ui'

const ui = useUiStore()
</script>

<template>
  <Transition name="celebration">
    <div v-if="ui.showSuccessCelebration" class="success-celebration">
      <div class="success-confetti"></div>
      <div class="success-card">
        <div class="success-icon">✓</div>
        <div class="success-message">{{ ui.successMessage }}</div>
      </div>
    </div>
  </Transition>
</template>

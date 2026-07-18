<script setup lang="ts">
defineProps<{
  visible?: boolean
  loading?: boolean
}>()
</script>

<template>
  <div class="pull-refresh-indicator" :class="{ visible: visible }">
    <div class="pull-refresh-icon">
      <svg class="pull-arrow" viewBox="0 0 24 24" style="width:20px;height:20px;">
        <polyline points="6 9 12 15 18 9" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
      <svg
        v-if="loading"
        class="pull-spinner"
        viewBox="0 0 24 24"
        style="width:20px;height:20px;"
      >
        <circle cx="12" cy="12" r="10" fill="none" stroke="currentColor" stroke-width="1.5" stroke-opacity="0.25"/>
        <path d="M12 2a10 10 0 0 1 10 10" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
      </svg>
    </div>
    <div class="pull-refresh-text">{{ loading ? '正在刷新...' : '下拉刷新' }}</div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  size?: number
  class?: string
}>()
</script>

<template>
  <svg
    viewBox="0 0 24 24"
    fill="currentColor"
    :width="size || 24"
    :height="size || 24"
    :class="$props.class"
  >
    <path d="M13 2L3 14h7l-1 8 10-12h-7l1-8z"/>
  </svg>
</template>
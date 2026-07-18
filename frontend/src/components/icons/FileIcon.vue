<script setup lang="ts">
import { useFilesStore } from '@/stores/files'

const files = useFilesStore()
defineProps<{ filename: string }>()
</script>

<template>
  <span class="file-icon" v-html="files.getFileIconSVG(filename)"></span>
</template>

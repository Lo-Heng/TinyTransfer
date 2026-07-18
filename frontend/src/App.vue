<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { useThemeStore } from '@/stores/theme'
import { useFilesStore } from '@/stores/files'
import { useDevicesStore } from '@/stores/devices'
import { useUiStore } from '@/stores/ui'
import { useSSE } from '@/composables/useSSE'
import { useGuidedTour } from '@/composables/useGuidedTour'
import { initDragDrop } from '@/composables/useDragDrop'

// 布局
import TopBar from '@/components/layout/TopBar.vue'
import ConnectionBanner from '@/components/layout/ConnectionBanner.vue'
import DeviceFlyout from '@/components/layout/DeviceFlyout.vue'
import Hero from '@/components/layout/Hero.vue'
import GuestWelcome from '@/components/layout/GuestWelcome.vue'
import GlobalDragOverlay from '@/components/layout/GlobalDragOverlay.vue'
// 文件
import FileSection from '@/components/files/FileSection.vue'
// 模态框
import UploadModal from '@/components/modals/UploadModal.vue'
import PasswordModal from '@/components/modals/PasswordModal.vue'
import SettingsModal from '@/components/modals/SettingsModal.vue'
import FilePreviewModal from '@/components/modals/FilePreviewModal.vue'
import ImagePreview from '@/components/modals/ImagePreview.vue'
import ConfirmDialog from '@/components/modals/ConfirmDialog.vue'
// UI
import Toast from '@/components/ui/Toast.vue'
import ContextMenu from '@/components/ui/ContextMenu.vue'
import BottomSheet from '@/components/ui/BottomSheet.vue'
import SuccessCelebration from '@/components/ui/SuccessCelebration.vue'
import GuidedTour from '@/components/ui/GuidedTour.vue'
import FileProperties from '@/components/ui/FileProperties.vue'

const auth = useAuthStore()
const theme = useThemeStore()
const files = useFilesStore()
const devices = useDevicesStore()
const ui = useUiStore()
const { connectSSE } = useSSE()
const { showGuideTour } = useGuidedTour()

// 模板引用：用于通过 defineExpose 暴露的 open/show 方法控制本地状态模态框
const passwordModalRef = ref<InstanceType<typeof PasswordModal> | null>(null)
const settingsModalRef = ref<InstanceType<typeof SettingsModal> | null>(null)
const filePropertiesRef = ref<InstanceType<typeof FileProperties> | null>(null)
const confirmDialogRef = ref<InstanceType<typeof ConfirmDialog> | null>(null)
const bottomSheetRef = ref<InstanceType<typeof BottomSheet> | null>(null)

function openSettings() {
  settingsModalRef.value?.show()
}
function showGuide() {
  showGuideTour()
}
function showProperties(filename: string) {
  filePropertiesRef.value?.show(filename)
}
function onLongPress(filename: string) {
  bottomSheetRef.value?.show(filename)
}

// 删除单个文件（来自右键菜单 / 底部面板）
function confirmDelete(filename: string) {
  confirmDialogRef.value?.show({
    title: '确认删除',
    message: `确定要删除「${filename}」吗？此操作不可撤销。`,
    confirmLabel: '删除',
    onConfirm: async () => {
      try {
        await files.deleteFiles([filename])
        ui.showToast('已删除', 'success')
      } catch (e) {
        ui.showToast('删除失败', 'error')
      }
    },
  })
}

// 批量删除（来自多选栏）
function deleteSelected() {
  const names = Array.from(files.selectedFileNames)
  if (names.length === 0) return
  confirmDialogRef.value?.show({
    title: '确认删除',
    message: `确定要删除选中的 ${names.length} 个文件吗？此操作不可撤销。`,
    confirmLabel: '删除',
    onConfirm: async () => {
      try {
        await files.deleteFiles(names)
        if (files.batchMode) files.toggleBatchMode()
        ui.showToast('已删除', 'success')
      } catch (e) {
        ui.showToast('删除失败', 'error')
      }
    },
  })
}

onMounted(async () => {
  theme.initTheme()
  await auth.checkAuth()
  document.body.classList.add('role-' + auth.userRole)

  await files.loadAllFiles()
  // 初始拉取设备列表
  devices.loadDevices()
  // 启动 SSE
  connectSSE()

  ui.completeStep(0)

  // 初始化全局拖拽上传（仅注册一次）
  initDragDrop()

  // 首次访问引导（仅 host）
  if (auth.userRole === 'host' && !localStorage.getItem('tiny-guided')) {
    setTimeout(() => showGuideTour(), 600)
  }

  // 需要密码但未认证
  if (auth.hasPassword && !auth.isAuthenticated) {
    passwordModalRef.value?.show()
  }
})
</script>

<template>
  <!-- Toast -->
  <Toast />

  <!-- TopBar -->
  <TopBar @open-settings="openSettings" />

  <!-- ConnectionBanner -->
  <ConnectionBanner />

  <!-- DeviceFlyout -->
  <DeviceFlyout />

  <!-- Main page -->
  <div class="page" id="mainPage">
    <!-- HOST VIEW -->
    <Hero />

    <!-- GUEST VIEW -->
    <GuestWelcome />

    <!-- FILES SECTION (both roles) -->
    <FileSection
      @long-press="onLongPress"
      @delete-selected="deleteSelected"
    />

    <div class="page-footer">
      <span class="page-footer-item">v0.1.0</span>
      <span class="page-footer-dot">·</span>
      <a href="https://github.com/Lo-Heng/TinyTransfer" target="_blank" class="page-footer-icon-link" title="GitHub">
        <svg viewBox="0 0 24 24" width="14" height="14" fill="currentColor"><path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/></svg>
      </a>
      <span class="page-footer-dot">·</span>
      <span class="page-footer-item">PolyForm Noncommercial</span>
      <span class="page-footer-dot">·</span>
      <span class="page-footer-item">© 2026 Lo-Heng</span>
    </div>
  </div>

  <!-- Modals -->
  <UploadModal />
  <PasswordModal ref="passwordModalRef" />
  <SettingsModal
    ref="settingsModalRef"
    @show-guide="showGuide"
  />
  <FilePreviewModal />
  <ImagePreview />
  <ConfirmDialog ref="confirmDialogRef" />

  <!-- UI -->
  <ContextMenu
    @show-properties="showProperties"
    @confirm-delete="confirmDelete"
  />
  <BottomSheet
    ref="bottomSheetRef"
    @confirm-delete="confirmDelete"
  />
  <SuccessCelebration />
  <GuidedTour />
  <FileProperties ref="filePropertiesRef" />
  <GlobalDragOverlay />
</template>

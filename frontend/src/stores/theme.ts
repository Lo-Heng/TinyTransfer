import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { setTitlebarColor as apiSetTitlebarColor } from '@/api/system'

export type ThemeMode = 'light' | 'dark' | 'auto'

export const useThemeStore = defineStore('theme', () => {
  const theme = ref<ThemeMode>(
    (localStorage.getItem('tiny-theme') as ThemeMode) || 'auto'
  )
  const isDark = computed(() => {
    const prefers = window.matchMedia('(prefers-color-scheme: dark)').matches
    return theme.value === 'dark' || (theme.value === 'auto' && prefers)
  })

  function applyTheme() {
    if (isDark.value) {
      document.documentElement.classList.add('dark')
    } else {
      document.documentElement.classList.remove('dark')
    }
    // 更新 theme-color meta
    const meta = document.getElementById('meta-theme-color') as HTMLMetaElement | null
    if (meta) {
      meta.content = isDark.value ? '#0F0F0F' : '#FBFBFC'
    }
    // 同步 Tauri 标题栏颜色
    apiSetTitlebarColor(isDark.value).catch(() => {})
  }

  function setTheme(newTheme: ThemeMode) {
    theme.value = newTheme
    localStorage.setItem('tiny-theme', newTheme)
    applyTheme()
  }

  function initTheme() {
    applyTheme()
    // 监听系统主题变化（auto 模式）
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
      if (theme.value === 'auto') applyTheme()
    })
  }

  return { theme, isDark, setTheme, initTheme, applyTheme }
})

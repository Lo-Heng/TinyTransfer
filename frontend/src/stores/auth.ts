import { defineStore } from 'pinia'
import { ref } from 'vue'
import * as authApi from '@/api/auth'
import { setAuthToken } from '@/api/client'

export const useAuthStore = defineStore('auth', () => {
  const isAuthenticated = ref(false)
  const hasPassword = ref(false)
  const userRole = ref<'host' | 'guest'>('guest')

  async function checkAuth() {
    try {
      const data = await authApi.checkAuth()
      isAuthenticated.value = data.authenticated
      hasPassword.value = data.has_password
      userRole.value = data.role as 'host' | 'guest'
    } catch (e: any) {
      if (e?.status === 401) {
        isAuthenticated.value = false
      }
    }
  }

  async function login(password: string): Promise<boolean> {
    const data = await authApi.login(password)
    if (data.success && data.token) {
      isAuthenticated.value = true
      setAuthToken(data.token)
      return true
    }
    return false
  }

  function reset() {
    isAuthenticated.value = false
    setAuthToken(null)
  }

  return { isAuthenticated, hasPassword, userRole, checkAuth, login, reset }
})

import { apiFetch } from './client'

export interface CheckAuthResponse {
  authenticated: boolean
  has_password: boolean
  role: string
}

export interface AuthResponse {
  success: boolean
  token?: string
  error?: string
}

export function checkAuth(): Promise<CheckAuthResponse> {
  return apiFetch<CheckAuthResponse>('/api/check-auth')
}

export function login(password: string): Promise<AuthResponse> {
  return apiFetch<AuthResponse>('/api/auth', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ password }),
  })
}

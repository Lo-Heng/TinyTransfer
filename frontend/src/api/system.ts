import { apiFetch } from './client'

export interface IpResponse {
  ip: string
  url: string
}

export interface DiskInfoResponse {
  free: number
  total: number
  used: number
  error?: string
}

export function getIP(): Promise<IpResponse> {
  return apiFetch<IpResponse>('/api/ip')
}

export function getDiskInfo(): Promise<DiskInfoResponse> {
  return apiFetch<DiskInfoResponse>('/api/disk-info')
}

export function setTitlebarColor(isDark: boolean): Promise<{ success: boolean }> {
  return apiFetch('/api/set-titlebar-color', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ is_dark: isDark }),
  })
}

export function openFolder(folderType: string): Promise<{ success: boolean }> {
  return apiFetch(`/api/open-folder/${folderType}`)
}

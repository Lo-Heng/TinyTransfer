import { apiFetch } from './client'

export interface DeviceSnapshot {
  ip: string
  type: string
  model: string
  detail: string
}

export interface DevicesResponse {
  connected: boolean
  devices: DeviceSnapshot[]
}

export function getDevices(): Promise<DevicesResponse> {
  return apiFetch<DevicesResponse>('/api/devices')
}

export function ping(clientId: string): Promise<void> {
  return apiFetch<void>('/api/ping', {
    method: 'POST',
    headers: { 'X-Client-Id': clientId },
    // keepalive: true 在 fetch 中支持
    keepalive: true,
  } as RequestInit)
}

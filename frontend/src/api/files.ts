import { apiFetch } from './client'

export interface FileInfo {
  name: string
  size: number
  modified: string
  is_dir?: boolean
  source?: string
}

export function getAllFiles(): Promise<FileInfo[]> {
  return apiFetch<FileInfo[]>('/api/all-files')
}

export function getSharedFiles(): Promise<FileInfo[]> {
  return apiFetch<FileInfo[]>('/api/files')
}

export function getUploadedFiles(): Promise<FileInfo[]> {
  return apiFetch<FileInfo[]>('/api/uploaded-files')
}

export function deleteFiles(filenames: string[]): Promise<{ success: boolean; deleted: number }> {
  return apiFetch('/api/delete-files', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ filenames }),
  })
}

export function downloadZipUrl(filenames: string[]): string {
  // ZIP 下载需用 POST + body，返回 blob，直接用 fetch
  return '/api/download-zip'
}

export async function downloadZipBlob(filenames: string[]): Promise<Blob> {
  const res = await fetch('/api/download-zip', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ filenames }),
  })
  if (!res.ok) throw new Error(`下载失败: ${res.status}`)
  return res.blob()
}

export function getDownloadUrl(filename: string): string {
  return `/api/download/${encodeURIComponent(filename)}`
}

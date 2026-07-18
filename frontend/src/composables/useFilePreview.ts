// 文件预览 — 图片/视频/音频/PDF/文本
// 原文件: 6348-6479 (openPreviewModal/closePreviewModal/openVideoPlayer/showVideoFallback/isImage/isVideo)

import { reactive } from 'vue'
import { getDownloadUrl } from '@/api/files'
import { apiFetch } from '@/api/client'
import { openFileWithPlayer } from './useTauri'

export type PreviewType = 'image' | 'video' | 'audio' | 'pdf' | 'text' | 'unsupported'

export interface PreviewState {
  visible: boolean
  filename: string
  type: PreviewType
  url: string
  videoUnsupported: boolean
  videoMime: string
  text: string
  textError: boolean
}

// 模块级单例状态（reactive 便于组件直接访问属性）
const previewState = reactive<PreviewState>({
  visible: false,
  filename: '',
  type: 'unsupported',
  url: '',
  videoUnsupported: false,
  videoMime: 'video/mp4',
  text: '',
  textError: false,
})

const IMAGE_EXTS = ['jpg', 'jpeg', 'png', 'gif', 'webp', 'heic', 'bmp']
const VIDEO_EXTS = ['mp4', 'webm', 'ogg', 'mov', 'avi', 'mkv', 'm4v', 'hevc']
const UNSUPPORTED_VIDEO_EXTS = ['mov', 'mkv', 'avi', 'hevc', 'flv', 'wmv', 'rmvb', 'ts']
const AUDIO_EXTS = ['mp3', 'wav', 'flac', 'aac', 'm4a']
const TEXT_EXTS = ['txt', 'md', 'json', 'xml', 'html', 'css', 'js']

const VIDEO_MIME_MAP: Record<string, string> = {
  mp4: 'video/mp4',
  webm: 'video/webm',
  ogg: 'video/ogg',
  m4v: 'video/x-m4v',
}

function isImage(ext: string): boolean {
  return IMAGE_EXTS.includes(ext)
}
function isVideo(ext: string): boolean {
  return VIDEO_EXTS.includes(ext)
}

/** 打开文件预览 */
export async function openPreview(filename: string): Promise<void> {
  const ext = filename.split('.').pop()?.toLowerCase() || ''
  const url = getDownloadUrl(filename)

  previewState.visible = true
  previewState.filename = filename
  previewState.url = url
  previewState.videoUnsupported = false
  previewState.videoMime = VIDEO_MIME_MAP[ext] || 'video/mp4'
  previewState.text = ''
  previewState.textError = false
  previewState.type = 'unsupported'

  if (isImage(ext)) {
    previewState.type = 'image'
  } else if (isVideo(ext)) {
    previewState.type = 'video'
    if (UNSUPPORTED_VIDEO_EXTS.includes(ext)) {
      previewState.videoUnsupported = true
    }
  } else if (AUDIO_EXTS.includes(ext)) {
    previewState.type = 'audio'
  } else if (ext === 'pdf') {
    previewState.type = 'pdf'
  } else if (TEXT_EXTS.includes(ext)) {
    previewState.type = 'text'
    // 异步加载文本内容
    try {
      const text = await apiFetch<string>(getDownloadUrl(filename))
      previewState.text = text.substring(0, 100000)
    } catch {
      previewState.textError = true
    }
  } else {
    previewState.type = 'unsupported'
  }
}

/** 关闭文件预览 */
export function closePreview(): void {
  previewState.visible = false
  // 延迟清理（等过渡动画结束）
  setTimeout(() => {
    previewState.filename = ''
    previewState.text = ''
    previewState.type = 'unsupported'
  }, 260)
}

/** 用系统播放器打开当前预览的视频（Tauri 环境） */
export async function openVideoPlayer(): Promise<void> {
  if (!previewState.filename) return
  await openFileWithPlayer(previewState.filename)
}

export function useFilePreview() {
  return {
    previewState,
    openPreview,
    closePreview,
    openVideoPlayer,
  }
}

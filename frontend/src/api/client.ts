// API 客户端封装：统一 fetch、token 注入、错误处理

let authToken: string | null = null

export function setAuthToken(token: string | null) {
  authToken = token
}

export function getAuthToken(): string | null {
  return authToken
}

export interface ApiError {
  status: number
  message: string
}

export async function apiFetch<T = any>(
  url: string,
  options: RequestInit = {}
): Promise<T> {
  const headers = new Headers(options.headers)
  if (authToken) {
    headers.set('X-Auth-Token', authToken)
  }
  if (options.body && !headers.has('Content-Type') && !(options.body instanceof FormData)) {
    headers.set('Content-Type', 'application/json')
  }
  const res = await fetch(url, { ...options, headers })
  if (res.status === 401) {
    // 认证失败，清除 token
    authToken = null
    throw { status: 401, message: '未授权' } as ApiError
  }
  if (!res.ok && res.status !== 401) {
    throw { status: res.status, message: `请求失败: ${res.status}` } as ApiError
  }
  const contentType = res.headers.get('content-type') || ''
  if (contentType.includes('application/json')) {
    return res.json() as Promise<T>
  }
  return res.text() as unknown as Promise<T>
}

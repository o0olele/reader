export interface IpcErrorPayload {
  kind?: string
  message?: string
}

export function getErrorMessage(cause: unknown, fallback = '操作失败'): string {
  if (cause instanceof Error && cause.message) return cause.message
  if (typeof cause === 'string' && cause.trim()) return cause
  if (cause && typeof cause === 'object') {
    const payload = cause as IpcErrorPayload
    if (payload.message) return payload.message
  }
  return fallback
}

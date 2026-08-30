import { emit as tauriEmit, listen } from '@tauri-apps/api/event'

export type AppEvent =
  | 'book-updated'
  | 'chapter-updated'
  | 'download-progress'
  | 'download-completed'
  | 'download-failed'
  | 'source-test-progress'

type Handler = (payload?: unknown) => void
const handlers = new Map<AppEvent, Set<Handler>>()
const tauriSubscriptions = new Set<AppEvent>()

function isTauri(): boolean {
  return (
    typeof window !== 'undefined' && Boolean((window as Window & { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__)
  )
}

function ensureTauriSubscription(event: AppEvent): void {
  if (!isTauri() || tauriSubscriptions.has(event)) return
  tauriSubscriptions.add(event)
  listen(event, (message) => {
    handlers.get(event)?.forEach((handler) => handler(message.payload))
  }).catch(() => {
    tauriSubscriptions.delete(event)
  })
}

export function on(event: AppEvent, handler: Handler): () => void {
  const eventHandlers = handlers.get(event) ?? new Set<Handler>()
  eventHandlers.add(handler)
  handlers.set(event, eventHandlers)
  ensureTauriSubscription(event)
  return () => eventHandlers.delete(handler)
}

export function emit(event: AppEvent, payload?: unknown): void {
  handlers.get(event)?.forEach((handler) => handler(payload))
  if (isTauri()) void tauriEmit(event, payload).catch(() => undefined)
}

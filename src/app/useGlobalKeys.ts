import { onBeforeUnmount, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { shortcutRoutes } from './shellNav'
import { useCommandPalette } from './useCommandPalette'
import { useTheme } from './useTheme'

function isTypingTarget(target: EventTarget | null): boolean {
  const element = target as HTMLElement | null
  if (!element) return false
  const tag = element.tagName?.toLowerCase() ?? ''
  return tag === 'input' || tag === 'textarea' || tag === 'select' || element.isContentEditable === true
}

/**
 * Prototype keyboard layer (`desktop-ui.html:2757–2776`):
 * `Esc` · `Ctrl+K` · `D` · `1–5`. `Ctrl+B` lives in `useRail`; the reader keys
 * (`→`/`Space`/`←`/`T`/`F`) are owned by the reader page.
 */
export function useGlobalKeys() {
  const router = useRouter()
  const { open, openPalette, closePalette } = useCommandPalette()
  const { toggleTheme } = useTheme()

  function onKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      closePalette()
      return
    }
    if (event.ctrlKey || event.metaKey) {
      if (event.key === 'k' || event.key === 'K') {
        event.preventDefault()
        openPalette()
      }
      return
    }
    if (isTypingTarget(event.target)) return
    if (event.key === 'd' || event.key === 'D') {
      toggleTheme()
      return
    }
    const name = shortcutRoutes[event.key]
    if (name && !open.value) void router.push({ name })
  }

  onMounted(() => window.addEventListener('keydown', onKeydown))
  onBeforeUnmount(() => window.removeEventListener('keydown', onKeydown))
}

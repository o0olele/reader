import { onBeforeUnmount, onMounted, ref } from 'vue'

const STORAGE_KEY = 'rail-extended'
const extended = ref(read())

function read(): boolean {
  try {
    return localStorage.getItem(STORAGE_KEY) !== '0'
  } catch {
    return true
  }
}

function persist(value: boolean) {
  try {
    localStorage.setItem(STORAGE_KEY, value ? '1' : '0')
  } catch {
    /* browser preview without storage */
  }
}

/**
 * Rail collapse state — the prototype's `Ctrl+B` toggle
 * (`desktop-ui.html:2763`) with its two fixed widths (56px / 220px).
 * Module-level state so the titlebar and the rail stay in sync.
 */
export function useRail() {
  const toggle = () => {
    extended.value = !extended.value
    persist(extended.value)
  }
  const onKeydown = (event: KeyboardEvent) => {
    if ((event.ctrlKey || event.metaKey) && (event.key === 'b' || event.key === 'B')) {
      event.preventDefault()
      toggle()
    }
  }
  onMounted(() => window.addEventListener('keydown', onKeydown))
  onBeforeUnmount(() => window.removeEventListener('keydown', onKeydown))
  return { extended, toggle }
}

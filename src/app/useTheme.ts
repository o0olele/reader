import { computed, readonly, ref } from 'vue'

export type AppTheme = 'light' | 'dark'

const STORAGE_KEY = 'app-theme'
const theme = ref<AppTheme>(read())

function read(): AppTheme {
  try {
    return localStorage.getItem(STORAGE_KEY) === 'dark' ? 'dark' : 'light'
  } catch {
    return 'light'
  }
}

/** Writes the theme to `<html>` so both the `.dark` variant used by the
 *  shadcn components and the `data-theme` hook used by reader themes agree. */
function apply(next: AppTheme) {
  theme.value = next
  const root = document.documentElement
  root.classList.toggle('dark', next === 'dark')
  root.dataset.theme = next
  try {
    localStorage.setItem(STORAGE_KEY, next)
  } catch {
    /* browser preview without storage */
  }
}

let initialized = false

export function useTheme() {
  if (!initialized) {
    initialized = true
    apply(theme.value)
  }
  return {
    theme: readonly(theme),
    isDark: computed(() => theme.value === 'dark'),
    setTheme: apply,
    toggleTheme: () => apply(theme.value === 'dark' ? 'light' : 'dark'),
  }
}

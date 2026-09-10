import { ref } from 'vue'

const open = ref(false)

/** Single `Ctrl+K` command palette shared by the rail, the titlebar and the
 *  keyboard layer (prototype `desktop-ui.html:2764`). */
export function useCommandPalette() {
  return {
    open,
    openPalette: () => {
      open.value = true
    },
    closePalette: () => {
      open.value = false
    },
  }
}

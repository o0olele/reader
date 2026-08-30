import { reactive, ref } from 'vue'
import { getAppSettings, saveAppSettings } from '../../services/api'

/** Owns application settings. Currently just the global proxy. */
export function useSettings(report: (cause: unknown) => void, notify: (message: string) => void) {
  const proxyUrl = ref('')
  const saving = ref(false)

  async function refresh() {
    try {
      proxyUrl.value = (await getAppSettings()).proxy_url ?? ''
    } catch {
      /* preview mode */
    }
  }

  async function save() {
    saving.value = true
    try {
      proxyUrl.value = (await saveAppSettings(proxyUrl.value)).proxy_url ?? ''
      notify('设置已保存')
    } catch (cause) {
      report(cause)
    } finally {
      saving.value = false
    }
  }

  return reactive({ proxyUrl, saving, refresh, save, clear: () => (proxyUrl.value = '') })
}

import { reactive, ref } from 'vue'
import { getAppSettings, saveAppSettings } from '../../services/api'

/** Owns application settings: the global proxy and the User-Agent override. */
export function useSettings(report: (cause: unknown) => void, notify: (message: string) => void) {
  const proxyUrl = ref('')
  /** Blank means "track the webview" — see {@link effectiveUserAgent}. */
  const userAgent = ref('')
  const effectiveUserAgent = ref('')
  const saving = ref(false)

  function apply(settings: Awaited<ReturnType<typeof getAppSettings>>) {
    proxyUrl.value = settings.proxy_url ?? ''
    userAgent.value = settings.user_agent ?? ''
    effectiveUserAgent.value = settings.effective_user_agent ?? ''
  }

  async function refresh() {
    try {
      apply(await getAppSettings())
    } catch {
      /* preview mode */
    }
  }

  async function save() {
    saving.value = true
    try {
      apply(await saveAppSettings(proxyUrl.value, userAgent.value))
      notify('设置已保存')
    } catch (cause) {
      report(cause)
    } finally {
      saving.value = false
    }
  }

  return reactive({
    proxyUrl,
    userAgent,
    effectiveUserAgent,
    saving,
    refresh,
    save,
    clear: () => (proxyUrl.value = ''),
  })
}

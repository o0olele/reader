import { reactive, ref } from 'vue'
import { exportBackup, getAppSettings, restoreBackup, saveAppSettings } from '../../services/api'
import { open, save as saveDialog } from '@tauri-apps/plugin-dialog'

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

  async function backup() {
    const path = await saveDialog({
      defaultPath: 'reader-desktop-backup.json',
      filters: [{ name: 'JSON backup', extensions: ['json'] }],
    })
    if (!path) return
    try {
      const result = await exportBackup(path)
      notify(`备份已保存（${result.rows} 条记录）`)
    } catch (cause) {
      report(cause)
    }
  }

  async function restore() {
    const path = await open({
      multiple: false,
      directory: false,
      filters: [{ name: 'JSON backup', extensions: ['json'] }],
    })
    if (!path || Array.isArray(path)) return
    if (!window.confirm('恢复备份会覆盖当前本地数据，确定继续吗？')) return
    try {
      const result = await restoreBackup(path)
      notify(`备份已恢复（${result.rows} 条记录），正在刷新…`)
      window.setTimeout(() => window.location.reload(), 250)
    } catch (cause) {
      report(cause)
    }
  }

  return reactive({
    proxyUrl,
    userAgent,
    effectiveUserAgent,
    saving,
    refresh,
    save,
    backup,
    restore,
    clear: () => (proxyUrl.value = ''),
  })
}

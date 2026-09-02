/* global HTMLInputElement, Event */
import { reactive, ref } from 'vue'
import {
  clearBookSourceSession,
  importBookSourcesJson,
  importBookSourcesUrl,
  listBookSources,
  loginBookSource,
  openBookSourceBrowser,
  saveBookSourceBrowserSession,
  refreshBookSourceSession,
  saveBookSource,
  testBookSource,
  type BookSource,
  type SourceImportReport,
} from '../../services/api'

export type SourceForm = Record<string, string>

function emptyForm(): SourceForm {
  return {
    name: '',
    base_url: '',
    search_url: '',
    item: '',
    title: '',
    author: '',
    url: '',
    login_url: '',
    login_method: 'POST',
    login_body: '',
    token_path: '',
    sign_script: '',
    proxy_url: '',
    next_toc_url_selector: '',
    next_content_url_selector: '',
  }
}

function describeReport(report: SourceImportReport): string {
  const parts = [`已导入 ${report.imported} 个书源`]
  if (report.failed.length) parts.push(`失败 ${report.failed.length} 个`)
  if (report.partial.length) {
    parts.push(`${report.partial.length} 个含 CSS 引擎暂不支持的规则（XPath / JSONPath / JS）`)
  }
  return parts.join('，')
}

/** Owns book source management: CRUD, legado import, login and connectivity tests. */
export function useSources(report: (cause: unknown) => void, notify: (message: string) => void) {
  const sources = ref<BookSource[]>([])
  const form = ref<SourceForm>(emptyForm())
  const saving = ref(false)
  const sourceUrl = ref('')
  const importing = ref(false)
  const testing = ref<number>()
  const loginForm = ref({ sourceId: 0, username: '', password: '' })
  const loggingIn = ref(false)
  const lastProbe = ref<{
    source_name: string
    status: number
    result_count: number
    auth_required: boolean
    session_state: string
    request_url: string
  }>()

  async function refresh() {
    try {
      sources.value = await listBookSources()
    } catch {
      /* preview mode */
    }
  }

  function updateForm(key: string, value: string) {
    form.value[key] = value
  }

  async function save() {
    const current = form.value
    const required = [current.name, current.base_url, current.search_url, current.item, current.title, current.url]
    if (required.some((field) => !field.trim())) {
      notify('请完整填写书源名称、URL 和必需选择器')
      return
    }
    saving.value = true
    try {
      await saveBookSource({
        name: current.name,
        base_url: current.base_url,
        search_url: current.search_url,
        search_rule: {
          item: current.item,
          title: current.title,
          author: current.author || undefined,
          url: current.url,
        },
        login_url: current.login_url || undefined,
        login_method: current.login_method,
        login_body: current.login_body || undefined,
        token_path: current.token_path || undefined,
        sign_script: current.sign_script || undefined,
        proxy_url: current.proxy_url || undefined,
        next_toc_url_selector: current.next_toc_url_selector || undefined,
        next_content_url_selector: current.next_content_url_selector || undefined,
        enabled: true,
      })
      form.value = emptyForm()
      await refresh()
      notify('书源已保存')
    } catch (cause) {
      report(cause)
    } finally {
      saving.value = false
    }
  }

  async function importFromFile(event: Event) {
    const input = event.target as HTMLInputElement
    const file = input.files?.[0]
    if (!file) return
    importing.value = true
    try {
      notify(describeReport(await importBookSourcesJson(await file.text())))
      await refresh()
    } catch (cause) {
      report(cause)
    } finally {
      importing.value = false
      input.value = ''
    }
  }

  async function importFromUrl() {
    if (!sourceUrl.value.trim()) return
    importing.value = true
    try {
      notify(describeReport(await importBookSourcesUrl(sourceUrl.value)))
      sourceUrl.value = ''
      await refresh()
    } catch (cause) {
      report(cause)
    } finally {
      importing.value = false
    }
  }

  async function test(source: BookSource, query: string) {
    testing.value = source.id
    try {
      const result = await testBookSource(source.id, query || '测试')
      lastProbe.value = result
      const status = `HTTP ${result.status}`
      notify(
        result.auth_required
          ? `${result.source_name} 返回 ${status}，会话已标记为过期，请刷新登录`
          : `${result.source_name} 返回 ${status}，解析到 ${result.result_count} 条结果`,
      )
      await refresh()
    } catch (cause) {
      report(cause)
    } finally {
      testing.value = undefined
    }
  }

  async function login() {
    const { sourceId, username, password } = loginForm.value
    if (!sourceId || !username || !password) return
    loggingIn.value = true
    try {
      const result = await loginBookSource(sourceId, username, password)
      notify(result.authenticated ? '登录成功，会话已保存' : '登录响应中没有 Token 或 Cookie')
      await refresh()
    } catch (cause) {
      report(cause)
    } finally {
      loggingIn.value = false
    }
  }

  async function clearSession(source: BookSource) {
    try {
      await clearBookSourceSession(source.id)
      notify(`已清除 ${source.name} 的会话`)
      await refresh()
    } catch (cause) {
      report(cause)
    }
  }

  async function refreshSession() {
    const { sourceId, username, password } = loginForm.value
    if (!sourceId || !username || !password) return
    loggingIn.value = true
    try {
      const result = await refreshBookSourceSession(sourceId, username, password)
      notify(result.authenticated ? '会话已刷新' : '刷新响应中没有 Token 或 Cookie')
      await refresh()
    } catch (cause) {
      report(cause)
    } finally {
      loggingIn.value = false
    }
  }

  async function browserAuth(source: BookSource) {
    try {
      await openBookSourceBrowser(source.id)
      notify(`已打开 ${source.name} 的浏览器认证窗口，请完成页面验证后点击“读取浏览器会话”`)
    } catch (cause) {
      report(cause)
    }
  }

  async function saveBrowserSession(source: BookSource) {
    try {
      const result = await saveBookSourceBrowserSession(source.id)
      notify(result.authenticated ? `${source.name} 的浏览器会话已保存` : '浏览器中没有可保存的会话')
      await refresh()
    } catch (cause) {
      report(cause)
    }
  }

  return reactive({
    sources,
    form,
    saving,
    sourceUrl,
    importing,
    testing,
    loginForm,
    loggingIn,
    lastProbe,
    refresh,
    updateForm,
    save,
    importFromFile,
    importFromUrl,
    test,
    login,
    refreshSession,
    browserAuth,
    saveBrowserSession,
    clearSession,
  })
}

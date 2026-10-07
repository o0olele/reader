import { reactive, ref } from 'vue'
import {
  clearBookSourceSession,
  getErrorMessage,
  exportBookSources,
  importBookSourcesJson,
  importBookSourcesUrl,
  listBookSources,
  loginBookSource,
  openBookSourceBrowser,
  saveBookSourceBrowserSession,
  refreshBookSourceSession,
  saveBookSource,
  setBookSourceEnabled,
  testBookSource,
  updateBookSourceManagement,
  validateAllSources,
  type BookSource,
  type SourceImportReport,
  type SourceTestResult,
} from '../../services/api'
import { probeNeedsAttention } from './sourceView'

export type SourceForm = Record<string, string>

/** Keep management drafts outside the windowed rows so an edit survives
 *  scrolling or closing the management dialog before saving. */
export interface SourceManagementDraft {
  group: string
  order: number
  weight: number
}

function emptyForm(): SourceForm {
  return {
    name: '',
    base_url: '',
    search_url: '',
    explore_url: '',
    book_url_pattern: '',
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
    source_group: '',
    custom_order: '0',
    weight: '0',
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
  const managementDrafts = reactive<Record<number, SourceManagementDraft>>({})
  const form = ref<SourceForm>(emptyForm())
  const saving = ref(false)
  const sourceUrl = ref('')
  const importing = ref(false)
  const testing = ref<number>()
  const toggling = reactive(new Set<number>())
  const testResults = reactive<Record<number, SourceTestResult>>({})
  const testErrors = reactive<Record<number, string>>({})
  const batchTesting = ref(false)
  const batchResults = ref<SourceTestResult[]>([])
  const exporting = ref(false)
  const loginForm = ref({ sourceId: 0, username: '', password: '' })
  const loggingIn = ref(false)
  const lastProbe = ref<SourceTestResult>()

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
      return false
    }
    saving.value = true
    try {
      await saveBookSource({
        name: current.name,
        base_url: current.base_url,
        search_url: current.search_url,
        explore_url: current.explore_url || undefined,
        book_url_pattern: current.book_url_pattern || undefined,
        enabled_cookie_jar: true,
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
        source_group: current.source_group || undefined,
        custom_order: Number(current.custom_order) || 0,
        weight: Number(current.weight) || 0,
        enabled_explore: true,
        enabled: true,
      })
      form.value = emptyForm()
      await refresh()
      notify('书源已保存')
      return true
    } catch (cause) {
      report(cause)
      return false
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
      return true
    } catch (cause) {
      report(cause)
      return false
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
      return true
    } catch (cause) {
      report(cause)
      return false
    } finally {
      importing.value = false
    }
  }

  async function test(source: BookSource, query: string) {
    if (testing.value != null || batchTesting.value) return
    testing.value = source.id
    try {
      const result = await testBookSource(source.id, query || '测试')
      lastProbe.value = result
      testResults[source.id] = result
      delete testErrors[source.id]
      const status = `HTTP ${result.status}`
      notify(
        result.cloudflare_challenge
          ? `${result.source_name} 仍被 Cloudflare 拦截，请在浏览器认证窗口完成验证后读取会话`
          : result.auth_required
            ? `${result.source_name} 返回 ${status}，会话已标记为过期，请刷新登录`
            : `${result.source_name} 返回 ${status}，解析到 ${result.result_count} 条结果`,
      )
      await refresh()
    } catch (cause) {
      testErrors[source.id] = getErrorMessage(cause)
      delete testResults[source.id]
      if (lastProbe.value?.source_id === source.id) lastProbe.value = undefined
      report(cause)
    } finally {
      testing.value = undefined
    }
  }

  async function batchTest(query: string) {
    if (testing.value != null || batchTesting.value) return
    batchTesting.value = true
    try {
      batchResults.value = await validateAllSources(query || '测试')
      for (const result of batchResults.value) {
        testResults[result.source_id] = result
        delete testErrors[result.source_id]
      }
      const failed = batchResults.value.filter((result) => probeNeedsAttention(result)).length
      notify(`批量验证完成：${batchResults.value.length - failed} 个有搜索结果，${failed} 个需关注`)
      await refresh()
    } catch (cause) {
      report(cause)
    } finally {
      batchTesting.value = false
    }
  }

  async function toggle(source: BookSource) {
    if (toggling.has(source.id)) return
    toggling.add(source.id)
    try {
      await setBookSourceEnabled(source.id, !source.enabled)
      source.enabled = !source.enabled
      notify(`${source.name} 已${source.enabled ? '启用' : '停用'}`)
    } catch (cause) {
      report(cause)
    } finally {
      toggling.delete(source.id)
    }
  }

  /** Seeds a row's draft from what is persisted; later edits stay in the store.
   *  The draft is read back through the reactive record so the row binds to the
   *  proxy, not to the raw object the seed created. */
  function managementDraft(source: BookSource): SourceManagementDraft {
    if (!managementDrafts[source.id]) {
      managementDrafts[source.id] = {
        group: source.source_group ?? '',
        order: source.custom_order,
        weight: source.weight,
      }
    }
    return managementDrafts[source.id]
  }

  async function saveManagement(source: BookSource) {
    const draft = managementDraft(source)
    try {
      await updateBookSourceManagement(
        source.id,
        draft.group || undefined,
        Number(draft.order) || 0,
        Number(draft.weight) || 0,
        source.enabled_explore,
      )
      delete managementDrafts[source.id]
      await refresh()
      notify(`${source.name} 的分组与排序已保存`)
      return true
    } catch (cause) {
      report(cause)
      return false
    }
  }

  async function exportTo(targetPath: string) {
    exporting.value = true
    try {
      const result = await exportBookSources(targetPath)
      notify(`已导出 ${result.exported} 个书源`)
    } catch (cause) {
      report(cause)
    } finally {
      exporting.value = false
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
    toggling,
    testResults,
    testErrors,
    batchTesting,
    batchResults,
    exporting,
    loginForm,
    loggingIn,
    lastProbe,
    refresh,
    updateForm,
    save,
    importFromFile,
    importFromUrl,
    test,
    batchTest,
    toggle,
    managementDraft,
    saveManagement,
    exportTo,
    login,
    refreshSession,
    browserAuth,
    saveBrowserSession,
    clearSession,
  })
}

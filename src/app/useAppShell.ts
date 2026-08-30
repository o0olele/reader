/* global HTMLInputElement, Event, HTMLSelectElement, localStorage, setTimeout, clearTimeout */
import { nextTick, onBeforeUnmount, onMounted, watch } from 'vue'
import packageJson from '../../package.json'
import {
  addOnlineBook,
  clearBookSourceSession,
  createGroup,
  deleteBook,
  fetchOnlineContent,
  getAppSettings,
  getReadingProgress,
  healthCheck,
  importBookSourcesJson,
  importBookSourcesUrl,
  importEpubBook,
  importTxtBook,
  listBookSources,
  listBooks,
  listChapters,
  refreshCatalog,
  listGroups,
  loginBookSource,
  moveBookToGroup,
  searchBooks,
  saveAppSettings,
  saveBookSource,
  saveReadingProgress,
  testBookSource,
  type Book,
  type BookSearchResult,
  type BookSource,
  type Chapter,
} from '../services/api'
import { createAppShellState } from './useAppShellState'

export function useAppShell() {
  const state = createAppShellState()
  const { status, error, books, groups, activeGroup, fileInput, selectedBook, chapters, selectedChapter, loadingChapter,
    onlineSearch, settingsTab, proxyUrl, savingSettings, searchQuery, searchResults, searching, addingResult, sourceForm,
    savingSource, sourceUrl, importingSources, bookSources, testingSource, loginForm, loggingIn, readerContent, fontSize, theme } = state
  let saveTimer: ReturnType<typeof setTimeout> | undefined
  const appVersion = packageJson.version

  function reportError(cause: unknown) {
    error.value = cause instanceof Error ? cause.message : String(cause)
  }

  async function refreshBooks() {
    try { books.value = await listBooks() } catch { /* preview mode */ }
    try { groups.value = await listGroups() } catch { /* preview mode */ }
    try { bookSources.value = await listBookSources() } catch { /* preview mode */ }
    try { proxyUrl.value = (await getAppSettings()).proxy_url ?? '' } catch { /* preview mode */ }
  }

  async function saveSettings() {
    savingSettings.value = true
    try { proxyUrl.value = (await saveAppSettings(proxyUrl.value)).proxy_url ?? ''; error.value = '设置已保存' }
    catch (cause) { reportError(cause) }
    finally { savingSettings.value = false }
  }

  async function runSearch() {
    if (!searchQuery.value.trim()) return
    searching.value = true; error.value = ''
    try { searchResults.value = await searchBooks(searchQuery.value) } catch (cause) { reportError(cause) }
    finally { searching.value = false }
  }

  async function addSearchResult(result: BookSearchResult) {
    addingResult.value = result.url
    try { const book = await addOnlineBook(result); books.value = [book, ...books.value.filter((item) => item.id !== book.id)]; error.value = '' }
    catch (cause) { reportError(cause) }
    finally { addingResult.value = undefined }
  }

  async function addSource() {
    const form = sourceForm.value
    if (!form.name.trim() || !form.base_url.trim() || !form.search_url.trim() || !form.item.trim() || !form.title.trim() || !form.url.trim()) {
      error.value = '请完整填写书源名称、URL 和必需选择器'; return
    }
    savingSource.value = true
    try {
      await saveBookSource({ name: form.name, base_url: form.base_url, search_url: form.search_url,
        search_rule: { item: form.item, title: form.title, author: form.author || undefined, url: form.url },
        login_url: form.login_url || undefined, login_method: form.login_method, login_body: form.login_body || undefined,
        token_path: form.token_path || undefined, sign_script: form.sign_script || undefined,
        proxy_url: form.proxy_url || undefined, enabled: true })
      await refreshBooks(); error.value = ''
    } catch (cause) { reportError(cause) } finally { savingSource.value = false }
  }

  async function loginSource() {
    if (!loginForm.value.sourceId || !loginForm.value.username || !loginForm.value.password) return
    loggingIn.value = true
    try { const result = await loginBookSource(loginForm.value.sourceId, loginForm.value.username, loginForm.value.password); error.value = result.authenticated ? '登录成功，会话已保存' : '登录响应中没有 Token 或 Cookie'; await refreshBooks() }
    catch (cause) { reportError(cause) } finally { loggingIn.value = false }
  }

  async function clearSourceSession(source: BookSource) {
    try { await clearBookSourceSession(source.id); error.value = `已清除 ${source.name} 的会话`; await refreshBooks() }
    catch (cause) { reportError(cause) }
  }

  async function importSourceFile(event: Event) {
    const file = (event.target as HTMLInputElement).files?.[0]; if (!file) return
    importingSources.value = true
    try { error.value = formatImportReport(await importBookSourcesJson(await file.text())) }
    catch (cause) { reportError(cause) }
    finally { importingSources.value = false; (event.target as HTMLInputElement).value = '' }
  }

  async function importSourceFromUrl() {
    if (!sourceUrl.value.trim()) return
    importingSources.value = true
    try { error.value = formatImportReport(await importBookSourcesUrl(sourceUrl.value)); sourceUrl.value = '' }
    catch (cause) { reportError(cause) } finally { importingSources.value = false }
  }

  function formatImportReport(report: { imported: number; failed: string[]; partial?: string[] }) {
    const parts = [`已导入 ${report.imported} 个书源`]
    if (report.failed.length) parts.push(`失败 ${report.failed.length} 个`)
    if (report.partial?.length) parts.push(`${report.partial.length} 个含暂不支持的规则`)
    return parts.join('，')
  }

  async function testSource(source: BookSource) {
    testingSource.value = source.id
    try { const result = await testBookSource(source.id, searchQuery.value || '测试'); error.value = `${result.source_name} 连接成功，解析到 ${result.result_count} 条结果` }
    catch (cause) { reportError(cause) } finally { testingSource.value = undefined }
  }

  const visibleBooks = () => activeGroup.value === null ? books.value : books.value.filter((book) => book.group_id === activeGroup.value)
  const showAllBooks = () => { onlineSearch.value = false; settingsTab.value = false; activeGroup.value = null }
  const selectGroup = (groupId: number) => { onlineSearch.value = false; settingsTab.value = false; activeGroup.value = groupId }
  const showSearch = () => { onlineSearch.value = true; settingsTab.value = false; selectedBook.value = undefined }
  const showSettings = () => { settingsTab.value = true; onlineSearch.value = false; selectedBook.value = undefined }

  async function addGroup() {
    const name = window.prompt('新分组名称')?.trim(); if (!name) return
    try { groups.value.push(await createGroup(name)) } catch (cause) { reportError(cause) }
  }
  async function removeBook(book: Book) {
    if (!window.confirm(`确定删除《${book.title}》吗？`)) return
    try { await deleteBook(book.id); books.value = books.value.filter((item) => item.id !== book.id); await refreshBooks() } catch (cause) { reportError(cause) }
  }
  async function moveBook(book: Book, event: Event) {
    try { const groupId = Number((event.target as HTMLSelectElement).value); await moveBookToGroup(book.id, groupId); book.group_id = groupId; await refreshBooks() } catch (cause) { reportError(cause) }
  }
  const chooseFile = () => fileInput.value?.click()
  async function handleFile(event: Event) {
    const input = event.target as HTMLInputElement; const file = input.files?.[0]; if (!file) return
    error.value = ''
    try { const bytes = Array.from(new Uint8Array(await file.arrayBuffer())); const book = file.name.toLowerCase().endsWith('.epub') ? await importEpubBook(file.name, bytes) : await importTxtBook(file.name, bytes); books.value = [book, ...books.value.filter((item) => item.id !== book.id)] }
    catch (cause) { reportError(cause) } finally { input.value = '' }
  }
  async function openBook(book: Book) {
    error.value = ''
    try { selectedBook.value = book; chapters.value = await listChapters(book.id); const progress = await getReadingProgress(book.id).catch(() => null); selectedChapter.value = chapters.value.find((chapter) => chapter.id === progress?.chapter_id) ?? chapters.value[0]; await loadChapterContent(selectedChapter.value); await nextTick(); if (readerContent.value && progress && selectedChapter.value?.id === progress.chapter_id) readerContent.value.scrollTop = progress.offset }
    catch (cause) { reportError(cause) }
  }
  async function loadChapterContent(chapter?: Chapter) {
    if (!chapter || chapter.content || !selectedBook.value?.source_id || !chapter.remote_url) return
    loadingChapter.value = true
    try { chapter.content = await fetchOnlineContent(selectedBook.value.source_id, chapter.remote_url, chapter.id) } catch (cause) { reportError(cause) } finally { loadingChapter.value = false }
  }
  async function refreshCatalogForBook() {
    if (!selectedBook.value) return
    try { chapters.value = await refreshCatalog(selectedBook.value.id); selectedChapter.value = chapters.value.find((chapter) => chapter.id === selectedChapter.value?.id) ?? chapters.value[0] } catch (cause) { reportError(cause) }
  }
  function scheduleProgressSave() {
    if (!selectedBook.value || !selectedChapter.value) return
    if (saveTimer) clearTimeout(saveTimer)
    saveTimer = setTimeout(() => { if (selectedBook.value && selectedChapter.value && readerContent.value) void saveReadingProgress(selectedBook.value.id, selectedChapter.value.id, readerContent.value.scrollTop) }, 350)
  }
  async function selectChapter(chapter: Chapter) { selectedChapter.value = chapter; await loadChapterContent(chapter); await nextTick(); if (readerContent.value) readerContent.value.scrollTop = 0; scheduleProgressSave() }
  async function closeBook() { scheduleProgressSave(); if (saveTimer) await new Promise<void>((resolve) => setTimeout(resolve, 380)); selectedBook.value = undefined; selectedChapter.value = undefined; chapters.value = [] }

  function updateSourceForm(key: string, value: string) { sourceForm.value[key] = value }
  onMounted(() => { void refreshBooks() })
  watch(fontSize, (value) => localStorage.setItem('reader-font-size', String(value)))
  watch(theme, (value) => localStorage.setItem('reader-theme', value))
  healthCheck().then((value) => { status.value = value }).catch(() => { status.value = '前端预览模式' })
  onBeforeUnmount(() => { scheduleProgressSave() })

  return { appVersion, status, error, books, groups, activeGroup, fileInput, selectedBook, chapters, selectedChapter, loadingChapter,
    onlineSearch, settingsTab, proxyUrl, savingSettings, searchQuery, searchResults, searching, addingResult, sourceForm, savingSource,
    sourceUrl, importingSources, bookSources, testingSource, loginForm, loggingIn, readerContent, fontSize, theme, visibleBooks,
    saveSettings, runSearch, addSearchResult, addSource, loginSource, clearSourceSession, importSourceFile, importSourceFromUrl,
    testSource, showAllBooks, selectGroup, showSearch, showSettings, addGroup, removeBook, moveBook, chooseFile, handleFile, openBook,
    refreshCatalogForBook, selectChapter, closeBook, scheduleProgressSave, updateSourceForm }
}

<script setup lang="ts">
import { nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import packageJson from '../../package.json'
import ReaderPane from '../features/reader/ReaderPane.vue'
import SettingsPage from '../features/settings/SettingsPage.vue'
import BookshelfPage from '../features/bookshelf/BookshelfPage.vue'
import SearchPage from '../features/search/SearchPage.vue'
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
  type BookshelfGroup,
} from '../services/api'

const status = ref('检查中...')
const error = ref('')
const books = ref<Book[]>([])
const groups = ref<BookshelfGroup[]>([])
const activeGroup = ref<number | null>(null)
const fileInput = ref<HTMLInputElement>()
const selectedBook = ref<Book>()
const chapters = ref<Chapter[]>([])
const selectedChapter = ref<Chapter>()
const loadingChapter = ref(false)
const onlineSearch = ref(false)
const settingsTab = ref(false)
const proxyUrl = ref('')
const savingSettings = ref(false)
const searchQuery = ref('')
const searchResults = ref<BookSearchResult[]>([])
const searching = ref(false)
const addingResult = ref<string>()
const sourceForm = ref({
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
})
const savingSource = ref(false)
const sourceFileInput = ref<HTMLInputElement>()
const sourceUrl = ref('')
const importingSources = ref(false)
const bookSources = ref<BookSource[]>([])
const testingSource = ref<number>()
const loginForm = ref({ sourceId: 0, username: '', password: '' })
const loggingIn = ref(false)
const readerContent = ref<HTMLElement | null>(null)
const fontSize = ref(Number(localStorage.getItem('reader-font-size') ?? '17'))
const theme = ref(localStorage.getItem('reader-theme') ?? 'light')
let saveTimer: ReturnType<typeof setTimeout> | undefined
const appVersion = packageJson.version

healthCheck()
  .then((value) => {
    status.value = value
  })
  .catch(() => {
    // Browser/Vite mode has no Tauri IPC; this keeps the shell previewable.
    status.value = '前端预览模式'
  })

async function refreshBooks() {
  try {
    books.value = await listBooks()
  } catch {
    /* Browser preview has no database. */
  }
  try {
    groups.value = await listGroups()
  } catch {
    /* Browser preview has no database. */
  }
  try {
    bookSources.value = await listBookSources()
  } catch {
    /* Browser preview has no database. */
  }
  try {
    proxyUrl.value = (await getAppSettings()).proxy_url ?? ''
  } catch {
    /* Browser preview has no database. */
  }
}

async function saveSettings() {
  savingSettings.value = true
  try {
    proxyUrl.value = (await saveAppSettings(proxyUrl.value)).proxy_url ?? ''
    error.value = '设置已保存'
  } catch (cause) {
    error.value = cause instanceof Error ? cause.message : String(cause)
  } finally {
    savingSettings.value = false
  }
}

async function runSearch() {
  if (!searchQuery.value.trim()) return
  searching.value = true
  error.value = ''
  try {
    searchResults.value = await searchBooks(searchQuery.value)
  } catch (cause) {
    error.value = cause instanceof Error ? cause.message : String(cause)
  } finally {
    searching.value = false
  }
}

async function addSearchResult(result: BookSearchResult) {
  addingResult.value = result.url
  try {
    const book = await addOnlineBook(result)
    books.value = [book, ...books.value.filter((item) => item.id !== book.id)]
    error.value = ''
  } catch (cause) {
    error.value = cause instanceof Error ? cause.message : String(cause)
  } finally {
    addingResult.value = undefined
  }
}

async function addSource() {
  const form = sourceForm.value
  if (
    !form.name.trim() ||
    !form.base_url.trim() ||
    !form.search_url.trim() ||
    !form.item.trim() ||
    !form.title.trim() ||
    !form.url.trim()
  ) {
    error.value = '请完整填写书源名称、URL 和必需选择器'
    return
  }
  savingSource.value = true
  try {
    await saveBookSource({
      name: form.name,
      base_url: form.base_url,
      search_url: form.search_url,
      search_rule: { item: form.item, title: form.title, author: form.author || undefined, url: form.url },
      login_url: form.login_url || undefined,
      login_method: form.login_method,
      login_body: form.login_body || undefined,
      token_path: form.token_path || undefined,
      sign_script: form.sign_script || undefined,
      proxy_url: form.proxy_url || undefined,
      enabled: true,
    })
    await refreshBooks()
    error.value = ''
  } catch (cause) {
    error.value = cause instanceof Error ? cause.message : String(cause)
  } finally {
    savingSource.value = false
  }
}

async function loginSource() {
  if (!loginForm.value.sourceId || !loginForm.value.username || !loginForm.value.password) return
  loggingIn.value = true
  try {
    const result = await loginBookSource(loginForm.value.sourceId, loginForm.value.username, loginForm.value.password)
    error.value = result.authenticated ? '登录成功，会话已保存' : '登录响应中没有 Token 或 Cookie'
    await refreshBooks()
  } catch (cause) {
    error.value = cause instanceof Error ? cause.message : String(cause)
  } finally {
    loggingIn.value = false
  }
}

async function clearSourceSession(source: BookSource) {
  try {
    await clearBookSourceSession(source.id)
    error.value = `已清除 ${source.name} 的会话`
    await refreshBooks()
  } catch (cause) {
    error.value = cause instanceof Error ? cause.message : String(cause)
  }
}

function chooseSourceFile() {
  sourceFileInput.value?.click()
}
async function importSourceFile(event: Event) {
  const file = (event.target as HTMLInputElement).files?.[0]
  if (!file) return
  importingSources.value = true
  try {
    const report = await importBookSourcesJson(await file.text())
    error.value = formatImportReport(report)
  } catch (cause) {
    error.value = cause instanceof Error ? cause.message : String(cause)
  } finally {
    importingSources.value = false
    ;(event.target as HTMLInputElement).value = ''
  }
}
async function importSourceFromUrl() {
  if (!sourceUrl.value.trim()) return
  importingSources.value = true
  try {
    const report = await importBookSourcesUrl(sourceUrl.value)
    error.value = formatImportReport(report)
    sourceUrl.value = ''
  } catch (cause) {
    error.value = cause instanceof Error ? cause.message : String(cause)
  } finally {
    importingSources.value = false
  }
}

function formatImportReport(report: { imported: number; failed: string[]; partial?: string[] }): string {
  const parts = [`已导入 ${report.imported} 个书源`]
  if (report.failed.length) parts.push(`失败 ${report.failed.length} 个`)
  if (report.partial?.length) parts.push(`${report.partial.length} 个含暂不支持的规则`)
  return parts.join('，')
}

async function testSource(source: BookSource) {
  testingSource.value = source.id
  try {
    const result = await testBookSource(source.id, searchQuery.value || '测试')
    error.value = `${result.source_name} 连接成功，解析到 ${result.result_count} 条结果`
  } catch (cause) {
    error.value = cause instanceof Error ? cause.message : String(cause)
  } finally {
    testingSource.value = undefined
  }
}

const visibleBooks = () =>
  activeGroup.value === null ? books.value : books.value.filter((book) => book.group_id === activeGroup.value)

function showAllBooks() {
  onlineSearch.value = false
  settingsTab.value = false
  activeGroup.value = null
}

function selectGroup(groupId: number) {
  onlineSearch.value = false
  settingsTab.value = false
  activeGroup.value = groupId
}

function showSearch() {
  onlineSearch.value = true
  settingsTab.value = false
  selectedBook.value = undefined
}

function showSettings() {
  settingsTab.value = true
  onlineSearch.value = false
  selectedBook.value = undefined
}

async function addGroup() {
  const name = window.prompt('新分组名称')?.trim()
  if (!name) return
  try {
    groups.value.push(await createGroup(name))
  } catch (cause) {
    error.value = cause instanceof Error ? cause.message : String(cause)
  }
}

async function removeBook(book: Book) {
  if (!window.confirm(`确定删除《${book.title}》吗？`)) return
  try {
    await deleteBook(book.id)
    books.value = books.value.filter((item) => item.id !== book.id)
    await refreshBooks()
  } catch (cause) {
    error.value = cause instanceof Error ? cause.message : String(cause)
  }
}

async function moveBook(book: Book, event: Event) {
  const groupId = Number((event.target as HTMLSelectElement).value)
  try {
    await moveBookToGroup(book.id, groupId)
    book.group_id = groupId
    await refreshBooks()
  } catch (cause) {
    error.value = cause instanceof Error ? cause.message : String(cause)
  }
}

function chooseFile() {
  fileInput.value?.click()
}

async function handleFile(event: Event) {
  const input = event.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return
  error.value = ''
  try {
    const bytes = Array.from(new Uint8Array(await file.arrayBuffer()))
    const book = file.name.toLowerCase().endsWith('.epub')
      ? await importEpubBook(file.name, bytes)
      : await importTxtBook(file.name, bytes)
    books.value = [book, ...books.value.filter((item) => item.id !== book.id)]
  } catch (cause) {
    error.value = cause instanceof Error ? cause.message : String(cause)
  } finally {
    input.value = ''
  }
}

onMounted(refreshBooks)

async function openBook(book: Book) {
  error.value = ''
  try {
    selectedBook.value = book
    chapters.value = await listChapters(book.id)
    const progress = await getReadingProgress(book.id).catch(() => null)
    selectedChapter.value = chapters.value.find((chapter) => chapter.id === progress?.chapter_id) ?? chapters.value[0]
    await loadChapterContent(selectedChapter.value)
    await nextTick()
    if (readerContent.value && progress && selectedChapter.value?.id === progress.chapter_id)
      readerContent.value.scrollTop = progress.offset
  } catch (cause) {
    error.value = cause instanceof Error ? cause.message : String(cause)
  }
}

async function loadChapterContent(chapter?: Chapter) {
  if (!chapter || chapter.content || !selectedBook.value?.source_id || !chapter.remote_url) return
  loadingChapter.value = true
  try {
    chapter.content = await fetchOnlineContent(selectedBook.value.source_id, chapter.remote_url, chapter.id)
  } catch (cause) {
    error.value = cause instanceof Error ? cause.message : String(cause)
  } finally {
    loadingChapter.value = false
  }
}

async function refreshCatalogForBook() {
  if (!selectedBook.value) return
  try {
    chapters.value = await refreshCatalog(selectedBook.value.id)
    selectedChapter.value =
      chapters.value.find((chapter) => chapter.id === selectedChapter.value?.id) ?? chapters.value[0]
  } catch (cause) {
    error.value = cause instanceof Error ? cause.message : String(cause)
  }
}

function scheduleProgressSave() {
  if (!selectedBook.value || !selectedChapter.value) return
  if (saveTimer) clearTimeout(saveTimer)
  saveTimer = setTimeout(() => {
    if (selectedBook.value && selectedChapter.value && readerContent.value) {
      saveReadingProgress(selectedBook.value.id, selectedChapter.value.id, readerContent.value.scrollTop).catch(
        () => undefined,
      )
    }
  }, 350)
}

async function selectChapter(chapter: Chapter) {
  selectedChapter.value = chapter
  await loadChapterContent(chapter)
  await nextTick()
  if (readerContent.value) readerContent.value.scrollTop = 0
  scheduleProgressSave()
}

async function closeBook() {
  scheduleProgressSave()
  if (saveTimer) await new Promise<void>((resolve) => setTimeout(resolve, 380))
  selectedBook.value = undefined
  selectedChapter.value = undefined
  chapters.value = []
}

watch(fontSize, (value) => localStorage.setItem('reader-font-size', String(value)))
watch(theme, (value) => localStorage.setItem('reader-theme', value))
onBeforeUnmount(() => {
  scheduleProgressSave()
})
</script>

<template>
  <main class="shell">
    <header class="topbar">
      <div class="brand"><span class="brand-mark">R</span><span>Reader</span></div>
      <div class="topbar-actions">
        <span class="version">v{{ appVersion }}</span
        ><button
          v-if="onlineSearch && bookSources.length"
          type="button"
          class="source-test topbar-test"
          :disabled="testingSource !== undefined"
          @click="testSource(bookSources[0])"
        >
          {{ testingSource !== undefined ? '测试中...' : '测试书源' }}
        </button>
      </div>
    </header>
    <section class="workspace">
      <aside class="sidebar">
        <nav aria-label="主导航">
          <a
            :class="{ active: activeGroup === null && !onlineSearch && !settingsTab }"
            href="#"
            @click.prevent="showAllBooks"
            >全部书籍</a
          >
          <a
            v-for="group in groups"
            :key="group.id"
            :class="{ active: activeGroup === group.id && !onlineSearch }"
            href="#"
            @click.prevent="selectGroup(group.id)"
            >{{ group.name }} <small>{{ group.book_count }}</small></a
          >
          <a href="#" @click.prevent="addGroup">+ 新建分组</a>
          <a href="#" @click.prevent="showAllBooks">本地书籍</a>
          <a :class="{ active: onlineSearch }" href="#" @click.prevent="showSearch">在线搜索</a>
          <a :class="{ active: settingsTab }" href="#" @click.prevent="showSettings">设置</a>
          <a href="#">阅读历史</a>
        </nav>
        <div class="sidebar-footer"><span class="dot" />{{ status }}</div>
      </aside>
      <section class="content">
        <div v-if="selectedBook" class="content-heading">
          <div>
            <button type="button" class="back-button" @click="closeBook">返回书架</button>
            <p class="eyebrow">正在阅读</p>
            <h1>{{ selectedBook.title }}</h1>
          </div>
          <button v-if="selectedBook.source_id" type="button" class="secondary" @click="refreshCatalogForBook">
            刷新目录
          </button>
          <div class="reader-settings">
            <label
              >主题
              <select v-model="theme">
                <option value="light">浅色</option>
                <option value="sepia">护眼</option>
                <option value="dark">深色</option>
              </select></label
            ><label>字号 <input v-model.number="fontSize" type="range" min="14" max="25" step="1" /></label>
          </div>
        </div>
        <div v-else-if="settingsTab" class="content-heading">
          <div>
            <p class="eyebrow">应用设置</p>
            <h1>设置</h1>
          </div>
        </div>
        <div v-else-if="onlineSearch" class="content-heading online-heading">
          <div>
            <p class="eyebrow">在线书源</p>
            <h1>搜索书籍</h1>
          </div>
          <form class="search-form" @submit.prevent="runSearch">
            <input v-model="searchQuery" placeholder="输入书名或作者" aria-label="搜索书名或作者" /><button
              type="submit"
              class="primary"
              :disabled="searching"
            >
              {{ searching ? '搜索中...' : '搜索' }}
            </button>
          </form>
        </div>
        <div v-else class="content-heading">
          <div>
            <p class="eyebrow">我的阅读空间</p>
            <h1>书架</h1>
          </div>
          <button type="button" class="primary" @click="chooseFile">导入书籍</button>
          <input
            ref="fileInput"
            class="visually-hidden"
            type="file"
            accept=".txt,.epub,text/plain,application/epub+zip"
            @change="handleFile"
          />
        </div>
        <div v-if="error" class="error-banner">{{ error }}</div>
        <ReaderPane
          v-if="selectedBook"
          :chapters="chapters"
          :selected-chapter="selectedChapter"
          :theme="theme"
          :font-size="fontSize"
          :loading="loadingChapter"
          @select-chapter="selectChapter"
          @scroll="scheduleProgressSave"
          @reader-content="readerContent = $event"
        />
        <SettingsPage
          v-else-if="settingsTab"
          :proxy-url="proxyUrl"
          :saving="savingSettings"
          @update:proxy-url="proxyUrl = $event"
          @save="saveSettings"
          @clear="proxyUrl = ''"
        />
        <SearchPage
          v-else-if="onlineSearch"
          :query="searchQuery"
          :results="searchResults"
          :sources="bookSources"
          :searching="searching"
          :importing-sources="importingSources"
          :saving-source="savingSource"
          :adding-result="addingResult"
          :logging-in="loggingIn"
          :source-url="sourceUrl"
          :login-source-id="loginForm.sourceId"
          :username="loginForm.username"
          :password="loginForm.password"
          :source-form="sourceForm"
          @update:query="searchQuery = $event"
          @update:source-url="sourceUrl = $event"
          @update:login-source-id="loginForm.sourceId = $event"
          @update:username="loginForm.username = $event"
          @update:password="loginForm.password = $event"
          @search="runSearch"
          @import-file="importSourceFile"
          @import-url="importSourceFromUrl"
          @save-source="addSource"
          @login="loginSource"
          @clear-session="clearSourceSession"
          @add="addSearchResult"
        />
        <!-- Legacy inline search markup is retained below during the extraction and is disabled. -->
        <!--
        <div v-else-if="onlineSearch" class="search-results">
          <details class="source-editor">
            <summary>添加或导入书源</summary>
            <div class="source-import">
              <button type="button" class="secondary" :disabled="importingSources" @click="chooseSourceFile">
                导入 JSON 文件</button
              ><input
                ref="sourceFileInput"
                class="visually-hidden"
                type="file"
                accept=".json,application/json"
                @change="importSourceFile"
              />
              <form class="source-url-form" @submit.prevent="importSourceFromUrl">
                <input
                  v-model="sourceUrl"
                  type="url"
                  placeholder="从 URL 导入 JSON"
                  aria-label="书源 JSON URL"
                /><button type="submit" class="secondary" :disabled="importingSources">导入 URL</button>
              </form>
            </div>
            <form @submit.prevent="addSource">
              <div class="source-fields">
                <input v-model="sourceForm.name" placeholder="名称" aria-label="书源名称" /><input
                  v-model="sourceForm.base_url"
                  placeholder="基础 URL，如 https://example.com"
                  aria-label="基础 URL"
                /><input
                  v-model="sourceForm.search_url"
                  placeholder="搜索 URL，使用 {{key}}"
                  aria-label="搜索 URL"
                /><input v-model="sourceForm.item" placeholder="结果项 CSS，如 .book" aria-label="结果项 CSS" /><input
                  v-model="sourceForm.title"
                  placeholder="标题 CSS，如 .title"
                  aria-label="标题 CSS"
                /><input v-model="sourceForm.author" placeholder="作者 CSS（可选）" aria-label="作者 CSS" /><input
                  v-model="sourceForm.url"
                  placeholder="链接 CSS，如 a::attr(href)"
                  aria-label="链接 CSS"
                /><input v-model="sourceForm.login_url" placeholder="登录 URL（可选）" aria-label="登录 URL" /><select
                  v-model="sourceForm.login_method"
                  aria-label="登录方法"
                >
                  <option>POST</option>
                  <option>GET</option>
                  <option>PUT</option></select
                ><input
                  v-model="sourceForm.login_body"
                  placeholder="登录 Body，使用 {{username}}/{{password}}"
                  aria-label="登录 Body"
                /><input
                  v-model="sourceForm.token_path"
                  placeholder="Token 路径，如 data.token"
                  aria-label="Token 路径"
                /><input
                  v-model="sourceForm.sign_script"
                  placeholder="签名表达式，如 sha256({{url}}:secret)"
                  aria-label="签名表达式"
                />
              </div>
              <button type="submit" class="secondary" :disabled="savingSource">
                {{ savingSource ? '保存中...' : '保存书源' }}
              </button>
            </form>
          </details>
          <details v-if="bookSources.some((source) => source.login_url)" class="source-editor">
            <summary>登录书源</summary>
            <form class="source-import" @submit.prevent="loginSource">
              <select v-model.number="loginForm.sourceId" aria-label="登录书源">
                <option :value="0">选择书源</option>
                <option
                  v-for="source in bookSources.filter((item) => item.login_url)"
                  :key="source.id"
                  :value="source.id"
                >
                  {{ source.name }}
                </option></select
              ><input
                v-model="loginForm.username"
                placeholder="用户名"
                aria-label="用户名"
                autocomplete="username"
              /><input
                v-model="loginForm.password"
                type="password"
                placeholder="密码"
                aria-label="密码"
                autocomplete="current-password"
              /><button type="submit" class="secondary" :disabled="loggingIn">
                {{ loggingIn ? '登录中...' : '登录并保存会话' }}
              </button>
            </form>
          </details>
          <div
            v-if="bookSources.some((source) => source.session_cookie || source.access_token)"
            class="source-sessions"
          >
            <span
              v-for="source in bookSources.filter((item) => item.session_cookie || item.access_token)"
              :key="source.id"
              >{{ source.name }}：已认证
              <button type="button" class="secondary" @click="clearSourceSession(source)">清除</button></span
            >
          </div>
          <details class="source-editor">
            <summary>代理设置</summary>
            <div class="source-import">
              <input
                v-model="sourceForm.proxy_url"
                placeholder="代理 URL，如 http://127.0.0.1:7890"
                aria-label="代理 URL"
              /><span class="proxy-hint">填写后保存书源即可生效，支持 HTTP、HTTPS、SOCKS 代理</span>
            </div>
          </details>
          <div v-if="!searching && !searchResults.length" class="search-empty">输入关键词后开始搜索</div>
          <article v-for="result in searchResults" :key="result.url" class="search-result">
            <div class="book-cover">{{ result.title.slice(0, 1) }}</div>
            <div class="search-result-meta">
              <h2>{{ result.title }}</h2>
              <p>{{ result.author || '作者未知' }} · {{ result.source_name }}</p>
              <button
                type="button"
                class="secondary"
                :disabled="addingResult === result.url"
                @click="addSearchResult(result)"
              >
                {{ addingResult === result.url ? '加入中...' : '加入书架' }}
              </button>
            </div>
          </article>
        </div>
        -->
        <BookshelfPage
          v-else
          :books="visibleBooks()"
          :groups="groups"
          @open="openBook"
          @move="moveBook"
          @remove="removeBook"
          @choose="chooseFile"
        />
      </section>
    </section>
  </main>
</template>

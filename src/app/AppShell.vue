<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, provide, ref } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import {
  ArrowLeft,
  ArrowRight,
  BookOpen,
  Bookmark,
  Clock3,
  Cloud,
  Compass,
  Database,
  Download,
  List,
  Minus,
  Moon,
  MoreHorizontal,
  Plus,
  Rss,
  Search,
  Settings,
  Square,
  Sun,
  Type,
  ALargeSmall,
  X,
} from 'lucide-vue-next'
import type { LucideIcon } from 'lucide-vue-next'
import BookshelfPage from '../features/bookshelf/BookshelfPage.vue'
import DownloadPage from '../features/download/DownloadPage.vue'
import LibraryTabPage from '../features/bookshelf/LibraryTabPage.vue'
import ReaderPane from '../features/reader/ReaderPane.vue'
import ExplorePage from '../features/search/ExplorePage.vue'
import SearchPage from '../features/search/SearchPage.vue'
import SettingsPage from '../features/settings/SettingsPage.vue'
import SourceDebugPage from '../features/source/SourceDebugPage.vue'
import { useAppShell, type ShellView } from './useAppShell'
import { searchKey, settingsKey, sourceDebugKey, sourcesKey } from './shellKeys'

const { view, message, bookshelf, reader, search, settings, sources, sourceDebug, show, openBook } = useAppShell()
provide(searchKey, search)
provide(settingsKey, settings)
provide(sourceDebugKey, sourceDebug)
provide(sourcesKey, sources)

const fileInput = ref<HTMLInputElement>()
const headerQuery = ref('')
const autoBrightness = ref(false)
const fullScreen = ref(false)
const showStatusBar = ref(true)
const appTheme = ref<'light' | 'dark'>((localStorage.getItem('app-theme') as 'light' | 'dark') ?? 'light')
// Side panels start closed for a clean reading surface.  They can be opened
// explicitly from the chapter bar and their state is kept independent.
const readerSettingsOpen = ref(false)
const chapterListOpen = ref(false)
const fontMenuOpen = ref(false)
const fontSizeMenuOpen = ref(false)
const chooseFile = () => fileInput.value?.click()
const openBookFromShelf = async (book: Parameters<typeof openBook>[0]) => {
  chapterListOpen.value = false
  readerSettingsOpen.value = false
  fontMenuOpen.value = false
  fontSizeMenuOpen.value = false
  await openBook(book)
}
const win = getCurrentWindow()
const windowAction = async (action: 'minimize' | 'maximize' | 'close') => {
  try {
    if (action === 'minimize') await win.minimize()
    else if (action === 'maximize') await win.toggleMaximize()
    else await win.close()
  } catch {
    /* browser preview */
  }
}
const dragWindow = async (event?: MouseEvent) => {
  if (event && (event.button !== 0 || (event.target as HTMLElement).closest('button, input, form, select'))) return
  try {
    await win.startDragging()
  } catch {
    /* browser preview */
  }
}
const toggleAppTheme = () => {
  appTheme.value = appTheme.value === 'light' ? 'dark' : 'light'
  localStorage.setItem('app-theme', appTheme.value)
}
const chapterIndex = computed(() => reader.chapters.findIndex((chapter) => chapter.id === reader.selectedChapter?.id))
const selectRelativeChapter = (offset: number) => {
  const chapter = reader.chapters[chapterIndex.value + offset]
  if (chapter) void reader.selectChapter(chapter)
}
const toggleReaderTheme = () => {
  reader.theme = reader.theme === 'light' ? 'dark' : 'light'
}
const toggleFontMenu = () => {
  fontMenuOpen.value = !fontMenuOpen.value
  fontSizeMenuOpen.value = false
}
const toggleFontSizeMenu = () => {
  fontSizeMenuOpen.value = !fontSizeMenuOpen.value
  fontMenuOpen.value = false
}
const setFontFamily = (family: string) => {
  reader.fontFamily = family
  fontMenuOpen.value = false
}
const setFontSize = (size: number) => {
  reader.fontSize = size
  fontSizeMenuOpen.value = false
}
const closeReaderMenus = (event: PointerEvent) => {
  const target = event.target
  if (target instanceof Element && target.closest('.reader-dropdown')) return
  fontMenuOpen.value = false
  fontSizeMenuOpen.value = false
}
onMounted(() => document.addEventListener('pointerdown', closeReaderMenus))
onBeforeUnmount(() => document.removeEventListener('pointerdown', closeReaderMenus))
const submitSearch = () => {
  const query = headerQuery.value.trim()
  if (!query) {
    void show('search')
    return
  }
  search.query = query
  void show('search')
  // Let the search page mount and register its source provider before the
  // header-triggered search starts.
  window.setTimeout(() => void search.run(), 0)
}
const navItems: { id: ShellView; label: string; icon: LucideIcon }[] = [
  { id: 'bookshelf', label: '书架', icon: BookOpen },
  { id: 'explore', label: '发现', icon: Compass },
  { id: 'search', label: '搜索', icon: Search },
  { id: 'sources', label: '书源', icon: Database },
  { id: 'downloads', label: '下载', icon: Download },
  { id: 'rss', label: 'RSS', icon: Rss },
  { id: 'history', label: '历史', icon: Clock3 },
  { id: 'bookmarks', label: '书签', icon: Bookmark },
]
const pageTitle = computed(
  () =>
    ({
      bookshelf: '书架',
      explore: '发现',
      sources: '书源',
      search: '搜索书籍',
      settings: '设置',
      downloads: '下载',
      rss: 'RSS',
      history: '历史',
      bookmarks: '书签',
    })[view.value],
)
const resetReaderSettings = () => {
  reader.theme = 'light'
  reader.fontFamily = '思源宋体'
  reader.fontSize = 17
  reader.lineHeight = 1.8
  reader.pageMargin = 32
  reader.readerMode = 'paged'
}
</script>

<template>
  <main :class="['reader-app', { 'app-theme-dark': appTheme === 'dark' }]">
    <header class="appbar" data-tauri-drag-region @mousedown="dragWindow" @dblclick="windowAction('maximize')">
      <div class="appbar-brand" data-tauri-drag-region>
        <span class="brand-mark"><BookOpen :size="20" stroke-width="2.5" /></span><strong>阅读</strong>
      </div>
      <form class="app-search" @submit.prevent="submitSearch">
        <Search :size="17" /><input
          v-model="headerQuery"
          placeholder="搜索书名、作者、书源..."
          aria-label="搜索书名、作者、书源"
        />
      </form>
      <div class="appbar-tools">
        <button
          type="button"
          aria-label="切换主题"
          :title="appTheme === 'light' ? '切换深色主题' : '切换浅色主题'"
          @click.stop="toggleAppTheme"
        >
          <Moon v-if="appTheme === 'dark'" :size="18" /><Sun v-else :size="18" /></button
        ><button aria-label="云同步" title="云同步"><Cloud :size="18" /></button
        ><button aria-label="设置" title="设置" @click.stop="show('settings')"><Settings :size="18" /></button><i></i
        ><button aria-label="最小化" @click.stop="windowAction('minimize')"><Minus :size="18" /></button
        ><button aria-label="最大化" @click.stop="windowAction('maximize')"><Square :size="15" /></button
        ><button aria-label="关闭" @click.stop="windowAction('close')"><X :size="18" /></button>
      </div>
    </header>
    <section class="app-body">
      <aside class="left-nav">
        <nav>
          <button
            v-for="item in navItems"
            :key="item.id"
            :class="['nav-item', { active: view === item.id && !reader.selectedBook }]"
            @click="show(item.id)"
          >
            <component :is="item.icon" :size="18" /><span>{{ item.label }}</span>
          </button>
        </nav>
        <div class="my-books">
          <div class="my-books-title">
            <span>我的书架</span
            ><button aria-label="新建分组" @click="bookshelf.addGroup()"><Plus :size="16" /></button>
          </div>
          <button
            v-for="book in bookshelf.visibleBooks.slice(0, 5)"
            :key="book.id"
            :class="['mini-book', { selected: reader.selectedBook?.id === book.id }]"
            @click="openBookFromShelf(book)"
          >
            <span class="mini-cover"
              ><img v-if="book.cover_data" :src="book.cover_data" :alt="book.title" /><BookOpen
                v-else
                :size="16" /></span
            ><span class="mini-meta"
              ><b>{{ book.title }}</b
              ><small>{{ book.author || '未知作者' }}</small
              ><em>已读 {{ book.chapter_count ? '62.3' : '0' }}%</em></span
            >
          </button>
          <div v-if="!bookshelf.visibleBooks.length" class="mini-empty">导入一本书开始阅读</div>
        </div>
        <button class="settings-link" @click="show('settings')"><Settings :size="17" /><span>设置</span></button>
      </aside>
      <section class="main-area">
        <div v-if="message" class="error-banner">
          <span>{{ message }}</span
          ><button aria-label="关闭提示" @click="message = ''"><X :size="15" /></button>
        </div>
        <template v-if="reader.selectedBook">
          <div class="reader-topbar">
            <div class="reader-nav-actions">
              <button aria-label="上一章" :disabled="chapterIndex <= 0" @click="selectRelativeChapter(-1)">
                <ArrowLeft :size="18" /></button
              ><button
                aria-label="下一章"
                :disabled="chapterIndex < 0 || chapterIndex >= reader.chapters.length - 1"
                @click="selectRelativeChapter(1)"
              >
                <ArrowRight :size="18" /></button
              ><button aria-label="关闭阅读" title="关闭阅读" @click="reader.closeBook"><X :size="17" /></button>
            </div>
            <strong
              >第{{ reader.chapters.findIndex((c) => c.id === reader.selectedChapter?.id) + 1 || 1 }}章
              {{ reader.selectedChapter?.title || '' }}</strong
            >
            <div class="reader-top-tools">
              <div class="reader-dropdown">
                <button
                  type="button"
                  class="reader-dropdown-trigger"
                  aria-label="选择字体"
                  @click.stop="toggleFontMenu"
                >
                  <Type :size="16" aria-hidden="true" /><span class="reader-dropdown-label">{{
                    reader.fontFamily
                  }}</span>
                </button>
                <div v-if="fontMenuOpen" class="reader-menu" role="menu">
                  <button
                    v-for="family in ['思源宋体', '霞鹜文楷', '系统默认']"
                    :key="family"
                    role="menuitem"
                    :class="{ selected: reader.fontFamily === family }"
                    @click="setFontFamily(family)"
                  >
                    {{ family }}
                  </button>
                </div>
              </div>
              <div class="reader-dropdown">
                <button
                  type="button"
                  class="reader-dropdown-trigger"
                  aria-label="选择字号"
                  @click.stop="toggleFontSizeMenu"
                >
                  <ALargeSmall :size="16" aria-hidden="true" /><span class="reader-dropdown-label"
                    >{{ reader.fontSize }}px</span
                  >
                </button>
                <div v-if="fontSizeMenuOpen" class="reader-menu reader-size-menu" role="menu">
                  <button
                    v-for="size in [14, 16, 17, 18, 20, 22, 24]"
                    :key="size"
                    role="menuitem"
                    :class="{ selected: reader.fontSize === size }"
                    @click="setFontSize(size)"
                  >
                    {{ size }} px
                  </button>
                </div>
              </div>
              <i></i
              ><button aria-label="切换阅读主题" title="切换阅读主题" @click="toggleReaderTheme">
                <Sun :size="17" /></button
              ><button
                :class="{ active: chapterListOpen }"
                aria-label="显示或隐藏目录"
                title="显示或隐藏目录"
                @click="chapterListOpen = !chapterListOpen"
              >
                <List :size="18" /></button
              ><button
                :class="{ active: readerSettingsOpen }"
                aria-label="显示或隐藏阅读设置"
                title="显示或隐藏阅读设置"
                @click="readerSettingsOpen = !readerSettingsOpen"
              >
                <MoreHorizontal :size="18" />
              </button>
            </div>
          </div>
          <div class="reader-workspace">
            <ReaderPane
              :chapters="reader.chapters"
              :selected-chapter="reader.selectedChapter"
              :theme="reader.theme"
              :font-size="reader.fontSize"
              :font-family="reader.fontFamily"
              :line-height="reader.lineHeight"
              :page-margin="reader.pageMargin"
              :reader-mode="reader.readerMode"
              :chapter-list-open="chapterListOpen"
              :loading="reader.loadingChapter"
              :book="reader.selectedBook"
              @select-chapter="reader.selectChapter"
              @scroll="reader.scheduleProgressSave"
              @reader-content="reader.readerContent = $event"
              @reader-mode="reader.readerMode = $event"
            />
            <aside v-if="readerSettingsOpen" class="settings-panel">
              <div class="settings-head">
                <strong>阅读设置</strong
                ><button aria-label="关闭阅读设置" title="关闭阅读设置" @click="readerSettingsOpen = false">
                  <X :size="18" />
                </button>
              </div>
              <label class="setting-label">主题</label>
              <div class="theme-grid">
                <button
                  v-for="themeOption in [
                    ['light', '浅色'],
                    ['sepia', '护眼'],
                    ['dark', '深色'],
                    ['black', '黑夜'],
                  ]"
                  :key="themeOption[0]"
                  :class="['theme-card', { active: reader.theme === themeOption[0] }]"
                  @click="reader.theme = themeOption[0]"
                >
                  <span :class="`theme-swatch ${themeOption[0]}`"></span><small>{{ themeOption[1] }}</small>
                </button>
              </div>
              <label class="setting-label">字体</label
              ><select v-model="reader.fontFamily" class="setting-select">
                <option value="思源宋体">思源宋体</option>
                <option value="霞鹜文楷">霞鹜文楷</option>
                <option value="系统默认">系统默认</option></select
              ><label class="setting-label">字号</label>
              <div class="font-step">
                <button @click="reader.fontSize = Math.max(14, reader.fontSize - 1)">A-</button
                ><span>{{ reader.fontSize }}</span
                ><button @click="reader.fontSize = Math.min(28, reader.fontSize + 1)">A+</button>
              </div>
              <label class="setting-label"
                >行距 <output>{{ reader.lineHeight.toFixed(1) }}</output></label
              ><input v-model.number="reader.lineHeight" type="range" min="1.4" max="2.4" step="0.1" /><label
                class="setting-label"
                >页面宽度 <output>{{ reader.pageMargin }}%</output></label
              ><input v-model.number="reader.pageMargin" type="range" min="16" max="64" step="4" /><label
                class="setting-label"
                >阅读模式</label
              >
              <div class="mode-switch">
                <button :class="{ active: reader.readerMode === 'paged' }" @click="reader.readerMode = 'paged'">
                  ◆ 翻页</button
                ><button :class="{ active: reader.readerMode === 'scroll' }" @click="reader.readerMode = 'scroll'">
                  滚动
                </button>
              </div>
              <div class="toggle-row">
                <span>自动亮度</span
                ><button :class="['toggle', { on: autoBrightness }]" @click="autoBrightness = !autoBrightness"></button>
              </div>
              <div class="toggle-row">
                <span>全屏阅读</span
                ><button :class="['toggle', { on: fullScreen }]" @click="fullScreen = !fullScreen"></button>
              </div>
              <div class="toggle-row">
                <span>显示状态栏</span
                ><button :class="['toggle', { on: showStatusBar }]" @click="showStatusBar = !showStatusBar"></button>
              </div>
              <button class="reset-settings" @click="resetReaderSettings">恢复默认</button>
            </aside>
          </div>
        </template>
        <template v-else
          ><div class="page-heading">
            <div>
              <span>我的阅读空间</span>
              <h1>{{ pageTitle }}</h1>
            </div>
            <button v-if="view === 'bookshelf'" class="primary" @click="chooseFile">
              <Plus :size="16" /> 导入书籍
            </button>
          </div>
          <input
            ref="fileInput"
            class="visually-hidden"
            type="file"
            accept=".txt,.epub,text/plain,application/epub+zip"
            @change="bookshelf.handleFile" /><SettingsPage v-if="view === 'settings'" /><SourceDebugPage
            v-else-if="view === 'sources'" /><SearchPage v-else-if="view === 'search'" /><ExplorePage
            v-else-if="view === 'explore'" /><DownloadPage
            v-else-if="view === 'downloads'"
            :books="bookshelf.visibleBooks" /><LibraryTabPage
            v-else-if="view === 'rss' || view === 'history' || view === 'bookmarks'"
            :kind="view"
            :books="bookshelf.visibleBooks"
            @open="openBookFromShelf" /><BookshelfPage
            v-else
            :books="bookshelf.visibleBooks"
            :groups="bookshelf.groups"
            @open="openBookFromShelf"
            @move="bookshelf.moveBook"
            @remove="bookshelf.removeBook"
            @choose="chooseFile"
        /></template>
      </section>
    </section>
  </main>
</template>

<script setup lang="ts">
import { computed, provide, ref } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import {
  ArrowLeft,
  ArrowRight,
  BookOpen,
  Bookmark,
  ChevronDown,
  Clock3,
  Cloud,
  Compass,
  Database,
  Download,
  List,
  Minus,
  MoreHorizontal,
  Plus,
  Rss,
  Search,
  Settings,
  Square,
  Sun,
  X,
} from 'lucide-vue-next'
import type { LucideIcon } from 'lucide-vue-next'
import BookshelfPage from '../features/bookshelf/BookshelfPage.vue'
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
const chooseFile = () => fileInput.value?.click()
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
const dragWindow = async () => {
  try {
    await win.startDragging()
  } catch {
    /* browser preview */
  }
}
const submitSearch = () => {
  const query = headerQuery.value.trim()
  if (!query) {
    void show('search')
    return
  }
  search.query = query
  void show('search')
  void search.run()
}
const navItems: { id: ShellView; label: string; icon: LucideIcon }[] = [
  { id: 'bookshelf', label: '书架', icon: BookOpen },
  { id: 'explore', label: '发现', icon: Compass },
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
  reader.fontSize = 17
  reader.lineHeight = 1.8
  reader.pageMargin = 32
  reader.readerMode = 'paged'
}
</script>

<template>
  <main class="reader-app">
    <header class="appbar" @dblclick="windowAction('maximize')">
      <div class="appbar-brand" data-tauri-drag-region @mousedown="dragWindow">
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
        <button aria-label="亮色模式" title="亮色模式"><Sun :size="18" /></button
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
            @click="openBook(book)"
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
              <button aria-label="返回书架" @click="reader.closeBook"><ArrowLeft :size="18" /></button
              ><button aria-label="前进" disabled><ArrowRight :size="18" /></button>
            </div>
            <strong
              >第{{ reader.chapters.findIndex((c) => c.id === reader.selectedChapter?.id) + 1 || 1 }}章
              {{ reader.selectedChapter?.title || '' }}</strong
            >
            <div class="reader-top-tools">
              <button @click="reader.fontSize = Math.max(14, reader.fontSize - 1)">Aa</button
              ><ChevronDown :size="14" /><button @click="reader.fontSize++">{{ reader.fontSize }}</button
              ><ChevronDown :size="14" /><i></i><button><Sun :size="17" /></button><button><List :size="18" /></button
              ><button><MoreHorizontal :size="18" /></button>
            </div>
          </div>
          <div class="reader-workspace">
            <ReaderPane
              :chapters="reader.chapters"
              :selected-chapter="reader.selectedChapter"
              :theme="reader.theme"
              :font-size="reader.fontSize"
              :line-height="reader.lineHeight"
              :page-margin="reader.pageMargin"
              :reader-mode="reader.readerMode"
              :loading="reader.loadingChapter"
              :book="reader.selectedBook"
              @select-chapter="reader.selectChapter"
              @scroll="reader.scheduleProgressSave"
              @reader-content="reader.readerContent = $event"
              @reader-mode="reader.readerMode = $event"
            />
            <aside class="settings-panel">
              <div class="settings-head">
                <strong>阅读设置</strong><button @click="reader.closeBook"><X :size="18" /></button>
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
              ><select v-model="reader.fontSize" class="setting-select">
                <option :value="17">思源宋体</option>
                <option :value="18">霞鹜文楷</option></select
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
            v-else-if="view === 'explore'" /><LibraryTabPage
            v-else-if="view === 'downloads' || view === 'rss' || view === 'history' || view === 'bookmarks'"
            :kind="view"
            :books="bookshelf.visibleBooks"
            @open="openBook" /><BookshelfPage
            v-else
            :books="bookshelf.visibleBooks"
            :groups="bookshelf.groups"
            @open="openBook"
            @move="bookshelf.moveBook"
            @remove="bookshelf.removeBook"
            @choose="chooseFile"
        /></template>
      </section>
    </section>
  </main>
</template>

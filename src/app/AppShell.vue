<script setup lang="ts">
import { provide, ref } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import BookshelfPage from '../features/bookshelf/BookshelfPage.vue'
import ReaderPane from '../features/reader/ReaderPane.vue'
import ExplorePage from '../features/search/ExplorePage.vue'
import SearchPage from '../features/search/SearchPage.vue'
import SettingsPage from '../features/settings/SettingsPage.vue'
import SourceDebugPage from '../features/source/SourceDebugPage.vue'
import { useAppShell } from './useAppShell'
import { searchKey, settingsKey, sourceDebugKey, sourcesKey } from './shellKeys'

const { view, message, bookshelf, reader, search, settings, sources, sourceDebug, show, openBook } = useAppShell()
provide(searchKey, search)
provide(settingsKey, settings)
provide(sourceDebugKey, sourceDebug)
provide(sourcesKey, sources)
const fileInput = ref<HTMLInputElement>()
const chooseFile = () => fileInput.value?.click()
const win = getCurrentWindow()
const windowAction = async (action: 'minimize' | 'maximize' | 'close') => { try { if (action === 'minimize') await win.minimize(); else if (action === 'maximize') await win.toggleMaximize(); else await win.close() } catch { /* browser preview */ } }
const dragWindow = async () => { try { await win.startDragging() } catch { /* browser preview */ } }
const navItems = [
  { id: 'bookshelf', label: '书架', icon: '▥' }, { id: 'explore', label: '发现', icon: '◉' },
  { id: 'sources', label: '书源', icon: '◇' }, { id: 'search', label: '下载', icon: '↓' },
  { id: 'sources', label: 'RSS', icon: '◔' }, { id: 'bookshelf', label: '历史', icon: '◷' }, { id: 'bookshelf', label: '书签', icon: '♧' },
] as const
</script>

<template>
  <main class="reader-app">
    <header class="appbar" @dblclick="windowAction('maximize')">
      <div class="appbar-brand" data-tauri-drag-region @mousedown="dragWindow"><span class="brand-book">▮</span><strong>阅读</strong></div>
      <div class="app-search" @click="show('search')"><span class="search-icon">⌕</span><span>搜索书名、作者、书源...</span></div>
      <div class="appbar-tools"><button>☼</button><button>♧</button><button @click.stop="show('settings')">⚙</button><i></i><button @click.stop="windowAction('minimize')">−</button><button @click.stop="windowAction('maximize')">□</button><button @click.stop="windowAction('close')">×</button></div>
    </header>
    <section class="app-body">
      <aside class="left-nav">
        <nav><button v-for="item in navItems" :key="item.label" :class="['nav-item', { active: view === item.id && !reader.selectedBook }]" @click="show(item.id as any)"><span>{{ item.icon }}</span>{{ item.label }}</button></nav>
        <div class="my-books"><div class="my-books-title"><span>⌄ 我的书架</span><button @click="bookshelf.addGroup()">＋</button></div><button v-for="book in bookshelf.visibleBooks.slice(0, 5)" :key="book.id" :class="['mini-book', { selected: reader.selectedBook?.id === book.id }]" @click="openBook(book)"><span class="mini-cover"><img v-if="book.cover_data" :src="book.cover_data" :alt="book.title" /><span v-else>{{ book.title.slice(0, 1) }}</span></span><span class="mini-meta"><b>{{ book.title }}</b><small>{{ book.author || '未知作者' }}</small><em>{{ book.chapter_count }} 个章节</em></span></button></div>
        <button class="settings-link" @click="show('settings')">⚙ <span>设置</span></button>
      </aside>
      <section class="main-area">
        <div v-if="message" class="error-banner">{{ message }}</div>
        <template v-if="reader.selectedBook">
          <div class="reader-topbar"><div class="reader-nav-actions"><button @click="reader.closeBook">←</button><button @click="reader.closeBook">→</button></div><strong>第{{ (reader.chapters.findIndex(c => c.id === reader.selectedChapter?.id) + 1) || 1 }}章 {{ reader.selectedChapter?.title || '' }}</strong><div class="reader-top-tools"><button @click="reader.fontSize = Math.max(14, reader.fontSize - 1)">Aa</button><span>⌄</span><button @click="reader.fontSize++">16</button><span>⌄</span><i></i><button>☼</button><button>☰</button><button>•••</button></div></div>
          <div class="reader-workspace">
            <ReaderPane :chapters="reader.chapters" :selected-chapter="reader.selectedChapter" :theme="reader.theme" :font-size="reader.fontSize" :line-height="reader.lineHeight" :page-margin="reader.pageMargin" :reader-mode="reader.readerMode" :loading="reader.loadingChapter" :book="reader.selectedBook" @select-chapter="reader.selectChapter" @scroll="reader.scheduleProgressSave" @reader-content="reader.readerContent = $event" @reader-mode="reader.readerMode = $event" />
            <aside class="settings-panel"><div class="settings-head"><strong>阅读设置</strong><button @click="reader.closeBook">×</button></div><label class="setting-label">主题</label><div class="theme-grid"><button v-for="themeOption in [['light','浅色'],['sepia','护眼'],['dark','深色'],['black','黑夜']]" :key="themeOption[0]" :class="['theme-card', { active: reader.theme === themeOption[0] }]" @click="reader.theme = themeOption[0]"><span :class="`theme-swatch ${themeOption[0]}`">▤</span><small>{{ themeOption[1] }}</small></button></div><label class="setting-label">字体</label><select v-model="reader.fontSize" class="setting-select"><option :value="17">思源宋体</option><option :value="18">霞鹜文楷</option></select><label class="setting-label">字号</label><div class="font-step"><button @click="reader.fontSize--">A-</button><span>{{ reader.fontSize }}</span><button @click="reader.fontSize++">A+</button></div><label class="setting-label">行距 <output>{{ reader.lineHeight.toFixed(1) }}</output></label><input v-model.number="reader.lineHeight" type="range" min="1.4" max="2.4" step="0.1" /><label class="setting-label">段距 <output>0.8</output></label><input type="range" min="0" max="2" step="0.1" value="0.8" /><label class="setting-label">字间距 <output>0</output></label><input type="range" min="0" max="2" step="0.1" value="0" /><label class="setting-label">页面宽度 <output>100%</output></label><input v-model.number="reader.pageMargin" type="range" min="16" max="64" step="4" /><label class="setting-label">阅读模式</label><div class="mode-switch"><button :class="{ active: reader.readerMode === 'paged' }" @click="reader.readerMode = 'paged'">◆ 翻页</button><button :class="{ active: reader.readerMode === 'scroll' }" @click="reader.readerMode = 'scroll'">滚动</button></div><div class="toggle-row"><span>自动亮度</span><button class="toggle"></button></div><div class="toggle-row"><span>全屏阅读</span><button class="toggle"></button></div><div class="toggle-row"><span>显示状态栏</span><button class="toggle on"></button></div><button class="reset-settings" @click="reader.theme = 'light'; reader.fontSize = 17; reader.lineHeight = 1.8; reader.pageMargin = 32">恢复默认</button></aside>
          </div>
        </template>
        <template v-else><div class="page-heading"><div><span>我的阅读空间</span><h1>{{ view === 'bookshelf' ? '书架' : view === 'settings' ? '设置' : view === 'explore' ? '发现' : view === 'search' ? '搜索书籍' : '书源调试' }}</h1></div><button v-if="view === 'bookshelf'" class="primary" @click="chooseFile">导入书籍</button></div><input ref="fileInput" class="visually-hidden" type="file" accept=".txt,.epub,text/plain,application/epub+zip" @change="bookshelf.handleFile" /><SettingsPage v-if="view === 'settings'" /><SourceDebugPage v-else-if="view === 'sources'" /><SearchPage v-else-if="view === 'search'" /><ExplorePage v-else-if="view === 'explore'" /><BookshelfPage v-else :books="bookshelf.visibleBooks" :groups="bookshelf.groups" @open="openBook" @move="bookshelf.moveBook" @remove="bookshelf.removeBook" @choose="chooseFile" /></template>
      </section>
    </section>
  </main>
</template>

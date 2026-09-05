<script setup lang="ts">
/* global HTMLInputElement */
import { provide, ref } from 'vue'
import BookshelfPage from '../features/bookshelf/BookshelfPage.vue'
import ReaderPane from '../features/reader/ReaderPane.vue'
import ExplorePage from '../features/search/ExplorePage.vue'
import SearchPage from '../features/search/SearchPage.vue'
import SettingsPage from '../features/settings/SettingsPage.vue'
import SourceDebugPage from '../features/source/SourceDebugPage.vue'
import { useAppShell } from './useAppShell'
import { searchKey, settingsKey, sourceDebugKey, sourcesKey } from './shellKeys'

const {
  appVersion,
  view,
  status,
  message,
  bookshelf,
  reader,
  search,
  settings,
  sources,
  sourceDebug,
  show,
  selectGroup,
  openBook,
} = useAppShell()

provide(searchKey, search)
provide(settingsKey, settings)
provide(sourceDebugKey, sourceDebug)
provide(sourcesKey, sources)

const fileInput = ref<HTMLInputElement>()
const chooseFile = () => fileInput.value?.click()
</script>

<template>
  <main class="shell">
    <header class="topbar">
      <div class="brand"><span class="brand-mark">R</span><div><strong>Reader</strong><small>桌面阅读工作台</small></div></div>
      <div class="global-search" @click="show('search')"><span>⌕</span><span>搜索书名、作者或书源</span><kbd>⌘ K</kbd></div>
      <div class="topbar-actions"><span class="sync-pill"><i class="dot" />{{ status }}</span><span class="version">v{{ appVersion }}</span><button class="avatar">R</button></div>
    </header>

    <section class="workspace">
      <aside class="sidebar">
        <nav aria-label="主导航">
          <p class="nav-label">阅读空间</p>
          <a
            :class="{ active: view === 'bookshelf' && bookshelf.activeGroup === null && !reader.selectedBook }"
            href="#"
            @click.prevent="show('bookshelf')"
          >
            全部书籍
          </a>
          <a
            v-for="group in bookshelf.groups"
            :key="group.id"
            :class="{ active: view === 'bookshelf' && bookshelf.activeGroup === group.id }"
            href="#"
            @click.prevent="selectGroup(group.id)"
          >
            {{ group.name }} <small>{{ group.book_count }}</small>
          </a>
          <a href="#" class="subtle-link" @click.prevent="bookshelf.addGroup()">＋ 新建分组</a>
          <p class="nav-label nav-label-spaced">书源中心</p>
          <a :class="{ active: view === 'search' }" href="#" @click.prevent="show('search')">在线搜索</a>
          <a :class="{ active: view === 'explore' }" href="#" @click.prevent="show('explore')">发现</a>
          <a :class="{ active: view === 'sources' }" href="#" @click.prevent="show('sources')">书源调试</a>
          <p class="nav-label nav-label-spaced">系统</p>
          <a :class="{ active: view === 'settings' }" href="#" @click.prevent="show('settings')">⚙ 设置</a>
        </nav>
        <div class="sidebar-footer"><span class="dot" />{{ status }}</div>
      </aside>

      <section class="content">
        <div v-if="!reader.selectedBook && view === 'bookshelf'" class="welcome-strip"><div><span class="eyebrow">今日阅读</span><h2>继续你的阅读旅程</h2><p>在书架中打开一本书，或从发现页添加新书。</p></div><span class="welcome-mark">✦</span></div>
        <div v-if="reader.selectedBook" class="content-heading">
          <div>
            <button type="button" class="back-button" @click="reader.closeBook()">返回书架</button>
            <p class="eyebrow">正在阅读</p>
            <h1>{{ reader.selectedBook.title }}</h1>
            <p
              v-if="reader.selectedBook.author || reader.selectedBook.kind || reader.selectedBook.latest_chapter"
              class="book-summary"
            >
              {{ reader.selectedBook.author || '作者未知' }}
              <span v-if="reader.selectedBook.kind"> · {{ reader.selectedBook.kind }}</span>
              <span v-if="reader.selectedBook.latest_chapter"> · 最新：{{ reader.selectedBook.latest_chapter }}</span>
            </p>
          </div>
          <button
            v-if="reader.selectedBook.source_id"
            type="button"
            class="secondary"
            :disabled="reader.refreshingCatalog"
            @click="reader.refreshCatalogForBook()"
          >
            {{ reader.refreshingCatalog ? '刷新中...' : '刷新目录' }}
          </button>
          <button
            v-if="reader.selectedBook.source_id"
            type="button"
            class="secondary"
            :disabled="reader.switchingSource"
            @click="reader.switchSource()"
          >
            {{ reader.switchingSource ? '换源中...' : '换源' }}
          </button>
          <div class="reader-settings">
            <label>
              主题
              <select v-model="reader.theme">
                <option value="light">浅色</option>
                <option value="sepia">护眼</option>
                <option value="dark">深色</option>
              </select>
            </label>
            <label>
              字号
              <input v-model.number="reader.fontSize" type="range" min="14" max="25" step="1" />
            </label>
            <label>
              行距
              <input v-model.number="reader.lineHeight" type="range" min="1.4" max="2.4" step="0.1" />
            </label>
            <label>
              页边距
              <input v-model.number="reader.pageMargin" type="range" min="16" max="64" step="4" />
            </label>
          </div>
        </div>

        <div v-else-if="view === 'settings'" class="content-heading">
          <div>
            <p class="eyebrow">应用设置</p>
            <h1>设置</h1>
          </div>
        </div>

        <div v-else-if="view === 'sources'" class="content-heading">
          <div>
            <p class="eyebrow">在线书源</p>
            <h1>书源调试</h1>
          </div>
        </div>

        <div v-else-if="view === 'search'" class="content-heading online-heading">
          <div>
            <p class="eyebrow">在线书源</p>
            <h1>搜索书籍</h1>
          </div>
          <form class="search-form" @submit.prevent="search.run()">
            <input v-model="search.query" placeholder="输入书名或作者" aria-label="搜索书名或作者" />
            <button type="submit" class="primary" :disabled="search.searching">
              {{ search.searching ? '搜索中...' : '搜索' }}
            </button>
          </form>
        </div>

        <div v-else-if="view === 'explore'" class="content-heading">
          <div>
            <p class="eyebrow">在线书源</p>
            <h1>发现</h1>
          </div>
        </div>

        <div v-else class="content-heading">
          <div>
            <p class="eyebrow">我的阅读空间</p>
            <h1>书架</h1>
          </div>
          <button type="button" class="primary" :disabled="bookshelf.importing" @click="chooseFile">
            {{ bookshelf.importing ? '导入中...' : '导入书籍' }}
          </button>
          <input
            ref="fileInput"
            class="visually-hidden"
            type="file"
            accept=".txt,.epub,text/plain,application/epub+zip"
            @change="bookshelf.handleFile"
          />
        </div>

        <div v-if="message" class="error-banner">{{ message }}</div>

        <ReaderPane
          v-if="reader.selectedBook"
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
        <SettingsPage v-else-if="view === 'settings'" />
        <SourceDebugPage v-else-if="view === 'sources'" />
        <SearchPage v-else-if="view === 'search'" />
        <ExplorePage v-else-if="view === 'explore'" />
        <BookshelfPage
          v-else
          :books="bookshelf.visibleBooks"
          :groups="bookshelf.groups"
          @open="openBook"
          @move="bookshelf.moveBook"
          @remove="bookshelf.removeBook"
          @choose="chooseFile"
        />
      </section>
    </section>
  </main>
</template>

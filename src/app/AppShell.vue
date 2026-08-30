<script setup lang="ts">
import ReaderPane from '../features/reader/ReaderPane.vue'
import SettingsPage from '../features/settings/SettingsPage.vue'
import BookshelfPage from '../features/bookshelf/BookshelfPage.vue'
import SearchPage from '../features/search/SearchPage.vue'
import { useAppShell } from './useAppShell'

const shell = useAppShell()
const {
  appVersion, status, error, groups, activeGroup, selectedBook, chapters, selectedChapter, loadingChapter,
  onlineSearch, settingsTab, proxyUrl, savingSettings, searchQuery, searchResults, searching, addingResult,
  sourceForm, savingSource, sourceUrl, importingSources, bookSources, testingSource, loginForm, loggingIn,
  readerContent, fontSize, theme, visibleBooks, saveSettings, runSearch, addSearchResult, addSource, loginSource,
  clearSourceSession, importSourceFile, importSourceFromUrl, testSource, showAllBooks, selectGroup, showSearch,
  showSettings, addGroup, removeBook, moveBook, chooseFile, handleFile, openBook, refreshCatalogForBook, selectChapter,
  closeBook, scheduleProgressSave, updateSourceForm, fileInput,
} = shell
</script>

<template>
  <main class="shell">
    <header class="topbar">
      <div class="brand"><span class="brand-mark">R</span><span>Reader</span></div>
      <div class="topbar-actions">
        <span class="version">v{{ appVersion }}</span>
        <button v-if="onlineSearch && bookSources.length" type="button" class="source-test topbar-test" :disabled="testingSource !== undefined" @click="testSource(bookSources[0])">
          {{ testingSource !== undefined ? '测试中...' : '测试书源' }}
        </button>
      </div>
    </header>
    <section class="workspace">
      <aside class="sidebar">
        <nav aria-label="主导航">
          <a :class="{ active: activeGroup === null && !onlineSearch && !settingsTab }" href="#" @click.prevent="showAllBooks">全部书籍</a>
          <a v-for="group in groups" :key="group.id" :class="{ active: activeGroup === group.id && !onlineSearch }" href="#" @click.prevent="selectGroup(group.id)">{{ group.name }} <small>{{ group.book_count }}</small></a>
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
          <div><button type="button" class="back-button" @click="closeBook">返回书架</button><p class="eyebrow">正在阅读</p><h1>{{ selectedBook.title }}</h1></div>
          <button v-if="selectedBook.source_id" type="button" class="secondary" @click="refreshCatalogForBook">刷新目录</button>
          <div class="reader-settings"><label>主题 <select v-model="theme"><option value="light">浅色</option><option value="sepia">护眼</option><option value="dark">深色</option></select></label><label>字号 <input v-model.number="fontSize" type="range" min="14" max="25" step="1" /></label></div>
        </div>
        <div v-else-if="settingsTab" class="content-heading"><div><p class="eyebrow">应用设置</p><h1>设置</h1></div></div>
        <div v-else-if="onlineSearch" class="content-heading online-heading">
          <div><p class="eyebrow">在线书源</p><h1>搜索书籍</h1></div>
          <form class="search-form" @submit.prevent="runSearch"><input v-model="searchQuery" placeholder="输入书名或作者" aria-label="搜索书名或作者" /><button type="submit" class="primary" :disabled="searching">{{ searching ? '搜索中...' : '搜索' }}</button></form>
        </div>
        <div v-else class="content-heading"><div><p class="eyebrow">我的阅读空间</p><h1>书架</h1></div><button type="button" class="primary" @click="chooseFile">导入书籍</button><input ref="fileInput" class="visually-hidden" type="file" accept=".txt,.epub,text/plain,application/epub+zip" @change="handleFile" /></div>
        <div v-if="error" class="error-banner">{{ error }}</div>
        <ReaderPane v-if="selectedBook" :chapters="chapters" :selected-chapter="selectedChapter" :theme="theme" :font-size="fontSize" :loading="loadingChapter" @select-chapter="selectChapter" @scroll="scheduleProgressSave" @reader-content="readerContent = $event" />
        <SettingsPage v-else-if="settingsTab" :proxy-url="proxyUrl" :saving="savingSettings" @update:proxy-url="proxyUrl = $event" @save="saveSettings" @clear="proxyUrl = ''" />
        <SearchPage v-else-if="onlineSearch" :query="searchQuery" :results="searchResults" :sources="bookSources" :searching="searching" :importing-sources="importingSources" :saving-source="savingSource" :adding-result="addingResult" :logging-in="loggingIn" :source-url="sourceUrl" :login-source-id="loginForm.sourceId" :username="loginForm.username" :password="loginForm.password" :source-form="sourceForm" @update:query="searchQuery = $event" @update:source-url="sourceUrl = $event" @update:login-source-id="loginForm.sourceId = $event" @update:username="loginForm.username = $event" @update:password="loginForm.password = $event" @update:source-form="updateSourceForm" @search="runSearch" @import-file="importSourceFile" @import-url="importSourceFromUrl" @save-source="addSource" @login="loginSource" @clear-session="clearSourceSession" @add="addSearchResult" />
        <BookshelfPage v-else :books="visibleBooks()" :groups="groups" @open="openBook" @move="moveBook" @remove="removeBook" @choose="chooseFile" />
      </section>
    </section>
  </main>
</template>

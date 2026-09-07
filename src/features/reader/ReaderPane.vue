<script setup lang="ts">
import type { Chapter } from '../../services/api'
import { useBookmark } from './useBookmark'
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { Bookmark, List } from 'lucide-vue-next'
import { captureReadingLocator, restoreReadingLocator } from './readerPosition'

const props = defineProps<{
  chapters: Chapter[]
  selectedChapter?: Chapter
  theme: string
  fontSize: number
  fontFamily: string
  lineHeight: number
  pageMargin: number
  readerMode: 'scroll' | 'paged'
  chapterListOpen: boolean
  loading: boolean
  book?: { id?: number; intro?: string; kind?: string; latest_chapter?: string; cover_data?: string }
}>()

const emit = defineEmits<{
  selectChapter: [chapter: Chapter]
  scroll: []
  readerContent: [element: HTMLElement | null]
  readerMode: [mode: 'scroll' | 'paged']
}>()

const contentRef = ref<HTMLElement | null>(null)
const activeTab = ref<'catalog' | 'bookmarks'>('catalog')
const searchQuery = ref('')
const {
  bookmark,
  busy: bookmarkBusy,
  error: bookmarkError,
  toggleBookmark,
  jumpToBookmark,
} = useBookmark(
  () => [props.book?.id, props.selectedChapter?.id],
  () => props.readerMode,
  contentRef,
  (mode) => emit('readerMode', mode),
)
const resolvedFontFamily = computed(() => {
  const stacks: Record<string, string> = {
    思源宋体: '"Source Han Serif SC", "Noto Serif CJK SC", "Noto Serif SC", serif',
    霞鹜文楷: '"LXGW WenKai", "Kaiti SC", "STKaiti", cursive',
    系统默认: 'system-ui, -apple-system, "Segoe UI", sans-serif',
  }
  return stacks[props.fontFamily] ?? stacks['系统默认']
})
const paragraphs = computed(() => (props.selectedChapter?.content ?? '').split(/\r?\n/))
// Keep the scroll-mode DOM stable. Replacing paragraphs while scrolling changes
// scrollHeight and makes the native scrollbar thumb lag or jump.
const visibleParagraphs = computed(() => paragraphs.value)
const searchMatches = computed(() => {
  const query = searchQuery.value.trim().toLocaleLowerCase()
  if (!query) return []
  return paragraphs.value.reduce<number[]>((matches, paragraph, index) => {
    if (paragraph.toLocaleLowerCase().includes(query)) matches.push(index)
    return matches
  }, [])
})

function onScroll() {
  emit('scroll')
}
function changeMode(mode: 'scroll' | 'paged') {
  const element = contentRef.value
  const locator = element ? captureReadingLocator(element, props.readerMode) : undefined
  emit('readerMode', mode)
  void nextTick(() => {
    if (!contentRef.value || !locator) return
    restoreReadingLocator(contentRef.value, mode, locator)
    emit('scroll')
  })
}
function turnPage(direction: number) {
  const element = contentRef.value
  if (!element || props.readerMode !== 'paged') return
  element.scrollBy({ left: direction * Math.max(1, element.clientWidth - 48), behavior: 'smooth' })
}
function onKeydown(event: KeyboardEvent) {
  if (props.readerMode !== 'paged') return
  if (event.key === 'ArrowRight' || event.key === 'PageDown') {
    event.preventDefault()
    turnPage(1)
  } else if (event.key === 'ArrowLeft' || event.key === 'PageUp') {
    event.preventDefault()
    turnPage(-1)
  }
}
onMounted(() => emit('readerContent', contentRef.value))
watch(contentRef, (element) => emit('readerContent', element))
watch(
  () => props.selectedChapter?.id,
  () => {
    void nextTick(() => {
      if (!contentRef.value) return
      contentRef.value.scrollTop = 0
      contentRef.value.scrollLeft = 0
    })
  },
)
watch(searchMatches, (matches) => {
  const index = matches[0]
  if (index === undefined || !contentRef.value || props.readerMode !== 'scroll') return
  const target = contentRef.value.querySelectorAll('.reader-paragraph')[index]
  target?.scrollIntoView({ block: 'start' })
})
onMounted(() => {
  window.addEventListener('keydown', onKeydown)
})
onBeforeUnmount(() => {
  window.removeEventListener('keydown', onKeydown)
})
</script>

<template>
  <div class="reader-layout">
    <aside v-if="chapterListOpen" class="chapter-list">
      <div class="reader-side-tabs">
        <button :class="{ active: activeTab === 'catalog' }" type="button" @click="activeTab = 'catalog'">
          <List :size="15" />目录</button
        ><button :class="{ active: activeTab === 'bookmarks' }" type="button" @click="activeTab = 'bookmarks'">
          <Bookmark :size="15" />书签
        </button>
      </div>
      <template v-if="activeTab === 'catalog'">
        <button
          v-for="chapter in chapters"
          :key="chapter.id"
          type="button"
          :class="['chapter-button', { selected: selectedChapter?.id === chapter.id }]"
          @click="emit('selectChapter', chapter)"
        >
          {{ chapter.title }}
        </button>
      </template>
      <div v-else class="bookmark-panel">
        <button v-if="bookmark" type="button" class="bookmark-item" @click="jumpToBookmark">
          <Bookmark :size="16" />{{ selectedChapter?.title }}<span>跳转</span>
        </button>
        <p v-else class="bookmark-empty">当前章节还没有书签。</p>
      </div>
    </aside>
    <article
      v-if="selectedChapter"
      ref="contentRef"
      :class="['reader-content', `theme-${theme}`, `mode-${readerMode}`]"
      :style="{
        '--reader-font-size': `${fontSize}px`,
        '--reader-font-family': resolvedFontFamily,
        '--reader-line-height': lineHeight,
        '--reader-margin': `${pageMargin}px`,
      }"
      tabindex="0"
      @scroll="onScroll"
    >
      <div class="reader-toolbar">
        <div class="reader-mode-toggle" role="group" aria-label="阅读模式">
          <button type="button" :class="{ active: readerMode === 'scroll' }" @click="changeMode('scroll')">滚动</button>
          <button type="button" :class="{ active: readerMode === 'paged' }" @click="changeMode('paged')">分页</button>
        </div>
        <div v-if="readerMode === 'paged'" class="reader-page-actions">
          <button type="button" aria-label="上一页" @click="turnPage(-1)">上一页</button>
          <button type="button" aria-label="下一页" @click="turnPage(1)">下一页</button>
        </div>
        <div class="reader-tools">
          <input v-model="searchQuery" type="search" placeholder="搜索本章" aria-label="搜索本章" />
          <span v-if="searchQuery.trim()" class="reader-search-count">{{ searchMatches.length }} 处</span>
          <button
            type="button"
            :aria-pressed="Boolean(bookmark)"
            :disabled="bookmarkBusy || loading"
            @click="toggleBookmark"
          >
            {{ bookmark ? '移除书签' : '添加书签' }}
          </button>
        </div>
      </div>
      <p v-if="bookmarkError" role="alert">{{ bookmarkError }}</p>
      <button v-if="bookmark" type="button" :disabled="bookmarkBusy || loading" @click="jumpToBookmark">
        跳到书签
      </button>
      <h2 class="reader-title">{{ selectedChapter.title }}</h2>
      <p v-if="book?.intro" class="book-intro">{{ book.intro }}</p>
      <p v-if="loading" class="reader-loading">正在获取正文...</p>
      <template v-else>
        <p
          v-for="(paragraph, index) in visibleParagraphs"
          :key="index"
          class="reader-paragraph"
          :data-reader-index="index"
        >
          {{ paragraph }}
        </p>
      </template>
    </article>
  </div>
</template>

<script setup lang="ts">
import type { Chapter } from '../../services/api'
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue'

const props = defineProps<{
  chapters: Chapter[]
  selectedChapter?: Chapter
  theme: string
  fontSize: number
  lineHeight: number
  pageMargin: number
  readerMode: 'scroll' | 'paged'
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
const searchQuery = ref('')
const bookmarked = ref(false)
const bookmarkOffset = ref(0)
const scrollTop = ref(0)
const viewportHeight = ref(640)
const estimatedParagraphHeight = computed(() => Math.max(32, props.fontSize * props.lineHeight * 1.8))
const paragraphs = computed(() => (props.selectedChapter?.content ?? '').split(/\r?\n/))
const windowStart = computed(() => {
  if (props.readerMode === 'paged') return 0
  return Math.max(0, Math.floor(scrollTop.value / estimatedParagraphHeight.value) - 8)
})
const windowEnd = computed(() => {
  if (props.readerMode === 'paged') return paragraphs.value.length
  const visible = Math.ceil(viewportHeight.value / estimatedParagraphHeight.value) + 16
  return Math.min(paragraphs.value.length, windowStart.value + visible)
})
const visibleParagraphs = computed(() => paragraphs.value.slice(windowStart.value, windowEnd.value))
const searchMatches = computed(() => {
  const query = searchQuery.value.trim().toLocaleLowerCase()
  if (!query) return []
  return paragraphs.value.reduce<number[]>((matches, paragraph, index) => {
    if (paragraph.toLocaleLowerCase().includes(query)) matches.push(index)
    return matches
  }, [])
})
const topSpacer = computed(() => windowStart.value * estimatedParagraphHeight.value)
const bottomSpacer = computed(
  () => Math.max(0, paragraphs.value.length - windowEnd.value) * estimatedParagraphHeight.value,
)

function updateViewport() {
  if (contentRef.value) viewportHeight.value = contentRef.value.clientHeight
}
function onScroll() {
  scrollTop.value = contentRef.value?.scrollTop ?? 0
  emit('scroll')
}
function changeMode(mode: 'scroll' | 'paged') {
  const element = contentRef.value
  const currentExtent =
    props.readerMode === 'paged'
      ? element && element.scrollWidth > element.clientWidth
        ? element.scrollWidth - element.clientWidth
        : 0
      : element && element.scrollHeight > element.clientHeight
        ? element.scrollHeight - element.clientHeight
        : 0
  const ratio = currentExtent
    ? (props.readerMode === 'paged' ? element!.scrollLeft : element!.scrollTop) / currentExtent
    : 0
  emit('readerMode', mode)
  void nextTick(() => {
    updateViewport()
    if (!contentRef.value || !ratio) return
    if (mode === 'paged') {
      contentRef.value.scrollLeft = ratio * Math.max(0, contentRef.value.scrollWidth - contentRef.value.clientWidth)
    } else {
      contentRef.value.scrollTop = ratio * Math.max(0, contentRef.value.scrollHeight - contentRef.value.clientHeight)
    }
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
function bookmarkStorageKey() {
  return props.book?.id && props.selectedChapter?.id
    ? `reader-bookmark:${props.book.id}:${props.selectedChapter.id}`
    : ''
}
function loadBookmark() {
  const key = bookmarkStorageKey()
  const value = key ? localStorage.getItem(key) : null
  bookmarked.value = Boolean(value)
  bookmarkOffset.value = 0
  if (!value) return
  try {
    bookmarkOffset.value = Number(JSON.parse(value).offset ?? 0)
  } catch {
    // Backward-compatible with the original boolean bookmark marker.
  }
}
function toggleBookmark() {
  const key = bookmarkStorageKey()
  if (!key) return
  bookmarked.value = !bookmarked.value
  if (bookmarked.value) {
    bookmarkOffset.value =
      props.readerMode === 'paged' ? (contentRef.value?.scrollLeft ?? 0) : (contentRef.value?.scrollTop ?? 0)
    localStorage.setItem(key, JSON.stringify({ offset: Math.round(bookmarkOffset.value) }))
  } else localStorage.removeItem(key)
}
onMounted(() => emit('readerContent', contentRef.value))
watch(contentRef, (element) => emit('readerContent', element))
watch(
  () => props.selectedChapter?.id,
  () => {
    scrollTop.value = 0
    loadBookmark()
    void nextTick(() => {
      updateViewport()
      if (!contentRef.value) return
      contentRef.value.scrollTop = 0
      contentRef.value.scrollLeft = 0
      if (bookmarkOffset.value) {
        if (props.readerMode === 'paged') contentRef.value.scrollLeft = bookmarkOffset.value
        else contentRef.value.scrollTop = bookmarkOffset.value
      }
    })
  },
)
watch(
  () => [props.book?.id, props.selectedChapter?.id],
  () => {
    loadBookmark()
  },
  { immediate: true },
)
watch(searchMatches, (matches) => {
  const index = matches[0]
  if (index === undefined || !contentRef.value || props.readerMode !== 'scroll') return
  contentRef.value.scrollTop = index * estimatedParagraphHeight.value
})
onMounted(() => {
  updateViewport()
  window.addEventListener('resize', updateViewport)
  window.addEventListener('keydown', onKeydown)
})
onBeforeUnmount(() => {
  window.removeEventListener('resize', updateViewport)
  window.removeEventListener('keydown', onKeydown)
})
</script>

<template>
  <div class="reader-layout">
    <aside class="chapter-list">
      <h2>目录</h2>
      <button
        v-for="chapter in chapters"
        :key="chapter.id"
        type="button"
        :class="['chapter-button', { selected: selectedChapter?.id === chapter.id }]"
        @click="emit('selectChapter', chapter)"
      >
        {{ chapter.title }}
      </button>
    </aside>
    <article
      v-if="selectedChapter"
      ref="contentRef"
      :class="['reader-content', `theme-${theme}`, `mode-${readerMode}`]"
      :style="{
        '--reader-font-size': `${fontSize}px`,
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
          <button type="button" :aria-pressed="bookmarked" @click="toggleBookmark">
            {{ bookmarked ? '已书签' : '书签' }}
          </button>
        </div>
      </div>
      <h2 class="reader-title">{{ selectedChapter.title }}</h2>
      <p v-if="book?.intro" class="book-intro">{{ book.intro }}</p>
      <p v-if="loading" class="reader-loading">正在获取正文...</p>
      <template v-else>
        <div v-if="readerMode === 'scroll'" :style="{ height: `${topSpacer}px` }" aria-hidden="true" />
        <p v-for="(paragraph, index) in visibleParagraphs" :key="windowStart + index">
          {{ paragraph }}
        </p>
        <div v-if="readerMode === 'scroll'" :style="{ height: `${bottomSpacer}px` }" aria-hidden="true" />
      </template>
    </article>
  </div>
</template>

<script setup lang="ts">
import type { Chapter } from '../../services/api'
import { useBookmark } from './useBookmark'
import { computed, nextTick, onMounted, ref, watch } from 'vue'
import ReaderCatalog from './ReaderCatalog.vue'
import ReaderContentToolbar from './ReaderContentToolbar.vue'
import { useReaderPaging } from './useReaderPaging'

const props = defineProps<{
  chapters: Chapter[]
  selectedChapter?: Chapter
  theme: string
  fontSize: number
  fontFamily: string
  lineHeight: number
  pageMargin: number
  paragraphSpacing: number
  textIndent: number
  justify: boolean
  pageAnimation: string
  brightness: number
  eyeCare: boolean
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
const searchMatches = computed(() => {
  const query = searchQuery.value.trim().toLocaleLowerCase()
  if (!query) return []
  return paragraphs.value.reduce<number[]>((matches, paragraph, index) => {
    if (paragraph.toLocaleLowerCase().includes(query)) matches.push(index)
    return matches
  }, [])
})

const { turnPage, advance, changeMode } = useReaderPaging(
  contentRef,
  () => props.readerMode,
  (mode) => emit('readerMode', mode),
  () => emit('scroll'),
)

onMounted(() => {
  emit('readerContent', contentRef.value)
})
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

/** The bottom toolbar owns the bookmark button, so the state lives here. */
defineExpose({ bookmark, bookmarkBusy, toggleBookmark, jumpToBookmark, turnPage, advance })
</script>

<template>
  <div class="reader-layout">
    <ReaderCatalog
      v-if="chapterListOpen"
      :chapters="chapters"
      :selected-chapter="selectedChapter"
      :has-bookmark="Boolean(bookmark)"
      @select-chapter="emit('selectChapter', $event)"
      @jump="jumpToBookmark"
    />

    <article
      v-if="selectedChapter"
      ref="contentRef"
      :class="[
        'reader-content',
        `theme-${theme}`,
        `mode-${readerMode}`,
        `anim-${pageAnimation}`,
        { 'justify-text': justify, 'eye-care': eyeCare },
      ]"
      :style="{
        '--reader-font-size': `${fontSize}px`,
        '--reader-font-family': resolvedFontFamily,
        '--reader-line-height': lineHeight,
        '--reader-margin': `${pageMargin}px`,
        '--reader-paragraph-spacing': `${paragraphSpacing}em`,
        '--reader-text-indent': `${textIndent}em`,
        '--reader-brightness': brightness,
      }"
      tabindex="0"
      @scroll="emit('scroll')"
    >
      <ReaderContentToolbar
        v-model="searchQuery"
        :reader-mode="readerMode"
        :has-bookmark="Boolean(bookmark)"
        :busy="bookmarkBusy"
        :loading="loading"
        :matches="searchMatches.length"
        @mode="changeMode"
        @turn="turnPage"
        @toggle-bookmark="toggleBookmark"
      />
      <p v-if="bookmarkError" role="alert" class="text-destructive">{{ bookmarkError }}</p>
      <button v-if="bookmark" type="button" :disabled="bookmarkBusy || loading" @click="jumpToBookmark">
        跳到书签
      </button>
      <h2 class="reader-title">{{ selectedChapter.title }}</h2>
      <p v-if="book?.intro" class="book-intro">{{ book.intro }}</p>
      <p v-if="loading" class="reader-loading">正在获取正文…</p>
      <div v-else class="reader-flow">
        <p v-for="(paragraph, index) in paragraphs" :key="index" class="reader-paragraph" :data-reader-index="index">
          {{ paragraph }}
        </p>
      </div>
    </article>
  </div>
</template>

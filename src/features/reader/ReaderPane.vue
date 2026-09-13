<script setup lang="ts">
import { computed, nextTick, onMounted, ref, watch } from 'vue'
import { useShellContext } from '@/app/shellKeys'
import type { Chapter } from '../../services/api'
import { useBookmark } from './useBookmark'
import { useContentHighlight } from './useContentHighlight'
import { useReaderPaging } from './useReaderPaging'
import { isDialogueLine, splitParagraphs, volumeLabel } from './readerTypography'
import ReaderCatalog from './ReaderCatalog.vue'
import ReaderHighlight from './ReaderHighlight.vue'
import ReaderPager from './ReaderPager.vue'

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
  /** 阅读器只需要书籍 id（书签读写）；书籍信息由 `ReaderBookPanel` 展示。 */
  book?: { id?: number }
}>()

const emit = defineEmits<{
  selectChapter: [chapter: Chapter]
  scroll: []
  readerContent: [element: HTMLElement | null]
  readerMode: [mode: 'scroll' | 'paged']
}>()

const { reader } = useShellContext()
const contentRef = ref<HTMLElement | null>(null)
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
const paragraphs = computed(() => splitParagraphs(props.selectedChapter?.content ?? ''))
/** Prototype :851–868 — first paragraph is the drop cap; quoted lines keep the shared first-line indent. */
function paragraphClass(index: number) {
  return { lead: index === 0, 'dialog-line': isDialogueLine(paragraphs.value[index] ?? '') }
}

const { turnPage, advance, spreadEligible, pageIndex, pageCount, chapterPercent } = useReaderPaging(
  contentRef,
  () => props.readerMode,
  (mode) => emit('readerMode', mode),
  () => emit('scroll'),
)

/** 正文搜索 lands through here: switch chapter, mark the hit, scroll to it. */
const {
  highlight: highlightRange,
  titleRange,
  paragraphRange,
  busy: highlightBusy,
  jumpToHit,
} = useContentHighlight(
  contentRef,
  () => props.selectedChapter?.id,
  // `paged` only scrolls sideways in the two-column spread; below the prototype's
  // breakpoints that same mode falls back to a vertical single column.
  () => (props.readerMode === 'paged' && spreadEligible.value ? 'paged' : 'scroll'),
  async (chapterId) => {
    const chapter = props.chapters.find((item) => item.id === chapterId)
    if (chapter) await reader.selectChapter(chapter)
  },
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
      // A jump owns the reading position: it is either still running or has
      // already marked its hit. Only a plain chapter change rewinds to the top.
      if (highlightBusy.value || highlightRange.value) return
      contentRef.value.scrollTop = 0
      contentRef.value.scrollLeft = 0
    })
  },
)

/** The bottom toolbar owns the bookmark button, so the state lives here. */
defineExpose({ bookmark, bookmarkBusy, toggleBookmark, jumpToBookmark, turnPage, advance, jumpToHit })
</script>

<template>
  <div class="reader-layout">
    <ReaderCatalog
      v-if="chapterListOpen"
      :chapters="chapters"
      :selected-chapter="selectedChapter"
      :has-bookmark="Boolean(bookmark)"
      :read-up-to-chapter-id="reader.lastReadChapterId"
      :current-percent="chapterPercent"
      @select-chapter="emit('selectChapter', $event)"
      @jump="jumpToBookmark"
    />

    <div class="reader-stage">
      <article
        v-if="selectedChapter"
        ref="contentRef"
        :class="[
          'reader-content',
          `theme-${theme}`,
          `mode-${readerMode}`,
          `anim-${pageAnimation}`,
          { 'justify-text': justify, 'eye-care': eyeCare, 'is-narrow': readerMode === 'paged' && !spreadEligible },
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
      >
        <p v-if="bookmarkError" role="alert" class="text-destructive">{{ bookmarkError }}</p>
        <button v-if="bookmark" type="button" :disabled="bookmarkBusy || loading" @click="jumpToBookmark">
          跳到书签
        </button>
        <div class="reader-flow">
          <div class="chapter-eyebrow">{{ volumeLabel(selectedChapter.title) }}</div>
          <h2 class="reader-title">
            <ReaderHighlight :text="selectedChapter.title" :offset="titleRange?.offset" :length="titleRange?.length" />
          </h2>
          <p v-if="loading" class="reader-loading">正在获取正文…</p>
          <template v-else>
            <p
              v-for="(paragraph, index) in paragraphs"
              :key="index"
              :class="['reader-paragraph', paragraphClass(index)]"
              :data-reader-index="index"
            >
              <ReaderHighlight
                :text="paragraph"
                :offset="paragraphRange(index)?.offset"
                :length="paragraphRange(index)?.length"
              />
            </p>
          </template>
        </div>
      </article>

      <ReaderPager
        v-if="readerMode === 'paged' && spreadEligible"
        :page-index="pageIndex"
        :page-count="pageCount"
        :chapter-percent="chapterPercent"
        @turn="turnPage"
      />
      <p v-else-if="readerMode === 'paged'" class="reader-pager-hint">
        窗口高 &lt; 820px 或正文宽 &lt; 760px，按原型退回单栏滚动。
      </p>
    </div>
  </div>
</template>

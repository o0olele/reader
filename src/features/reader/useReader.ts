/* global HTMLElement, localStorage, setTimeout, clearTimeout, setInterval, clearInterval, document */
import { nextTick, reactive, ref, watch } from 'vue'
import {
  readChapter,
  addReadingTime,
  getReadingRecord,
  fetchBookInfo,
  getReadingProgress,
  listChapters,
  refreshCatalog,
  saveReadingProgress,
  searchBooks,
  switchBookSource,
  type Book,
  type Chapter,
} from '../../services/api'
import { captureReadingLocator, restoreReadingLocator } from './readerPosition'

const PROGRESS_DEBOUNCE_MS = 350
const READING_TIME_TICK_MS = 15_000

function chapterKey(title: string): string {
  return title
    .toLocaleLowerCase()
    .replace(/^第[0-9零一二三四五六七八九十百千万两]+[章节卷回集部篇]\s*/u, '')
    .replace(/[\s\p{P}\p{S}]/gu, '')
}

/** Owns the open book: its catalog, the current chapter and reading progress. */
export function useReader(report: (cause: unknown) => void) {
  const selectedBook = ref<Book>()
  const chapters = ref<Chapter[]>([])
  const selectedChapter = ref<Chapter>()
  const readingSeconds = ref(0)
  const loadingChapter = ref(false)
  const refreshingCatalog = ref(false)
  const switchingSource = ref(false)
  const readerContent = ref<HTMLElement | null>(null)
  const fontSize = ref(Number(localStorage.getItem('reader-font-size') ?? '17'))
  const storedFontFamily = localStorage.getItem('reader-font-family')
  const fontFamily = ref(
    storedFontFamily === '思源宋体' || storedFontFamily === '霞鹜文楷' || storedFontFamily === '系统默认'
      ? storedFontFamily
      : '思源宋体',
  )
  const theme = ref(localStorage.getItem('reader-theme') ?? 'light')
  const lineHeight = ref(Number(localStorage.getItem('reader-line-height') ?? '1.8'))
  const pageMargin = ref(Number(localStorage.getItem('reader-page-margin') ?? '32'))
  const readerMode = ref<'scroll' | 'paged'>((localStorage.getItem('reader-mode') as 'scroll' | 'paged') ?? 'scroll')

  let saveTimer: ReturnType<typeof setTimeout> | undefined
  let readingTimer: ReturnType<typeof setInterval> | undefined
  let readingTickAt = 0
  let chapterRequest = 0

  async function flushReadingTime() {
    const book = selectedBook.value
    if (!book || !readingTickAt) return
    if (document.visibilityState !== 'visible') {
      readingTickAt = Date.now()
      return
    }
    const elapsed = Math.floor((Date.now() - readingTickAt) / 1000)
    if (elapsed <= 0) return
    readingTickAt += elapsed * 1000
    try {
      await addReadingTime(book.id, elapsed)
      readingSeconds.value += elapsed
    } catch (cause) {
      readingTickAt -= elapsed * 1000
      report(cause)
    }
  }

  function startReadingTimer() {
    if (readingTimer) clearInterval(readingTimer)
    readingTickAt = Date.now()
    readingTimer = setInterval(() => void flushReadingTime(), READING_TIME_TICK_MS)
  }

  async function stopReadingTimer() {
    if (readingTimer) clearInterval(readingTimer)
    readingTimer = undefined
    await flushReadingTime()
    readingTickAt = 0
  }

  watch(fontSize, (value) => localStorage.setItem('reader-font-size', String(value)))
  watch(fontFamily, (value) => localStorage.setItem('reader-font-family', value))
  watch(theme, (value) => localStorage.setItem('reader-theme', value))
  watch(lineHeight, (value) => localStorage.setItem('reader-line-height', String(value)))
  watch(pageMargin, (value) => localStorage.setItem('reader-page-margin', String(value)))
  watch(readerMode, (value) => localStorage.setItem('reader-mode', value))
  watch([fontSize, fontFamily, lineHeight, pageMargin], async () => {
    const element = readerContent.value
    if (!element || !selectedChapter.value) return
    const locator = captureReadingLocator(element, readerMode.value)
    await nextTick()
    restoreReadingLocator(element, readerMode.value, locator)
    scheduleProgressSave()
  })

  const isOnline = (book: Book) => book.source_id !== undefined && book.source_id !== null

  async function openBook(book: Book) {
    await stopReadingTimer()
    selectedBook.value = book
    startReadingTimer()
    try {
      readingSeconds.value = (await getReadingRecord(book.id).catch(() => null))?.duration_seconds ?? 0
      if (isOnline(book) && !book.intro && !book.cover_data) {
        selectedBook.value = await fetchBookInfo(book.id).catch(() => book)
      }
      chapters.value = await listChapters(book.id)
      // `list_chapters` is deliberately local-only, so a book just added from a
      // search has no catalog yet. Fetch it once instead of showing an empty list.
      if (!chapters.value.length && isOnline(book)) {
        await loadCatalog()
      }
      const progress = await getReadingProgress(book.id).catch(() => null)
      selectedChapter.value = chapters.value.find((chapter) => chapter.id === progress?.chapter_id) ?? chapters.value[0]
      await loadChapterContent(selectedChapter.value)
      await nextTick()
      if (readerContent.value && progress && selectedChapter.value?.id === progress.chapter_id) {
        if (progress.anchor_index >= 0) {
          restoreReadingLocator(readerContent.value, readerMode.value, {
            index: progress.anchor_index,
            ratio: progress.anchor_ratio,
          })
        } else if (readerMode.value === 'paged') readerContent.value.scrollLeft = progress.offset
        else readerContent.value.scrollTop = progress.offset
      }
    } catch (cause) {
      report(cause)
    }
  }

  async function loadCatalog() {
    if (!selectedBook.value) return
    refreshingCatalog.value = true
    try {
      chapters.value = await refreshCatalog(selectedBook.value.id)
    } finally {
      refreshingCatalog.value = false
    }
  }

  async function refreshCatalogForBook() {
    if (!selectedBook.value) return
    try {
      await loadCatalog()
      selectedChapter.value =
        chapters.value.find((chapter) => chapter.id === selectedChapter.value?.id) ?? chapters.value[0]
    } catch (cause) {
      report(cause)
    }
  }

  async function switchSource() {
    const book = selectedBook.value
    if (!book?.source_id) return
    const previousChapter = selectedChapter.value
    const previousLocator = readerContent.value
      ? captureReadingLocator(readerContent.value, readerMode.value)
      : undefined
    switchingSource.value = true
    try {
      const response = await searchBooks(book.title)
      const group = response.groups.find((item) => item.title.replace(/\s/g, '') === book.title.replace(/\s/g, ''))
      const alternative = group?.sources.find((source) => source.source_id !== book.source_id)
      if (!alternative) throw new Error('没有找到其他可用书源')
      selectedBook.value = await switchBookSource(book.id, alternative)
      chapters.value = []
      selectedChapter.value = undefined
      await loadCatalog()
      const previousKey = previousChapter ? chapterKey(previousChapter.title) : ''
      selectedChapter.value =
        chapters.value.find((chapter) => previousKey && chapterKey(chapter.title) === previousKey) ??
        chapters.value.find((chapter) => chapter.number === previousChapter?.number) ??
        chapters.value[0]
      await loadChapterContent(selectedChapter.value)
      await nextTick()
      if (readerContent.value && previousLocator) {
        restoreReadingLocator(readerContent.value, readerMode.value, previousLocator)
        scheduleProgressSave()
      }
    } catch (cause) {
      report(cause)
    } finally {
      switchingSource.value = false
    }
  }

  // Keep an open reader in sync when another surface refreshes this book's catalog.
  async function handleCatalogUpdated(payload?: unknown) {
    const bookId = (payload as { book_id?: unknown } | undefined)?.book_id
    if (!selectedBook.value || (typeof bookId === 'number' && bookId !== selectedBook.value.id)) return
    const selectedId = selectedChapter.value?.id
    chapters.value = await listChapters(selectedBook.value.id)
    selectedChapter.value = chapters.value.find((chapter) => chapter.id === selectedId) ?? chapters.value[0]
  }

  async function loadChapterContent(chapter?: Chapter) {
    if (!chapter) return
    const request = ++chapterRequest
    loadingChapter.value = true
    try {
      const processed = await readChapter(chapter.id)
      if (request === chapterRequest && selectedChapter.value?.id === chapter.id) selectedChapter.value = processed
    } catch (cause) {
      if (request === chapterRequest) report(cause)
    } finally {
      if (request === chapterRequest) loadingChapter.value = false
    }
  }

  function scheduleProgressSave() {
    if (!selectedBook.value || !selectedChapter.value) return
    if (saveTimer) clearTimeout(saveTimer)
    saveTimer = setTimeout(() => {
      if (selectedBook.value && selectedChapter.value && readerContent.value) {
        const offset = readerMode.value === 'paged' ? readerContent.value.scrollLeft : readerContent.value.scrollTop
        const locator = captureReadingLocator(readerContent.value, readerMode.value)
        void saveReadingProgress(
          selectedBook.value.id,
          selectedChapter.value.id,
          Math.round(offset),
          locator.index,
          locator.ratio,
        )
      }
    }, PROGRESS_DEBOUNCE_MS)
  }

  async function selectChapter(chapter: Chapter) {
    selectedChapter.value = chapter
    await loadChapterContent(chapter)
    await nextTick()
    scheduleProgressSave()
  }

  async function closeBook() {
    chapterRequest++
    loadingChapter.value = false
    scheduleProgressSave()
    await stopReadingTimer()
    // Let the debounced write land before the refs it reads are cleared.
    if (saveTimer) await new Promise<void>((resolve) => setTimeout(resolve, PROGRESS_DEBOUNCE_MS + 30))
    selectedBook.value = undefined
    readingSeconds.value = 0
    selectedChapter.value = undefined
    chapters.value = []
  }

  return reactive({
    selectedBook,
    chapters,
    selectedChapter,
    readingSeconds,
    loadingChapter,
    refreshingCatalog,
    switchingSource,
    readerContent,
    fontSize,
    fontFamily,
    theme,
    lineHeight,
    pageMargin,
    readerMode,
    openBook,
    refreshCatalogForBook,
    switchSource,
    handleCatalogUpdated,
    selectChapter,
    closeBook,
    scheduleProgressSave,
    flushReadingTime,
  })
}

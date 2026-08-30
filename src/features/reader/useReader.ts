/* global HTMLElement, localStorage, setTimeout, clearTimeout */
import { nextTick, reactive, ref, watch } from 'vue'
import {
  fetchOnlineContent,
  getReadingProgress,
  listChapters,
  refreshCatalog,
  saveReadingProgress,
  type Book,
  type Chapter,
} from '../../services/api'

const PROGRESS_DEBOUNCE_MS = 350

/** Owns the open book: its catalog, the current chapter and reading progress. */
export function useReader(report: (cause: unknown) => void) {
  const selectedBook = ref<Book>()
  const chapters = ref<Chapter[]>([])
  const selectedChapter = ref<Chapter>()
  const loadingChapter = ref(false)
  const refreshingCatalog = ref(false)
  const readerContent = ref<HTMLElement | null>(null)
  const fontSize = ref(Number(localStorage.getItem('reader-font-size') ?? '17'))
  const theme = ref(localStorage.getItem('reader-theme') ?? 'light')

  let saveTimer: ReturnType<typeof setTimeout> | undefined

  watch(fontSize, (value) => localStorage.setItem('reader-font-size', String(value)))
  watch(theme, (value) => localStorage.setItem('reader-theme', value))

  const isOnline = (book: Book) => book.source_id !== undefined && book.source_id !== null

  async function openBook(book: Book) {
    selectedBook.value = book
    try {
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
        readerContent.value.scrollTop = progress.offset
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

  async function loadChapterContent(chapter?: Chapter) {
    const book = selectedBook.value
    if (!chapter || chapter.content || !book?.source_id || !chapter.remote_url) return
    loadingChapter.value = true
    try {
      chapter.content = await fetchOnlineContent(book.source_id, chapter.remote_url, chapter.id)
    } catch (cause) {
      report(cause)
    } finally {
      loadingChapter.value = false
    }
  }

  function scheduleProgressSave() {
    if (!selectedBook.value || !selectedChapter.value) return
    if (saveTimer) clearTimeout(saveTimer)
    saveTimer = setTimeout(() => {
      if (selectedBook.value && selectedChapter.value && readerContent.value) {
        void saveReadingProgress(selectedBook.value.id, selectedChapter.value.id, readerContent.value.scrollTop)
      }
    }, PROGRESS_DEBOUNCE_MS)
  }

  async function selectChapter(chapter: Chapter) {
    selectedChapter.value = chapter
    await loadChapterContent(chapter)
    await nextTick()
    if (readerContent.value) readerContent.value.scrollTop = 0
    scheduleProgressSave()
  }

  async function closeBook() {
    scheduleProgressSave()
    // Let the debounced write land before the refs it reads are cleared.
    if (saveTimer) await new Promise<void>((resolve) => setTimeout(resolve, PROGRESS_DEBOUNCE_MS + 30))
    selectedBook.value = undefined
    selectedChapter.value = undefined
    chapters.value = []
  }

  return reactive({
    selectedBook,
    chapters,
    selectedChapter,
    loadingChapter,
    refreshingCatalog,
    readerContent,
    fontSize,
    theme,
    openBook,
    refreshCatalogForBook,
    selectChapter,
    closeBook,
    scheduleProgressSave,
  })
}

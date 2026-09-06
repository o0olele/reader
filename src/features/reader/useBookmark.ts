/* global HTMLElement, localStorage */
import { nextTick, onBeforeUnmount, ref, watch, type Ref } from 'vue'
import { deleteBookmark, getBookmark, getErrorMessage, saveBookmark, type Bookmark } from '../../services/api'

type Mode = 'scroll' | 'paged'

export function useBookmark(
  identity: () => readonly [number | undefined, number | undefined],
  mode: () => Mode,
  element: Ref<HTMLElement | null>,
  setMode: (mode: Mode) => void,
) {
  const bookmark = ref<Bookmark | null>(null)
  const busy = ref(false)
  const error = ref('')
  let generation = 0

  watch(
    identity,
    async ([bookId, chapterId]) => {
      const request = ++generation
      bookmark.value = null
      error.value = ''
      busy.value = true
      try {
        if (bookId === undefined || chapterId === undefined) return
        let saved = await getBookmark(bookId, chapterId)
        // Remove the old record only after SQLite confirms it has been preserved.
        const key = `reader-bookmark:${bookId}:${chapterId}`
        const legacy = localStorage.getItem(key)
        if (!saved && legacy) {
          let offset = 0
          try {
            const parsed: unknown = JSON.parse(legacy)
            if (parsed && typeof parsed === 'object' && 'offset' in parsed) offset = Number(parsed.offset)
          } catch {
            /* Older boolean markers represent the chapter start. */
          }
          if (!Number.isSafeInteger(offset) || offset < 0) offset = 0
          const legacyMode = localStorage.getItem('reader-mode') === 'paged' ? 'paged' : 'scroll'
          saved = { book_id: bookId, chapter_id: chapterId, offset, mode: legacyMode }
          await saveBookmark(bookId, chapterId, offset, legacyMode)
        }
        if (legacy) localStorage.removeItem(key)
        if (request === generation) bookmark.value = saved
      } catch (cause) {
        if (request === generation) error.value = getErrorMessage(cause)
      } finally {
        if (request === generation) busy.value = false
      }
    },
    { immediate: true },
  )

  async function toggleBookmark() {
    const [bookId, chapterId] = identity()
    if (busy.value || bookId === undefined || chapterId === undefined || !element.value) return
    const request = generation
    busy.value = true
    error.value = ''
    try {
      if (bookmark.value) {
        await deleteBookmark(bookId, chapterId)
        if (request === generation) bookmark.value = null
      } else {
        const offset = Math.max(0, Math.round(mode() === 'paged' ? element.value.scrollLeft : element.value.scrollTop))
        const saved: Bookmark = { book_id: bookId, chapter_id: chapterId, offset, mode: mode() }
        await saveBookmark(bookId, chapterId, offset, saved.mode)
        if (request === generation) bookmark.value = saved
      }
    } catch (cause) {
      if (request === generation) error.value = getErrorMessage(cause)
    } finally {
      if (request === generation) busy.value = false
    }
  }

  async function jumpToBookmark() {
    const saved = bookmark.value
    if (!saved || busy.value) return
    const request = generation
    setMode(saved.mode)
    await nextTick()
    if (!element.value || request !== generation) return
    if (saved.mode === 'paged') element.value.scrollLeft = saved.offset
    else element.value.scrollTop = saved.offset
  }

  onBeforeUnmount(() => {
    generation++
  })
  return { bookmark, busy, error, toggleBookmark, jumpToBookmark }
}

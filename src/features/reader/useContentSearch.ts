import { computed, onBeforeUnmount, reactive, ref, watch } from 'vue'
import {
  cancelBookContentSearch,
  getErrorMessage,
  searchBookContent,
  type SearchContentHit,
  type SearchContentProgress,
} from '@/services/api'
import { on as onAppEvent } from '@/services/events'

/** Typing pause before a scan starts — a scan walks the whole book. */
const DEBOUNCE_MS = 350

export type SearchScope = 'book' | 'chapter'

/**
 * Owns one full-text search: the query, the streamed progress and the hits.
 *
 * A scan is debounced and cancellable. Every run bumps `generation`, so a slow
 * scan that resolves after a newer query is dropped instead of overwriting the
 * results the user is looking at — the backend drops it for the same reason
 * (`cancel_search_content` invalidates its generation).
 */
export function useContentSearch(bookId: () => number | undefined, currentChapterId: () => number | undefined) {
  const query = ref('')
  const regex = ref(false)
  const scope = ref<SearchScope>('book')
  const hits = ref<SearchContentHit[]>([])
  const searching = ref(false)
  const progress = ref<SearchContentProgress>()
  const searchedChapters = ref(0)
  const totalChapters = ref(0)
  const truncated = ref(false)
  const error = ref('')
  let generation = 0
  let timer: ReturnType<typeof setTimeout> | undefined

  const visibleHits = computed(() =>
    scope.value === 'chapter' && currentChapterId() !== undefined
      ? hits.value.filter((hit) => hit.chapter_id === currentChapterId())
      : hits.value,
  )
  /** How many hits the open chapter holds — shown even while the scope is 全书. */
  const currentChapterHits = computed(() =>
    currentChapterId() === undefined ? 0 : hits.value.filter((hit) => hit.chapter_id === currentChapterId()).length,
  )

  const stopProgress = onAppEvent('search-content-progress', (payload) => {
    const update = payload as SearchContentProgress | undefined
    if (!update || update.book_id !== bookId()) return
    progress.value = update
  })
  onBeforeUnmount(() => {
    stopProgress()
    if (timer) clearTimeout(timer)
    stop()
  })

  function reset() {
    hits.value = []
    progress.value = undefined
    searchedChapters.value = 0
    totalChapters.value = 0
    truncated.value = false
    error.value = ''
  }

  /** Abandons the scan in flight and drops whatever it still returns. */
  function stop() {
    const id = bookId()
    generation += 1
    searching.value = false
    if (id !== undefined) void cancelBookContentSearch(id).catch(() => undefined)
  }

  async function run() {
    const id = bookId()
    const text = query.value.trim()
    stop()
    if (id === undefined) return
    reset()
    if (!text) return
    const request = generation
    searching.value = true
    try {
      const response = await searchBookContent(id, text, regex.value)
      if (request !== generation) return
      hits.value = response.hits
      searchedChapters.value = response.searched_chapters
      totalChapters.value = response.total_chapters
      truncated.value = response.truncated
    } catch (cause) {
      if (request === generation) error.value = getErrorMessage(cause)
    } finally {
      if (request === generation) {
        searching.value = false
        progress.value = undefined
      }
    }
  }

  function schedule() {
    if (timer) clearTimeout(timer)
    // Clearing the box drops the results at once instead of after the pause.
    if (!query.value.trim()) {
      stop()
      reset()
      return
    }
    timer = setTimeout(() => void run(), DEBOUNCE_MS)
  }

  watch([query, regex], schedule)
  // Another book means another scan: same query, different chapters.
  watch(bookId, () => void run())

  return reactive({
    query,
    regex,
    scope,
    hits,
    visibleHits,
    currentChapterHits,
    searching,
    progress,
    searchedChapters,
    totalChapters,
    truncated,
    error,
    run,
    stop,
  })
}

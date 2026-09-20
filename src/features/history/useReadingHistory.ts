import { computed, reactive, ref } from 'vue'
import {
  clearAllReadingHistory,
  clearReadingHistory,
  getErrorMessage,
  listReadingHistory,
  type ReadingHistoryEntry,
} from '../../services/api'
import { historyBucket, type HistoryBucket } from '../../lib/readingHistory'

const BUCKETS: HistoryBucket[] = ['今天', '昨天', '更早']

/**
 * Owns the cross-book 历史 list backed by `list_reading_history` (ROADMAP-v3 E1).
 * Both clears write through to the backend first and only then drop the local
 * row: the page never shows a state the database does not agree with.
 */
export function useReadingHistory() {
  const items = ref<ReadingHistoryEntry[]>([])
  const loading = ref(false)
  const error = ref('')
  const keyword = ref('')

  const visible = computed(() => {
    const needle = keyword.value.trim().toLowerCase()
    if (!needle) return items.value
    return items.value.filter((entry) =>
      `${entry.book_title} ${entry.book_author ?? ''} ${entry.chapter_title ?? ''}`.toLowerCase().includes(needle),
    )
  })

  /** Visible rows by local day, newest first; empty sections are dropped. */
  const groups = computed(() => {
    const now = new Date()
    return BUCKETS.map((label) => ({
      label,
      entries: visible.value.filter((entry) => historyBucket(entry.last_read_at, now) === label),
    })).filter((group) => group.entries.length > 0)
  })

  async function refresh() {
    loading.value = true
    error.value = ''
    try {
      items.value = await listReadingHistory()
    } catch (cause) {
      // Browser preview has no Tauri backend; say so instead of faking a list.
      error.value = getErrorMessage(cause)
      items.value = []
    } finally {
      loading.value = false
    }
  }

  async function remove(entry: ReadingHistoryEntry) {
    try {
      await clearReadingHistory(entry.book_id)
      items.value = items.value.filter((item) => item.book_id !== entry.book_id)
    } catch (cause) {
      error.value = getErrorMessage(cause)
    }
  }

  async function clearAll() {
    try {
      await clearAllReadingHistory()
      items.value = []
    } catch (cause) {
      error.value = getErrorMessage(cause)
    }
  }

  return reactive({ items, loading, error, keyword, visible, groups, refresh, remove, clearAll })
}

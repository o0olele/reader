import { computed, reactive, ref } from 'vue'
import { deleteBookmark, getErrorMessage, listBookmarks, type BookmarkEntry } from '../../services/api'

/**
 * Owns the cross-book 书签 list backed by `list_bookmarks` (ROADMAP-v3 E1).
 * Removing a bookmark writes through immediately: the page is the only surface
 * that can drop a bookmark from outside the reader.
 */
export function useBookmarks() {
  const items = ref<BookmarkEntry[]>([])
  const loading = ref(false)
  const error = ref('')
  const keyword = ref('')

  const visible = computed(() => {
    const needle = keyword.value.trim().toLowerCase()
    if (!needle) return items.value
    return items.value.filter((entry) =>
      `${entry.book_title} ${entry.book_author ?? ''} ${entry.chapter_title}`.toLowerCase().includes(needle),
    )
  })

  async function refresh() {
    loading.value = true
    error.value = ''
    try {
      items.value = await listBookmarks()
    } catch (cause) {
      // Browser preview has no Tauri backend; say so instead of faking a list.
      error.value = getErrorMessage(cause)
      items.value = []
    } finally {
      loading.value = false
    }
  }

  async function remove(entry: BookmarkEntry) {
    try {
      await deleteBookmark(entry.book_id, entry.chapter_id)
      items.value = items.value.filter((item) => item.chapter_id !== entry.chapter_id)
    } catch (cause) {
      error.value = getErrorMessage(cause)
    }
  }

  return reactive({ items, loading, error, keyword, visible, refresh, remove })
}

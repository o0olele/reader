import { computed, reactive, ref } from 'vue'
import {
  createGroup,
  deleteBook,
  importLocalBook,
  listBooks,
  listGroups,
  moveBookToGroup,
  type Book,
  type BookshelfGroup,
} from '../../services/api'

/** Owns the shelf: the book list, groups, and local file import. */
export function useBookshelf(report: (cause: unknown) => void) {
  const books = ref<Book[]>([])
  const groups = ref<BookshelfGroup[]>([])
  const activeGroup = ref<number | null>(null)
  const importing = ref(false)

  const visibleBooks = computed(() =>
    activeGroup.value === null ? books.value : books.value.filter((book) => book.group_id === activeGroup.value),
  )

  async function refresh() {
    // Browser preview has no Tauri backend; an empty shelf is the right fallback.
    try {
      books.value = await listBooks()
    } catch {
      /* preview mode */
    }
    try {
      groups.value = await listGroups()
    } catch {
      /* preview mode */
    }
  }

  function upsert(book: Book) {
    books.value = [book, ...books.value.filter((item) => item.id !== book.id)]
  }

  async function handleFile(event: Event) {
    const input = event.target as HTMLInputElement
    const file = input.files?.[0]
    if (!file) return
    importing.value = true
    try {
      const bytes = await file.arrayBuffer()
      upsert(await importLocalBook(file.name, bytes))
    } catch (cause) {
      report(cause)
    } finally {
      importing.value = false
      input.value = ''
    }
  }

  /** Creates a group; the caller collects the name through its own dialog. */
  async function addGroup(name: string) {
    const trimmed = name.trim()
    if (!trimmed) return false
    try {
      groups.value.push(await createGroup(trimmed))
      return true
    } catch (cause) {
      report(cause)
      return false
    }
  }

  async function removeBook(book: Book) {
    if (!window.confirm(`确定删除《${book.title}》吗？`)) return
    try {
      await deleteBook(book.id)
      books.value = books.value.filter((item) => item.id !== book.id)
      await refresh()
    } catch (cause) {
      report(cause)
    }
  }

  /** Moves books into one group and re-reads the shelf; returns how many moved. */
  async function moveBooks(bookIds: number[], groupId: number) {
    let moved = 0
    for (const bookId of bookIds) {
      try {
        await moveBookToGroup(bookId, groupId)
        moved += 1
      } catch (cause) {
        report(cause)
        break
      }
    }
    if (moved) await refresh()
    return moved
  }

  return reactive({
    books,
    groups,
    activeGroup,
    importing,
    visibleBooks,
    refresh,
    upsert,
    handleFile,
    addGroup,
    removeBook,
    moveBooks,
  })
}

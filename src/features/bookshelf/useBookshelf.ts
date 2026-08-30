/* global HTMLInputElement, HTMLSelectElement, Event */
import { computed, reactive, ref } from 'vue'
import {
  createGroup,
  deleteBook,
  importEpubBook,
  importTxtBook,
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
      const bytes = Array.from(new Uint8Array(await file.arrayBuffer()))
      const isEpub = file.name.toLowerCase().endsWith('.epub')
      upsert(isEpub ? await importEpubBook(file.name, bytes) : await importTxtBook(file.name, bytes))
    } catch (cause) {
      report(cause)
    } finally {
      importing.value = false
      input.value = ''
    }
  }

  async function addGroup() {
    const name = window.prompt('新分组名称')?.trim()
    if (!name) return
    try {
      groups.value.push(await createGroup(name))
    } catch (cause) {
      report(cause)
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

  async function moveBook(book: Book, event: Event) {
    const groupId = Number((event.target as HTMLSelectElement).value)
    try {
      await moveBookToGroup(book.id, groupId)
      book.group_id = groupId
      await refresh()
    } catch (cause) {
      report(cause)
    }
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
    moveBook,
  })
}

import type { Book } from '@/services/api'

export type SortKey = 'updated' | 'title' | 'author' | 'chapters'
export type FilterKey = 'all' | 'local' | 'online'
export type ViewMode = 'grid' | 'list'

export interface ShelfViewOptions {
  keyword: string
  sort: SortKey
  filter: FilterKey
}

/** Applies the shelf keyword/source filters and returns a newly ordered copy. */
export function arrangeBooks(books: Book[], options: ShelfViewOptions): Book[] {
  const needle = options.keyword.trim().toLowerCase()
  let list = books.filter((book) =>
    needle ? `${book.title} ${book.author ?? ''}`.toLowerCase().includes(needle) : true,
  )
  if (options.filter === 'local') list = list.filter((book) => !book.source_id)
  if (options.filter === 'online') list = list.filter((book) => Boolean(book.source_id))

  const sorted = [...list]
  if (options.sort === 'title') sorted.sort((a, b) => a.title.localeCompare(b.title, 'zh-Hans-CN'))
  else if (options.sort === 'author')
    sorted.sort((a, b) => (a.author ?? '').localeCompare(b.author ?? '', 'zh-Hans-CN'))
  else if (options.sort === 'chapters') sorted.sort((a, b) => b.chapter_count - a.chapter_count)
  else sorted.sort((a, b) => b.updated_at.localeCompare(a.updated_at))
  return sorted
}

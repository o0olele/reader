import { watch } from 'vue'
import { useRoute } from 'vue-router'
import { useShellContext } from '@/app/shellKeys'
import type { ReaderLocation } from './useReader'

function parseMode(value: unknown): ReaderLocation['mode'] {
  return value === 'paged' || value === 'scroll' ? value : undefined
}

/**
 * The reader is deep-linkable: `#/read/<bookId>?chapter=<id>&offset=<px>&mode=`
 * opens the book on a chapter, and the offset restores an exact bookmark
 * position. Route state is the only input, so the 书签 page jumps here by
 * pushing a route rather than reaching into reader state.
 */
export function useReaderDeepLink() {
  const route = useRoute()
  const { reader, bookshelf, openBook } = useShellContext()

  watch(
    [
      () => route.params.bookId,
      () => route.query.chapter,
      () => route.query.offset,
      () => route.query.mode,
      () => bookshelf.books.length,
    ],
    async () => {
      const bookId = Number(route.params.bookId)
      if (!bookId) return
      // `bookshelf.books` loads asynchronously; the dependency on its length
      // re-runs this watch once the shelf is there.
      const book = bookshelf.books.find((item) => item.id === bookId)
      if (!book) return
      const chapterId = Number(route.query.chapter)
      const location: ReaderLocation | undefined =
        Number.isSafeInteger(chapterId) && chapterId > 0
          ? {
              chapterId,
              offset: Math.max(0, Number(route.query.offset) || 0),
              mode: parseMode(route.query.mode),
            }
          : undefined
      if (reader.selectedBook?.id === bookId) {
        if (!location || (await reader.focusLocation(location))) return
      }
      await openBook(book, location)
    },
    { immediate: true },
  )
}

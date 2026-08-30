import { reactive, ref } from 'vue'
import {
  addOnlineBook,
  searchBooks,
  type Book,
  type BookSearchResult,
  type SearchResultGroup,
  type SourceFailure,
} from '../../services/api'

/** Owns online search: the query, grouped results and per-source failures. */
export function useSearch(report: (cause: unknown) => void, onAdded: (book: Book) => void) {
  const query = ref('')
  const groups = ref<SearchResultGroup[]>([])
  const failures = ref<SourceFailure[]>([])
  const searchedSources = ref(0)
  const searching = ref(false)
  const hasSearched = ref(false)
  const addingResult = ref<string>()

  async function run() {
    if (!query.value.trim()) return
    searching.value = true
    try {
      const response = await searchBooks(query.value)
      groups.value = response.groups
      failures.value = response.failures
      searchedSources.value = response.searched_sources
      hasSearched.value = true
    } catch (cause) {
      // Only pre-flight problems reject now: an empty query, or no enabled sources.
      groups.value = []
      failures.value = []
      report(cause)
    } finally {
      searching.value = false
    }
  }

  async function addToShelf(result: BookSearchResult) {
    addingResult.value = result.url
    try {
      onAdded(await addOnlineBook(result))
    } catch (cause) {
      report(cause)
    } finally {
      addingResult.value = undefined
    }
  }

  return reactive({
    query,
    groups,
    failures,
    searchedSources,
    searching,
    hasSearched,
    addingResult,
    run,
    addToShelf,
  })
}

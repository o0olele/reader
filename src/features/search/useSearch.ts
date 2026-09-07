import { reactive, ref } from 'vue'
import {
  addOnlineBook,
  exploreBooks,
  listExploreCategories,
  searchBooks,
  type Book,
  type BookSearchResult,
  type SearchResultGroup,
  type SourceFailure,
  type ExploreCategory,
} from '../../services/api'

/** Owns online search: the query, grouped results and per-source failures. */
export function useSearch(report: (cause: unknown) => void, onAdded: (book: Book) => void) {
  const query = ref('')
  const groups = ref<SearchResultGroup[]>([])
  const failures = ref<SourceFailure[]>([])
  const searchedSources = ref(0)
  const searching = ref(false)
  const paused = ref(false)
  const completedSources = ref(0)
  const totalSources = ref(0)
  const selectedSourceIds = ref<number[] | null>(null)
  let runToken = 0
  let resumeResolvers: Array<() => void> = []
  const hasSearched = ref(false)
  const addingResult = ref<string>()
  const exploreCategories = ref<ExploreCategory[]>([])
  const exploring = ref(false)
  const exploreResults = ref<BookSearchResult[]>([])
  const selectedExplore = ref<ExploreCategory>()

  function sourceIsSelected(sourceId: number, _enabledIds: number[]) {
    return selectedSourceIds.value === null || selectedSourceIds.value.includes(sourceId)
  }

  function toggleSource(sourceId: number, enabledIds: number[]) {
    const current =
      selectedSourceIds.value === null ? enabledIds.filter((id) => id !== sourceId) : [...selectedSourceIds.value]
    if (selectedSourceIds.value !== null) {
      if (current.includes(sourceId)) current.splice(current.indexOf(sourceId), 1)
      else current.push(sourceId)
    }
    selectedSourceIds.value = current.length === enabledIds.length ? null : current
  }

  function toggleAllSources() {
    selectedSourceIds.value = selectedSourceIds.value === null ? [] : null
  }

  function resumeWait() {
    if (!paused.value) return Promise.resolve()
    return new Promise<void>((resolve) => resumeResolvers.push(resolve))
  }

  function pause() {
    if (searching.value) paused.value = true
  }

  function resume() {
    paused.value = false
    const resolvers = resumeResolvers.splice(0)
    resolvers.forEach((resolve) => resolve())
  }

  function mergeGroups(incoming: SearchResultGroup[]) {
    const normalize = (value: string) => value.replace(/\s+/g, '').toLowerCase()
    for (const next of incoming) {
      const existing = groups.value.find(
        (group) =>
          normalize(group.title) === normalize(next.title) &&
          normalize(group.author ?? '') === normalize(next.author ?? ''),
      )
      if (!existing) groups.value.push(next)
      else {
        if (!existing.cover && next.cover) existing.cover = next.cover
        for (const source of next.sources) {
          if (!existing.sources.some((item) => item.source_id === source.source_id && item.url === source.url)) {
            existing.sources.push(source)
          }
        }
      }
    }
  }

  async function run() {
    if (!query.value.trim()) return
    const token = ++runToken
    const keyword = query.value.trim()
    resume()
    searching.value = true
    groups.value = []
    failures.value = []
    completedSources.value = 0
    searchedSources.value = 0
    hasSearched.value = false
    // SearchPage supplies the enabled/filtered source ids through this hook.
    const availableIds = sourceIdsProvider?.() ?? []
    const ids = availableIds.filter((id) => sourceIsSelected(id, availableIds))
    totalSources.value = ids.length
    if (!ids.length && selectedSourceIds.value === null) {
      try {
        const response = await searchBooks(keyword)
        if (token === runToken) {
          mergeGroups(response.groups)
          failures.value = response.failures
          searchedSources.value = response.searched_sources
          completedSources.value = response.searched_sources
          totalSources.value = response.searched_sources
        }
      } catch (cause) {
        if (token === runToken) report(cause)
      } finally {
        if (token === runToken) {
          searching.value = false
          hasSearched.value = true
        }
      }
      return
    }
    if (!ids.length) {
      searching.value = false
      hasSearched.value = true
      return
    }
    let cursor = 0
    const worker = async () => {
      while (token === runToken) {
        await resumeWait()
        if (token !== runToken) return
        const sourceId = ids[cursor++]
        if (sourceId === undefined) return
        try {
          const response = await searchBooks(keyword, sourceId)
          if (token !== runToken) return
          mergeGroups(response.groups)
          failures.value.push(...response.failures)
        } catch (cause) {
          if (token === runToken) report(cause)
        } finally {
          if (token === runToken) {
            completedSources.value += 1
            searchedSources.value = completedSources.value
          }
        }
      }
    }
    await Promise.all(Array.from({ length: Math.min(8, ids.length) }, worker))
    if (token === runToken) {
      searching.value = false
      paused.value = false
      hasSearched.value = true
    }
  }

  let sourceIdsProvider: (() => number[]) | undefined
  function setSourceIdsProvider(provider: () => number[]) {
    sourceIdsProvider = provider
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

  async function loadExplore() {
    try {
      exploreCategories.value = await listExploreCategories()
    } catch (cause) {
      report(cause)
    }
  }

  async function runExplore(category: ExploreCategory) {
    selectedExplore.value = category
    exploring.value = true
    try {
      exploreResults.value = await exploreBooks(category.source_id, category.url)
    } catch (cause) {
      exploreResults.value = []
      report(cause)
    } finally {
      exploring.value = false
    }
  }

  return reactive({
    query,
    groups,
    failures,
    searchedSources,
    searching,
    paused,
    completedSources,
    totalSources,
    hasSearched,
    selectedSourceIds,
    sourceIsSelected,
    toggleSource,
    toggleAllSources,
    pause,
    resume,
    setSourceIdsProvider,
    addingResult,
    run,
    addToShelf,
    exploreCategories,
    exploring,
    exploreResults,
    selectedExplore,
    loadExplore,
    runExplore,
  })
}

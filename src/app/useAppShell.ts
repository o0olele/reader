import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import packageJson from '../../package.json'
import { getErrorMessage, healthCheck, type Book } from '../services/api'
import { useBookshelf } from '../features/bookshelf/useBookshelf'
import { useReader } from '../features/reader/useReader'
import { useSearch } from '../features/search/useSearch'
import { useSettings } from '../features/settings/useSettings'
import { useSources } from '../features/source/useSources'
import { on as onAppEvent } from '../services/events'

export type ShellView = 'bookshelf' | 'search' | 'settings'

/**
 * Composes the feature composables and owns only what is genuinely shared:
 * the active view, the status line and the one-line message banner.
 */
export function useAppShell() {
  const route = useRoute()
  const router = useRouter()
  const view = computed<ShellView>(() => {
    if (route.name === 'search' || route.name === 'settings') return route.name
    return 'bookshelf'
  })
  const status = ref('检查中...')
  const message = ref('')

  const notify = (text: string) => (message.value = text)
  const report = (cause: unknown) => (message.value = getErrorMessage(cause))

  const bookshelf = useBookshelf(report)
  const reader = useReader(report)
  const settings = useSettings(report, notify)
  const sources = useSources(report, notify)
  const search = useSearch(report, (book) => bookshelf.upsert(book))

  function show(next: ShellView) {
    message.value = ''
    if (next !== 'bookshelf') reader.selectedBook = undefined
    if (next === 'bookshelf') bookshelf.activeGroup = null
    void router.push({ name: next })
  }

  function selectGroup(groupId: number) {
    message.value = ''
    bookshelf.activeGroup = groupId
    void router.push({ name: 'bookshelf' })
  }

  async function openBook(book: Book) {
    message.value = ''
    await reader.openBook(book)
  }

  async function refreshAll() {
    await Promise.all([bookshelf.refresh(), sources.refresh(), settings.refresh()])
  }

  let stopCatalogUpdates: (() => void) | undefined

  onMounted(() => {
    void refreshAll()
    stopCatalogUpdates = onAppEvent('chapter-updated', (payload) => {
      void reader.handleCatalogUpdated(payload).catch(report)
    })
    healthCheck()
      .then((value) => (status.value = value))
      .catch(() => (status.value = '前端预览模式'))
  })

  // A pending debounced progress write would otherwise be lost on teardown.
  onBeforeUnmount(() => {
    stopCatalogUpdates?.()
    reader.scheduleProgressSave()
  })

  return {
    appVersion: packageJson.version,
    view,
    status,
    message,
    bookshelf,
    reader,
    search,
    settings,
    sources,
    show,
    selectGroup,
    openBook,
  }
}

import { onBeforeUnmount, onMounted, provide, ref } from 'vue'
import packageJson from '../../package.json'
import { getErrorMessage, healthCheck, type Book } from '../services/api'
import { useBookshelf } from '../features/bookshelf/useBookshelf'
import { useReader } from '../features/reader/useReader'
import { useSearch } from '../features/search/useSearch'
import { useSettings } from '../features/settings/useSettings'
import { useSourceDebug } from '../features/source/useSourceDebug'
import { useSources } from '../features/source/useSources'
import { on as onAppEvent } from '../services/events'
import { notifyError, notifyInfo } from './useToast'
import { searchKey, settingsKey, shellKey, sourceDebugKey, sourcesKey, type ShellContext } from './shellKeys'

/**
 * Composition root: builds every feature composable once, provides them to the
 * routed pages, and wires the app-wide events. It owns no view state — the URL
 * is the single source of truth for navigation (ROADMAP-v3 F0).
 */
export function useShell(): ShellContext {
  const status = ref('检查中...')
  const report = (cause: unknown) => notifyError(getErrorMessage(cause))
  const notify = (text: string) => notifyInfo(text)

  const bookshelf = useBookshelf(report)
  const reader = useReader(report)
  const settings = useSettings(report, notify)
  const sources = useSources(report, notify)
  const sourceDebug = useSourceDebug(report, notify, sources)
  const search = useSearch(report, (book) => bookshelf.upsert(book))

  const openBook = (book: Book) => reader.openBook(book)

  const context: ShellContext = {
    appVersion: packageJson.version,
    status,
    bookshelf,
    reader,
    search,
    settings,
    sources,
    sourceDebug,
    openBook,
  }

  provide(searchKey, search)
  provide(settingsKey, settings)
  provide(sourceDebugKey, sourceDebug)
  provide(sourcesKey, sources)
  provide(shellKey, context)

  let stopCatalogUpdates: (() => void) | undefined

  onMounted(() => {
    void Promise.all([bookshelf.refresh(), sources.refresh(), settings.refresh()])
      .then(() => search.loadExplore())
      .catch(report)
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
    void reader.closeBook()
  })

  return context
}

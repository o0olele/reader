import { onBeforeUnmount, onMounted, ref } from 'vue'
import packageJson from '../../package.json'
import { getErrorMessage, healthCheck, type Book } from '../services/api'
import { useBookshelf } from '../features/bookshelf/useBookshelf'
import { useReader } from '../features/reader/useReader'
import { useSearch } from '../features/search/useSearch'
import { useSettings } from '../features/settings/useSettings'
import { useSources } from '../features/source/useSources'

export type ShellView = 'bookshelf' | 'search' | 'settings'

/**
 * Composes the feature composables and owns only what is genuinely shared:
 * the active view, the status line and the one-line message banner.
 */
export function useAppShell() {
  const view = ref<ShellView>('bookshelf')
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
    view.value = next
    message.value = ''
    if (next !== 'bookshelf') reader.selectedBook = undefined
    if (next === 'bookshelf') bookshelf.activeGroup = null
  }

  function selectGroup(groupId: number) {
    view.value = 'bookshelf'
    message.value = ''
    bookshelf.activeGroup = groupId
  }

  async function openBook(book: Book) {
    message.value = ''
    await reader.openBook(book)
  }

  async function refreshAll() {
    await Promise.all([bookshelf.refresh(), sources.refresh(), settings.refresh()])
  }

  onMounted(() => {
    void refreshAll()
    healthCheck()
      .then((value) => (status.value = value))
      .catch(() => (status.value = '前端预览模式'))
  })

  // A pending debounced progress write would otherwise be lost on teardown.
  onBeforeUnmount(() => reader.scheduleProgressSave())

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

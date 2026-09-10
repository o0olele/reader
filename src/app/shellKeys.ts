import type { InjectionKey, Ref } from 'vue'
import { inject } from 'vue'
import type { Book } from '../services/api'
import type { useBookshelf } from '../features/bookshelf/useBookshelf'
import type { useReader } from '../features/reader/useReader'
import type { useSearch } from '../features/search/useSearch'
import type { useSettings } from '../features/settings/useSettings'
import type { useSourceDebug } from '../features/source/useSourceDebug'
import type { useSources } from '../features/source/useSources'

/**
 * Feature state that its page owns two-way. Provided by `AppShell` rather than
 * passed as props, so pages can bind with `v-model` without mutating props.
 */
export const searchKey: InjectionKey<ReturnType<typeof useSearch>> = Symbol('search')
export const settingsKey: InjectionKey<ReturnType<typeof useSettings>> = Symbol('settings')
export const sourceDebugKey: InjectionKey<ReturnType<typeof useSourceDebug>> = Symbol('sourceDebug')
export const sourcesKey: InjectionKey<ReturnType<typeof useSources>> = Symbol('sources')

/** Everything the shell owns on behalf of the routed pages. */
export interface ShellContext {
  appVersion: string
  status: Ref<string>
  bookshelf: ReturnType<typeof useBookshelf>
  reader: ReturnType<typeof useReader>
  search: ReturnType<typeof useSearch>
  settings: ReturnType<typeof useSettings>
  sources: ReturnType<typeof useSources>
  sourceDebug: ReturnType<typeof useSourceDebug>
  openBook: (book: Book) => Promise<void>
}

export const shellKey: InjectionKey<ShellContext> = Symbol('shell')

export function useShellContext(): ShellContext {
  const context = inject(shellKey)
  if (!context) throw new Error('useShellContext() must be called inside AppShell')
  return context
}

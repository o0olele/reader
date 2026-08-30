import type { InjectionKey } from 'vue'
import type { useSearch } from '../features/search/useSearch'
import type { useSettings } from '../features/settings/useSettings'
import type { useSources } from '../features/source/useSources'

/**
 * Feature state that its page owns two-way. Provided by `AppShell` rather than
 * passed as props, so the pages can bind with `v-model` without mutating props.
 */
export const searchKey: InjectionKey<ReturnType<typeof useSearch>> = Symbol('search')
export const settingsKey: InjectionKey<ReturnType<typeof useSettings>> = Symbol('settings')
export const sourcesKey: InjectionKey<ReturnType<typeof useSources>> = Symbol('sources')

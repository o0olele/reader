import { invoke } from '@tauri-apps/api/core'
import type { BookSearchResult, ExploreCategory, SearchResponse } from './types'

/**
 * Searches every enabled source. Resolves even when some sources fail; check
 * `failures` on the response rather than relying on a rejection.
 */
export function searchBooks(query: string, sourceId?: number): Promise<SearchResponse> {
  return invoke<SearchResponse>('search_books', { query, sourceId })
}

export function listExploreCategories(): Promise<ExploreCategory[]> {
  return invoke<ExploreCategory[]>('list_explore_categories')
}

export function exploreBooks(sourceId: number, url: string): Promise<BookSearchResult[]> {
  return invoke<BookSearchResult[]>('explore_books', { sourceId, url })
}

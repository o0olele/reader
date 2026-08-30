import { invoke } from '@tauri-apps/api/core'
import type { Chapter, ReadingProgress } from './types'

export function listChapters(bookId: number): Promise<Chapter[]> {
  return invoke<Chapter[]>('list_chapters', { bookId })
}

/** Re-fetches the catalog from the book's source and returns the merged list. */
export function refreshCatalog(bookId: number): Promise<Chapter[]> {
  return invoke<Chapter[]>('refresh_catalog', { bookId })
}

export function fetchOnlineContent(sourceId: number, chapterUrl: string, chapterId?: number): Promise<string> {
  return invoke<string>('fetch_online_content', { sourceId, chapterUrl, chapterId })
}

export function getReadingProgress(bookId: number): Promise<ReadingProgress | null> {
  return invoke<ReadingProgress | null>('get_reading_progress', { bookId })
}

export function saveReadingProgress(bookId: number, chapterId: number, offset: number): Promise<void> {
  return invoke<void>('save_reading_progress', { bookId, chapterId, offset })
}

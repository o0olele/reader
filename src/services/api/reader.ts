import { invoke } from '@tauri-apps/api/core'
import type { Chapter, ReadingProgress, ReadingRecord } from './types'

export interface Bookmark {
  book_id: number
  chapter_id: number
  offset: number
  mode: 'scroll' | 'paged'
}

export function getBookmark(bookId: number, chapterId: number): Promise<Bookmark | null> {
  return invoke('get_bookmark', { bookId, chapterId })
}

export function saveBookmark(
  bookId: number,
  chapterId: number,
  offset: number,
  mode: 'scroll' | 'paged',
): Promise<void> {
  return invoke('save_bookmark', { bookId, chapterId, offset, mode })
}

export function deleteBookmark(bookId: number, chapterId: number): Promise<void> {
  return invoke('delete_bookmark', { bookId, chapterId })
}

export function listChapters(bookId: number): Promise<Chapter[]> {
  return invoke<Chapter[]>('list_chapters', { bookId })
}

export function readChapter(chapterId: number): Promise<Chapter> {
  return invoke('read_chapter', { chapterId })
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

export function saveReadingProgress(
  bookId: number,
  chapterId: number,
  offset: number,
  anchorIndex?: number,
  anchorRatio?: number,
): Promise<void> {
  return invoke<void>('save_reading_progress', { bookId, chapterId, offset, anchorIndex, anchorRatio })
}

export function getReadingRecord(bookId: number): Promise<ReadingRecord | null> {
  return invoke<ReadingRecord | null>('get_reading_record', { bookId })
}

export function addReadingTime(bookId: number, durationSeconds: number): Promise<void> {
  return invoke<void>('add_reading_time', { bookId, durationSeconds })
}

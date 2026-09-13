import { invoke } from '@tauri-apps/api/core'
import type { Chapter, ReadingProgress, ReadingRecord, ReadingStats, SearchContentResponse } from './types'

export interface Bookmark {
  book_id: number
  chapter_id: number
  offset: number
  mode: 'scroll' | 'paged'
}

/** A bookmark joined with the book and chapter it points at (书签 page). */
export interface BookmarkEntry extends Bookmark {
  book_title: string
  book_author: string | null
  chapter_title: string
  chapter_number: number
  updated_at: string
}

/** Every bookmark in the library, newest first (`list_bookmarks`). */
export function listBookmarks(): Promise<BookmarkEntry[]> {
  return invoke<BookmarkEntry[]>('list_bookmarks')
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

/**
 * Warms the chapters around the open one, the way the reference app's
 * `ReadBook.preDownload()` does. Fire and forget: the backend answers
 * immediately and downloads in the background.
 */
export function prefetchChapters(bookId: number, chapterId: number): Promise<void> {
  return invoke<void>('prefetch_chapters', { bookId, chapterId })
}

/** Stops the prefetch window still running for this book (closing the reader). */
export function cancelPrefetch(bookId: number): Promise<void> {
  return invoke<void>('cancel_prefetch', { bookId })
}

/** Chapters prefetched ahead of the open one; `0` turns prefetching off. */
export function getReaderPrefetchNum(): Promise<number> {
  return invoke<number>('get_reader_prefetch_num')
}

export function setReaderPrefetchNum(chapters: number): Promise<number> {
  return invoke<number>('set_reader_prefetch_num', { chapters })
}

/** Re-fetches the catalog from the book's source and returns the merged list. */
export function refreshCatalog(bookId: number): Promise<Chapter[]> {
  return invoke<Chapter[]>('refresh_catalog', { bookId })
}

/**
 * 正文搜索：scans every chapter of the book whose body is available offline.
 * Resolves once with the full hit list (progress arrives on the
 * `search-content-progress` event), so the reader never has to poll.
 */
export function searchBookContent(bookId: number, query: string, regex = false): Promise<SearchContentResponse> {
  return invoke<SearchContentResponse>('search_book_content', { bookId, query, regex })
}

/** Stops the scan in flight for this book; safe to call when none is running. */
export function cancelBookContentSearch(bookId: number): Promise<void> {
  return invoke<void>('cancel_book_content_search', { bookId })
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

export function getReadingStats(): Promise<ReadingStats> {
  return invoke<ReadingStats>('get_reading_stats')
}

export function setReadingGoal(minutes: number): Promise<ReadingStats> {
  return invoke<ReadingStats>('set_reading_goal', { minutes })
}

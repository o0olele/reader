import { invoke } from '@tauri-apps/api/core'
import type { Book, BookSearchResult, ChapterRef, SourceBookPreview } from './types'

export function listBooks(): Promise<Book[]> {
  return invoke<Book[]>('list_books')
}

export function importTxtBook(filename: string, bytes: number[]): Promise<Book> {
  return invoke<Book>('import_txt_book', { filename, bytes })
}

export function importEpubBook(filename: string, bytes: number[]): Promise<Book> {
  return invoke<Book>('import_epub_book', { filename, bytes })
}

export function deleteBook(bookId: number): Promise<void> {
  return invoke<void>('delete_book', { bookId })
}

export function addOnlineBook(result: BookSearchResult): Promise<Book> {
  return invoke<Book>('add_online_book', { result })
}

export function fetchBookInfo(bookId: number): Promise<Book> {
  return invoke<Book>('fetch_book_info', { bookId })
}

/**
 * Reads what another source has for an already-shelved book without switching
 * to it — the 换源 sheet's 加载详情 / 加载目录 options.
 */
export function previewBookSource(
  bookId: number,
  result: BookSearchResult,
  withInfo: boolean,
  withToc: boolean,
): Promise<SourceBookPreview> {
  return invoke<SourceBookPreview>('preview_book_source', { bookId, result, withInfo, withToc })
}

/**
 * 换源: re-points a shelved book at another source and writes that source's
 * catalog. Pass the `chapters` from {@link previewBookSource} when 加载目录 was
 * on so the 目录 is not walked twice; the backend fetches it itself otherwise.
 * Nothing is written until a catalog is in hand, so a failed switch leaves the
 * current source, catalog and reading position intact.
 */
export function switchBookSource(bookId: number, result: BookSearchResult, chapters?: ChapterRef[]): Promise<Book> {
  return invoke<Book>('switch_book_source', { bookId, result, chapters })
}

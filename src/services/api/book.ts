import { invoke } from '@tauri-apps/api/core'
import type { Book, BookSearchResult } from './types'

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

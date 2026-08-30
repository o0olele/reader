import { invoke } from '@tauri-apps/api/core'

export function healthCheck(): Promise<string> {
  return invoke<string>('health_check')
}

export interface Book {
  id: number
  title: string
  author?: string
  path?: string
  group_id?: number
  source_id?: number
  remote_url?: string
  chapter_count: number
  updated_at: string
}

export function listBooks(): Promise<Book[]> {
  return invoke<Book[]>('list_books')
}

export function importTxtBook(filename: string, bytes: number[]): Promise<Book> {
  return invoke<Book>('import_txt_book', { filename, bytes })
}

export function importEpubBook(filename: string, bytes: number[]): Promise<Book> {
  return invoke<Book>('import_epub_book', { filename, bytes })
}

export interface Chapter {
  id: number
  book_id: number
  title: string
  number: number
  content: string
  remote_url?: string
}

export function listChapters(bookId: number): Promise<Chapter[]> {
  return invoke<Chapter[]>('list_chapters', { bookId })
}

export interface ReadingProgress {
  book_id: number
  chapter_id: number
  offset: number
}

export function getReadingProgress(bookId: number): Promise<ReadingProgress | null> {
  return invoke<ReadingProgress | null>('get_reading_progress', { bookId })
}

export function saveReadingProgress(bookId: number, chapterId: number, offset: number): Promise<void> {
  return invoke<void>('save_reading_progress', { bookId, chapterId, offset })
}

export interface BookshelfGroup {
  id: number
  name: string
  book_count: number
}

export function listGroups(): Promise<BookshelfGroup[]> {
  return invoke<BookshelfGroup[]>('list_groups')
}
export function createGroup(name: string): Promise<BookshelfGroup> {
  return invoke<BookshelfGroup>('create_group', { name })
}
export function deleteBook(bookId: number): Promise<void> {
  return invoke<void>('delete_book', { bookId })
}
export function moveBookToGroup(bookId: number, groupId: number): Promise<void> {
  return invoke<void>('move_book_to_group', { bookId, groupId })
}

export interface SearchRule {
  item: string
  title: string
  author?: string
  cover?: string
  url: string
}
export interface BookSource {
  id: number
  name: string
  base_url: string
  search_url: string
  search_rule: SearchRule
  enabled: boolean
  header?: string
  login_url?: string
  login_method?: string
  login_body?: string
  token_path?: string
  access_token?: string
  session_cookie?: string
  session_expires_at?: string
  sign_script?: string
  proxy_url?: string
}
export interface BookSearchResult {
  source_id: number
  source_name: string
  title: string
  author?: string
  cover?: string
  url: string
}
export function listBookSources(): Promise<BookSource[]> {
  return invoke<BookSource[]>('list_book_sources')
}
export function saveBookSource(input: Omit<BookSource, 'id'>): Promise<BookSource> {
  return invoke<BookSource>('save_book_source', { input })
}
export function searchBooks(query: string, sourceId?: number): Promise<BookSearchResult[]> {
  return invoke<BookSearchResult[]>('search_books', { query, sourceId })
}
export interface SourceTestResult {
  source_id: number
  source_name: string
  status: number
  result_count: number
}
export function testBookSource(sourceId: number, query: string): Promise<SourceTestResult> {
  return invoke<SourceTestResult>('test_book_source', { sourceId, query })
}
export function addOnlineBook(result: BookSearchResult): Promise<Book> {
  return invoke<Book>('add_online_book', { result })
}
export function fetchOnlineContent(sourceId: number, chapterUrl: string, chapterId?: number): Promise<string> {
  return invoke<string>('fetch_online_content', { sourceId, chapterUrl, chapterId })
}
export function refreshCatalog(bookId: number): Promise<Chapter[]> {
  return invoke<Chapter[]>('refresh_catalog', { bookId })
}
export interface SourceImportReport {
  imported: number
  failed: string[]
  partial: string[]
}
export function importBookSourcesJson(json: string): Promise<SourceImportReport> {
  return invoke<SourceImportReport>('import_book_sources_json', { json })
}
export function importBookSourcesUrl(url: string): Promise<SourceImportReport> {
  return invoke<SourceImportReport>('import_book_sources_url', { url })
}
export interface SourceLoginResult {
  source_id: number
  authenticated: boolean
  has_token: boolean
  has_cookie: boolean
}
export function loginBookSource(sourceId: number, username: string, password: string): Promise<SourceLoginResult> {
  return invoke<SourceLoginResult>('login_book_source', { input: { source_id: sourceId, username, password } })
}
export function clearBookSourceSession(sourceId: number): Promise<void> {
  return invoke<void>('clear_book_source_session', { sourceId })
}
export interface AppSettings {
  proxy_url?: string
}
export function getAppSettings(): Promise<AppSettings> {
  return invoke<AppSettings>('get_app_settings')
}
export function saveAppSettings(proxyUrl: string): Promise<AppSettings> {
  return invoke<AppSettings>('save_app_settings', { input: { proxy_url: proxyUrl || undefined } })
}

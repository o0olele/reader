import { invoke } from '@tauri-apps/api/core'
import type { BookSource, SourceImportReport, SourceLoginResult, SourceTestResult } from './types'

export function listBookSources(): Promise<BookSource[]> {
  return invoke<BookSource[]>('list_book_sources')
}

export function saveBookSource(input: Omit<BookSource, 'id'>): Promise<BookSource> {
  return invoke<BookSource>('save_book_source', { input })
}

export function testBookSource(sourceId: number, query: string): Promise<SourceTestResult> {
  return invoke<SourceTestResult>('test_book_source', { sourceId, query })
}

export function importBookSourcesJson(json: string): Promise<SourceImportReport> {
  return invoke<SourceImportReport>('import_book_sources_json', { json })
}

export function importBookSourcesUrl(url: string): Promise<SourceImportReport> {
  return invoke<SourceImportReport>('import_book_sources_url', { url })
}

export function loginBookSource(sourceId: number, username: string, password: string): Promise<SourceLoginResult> {
  return invoke<SourceLoginResult>('login_book_source', {
    input: { source_id: sourceId, username, password },
  })
}

export function clearBookSourceSession(sourceId: number): Promise<void> {
  return invoke<void>('clear_book_source_session', { sourceId })
}

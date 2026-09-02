import { invoke } from '@tauri-apps/api/core'
import type {
  BookSource,
  BookSourceInput,
  RawSourceRules,
  SourceDebugResult,
  SourceDebugStage,
  SourceImportReport,
  SourceLoginResult,
  SourceSessionStatus,
  SourceTestResult,
} from './types'

export function listBookSources(): Promise<BookSource[]> {
  return invoke<BookSource[]>('list_book_sources')
}

export function saveBookSource(input: BookSourceInput): Promise<BookSource> {
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

export function getBookSourceSessionStatus(sourceId: number): Promise<SourceSessionStatus> {
  return invoke<SourceSessionStatus>('get_book_source_session_status', { sourceId })
}

export function refreshBookSourceSession(
  sourceId: number,
  username: string,
  password: string,
): Promise<SourceLoginResult> {
  return invoke<SourceLoginResult>('refresh_book_source_session', {
    input: { source_id: sourceId, username, password },
  })
}

export function openBookSourceBrowser(sourceId: number): Promise<void> {
  return invoke<void>('open_book_source_browser', { sourceId })
}

export function saveBookSourceBrowserSession(sourceId: number): Promise<SourceLoginResult> {
  return invoke<SourceLoginResult>('save_book_source_browser_session', { sourceId })
}

/** Run one stage of the rule engine with editor-supplied input and rules. */
export function debugSourceStage(
  sourceId: number,
  stage: SourceDebugStage,
  input: string,
  rawRules: RawSourceRules,
): Promise<SourceDebugResult> {
  return invoke<SourceDebugResult>('debug_source_stage', { sourceId, stage, input, rawRules })
}

/** Persist editor-rewritten rules back onto a saved book source. */
export function updateBookSourceRules(sourceId: number, rawRules: RawSourceRules): Promise<BookSource> {
  return invoke<BookSource>('update_book_source_rules', { sourceId, rawRules })
}

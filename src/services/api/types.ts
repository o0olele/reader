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

export interface Chapter {
  id: number
  book_id: number
  title: string
  number: number
  content: string
  remote_url?: string
}

export interface ReadingProgress {
  book_id: number
  chapter_id: number
  offset: number
}

export interface BookshelfGroup {
  id: number
  name: string
  book_count: number
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

/** One book as returned by a single source. */
export interface BookSearchResult {
  source_id: number
  source_name: string
  title: string
  author?: string
  cover?: string
  url: string
}

/** The same book merged across every source that returned it. */
export interface SearchResultGroup {
  title: string
  author?: string
  cover?: string
  sources: BookSearchResult[]
}

export interface SourceFailure {
  source_id: number
  source_name: string
  reason: string
}

export interface SearchResponse {
  groups: SearchResultGroup[]
  failures: SourceFailure[]
  searched_sources: number
}

export interface SourceTestResult {
  source_id: number
  source_name: string
  status: number
  result_count: number
}

export interface SourceImportReport {
  imported: number
  failed: string[]
  partial: string[]
}

export interface SourceLoginResult {
  source_id: number
  authenticated: boolean
  has_token: boolean
  has_cookie: boolean
}

export interface AppSettings {
  proxy_url?: string
}

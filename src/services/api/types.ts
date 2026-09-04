export interface Book {
  id: number
  title: string
  author?: string
  path?: string
  group_id?: number
  source_id?: number
  remote_url?: string
  intro?: string
  kind?: string
  latest_chapter?: string
  cover_url?: string
  cover_data?: string
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

/** Legado rule objects kept verbatim; the rule engine prefers these over the
 *  flat selector fields, which only remain as a fallback. */
export interface RawSourceRules {
  search?: string
  book_info?: string
  toc?: string
  content?: string
  explore?: string
}

export interface BookSource {
  id: number
  name: string
  base_url: string
  search_url: string
  explore_url?: string
  search_rule: SearchRule
  raw_rules: RawSourceRules
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
  next_toc_url_selector?: string
  next_content_url_selector?: string
}

/**
 * What `save_book_source` accepts. Narrower than {@link BookSource}: a source
 * written by hand has no id, no session, and no legado rules — saving one
 * deliberately drops any rules a previous import had left behind.
 */
export type BookSourceInput = Omit<
  BookSource,
  'id' | 'raw_rules' | 'access_token' | 'session_cookie' | 'session_expires_at'
>

/** One book as returned by a single source. */
export interface BookSearchResult {
  source_id: number
  source_name: string
  title: string
  author?: string
  cover?: string
  url: string
  intro?: string
  kind?: string
  latest_chapter?: string
  word_count?: string
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
  auth_required: boolean
}

export interface SearchResponse {
  groups: SearchResultGroup[]
  failures: SourceFailure[]
  searched_sources: number
}

export interface ExploreCategory {
  source_id: number
  source_name: string
  title: string
  url: string
}

export interface SourceTestResult {
  source_id: number
  source_name: string
  status: number
  result_count: number
  auth_required: boolean
  cloudflare_challenge: boolean
  session_state: string
  request_url: string
  duration_ms: number
  has_token: boolean
  has_cookie: boolean
  user_agent: string
}

/** Stages a source can be debugged through, one rule group each. */
export type SourceDebugStage = 'search' | 'book_info' | 'toc' | 'content'

/** One parsed rule step inside a debug stage. */
export interface SourceDebugStep {
  field: string
  input_preview: string
  node_count: number
  output_preview: string
  error?: string
}

/** The actual HTTP exchange a debug stage performed. */
export interface SourceDebugRequest {
  method: string
  url: string
  headers: [string, string][]
  body?: string
  auth_attached: boolean
}

/** Result of one `debug_source_stage` call. */
export interface SourceDebugResult {
  source_id: number
  source_name: string
  stage: SourceDebugStage
  request?: SourceDebugRequest
  status?: number
  response_headers: [string, string][]
  duration_ms: number
  raw_html: string
  steps: SourceDebugStep[]
  final_json: unknown
  session_state: string
  error?: string
}

/** Payload streamed on the `source-test-progress` event while a stage runs. */
export interface SourceDebugProgress {
  source_id: number
  stage: SourceDebugStage
  state: 'started' | 'completed'
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
  session_expires_at?: string
}

export interface SourceSessionStatus {
  source_id: number
  state: 'anonymous' | 'authenticated' | 'expired' | string
  has_token: boolean
  has_cookie: boolean
  expires_at?: string
}

export interface AppSettings {
  proxy_url?: string
  /**
   * Explicit override. Leave blank to track the embedded webview, which is
   * what keeps the User-Agent and the client hints Cloudflare sees mutually
   * consistent — setting this changes the header but not the hints the
   * browser-auth window emits.
   */
  user_agent?: string
  /** Read-only: `navigator.userAgent` as last reported by the main window. */
  detected_user_agent?: string
  /** Read-only: what requests actually go out with right now. */
  effective_user_agent?: string
  /** Read-only: last-resort fallback when nothing else is known. */
  default_user_agent?: string
}

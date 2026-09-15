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
  anchor_index: number
  anchor_ratio: number
}

/**
 * One 正文搜索 hit. `match_offset` / `match_length` / `snippet_offset` count
 * UTF-16 code units inside the addressed unit (the chapter title when
 * `in_title`, otherwise paragraph `paragraph_index`), so they can be fed
 * straight to `String.prototype.slice`.
 */
export interface SearchContentHit {
  chapter_id: number
  chapter_number: number
  chapter_title: string
  /** 0-based occurrence index inside this chapter. */
  result_index: number
  in_title: boolean
  /** `null` for a hit in the chapter title. */
  paragraph_index: number | null
  match_offset: number
  match_length: number
  match_text: string
  snippet: string
  snippet_offset: number
  snippet_length: number
  progress_percent: number
}

/** Streamed on `search-content-progress` while a scan runs. */
export interface SearchContentProgress {
  book_id: number
  scanned: number
  total: number
  hits: number
}

/**
 * The whole answer. Chapters without an offline body are skipped, so
 * `searched_chapters` is normally smaller than `total_chapters` and the UI has
 * to say so instead of implying the whole book was covered.
 */
export interface SearchContentResponse {
  hits: SearchContentHit[]
  searched_chapters: number
  total_chapters: number
  truncated: boolean
  cancelled: boolean
}

export interface ReadingRecord {
  book_id: number
  duration_seconds: number
}

/** Home-dashboard aggregate returned by `get_reading_stats`. */
export interface ReadingStats {
  total_seconds: number
  today_seconds: number
  daily_goal_minutes: number
  streak_days: number
  finished_books: number
}

export interface DownloadTask {
  id: number
  book_id: number
  book_title: string
  status: 'pending' | 'running' | 'paused' | 'completed' | 'failed' | 'cancelled'
  total_chapters: number
  completed_chapters: number
  retry_count: number
  error?: string
  created_at: string
  updated_at: string
}

export interface ExportResult {
  path: string
  bytes_written: number
}

export interface CacheStats {
  used_bytes: number
  quota_bytes: number
  cached_chapters: number
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
  book_url_pattern?: string
  enabled_cookie_jar: boolean
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
  source_group?: string
  custom_order: number
  weight: number
  enabled_explore: boolean
  respond_time?: number
  last_update_time?: number
}

/**
 * What `save_book_source` accepts. Narrower than {@link BookSource}: a source
 * written by hand has no id, no session, and no legado rules — saving one
 * deliberately drops any rules a previous import had left behind.
 */
export type BookSourceInput = Omit<
  BookSource,
  'id' | 'raw_rules' | 'access_token' | 'session_cookie' | 'session_expires_at' | 'respond_time' | 'last_update_time'
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

/** One catalog entry of a candidate source (换源 preview → commit handoff). */
export interface ChapterRef {
  title: string
  url: string
}

/** What `preview_book_source` read from a candidate source, without writing. */
export interface SourceBookPreview {
  /** Only present when 详情 was requested; a source without a working info
   *  rule still yields a usable 目录. */
  info: {
    title?: string
    author?: string
    intro?: string
    cover?: string
    kind?: string
    latest_chapter?: string
  } | null
  /** Empty unless 目录 was requested. */
  chapters: ChapterRef[]
  /** Last chapter of the previewed 目录, or the source's own 最新章节. */
  latest_chapter: string | null
  /** Why 详情 failed, when it did. 换源 only needs a 目录, so this is a note
   *  rather than a row-level failure. */
  info_error: string | null
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
  charset?: string
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

export interface SourceExportResult {
  path: string
  exported: number
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

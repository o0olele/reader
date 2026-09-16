# Reader Desktop Architecture

## Current milestone

v0.2.0 (P0 metric fix) is complete: `rule-audit` feeds JSON sources a structured dummy input, reports failures per source and per category, and the engine has been aligned with legado semantics (empty-rule tolerance, Java regex normalization, JSoup selector compat, JS context variables, template tolerance, JSONPath/XPath residuals). P4 download/cache/export and P5 source management (batch validation, change-source, grouping, cookie persistence, legado JSON round-trip) are closed. The frontend talks to Rust through typed IPC commands, and browser preview mode remains available through Vite.

Roadmap v3 (`ROADMAP-v3.md`) is the current authority. F0–F3 have their main implementation in place: the shadcn-vue foundation, 13 page routes, rebuilt pages, reader pagination, paragraph anchors, full-text search, prefetch and change-source. E1 includes reading statistics/goals, cross-book bookmarks and a reader book-information panel. GUI, pagination geometry, performance and online end-to-end acceptance remain open. RSS/history and several advanced tools still show explicit unavailable states. E0 remains open: four selector fallbacks and incomplete rule semantics persist. The committed historical audit reports 131 blocked sources of 970 (86.5% execute without errors on dummy input); the original corpus is absent locally, so this is not a fresh measurement or proof of extraction correctness.

## Module boundaries

- `command/`: Tauri IPC handlers only; online workflows are delegated directly to `SourceService`, `SearchService` and `ReaderService`.
- `app/`: startup wiring (`bootstrap`), runtime state (`state`), and static configuration (`config`).
- `domain/`: business entities grouped by concern: `book`, `reader`, and `source`.
- `service`: application workflow boundaries for books, bookshelf, sources, search, reader, settings. `SourceService` owns book-source CRUD, legado import and authentication; `SearchService` owns concurrent search, result grouping and single-source probing; `ReaderService` owns chapter display, prefetch and 正文搜索. 换源 lives in `BookService`: `preview_source` reads a candidate source's 详情/目录 without writing anything, and `switch_source` fetches that catalog **before** it commits, so a source that cannot produce a 目录 leaves the shelf row's source, catalog and reading position untouched. `SourceSession::fetch_book_info` / `fetch_catalog` are the source-scoped reads shared by `BookService` and `ReaderService::refresh_catalog`.
- `repository`: persistence interfaces plus SQLite implementations for books, chapters, progress, and sources.
- `source_engine`: book-source import normalization, the multi-mode rule engine (`Default`/JSoup, XPath, JSONPath, regex and QuickJS), and `source_engine/url/` — the six-stage `AnalyzeUrl` request builder (options, encoding, rate limiting, transport). The rule engine lives in `source_engine/rule/` with `js_runtime/` split into runtime, bindings, script, normalize, statements, transport, time, chapter-numbers, request-options and types modules. Stage parsing lives in `source_engine/pipeline.rs` (shared rule helpers) plus `pipeline/stages/` — one file per stage (`search`, `explore`, `info`, `catalog`, `content`) behind the `stages.rs` facade. The `rule/` and `url/` modules are exercised by the source audit and fixture suites; unsupported Rhino JVM package access is reported explicitly.
- `service/explore_service.rs`: discovery-page parsing and pagination. `service/source_debug_service.rs` provides the four-stage source debugger and live rule retries.
- `infrastructure/`: adapters grouped by concern: `db`, `http` (client construction, request signing, URL resolution), and ebook decoding.

No separate `scheduler` crate exists; durable background work lives in `service/download_service.rs` as a persisted `download_tasks` state machine with startup resume, four-way concurrency, retry and per-source rate limiting.

## Data flow

`Vue feature -> services/api -> Tauri command -> service -> repository/infrastructure -> serialized result`

The UI never accesses SQLite directly. Commands stay thin and return structured `AppError` values.

Online search follows `search URL -> reqwest client (15s timeout, redirects, User-Agent) -> scraper CSS selectors -> normalized BookSearchResult`. Search URLs use `{{key}}` (or `{key}`) as the URL-encoded query placeholder. Results can be added to the bookshelf, refreshed into a local catalog, and read through persistent chapter caching.

Single-chapter reading warms its neighbours instead of downloading one chapter per open: `service/reader_service/prefetch.rs` ports the reference app's `ReadBook.preDownload()` — a forward window of `reader_prefetch_num` chapters (default 10, `0` disables it, editable under 设置 → 下载缓存) plus at most five backwards, two concurrent lanes under one semaphore, a per-chapter failure cap of three, and cancel-on-next-chapter. `prefetch_chapters` / `cancel_prefetch` sit behind the reader's post-load hook (`useReader.ts`). Local books and catalogs with nothing missing do no network work at all.

正文搜索 (full-text search) is `service/reader_service/search.rs` + `search/matcher.rs`, ported from the reference's `SearchContentRepository`: only chapters whose body is already offline are scanned (local bodies or `chapter_contents`), so a search never triggers a download. Each chapter is rendered through the reader's own display path (`ReaderService::display_chapter`, shared with `read_chapter`), so a hit addresses the characters the reader actually shows, and a hit is reported as (title | paragraph index, UTF-16 offset, length) — the shape `ReaderHighlight.vue` can mark and `useContentHighlight` can scroll to, with no offset maths across a joined `"title\nbody"` document. Results stream progress on `search-content-progress` and return in one response from `search_book_content`; `cancel_book_content_search` invalidates the scan's generation, the same pattern as the prefetch window. The reference's regex toggle is kept; its search history and replace-while-searching are not ported.

换源 (change source) is the reader topbar's ⇄ button, left of 书籍详情, and it opens a **fourth right-hand 320px panel** rather than a modal — the same slot as 书籍信息 / 正文搜索 / 阅读设置, which `ReaderPanels.vue` renders one at a time and `useReaderSidePanels.ts` keeps mutually exclusive (the 换源 panel owns its own open flag so a running search survives a panel switch). `useChangeSource.ts` searches every enabled source for the open book's title one source at a time (six lanes, results merged as they arrive) and shows the reference app's `ChangeSourceSheet` options — 校验作者 / 加载详情 / 加载目录 — each of which goes through `preview_book_source`, so nothing is written until the user picks a row. Picking one calls `switch_book_source` with the previewed catalog when there is one, which re-points the shelf row, replaces title/author/cover/简介 and writes the new catalog through separate repository operations (the overall source switch is not a single database transaction); downloaded bodies of the old chapters are deleted with them. The reader then locates the chapter it was on with `chapterMatch.ts`, a port of the reference's `BookHelp.getDurChapter()` (a Jaccard name match inside a ±10-chapter window around the index scaled by the old catalog size, falling back to the chapter number), and restores its paragraph locator. Per-book 置顶/置底 preferences live in `localStorage` under `change-source-pin:<bookId>`, mirroring the reference's per-book source score, so the sheet never reorders the source list globally.

The reference project at `E:\Code\legado-with-MD3` informed the initial boundaries: bookshelf items, chapter catalogs, and reader content are separate concerns. Its Android/Compose implementation is not copied into this Tauri core.

## Quality gates

`npm run check` runs ESLint, Prettier, tooling tests, structural checks and the production frontend build. `cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check`, `cargo test --locked --manifest-path src-tauri/Cargo.toml` and `cargo clippy --locked --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings` are the Rust gates. See `docs/quality-gates.md` for scope and commands.

The reusable `quality.yml` workflow runs on pull requests and manual dispatch. Main pushes run it through the release workflow's `quality` job; `windows` depends on that job, so failed checks prevent publication. The 970-source corpus is not committed; CI runs the audit's unit tests and committed fixture tests, not the unavailable full corpus benchmark.

`BookService` keeps source preview/switching in `book_service/change_source.rs`; `BookSource` import/authentication behavior lives in `domain/source/behavior.rs`; session cookie helpers live in `source_session/cookies.rs`. HTTP request identity and custom header/signature handling live in `infrastructure/http/request/{user_agent,headers}.rs`. These splits preserve the existing public call paths.

## Logging

Rust uses `tracing` targets for `book`, `reader`, `source`, `network`, `database`, and `download`; imports, source operations, requests, cache hit/miss, migration duration, and download-task failures are instrumented. The destination is chosen in `app/logging.rs`: dev builds write to stdout (what `tauri dev` shows), release builds append to `<app_data_dir>/logs/reader-desktop.log` with a single `.1` generation at 5 MiB. `RUST_LOG` overrides the level. Release builds link with `windows_subsystem = "windows"` (`src/main.rs`), so the app must never depend on a console.

## Database

SQLite is migrated from `src-tauri/migrations`. The initial schema is intentionally small; subsequent schema changes must be new numbered migration files.

## Frontend tooling

This application uses Tailwind v4 + shadcn-vue (Reka UI) with CSS-variable design tokens in `src/styles.css`; the reader's text surface is the deliberate exception (ROADMAP-v3 §3.4 — paged columns need a stable, measurable box model and the reader has its own paper/ink token set). Tokens come from the prototype's effective shadcn/slate layer, so no hex colour literals remain. ESLint, Prettier, and `vue-tsc` are required checks. Feature state remains in focused composables, so the unused Pinia dependency was removed. Vue Router exposes 13 real routes — every page is its own component under `AppLayout`, and the URL is the only navigation state, including deep links such as `#/read/<bookId>?toc=1&panel=1`. No `.vue` SFC exceeds 200 lines.

Anything that lists every book source is a first-paint risk once a legado import brings in several hundred of them, so both long lists — 书源管理 (`SourceList.vue`) and the 发现 sidebar (`ExploreSourceList.vue`) — mount only the rows around the viewport through the shared `src/lib/useWindowedList.ts`. 发现 browses one source at a time: a `搜索书源` filter replaced the old 全部书源 entry, which bounds the category chip bar to the selected source's own categories instead of every category of every source.

`Ctrl+K` (`src/app/CommandPalette.vue`) is the one search surface that spans the app: it matches shelf books (title, author, group), book sources (name, group, base URL) and the route table (title, route name, pinyin) locally, lists a bounded number of rows per group, and always keeps the online `search_books` round-trip available for the typed keyword. Two `Command` primitives were corrected for it: a row hands the fields it matched on to `CommandItem` as `keywords`, and `Command.vue` derives the visible set from the item registry instead of storing it in a watcher, so a row that mounts while a query is being typed is scored in the same tick rather than leaving the palette reporting no matches with rows on screen.

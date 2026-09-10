# Reader Desktop Architecture

## Current milestone

v0.2.0 (P0 metric fix) is complete: `rule-audit` feeds JSON sources a structured dummy input, reports failures per source and per category, and the engine has been aligned with legado semantics (empty-rule tolerance, Java regex normalization, JSoup selector compat, JS context variables, template tolerance, JSONPath/XPath residuals). P4 download/cache/export and P5 source management (batch validation, change-source, grouping, cookie persistence, legado JSON round-trip) are closed. The frontend talks to Rust through typed IPC commands, and browser preview mode remains available through Vite.

Roadmap v3 (`ROADMAP-v3.md`) is the current authority. F0–F2 are implemented: shadcn-vue foundation, real per-page routes, and the rebuilt bookshelf / source / source-debug / explore / search / download / settings / my / home pages. F3 rebuilt the reader: two-column spread pagination with the prototype's `<820px` / `<760px` single-column fallback, measured page geometry, a page pager, and catalog read state. E0 is still open — only the `pipeline/stages.rs` split is closed; the blocked-source count remains 182 of 970 (81.2% static coverage, `docs/coverage/rule-audit.md`).

## Module boundaries

- `command/`: Tauri IPC handlers only; online workflows are delegated directly to `SourceService`, `SearchService` and `ReaderService`.
- `app/`: startup wiring (`bootstrap`), runtime state (`state`), and static configuration (`config`).
- `domain/`: business entities grouped by concern: `book`, `reader`, and `source`.
- `service`: application workflow boundaries for books, bookshelf, sources, search, reader, settings. `SourceService` owns book-source CRUD, legado import and authentication; `SearchService` owns concurrent search, result grouping and single-source probing.
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

The reference project at `D:\Code\chatting\legado-with-MD3` informed the initial boundaries: bookshelf items, chapter catalogs, and reader content are separate concerns. Its Android/Compose implementation is not copied into this Tauri core.

## Logging

Rust uses `tracing` targets for `book`, `reader`, `source`, `network`, `database`, and `download`; imports, source operations, requests, cache hit/miss, migration duration, and download-task failures are instrumented. The destination is chosen in `app/logging.rs`: dev builds write to stdout (what `tauri dev` shows), release builds append to `<app_data_dir>/logs/reader-desktop.log` with a single `.1` generation at 5 MiB. `RUST_LOG` overrides the level. Release builds link with `windows_subsystem = "windows"` (`src/main.rs`), so the app must never depend on a console.

## Database

SQLite is migrated from `src-tauri/migrations`. The initial schema is intentionally small; subsequent schema changes must be new numbered migration files.

## Frontend tooling

This application uses Tailwind v4 + shadcn-vue (Reka UI) with CSS-variable design tokens in `src/styles.css`; the reader's text surface is the deliberate exception (ROADMAP-v3 §3.4 — paged columns need a stable, measurable box model and the reader has its own paper/ink token set). Tokens come from the prototype's effective shadcn/slate layer, so no hex colour literals remain. ESLint, Prettier, and `vue-tsc` are required checks. Feature state remains in focused composables, so the unused Pinia dependency was removed. Vue Router exposes 13 real routes — every page is its own component under `AppLayout`, and the URL is the only navigation state, including deep links such as `#/read/<bookId>?toc=0&panel=1`. No `.vue` SFC exceeds 200 lines.

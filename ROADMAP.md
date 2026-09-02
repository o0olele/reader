# Reader Desktop 执行路线图

> 本文件是 `plan.md`（原始总体规划）的**执行层补充**，基于 2026-08-30 对代码库的完整审计，以及与参考项目 `D:\Code\chatting\legado-with-MD3` 的对照。
>
> `plan.md` 回答"做什么"，本文件回答"接下来按什么顺序做、做到什么算完"。
> 两者冲突时以本文件为准，并回头修订 `plan.md`。
>
> **最近复核：2026-09-01。** §0 是 2026-08-30 的**起点快照，已成历史记录**，保留用于对照；
> **当前真实进度一律以 §10 为准。**§10 的每一条都要求「代码里能验证到什么」，不接受未经核对的打勾。

**一句话现状：Step 0 ✅ · Step 1 代码项 ✅（待真实书源手工验收）· Step 2 进行中（规则引擎与基础 JS Runtime 已接入，剩 WebJS/网络兼容层与公开语料验证）。**

---

## 0. 起点快照（2026-08-30 审计 · 历史记录）

### 0.1 代码规模（审计时的原始快照，已过时；现状见 §10.1）

### 0.2 里程碑定位

审计当时：**M1 完成 · M2 约 60% · M3 约 5%。**

（注：`App.vue:199` 顶栏硬编码 `M0 基础骨架`，`ARCHITECTURE.md:5` 写 M2，两处均需在 Step 0 校正 —— **均已修**，见 §10.2。）

### 0.3 已跑通的能力

- TXT 导入：UTF-8 / GBK / UTF-16LE / UTF-16BE 探测（`domain.rs:decode_text`）
- TXT 章节切分：启发式（`第X章/节/回`、`序章`、`楔子`、`番外`）（`domain.rs:split_chapters`）
- EPUB 导入：`META-INF/container.xml` → OPF → manifest/spine → 剥标签（`command.rs:163`）
- 书架、分组（CRUD + 移动）、删除
- 阅读进度：`chapter_id + offset`，350ms 防抖 + 关闭时保存
- 书源：CRUD、`Semaphore(8)` 并发搜索、目录抓取、正文抓取
- 全局代理 + 单书源代理（`build_source_client`）
- 书源登录：token / cookie 提取与持久化
- legado JSON 导入（**残缺，见 3.1**）

### 0.4 已确认的缺陷（Step 0 必修）

> **B1–B5 已于 2026-08-31 全部修复并各带回归测试，详见 §10.1。**下表保留作为问题记录。

| # | 位置 | 问题 |
| --- | --- | --- |
| B1 | `command.rs:1103` `fetch_online_content` | 正文只返回 String，从不落库；`listChapters` 插入 `content=''`；前端仅改内存。**每次打开同一章都重新联网**。`chapters.content NOT NULL` 的设计也让缓存无处安放。 |
| B2 | `command.rs:1135` `list_chapters` | `if rows.is_empty()` 才抓目录 —— 在线书**永远无法更新目录**，只能删书重加。 |
| B3 | `command.rs:505-510` `save_book_source` | INSERT 与后续 UPDATE 的字段列表**均不含 `proxy_url`**，但 `App.vue:243` 有该输入框。手动保存的单源代理静默丢失（JSON 导入路径正常）。 |
| B4 | `command.rs:647` `source_request` | `let Some(raw) = source.header.as_deref() else { return Ok(request) }` 提前返回，导致其后的 `sign_script` 处理**在未配置 header 的书源上被静默跳过**。 |
| B5 | `migrations/010_app_settings.sql` | 重复创建 `001` 已建的 `app_settings` 表（`IF NOT EXISTS` 兜住了，但说明 migration 有漂移，需复核）。 |

### 0.5 架构债

`ARCHITECTURE.md:9-16` 声明了 `command / app / domain / service / repository / source_engine / infrastructure / scheduler` 八个模块边界，**目录中一个都不存在**。实际为 5 个平铺文件，其中 `command.rs` 同时承担：Tauri IPC + SQL + HTTP 客户端构建 + ZIP 解包 + XML 解析 + 签名计算 + 登录流程。

对照 `plan.md` §39 八条原则（末列为 2026-08-31 复核后的现状）：

| # | 原则 | 2026-08-30 | 现状 |
| --- | --- | --- | --- |
| 一 | Command 永远不要写业务 | ❌ 业务全在 `command.rs` | ✅ `command/` 7 个文件共 417 行，无 `sqlx::query` |
| 二 | SQLite 不让 Vue 直接操作 | ✅ | ✅ |
| 三 | Source Engine 独立于 BookService | ⚠️ `source.rs` 只有解析，HTTP 在 `command.rs` | ✅ `source_engine/` 纯解析；HTTP 在 `infrastructure/http/` |
| 四 | JS Runtime 独立于 Source Engine | ❌ 没有 JS Runtime | ❌ 仍无（Step 2） |
| 五 | 下载必须持久化任务状态 | ❌ 无下载系统 | ❌ 仍无（Step 4） |
| 六 | 阅读器 UI 与章节获取分离 | ❌ 都在 `App.vue` | ✅ `ReaderPane.vue` + `useReader.ts` + `ReaderService` |
| 七 | 前后端明确 Schema | ⚠️ 有 TS 类型，无 repository trait | ⚠️ repository trait 已有；TS 类型仍是**手写镜像**，无 codegen |
| 八 | 第一天做 migration / logging / error | migration ✅；logging 仅 1 处；`AppError` 几乎未用 | ✅ 15 处 tracing 覆盖 5 个 target；全部 command 返回 `AppError` |

`plan.md` §"第一批 10 个基础任务"（末列为现状）：

| 项 | 2026-08-30 | 现状 |
| --- | --- | --- |
| Tailwind / shadcn-vue | 未安装 | **已决策不用** —— 改走原生 CSS + Design Tokens（`styles.css`），见 §2.5 |
| ESLint / Prettier | 未配置 | ✅ 已配置并全绿，`vue-tsc --noEmit` 纳入 `npm run build` |
| Rust workspace 结构 | 单 crate | 仍是单 crate（按 §2.5 决策，推迟到 Step 2 抽 `reader-core` 时一次性做） |
| Repository trait | 无 | ✅ `repository/mod.rs` 定义 4 个 trait + SQLite 实现 |
| Event 基础封装 | 纯内存 EventEmitter | ✅ `events.ts` 真实桥接 Tauri `listen`/`emit`，保留无 Tauri 时的内存降级 |

另：`pinia` 已移除（装了没用）；`vue-router` 已启用，但**三条路由目前都指向同一个 `AppShell`**，属登记而非真正的页面级路由，见 §10.4。

### 0.6 顺序偏差

migration `008_dynamic_auth`（登录 / token / 签名）属于 `plan.md` §41 优先级中的 **P5**，却在 P2 的 CSS/XPath/JSONPath 尚未完成时就已落地 —— 正是 `plan.md` §41 结尾明确警告的情况。

**原决策已调整（2026-09-01）：认证不再整体冻结。** 现实书源普遍要求登录、Cookie、token 或签名，
这些能力是在线阅读的前置条件。认证工作拆成两条轨道：

- **协议认证**（登录表单、Cookie/token 持久化、刷新、header/sign）：随 Step 2 的 JS Runtime
  立即补齐，并在 Step 3 调试器中可观测、可重试。
- **浏览器挑战**（Cloudflare JS challenge、Turnstile、指纹校验）：Step 2 先做 WebView
  spike，Step 3 提供“在浏览器中认证并回写 Cookie”的最小闭环；复杂挑战和自动化绕过仍留到 Step 7，
  不承诺无浏览器环境绕过 Cloudflare。

---

## 1. 总体判断与排序原则

两条核心判断决定了下面的顺序：

1. **架构债现在还成本最低。** 代码库目前约 1800 行 Rust，拆分是一天的活；到 1 万行时就是一周并且会引入回归。Step 0 不可跳过、不可与功能开发并行。
2. **规则引擎不做完，在线功能等于零。** 当前只支持纯 CSS，而真实 legado 书源大量使用 `@XPath:` / `$.` / `<js>` / `&&` / `##`。现在的"导入成功 N 个"是假象 —— 导进去的多半是坏的（见 3.1）。在此之前追加任何在线特性都是在坏地基上盖楼。

排序原则：**先还债 → 再让已有的在线链路真正可用 → 再攻规则引擎（含调试器同步）→ 最后才是下载 / RSS / 备份 / 发布。**

```
Step 0  架构补债 + 修 bug        3~5 天   ✅ 已完成（2026-08-31）
   ↓
Step 1  M2 收尾：在线阅读可用      1 周    ✅ 代码项完成，待真实书源手工验收
   ↓
Step 2  规则引擎（含 JS Runtime）  2~3 周  ◀ 进行中：基础 Runtime 已接入；先补协议认证 + WebView spike
Step 3  书源调试器 + 认证闭环      1 周                  ┘ 并行：认证状态可见、浏览器 Cookie 回写
   ↓
Step 4  下载 / 缓存               2 周
Step 5  阅读排版引擎              2 周
Step 6  RSS / 备份 / 设置         2 周
Step 7  性能 / 稳定性 / 发布       2 周
```

> §1 第一条判断（"架构债现在还成本最低"）在 Step 0 之后**又验证了一次**：Step 0 拆干净 `command.rs`
> 后，业务在 `SourceService` 里重新聚成 617 行。已于 2026-08-31 再次拆分，见 §10.3。
> **教训：拆分不是一次性动作，每个 Step 结束都要复查最大文件。**

---

## 2. Step 0 · 架构补债（3–5 天）

**目标：让 `ARCHITECTURE.md` 描述的结构真实存在，并清掉 5 个已知缺陷。本步不新增任何用户可见功能。**

### 2.1 Rust 侧拆分

把 `command.rs` 按 `plan.md` §37 的目标形态拆开：

```text
src-tauri/src/
├── main.rs
├── lib.rs
├── error.rs                    ← 补全 AppError 变体
├── app/
│   ├── mod.rs
│   ├── state.rs                ← 从 app.rs 迁入
│   ├── bootstrap.rs            ← 从 lib.rs setup() 迁入
│   └── config.rs
├── command/
│   ├── mod.rs
│   ├── book.rs                 ← import_txt/epub, list, delete
│   ├── bookshelf.rs            ← groups, move
│   ├── reader.rs               ← chapters, progress
│   ├── source.rs               ← source CRUD, import, test, login
│   ├── search.rs               ← search_books
│   └── settings.rs             ← app settings
├── domain/
│   ├── mod.rs
│   ├── book.rs                 ← Book, Chapter, BookshelfGroup
│   ├── reader.rs               ← ReadingProgress
│   └── source.rs               ← BookSource, SearchRule, ...（从 source.rs 迁入）
├── service/
│   ├── mod.rs
│   ├── book_service.rs         ← 导入流程编排
│   ├── reader_service.rs       ← 目录 / 正文获取 + 缓存策略
│   ├── source_service.rs       ← 书源 CRUD / 导入 / 测试
│   └── search_service.rs       ← 并发搜索 / 去重 / 排序
├── repository/
│   ├── mod.rs                  ← trait 定义
│   ├── book.rs
│   ├── chapter.rs
│   ├── source.rs
│   └── progress.rs
├── source_engine/
│   ├── mod.rs
│   ├── engine.rs               ← 执行编排
│   ├── selector.rs             ← 从 source.rs::extract 迁入
│   ├── css.rs
│   └── import.rs               ← 从 source.rs::parse_sources_json 迁入
└── infrastructure/
    ├── mod.rs
    ├── db/mod.rs               ← pool 获取、migration
    ├── http/
    │   ├── mod.rs
    │   ├── client.rs           ← build_source_client + 共享 client
    │   └── request.rs          ← source_request / send_source_request / response_error
    └── ebook/
        ├── mod.rs
        ├── txt.rs              ← decode_text / split_chapters
        └── epub.rs             ← 从 command.rs:99-282 迁入
```

不建 `scheduler/`（Step 4 再建）。

**Repository trait 最小形态**（按 `plan.md` §6.1）：

```rust
#[async_trait::async_trait]
pub trait BookRepository: Send + Sync {
    async fn get(&self, id: i64) -> Result<Option<Book>, AppError>;
    async fn list(&self) -> Result<Vec<Book>, AppError>;
    async fn find_by_path(&self, path: &str) -> Result<Option<Book>, AppError>;
    async fn create(&self, book: &NewBook) -> Result<i64, AppError>;
    async fn delete(&self, id: i64) -> Result<(), AppError>;
}
```

同理 `ChapterRepository` / `SourceRepository` / `ProgressRepository`。

> 注意：目前 `list_books()` 被当成"按 id 查一本"用了 5 处（`command.rs:64,257,1088,1095` 等），每次全表扫描 + LEFT JOIN 聚合。拆分时用 `get(id)` 替换。

### 2.2 错误体系

补全 `error.rs`（`plan.md` §4.3）：

```rust
pub enum AppError {
    Database(String),
    Network(String),
    Parse(String),
    Source(String),
    Io(String),
    InvalidArgument(String),
}
```

- 所有 command 签名改为 `Result<T, AppError>`
- 删除全部 `map_err(|e| e.to_string())`，改为 `#[from]` 或显式分类
- 前端 `services/api` 增加统一错误解包（现在 `App.vue` 里 12 处重复的 `cause instanceof Error ? ... : String(cause)`）

### 2.3 日志

按 `ARCHITECTURE.md:30` 已声明的 target 补齐埋点（当前只有 1 处）：

| target | 埋点位置 |
| --- | --- |
| `book` | 导入开始 / 完成 / 章节数 |
| `reader` | 章节加载、缓存命中 / 未命中 |
| `source` | 规则执行、解析结果条数、解析失败 |
| `network` | 请求 URL、状态码、耗时、重试 |
| `database` | migration、慢查询 |

### 2.4 前端拆分

```text
src/
├── main.ts
├── router/index.ts             ← 启用 vue-router
├── app/
│   └── AppShell.vue            ← 顶栏 + 侧栏骨架
├── features/
│   ├── bookshelf/              ← BookshelfPage.vue + BookCard.vue
│   ├── reader/                 ← ReaderPage.vue, ReaderToolbar, ReaderContent,
│   │                             ReaderChapterList, ReaderSettings
│   │   └── composables/        ← useReader, useReaderSettings, useReadingProgress
│   ├── search/                 ← SearchPage.vue, SearchResultItem.vue
│   ├── source/                 ← SourceListPage, SourceEditor, SourceImport, SourceLogin
│   └── settings/               ← SettingsPage.vue
├── services/
│   ├── api/                    ← 按 feature 拆：book.ts, reader.ts, source.ts, ...
│   └── events.ts               ← 接通 Tauri
└── shared/
    ├── stores/                 ← pinia：bookshelf, reader, source
    └── ui/
```

`events.ts` 改为真实桥接：

```ts
import { listen, emit as tauriEmit } from '@tauri-apps/api/event'
```
保留浏览器预览降级（无 Tauri 时退回内存 bus）。

### 2.5 工具链

- 安装并配置 Tailwind CSS + shadcn-vue，**或**明确从 `plan.md` §2 删除这两项、改为「原生 CSS + Design Tokens」。二选一，不留悬空。
  - 推荐：**装 Tailwind，不装 shadcn-vue**。当前 `styles.css` 只有 77 行，迁移成本低；shadcn-vue 对这个体量的应用是负担。
- 配置 ESLint + Prettier + `vue-tsc` 纳入 CI 前置检查
- 评估是否转 Cargo workspace：**Step 0 暂不转**，等 Step 2 的 `source_engine` 稳定后，按 `plan.md` §40 抽 `reader-core` 时一次性做。

### 2.6 缺陷修复

- **B1** — 新增 migration `011_chapter_cache.sql`：`chapters.content` 改可空（SQLite 需 rebuild table），或新增 `chapter_contents(chapter_id PK, content, cached_at)`。**推荐后者**，正文与目录分表，为 Step 4 的缓存分级留位置。`reader_service` 走「查缓存 → 未命中则抓取 → 落库」。
- **B2** — 新增 `refresh_catalog(book_id)` command；`list_chapters` 只读本地。UI 在书籍详情/阅读器加刷新入口。
- **B3** — `save_book_source` 的 INSERT / UPDATE 补上 `proxy_url`。
- **B4** — 把 `sign_script` 处理移到 `source.header` 分支之外。
- **B5** — 复核 `001` 与 `010` 的 `app_settings` 定义差异（`001` 有 `NOT NULL` on key，`010` 没有），保留一处，另一处改为空操作注释（**已上线的 migration 不可删改内容，只能新增修正 migration**）。

### 2.7 文档校正

- `ARCHITECTURE.md`：模块清单与实际目录对齐；把「introduced with M1/M2/M4」这类未兑现的注记改为真实状态
- `App.vue`（迁移后的 AppShell）顶栏版本号改为从 `package.json` 读取，不再硬编码 `M0`

### 2.8 验收标准（2026-08-31 逐条复核）

- [x] `command/` 下没有任何一个文件超过 200 行；无 `sqlx::query` 直接调用 —— 最大 `command/source.rs` 153 行
- [x] `grep -r "map_err(|e| e.to_string())" src-tauri/src` 无结果 —— 0 处
- [x] 所有 command 返回 `Result<_, AppError>` —— 25 个命令全部符合
- [x] 前端无任何文件超过 200 行；`App.vue` 不再存在 —— 最大 `useSources.ts` 193 行
- [x] `cargo test` + `cargo clippy -- -D warnings` + `npm run lint` + `vue-tsc --noEmit` 全绿
- [x] B1–B5 全部修复，并各有一条回归测试 —— **但 B5 的"测试"是文本断言而非行为回归，见 §10.5**
- [ ] 手工回归：导入 TXT → 导入 EPUB → 分组 → 阅读 → 退出重启 → 进度恢复；在线搜索 → 加入书架 → 读目录 → 读正文 → **二次打开同章不发网络请求** —— **未执行**，与 §3.3 一并归入 §10.6

---

## 3. Step 1 · M2 收尾：让在线阅读真正可用（1 周）

**目标：在只支持 CSS 的前提下，把在线阅读链路从"能演示"做到"能用"。**

### 3.1 前置认知：当前导入器为何是坏的

`source.rs:69-80`：

```rust
fn legacy_rule(value: Option<&serde_json::Value>, keys: &[&str]) -> Option<String> {
    ...
    let rule = rule.split("&&").next().unwrap_or(rule).trim();   // ← 丢弃 && 之后全部规则
    rule.strip_prefix("@css:").unwrap_or(rule).trim().to_owned() // ← 只认 @css:
}
```

后果：真实 legado 书源里的 `@XPath:` / `$.` / `<js>` / `class.xxx@tag.a@text` 会被原样送进 `scraper::Selector::parse()` 并失败；`&&` 串联规则被截断。

**Step 1 的处置：不重写导入器**（那是 Step 2 的活），而是：

- 导入时对每条规则做**可解析性检查**，不可解析的标记为 `unsupported` 并写入 `book_sources.import_warnings`
- `SourceImportReport` 增加 `partial: Vec<String>` 字段，UI 明确显示「导入 N 个，其中 M 个含不支持的规则」
- **不再让用户误以为导入成功**

### 3.2 任务清单

| # | 任务 | 说明 |
| --- | --- | --- |
| 1.1 | 正文二级缓存 | 内存 LRU（容量按章数，默认 50）→ SQLite `chapter_contents`。承接 B1。 |
| 1.2 | 目录刷新 | `refresh_catalog(book_id)`，diff 出新增章节，发 `chapter-updated` 事件。承接 B2。 |
| 1.3 | `nextTocUrl` 分页目录 | 新增 `book_sources.next_toc_url_selector`。循环抓取，**上限 50 页 + 环检测**。 |
| 1.4 | `nextContentUrl` 分页正文 | 新增 `book_sources.next_content_url_selector`。**上限 20 页 + 环检测**，拼接后落库。这是很多站点正文读不全的直接原因。 |
| 1.5 | 书籍详情页 | `parse_book_info`（`source.rs:290`）目前是**死代码，无任何 command 调用**。新增 `fetch_book_info` command + 详情页 UI（简介、分类、最新章节、封面）。 |
| 1.6 | 封面显示 | `BookSearchResult.cover` 已解析但 UI 只显示书名首字（`App.vue:246`）。走 Rust 侧代理下载 + 本地缓存（避免前端跨域与 Referer 问题）。 |
| 1.7 | 换源 | 同名同作者在其他书源重搜 → 章节按序号对齐 → 切换 `books.source_id` 并清正文缓存。 |
| 1.8 | 搜索结果排序与去重 | 现在只按 `url` 去重（`command.rs:960`）。改为按 `(标题, 作者)` 归并同一本书的多书源结果，UI 折叠展示。 |
| 1.9 | 搜索失败可见性 | 现在只要有一个源成功就吞掉全部失败（`command.rs:956`）。改为始终返回 `{ results, failures }`，UI 折叠显示失败原因。 |

### 3.3 验收标准（2026-08-31 复核：**代码项全部落地，三条验收均未执行**）

- [ ] 用 3 个真实、纯 CSS 可解析的书源，完成：搜索 → 详情 → 加书架 → 目录（含分页）→ 正文（含分页）→ 换源 → 刷新目录出新章
- [ ] 同一章第二次打开零网络请求 —— 代码路径已具备（LRU 50 → `chapter_contents`），但未在真实书源上实测
- [ ] 导入一份真实 legado 书源 JSON（≥100 个源），报告能准确区分「完全支持 / 部分支持 / 不支持」

> 这三条是 **v0.2.0 的发布门槛**；认证轨道已可并行推进，不再阻塞 Step 2 开工（见 §10.6）。
> 需要人工提供 3 个可用书源；代码侧无待办。

---

## 4. Step 2 · 规则引擎（2–3 周，项目成败点）

**目标：能执行真实 legado 书源的规则语法，并让需要登录/token/Cookie/sign 的书源具备可用的协议认证链路。**

### 4.1 参考实现对照

legado 侧（`app/src/main/java/io/legado/app/model/analyzeRule/`）：

| 文件 | 职责 |
| --- | --- |
| `RuleAnalyzer.kt` | 规则字符串词法切分（`&&` `\|\|` `%%` `##` `{{}}`） |
| `AnalyzeRule.kt` | 模式派发：`Default(JSoup) / XPath / Json / Js / WebJs / Regex` |
| `AnalyzeByJSoup.kt` | CSS + legado 私有语法（`class.xxx.0@tag.a@text`） |
| `AnalyzeByXPath.kt` | XPath |
| `AnalyzeByJSonPath.kt` | JSONPath |
| `AnalyzeByRegex.kt` | 正则 |
| `AnalyzeUrl.kt` | URL 模板、请求参数、`,{...}` 选项对象 |
| `CustomUrl.kt` / `RuleData.kt` | 变量传递 |

支持的语法标记（`AnalyzeRule.kt:531-690` 实测）：

```
@CSS:  @@  @XPath:  (以 / 开头亦判为 XPath)
@Json:  ($. 或 $[ 开头亦判为 Json)
<js>...</js>   @js:   :regex（allInOne 时以 : 开头）
{{表达式}}   @get:{key}   @put:{key:rule}
##正则##替换###   ||备选   &&串联   %%交叉   -反向
```

### 4.2 实施顺序（**不要直接上 XPath**）

**4.2.1 先写 `RuleAnalyzer`（1 周，最关键）— 🟡 解析器与 50 条代表性语法基线已完成**

规则字符串解析器是所有 selector 的前置。当前 `split("&&").next()` 就是缺它的直接后果。

```rust
// source_engine/rule/analyzer.rs
pub enum RuleMode { Default, XPath, Json, Js, Regex }

pub struct SourceRule {
    pub mode: RuleMode,
    pub rule: String,
    pub replace: Option<(Regex, String)>,   // ## 替换
    pub put: HashMap<String, String>,       // @put
    pub get: Vec<String>,                   // @get
}

pub fn split_rule(raw: &str) -> Vec<Vec<SourceRule>>;  // 外层 || 备选，内层 && 串联
pub fn expand_template(raw: &str, ctx: &RuleContext) -> String;  // {{}} 展开
```

必须先有一套**基于 legado 真实规则串的单元测试**（从 `legado-with-MD3` 的测试资源或公开书源集抽 50 条），红→绿驱动。

2026-08-31 进展：`source_engine/rule/` 已实现平衡扫描、`||` / `&&` / `%%` 组合、
Default / XPath / JSONPath / JS / WebJS / Regex 模式识别、`##` 替换、`@put` / `@get` 与模板变量收集；
`tests/rule_analyzer.rs` 对 `tests/fixtures/rules/legado_rules.jsonl` 的 85 条代表性语法逐条做快照断言。
当前夹具主要来自本机 legado 语法文档和默认源，**尚不能冒充 50 条公开真实书源样本**；4.2.1 的最终验收仍需用公开书源语料替换或补充。

**4.2.2 XPath / JSONPath / Regex（3–4 天）— ✅ evaluator 已完成并接入管线，待公开语料验证覆盖率**

| 能力 | Rust crate 建议 | 备注 |
| --- | --- | --- |
| XPath | `sxd-xpath` + `sxd-document`，或 `skyscraper` | 需先把 `scraper` 的 HTML 转成 XML DOM；注意 HTML 容错解析 |
| JSONPath | `jsonpath-rust` | 成熟度好 |
| Regex | `regex`（已有依赖） | 注意 legado 用的是 Java 正则，`\p{...}`、反向引用有差异，需做兼容层与降级 |

2026-09-01 进展：`source_engine::rule::execute_rule` 已提供统一模式调度；
`execute_json` 支持 legado 常用的对象键、数组索引、`[*]` 通配符，`execute_xpath`（已拆到
`rule/xpath.rs`）支持容错 HTML 上的节点、`text()`、属性、属性等值/包含谓词与位置谓词，
Regex 保留捕获组、替换、反向处理。三种模式均有单元测试，当前不支持的复杂 XPath（轴、函数组合）
和 JSONPath 递归下降会返回明确错误，不会静默返回空结果。

**4.2.2b JSoup(Default) 模式 + 编排层 + 接入真实管线（2026-09-01）— ✅**

> 本小节是复核 4.2.1/4.2.2 时**补记的缺口**，原路线图没有把它单列出来，导致 4.2.2 完成后
> 引擎实际上仍是悬空代码。教训与 §10.5 同源：「evaluator 写完」不等于「引擎能跑」。

发现的问题（2026-09-01 复核）：

| 事实 | 说明 |
| --- | --- |
| `execute_rule` 系列在生产路径**零调用点** | 只有 `compat.rs` 用 `split_rule` 做导入兼容判定，其余全在测试里 |
| `014` 写入的 `rule_*` 四列**只写不读** | `SOURCE_SELECT` 与 `SourceRow` 都不含这四列 |
| **legado 最主流的 Default 模式直接报 `UnsupportedMode`** | 4.2.2 只列了 XPath/JSONPath/Regex，漏掉了 `AnalyzeByJSoup` 对应的私有语法 |
| `\|\|` / `&&` / `%%` / `-` / `@put` / `@get` 解析出来了但无人执行 | `RuleAlternatives` 只被测试做指纹快照 |

本轮补上：

- `rule/jsoup.rs` + `rule/step.rs` —— Default 模式求值器：私有语法（`class.` / `id.` / `tag.` /
  `text.` / `children`，正负索引）与 CSS 混用、`@` 分步、终结符 `text` / `textNodes` / `ownText` /
  `html` / 任意属性。区间（`.0:1:2`）与排除（`!`）返回 `UnsupportedJsoup`，**不静默返回空**
- `rule/engine.rs` —— 编排层：`||` 逐个尝试取首个非空、`&&` 对同一输入求值后拼接、`%%` 交替合并、
  `Chain` 链式传递、`{{}}` 展开与 `@put`/`@get` 上下文。`Js` 已接入 QuickJS，`WebJs` 仍返回明确错误
- **值模型**：全程 `Vec<String>`。列表规则以 `Extraction::Nodes` 返回元素 outerHTML，逐项规则重新
  parse 该片段。三种模式共用一套签名，代价是每项一次重解析
- `source_engine/legado_rules.rs` —— `rule_*` 列的类型化（`serde(alias)` 吸收键名变体；
  `ruleContent` 为裸字符串时视作 `{content: "..."}`）
- `source_engine/pipeline.rs` —— **引擎优先、CSS 投影兜底**。有原始规则且解析出非空结果就走引擎；
  规则缺失 / 无法执行 / 结果为空则回退 `selector.rs`。回退点集中在这一个文件，便于日后一次性删除
- `raw_rules` 打通：`SourceImport` → `BookSource` → `upsert`（并入同一条 SQL，去掉 `save_raw_rules`
  的额外往返）→ `SOURCE_SELECT` 读回。`source_service::raw_rule_objects` 的重复 JSON 遍历随之删除
- 三个 service（search / reader / book）改用 `pipeline::*`，返回 `AppError`，
  顺带收敛 §10.5 第 1 条的 `Result<_, String>` 债

**语义决策：手动保存书源会清空 `raw_rules`。** UI 表单只能新建、保存后清空，按名字保存等于接管该书源；
若残留的 legado 规则继续优先，用户手写的 CSS 会被静默覆盖。

**顺带修掉一个真实 bug：** XPath 的 `contains()` 转译没吃掉外层方括号，
`//div[contains(@class,'x')]` 会译成非法的 `div[[class*='x']]`。拆分 `xpath.rs` 时新增的单测抓到的。

**4.2.3 JS Runtime（1 周）— 🟡 基础实现已落地（2026-09-01）**

按 `plan.md` §2 先定 trait：

```rust
#[async_trait::async_trait]
pub trait JsRuntime: Send + Sync {
    async fn execute(&self, script: &str, context: JsContext) -> Result<JsValue, AppError>;
}
```

**选型决策：`rquickjs`**

| 候选 | 评估 |
| --- | --- |
| **`rquickjs`（QuickJS 绑定）** ✅ 推荐 | ES2020 完整、启动快、内存小、易做沙箱与执行超时；缺点是需要 C 工具链 |
| `boa` | 纯 Rust、无 C 依赖，但性能弱、ES 覆盖不全，跑复杂书源脚本会踩坑 |
| `deno_core` | 功能最强但体积与启动开销过大，不适合每次规则执行都起一次 |

沙箱要求（`plan.md` §16）：禁文件系统、禁进程、禁环境变量；网络只走注入的 `java.*` API；**必须有执行超时（默认 5s）与内存上限**，防死循环。

**4.2.4 `java.*` 兼容层（与 4.2.3 同期）— 🟡 纯函数子集已落地**

legado 的 `JsExtensions.kt` 暴露 **100 个方法**。不要全做。第一批只做高频 15 个：

```
ajax  get  post  head  connect
base64Encode  base64Decode  hexEncodeToString  hexDecodeToString
md5Encode（在 EncoderUtils）  strToBytes  bytesToStr
encodeURI  timeFormat  log
```

当前已注入 `get`/`put`（上下文变量）、`base64Encode`/`base64Decode`、`encodeURI`、`log`；
网络型 API（`ajax`/`get`/`post`/`head`/`connect`）及编码/时间辅助函数仍待接入统一 HTTP 客户端。

第二批（Step 3 之后按实际书源失败率决定）：
```
cacheFile  getCookie  downloadFile  t2s/s2t  htmlFormat  toNumChapter
queryTTF / replaceFont   ← 字体反爬，工作量大，单列
webView / startBrowser   ← 需要 Tauri WebView 通道，Step 7 再评估
```

明确**不做**：`toast` / `longToast` / `androidId` / `getReadBookConfig*` / `getThemeConfig*`（Android 专有）。

**4.2.5 重写 legado 导入器（2–3 天）**

- 规则串**原样入库**，不再 `split("&&")`、不再 `strip_prefix("@css:")`
- 数据库 schema 从「每种规则一列」改为「规则 JSON 列」，对齐 legado：
  ```sql
  ALTER TABLE book_sources ADD COLUMN rule_search TEXT;    -- JSON
  ALTER TABLE book_sources ADD COLUMN rule_book_info TEXT;
  ALTER TABLE book_sources ADD COLUMN rule_toc TEXT;
  ALTER TABLE book_sources ADD COLUMN rule_content TEXT;
  ALTER TABLE book_sources ADD COLUMN rule_explore TEXT;
  ```
  旧列保留一个版本周期，写迁移脚本搬运，之后清理。
- 补齐 legado 顶层字段：`bookSourceGroup`、`bookSourceType`、`bookUrlPattern`、`weight`、`customOrder`、`respondTime`、`enabledExplore`、`exploreUrl`

### 4.3 BookSource 字段补齐

对照 legado（`data/entities/BookSource.kt` 27 字段 + 6 个 Rule 类）：

| Rule 类 | legado 字段数 | 当前支持 | 本步目标 |
| --- | ---: | ---: | --- |
| `SearchRule` | 11 | 5 | 补 `intro / kind / lastChapter / updateTime / wordCount / checkKeyWord` |
| `BookInfoRule` | 13 | 3 | 补 `init / kind / lastChapter / updateTime / coverUrl / tocUrl / wordCount / canReName / downloadUrls` |
| `TocRule` | 10 | 3 | 补 `nextTocUrl`(Step 1 已做) `isVolume / isVip / isPay / updateTime / preUpdateJs / formatJs` |
| `ContentRule` | 11 | 1 | 补 `nextContentUrl`(Step 1 已做) `title / subContent / sourceRegex / replaceRegex / imageStyle / webJs` |
| `ExploreRule` | 10 | 0 | 全新，配合发现页 |
| `ReviewRule` | 10 | 0 | **不做**（段评，优先级 P5） |

### 4.4 验收标准

- [ ] `RuleAnalyzer` 对 50 条真实 legado 规则串的切分结果与预期一致（单测）
- [ ] 导入一份 ≥300 源的公开 legado 书源集，**可成功搜索出结果的源 ≥ 60%**
- [ ] XPath / JSONPath / Regex / JS 四种模式各有 fixture 回归测试
- [ ] JS 死循环脚本 5 秒内被终止且不影响主进程
- [ ] 单个书源规则执行 P95 < 200ms（不含网络）

---

## 5. Step 3 · 书源调试器 + 回归测试体系（1 周，与 Step 2 并行）

`plan.md` §17 把调试器排在 Phase 5 末尾。**本路线图提前，与 Step 2 同步开发** —— 没有它，规则引擎的正确性只能靠盲改验证。

### 5.1 调试器

参考 legado `model/Debug.kt` + `ui/book/source/`。UI 布局按 `plan.md` §17：

```text
┌──────────────────────┬─────────────────────────┐
│ 规则编辑              │ 执行结果                 │
│  搜索 / 详情 /        │  原始 HTML               │
│  目录 / 正文 / 发现    │  每步中间结果             │
│                      │  最终解析结果（JSON）      │
│  [单步执行] [全流程]   │  请求 Header / 耗时        │
└──────────────────────┴─────────────────────────┘
```

必备能力：
- 输入 URL / 关键词，按阶段单步执行
- 每条规则显示：输入片段 → 匹配节点数 → 输出值
- 显示实际发出的请求（URL / method / headers / body）与响应状态、耗时
- JS 规则显示 `console.log` 输出与异常栈
- 后端通过 `source-test-progress` 事件流式推送（`events.ts` 已预留该事件名）

### 5.2 回归测试体系

按 `plan.md` §31 建 fixture：

```text
src-tauri/tests/
├── fixtures/
│   ├── source_a/{search.html, book.html, toc.html, chapter.html, source.json}
│   ├── source_b/...
│   ├── source_c/...                 ← legado 原生语法书源 ✅
│   └── rules/legado_rules.jsonl     ← 85 条代表性规则串 + 期望切分结果 ✅
├── rule_analyzer.rs                 ← ✅ 夹具逐条快照
├── rule_engine.rs                   ← ✅ 引擎端到端（source_c）
├── source_pipeline.rs               ← ✅ CSS 路径 + 双路径一致性
├── step0_regressions.rs             ← ✅
└── js_runtime.rs                    ← ✅ QuickJS 基础运行时；网络 `java.*` 与 WebJS 待补
```

（selector 各模式的单测就近放在 `source_engine/rule/{jsoup,xpath,evaluator,engine,step}.rs` 内联，
不再单列 `selector_*.rs` 文件。）

同时补 Step 0/1 欠的测试：`epub` 解析、`decode_text` 各编码、`split_chapters` 边界、B1–B5 各一条。

**2026-09-01 现状：** `epub`（10 条）/ `txt`（11 条）/ B1–B4 均已补齐。
`tests/fixtures/source_a/` `source_b/` `source_c/` 各含搜索、详情、目录、正文页面：
前两者由 `tests/source_pipeline.rs` 覆盖 CSS 路径与双路径一致性，`source_c/` 是**用 legado 原生语法
书写**的书源，由 `tests/rule_engine.rs` 端到端驱动引擎路径。
仍缺的是基于**公开语料**的更大规模容错样本 —— 见 §10.6 剩余项第 3 条：人工造的夹具会悄悄迁就实现。

### 5.3 验收标准

- [ ] 调试器能定位一个真实失败书源的失败步骤，并在不重启应用的前提下改规则重试
- [ ] 测试覆盖：`source_engine` 行覆盖率 ≥ 70%
- [ ] CI 跑全量 fixture < 30s

---

## 6. Step 4–7 概要

### Step 4 · 下载 / 缓存（2 周）

按 `plan.md` §18–21。

- `DownloadManager`：Queue + Scheduler + Worker + RetryPolicy + Persistence
- 任务状态机：`Pending / Running / Paused / Completed / Failed / Cancelled`，**状态必须落库**（`plan.md` §39 第五条）
- 批量写入走单事务（`plan.md` §29）
- 缓存三级：Memory LRU → SQLite → Network
- 新建 `scheduler/` 模块；启动时恢复未完成任务
- 参考 legado：`CacheBookService.kt` / `DownloadService.kt` / `ExportBookService.kt`

### Step 5 · 阅读排版引擎（2 周）

当前阅读器是 `content.split('\n')` 渲染成 `<p>` 塞进滚动 div（`App.vue:234`）。这在 10 万字章节上会卡死（`plan.md` §27.1 明确点名）。

参考 legado `ui/book/read/page/provider/`（`ChapterProvider` / `TextChapterLayout` / `TextPageFactory` / `ZhLayout`）与 `entities/`（`TextChapter` / `TextPage` / `TextLine` / `TextColumn`）。

- 真分页模式（当前只有滚动）
- 长章节增量渲染 / 虚拟化
- 主题走 Design Tokens（`plan.md` §8 明确要求不写死颜色），至少 Light / Dark / Sepia / Green / Black
- 净化替换规则（legado `ReplaceRule` + `ContentProcessor`）
- 书签、章内全文搜索、阅读时长统计（legado `bookmark` / `searchContent` / `readRecord`）

翻页动画（仿真/覆盖/滑动）优先级低，桌面端价值有限，可只做「滚动 + 平移分页」两种。

### Step 6 · RSS / 备份 / 设置（2 周）

- RSS：独立于书源引擎（`plan.md` §22）。参考 legado `ui/rss/{source,subscription,article,read,favorites}`
- 备份 / 恢复：参考 legado `help/storage/{Backup,Restore,BackupAES}`。产出 `backup.zip`（`plan.md` §24）
- WebDAV 同步：**推迟到 Step 7 之后**，`plan.md` §41 明确列为 P5
- 设置系统按 `plan.md` §26 分区

### Step 7 · 性能 / 稳定性 / 发布（2 周）

- SQLite：WAL、索引、Prepared Statement、批量事务（`plan.md` §29）
- 异常场景矩阵（`plan.md` §30）：网络超时/断网/DNS/TLS/500/429/403；DB 锁/损坏/磁盘满；文件缺失/无权限；JS 死循环/超时/大对象
- 桌面体验（`plan.md` §32）：启动速度、窗口恢复、快捷键、`.txt`/`.epub` 文件关联、Command Palette
- 打包 Windows + Linux，Installer + Portable，Updater + DB migration 前自动备份（`plan.md` §34）

---

## 7. 明确不做的部分

对照 legado 的完整功能面，以下**本路线图周期内不做**，避免范围蔓延：

| 功能 | legado 位置 | 不做的理由 |
| --- | --- | --- |
| TTS / 朗读 / 有声书 | `TTSReadAloudService`, `HttpReadAloudService`, `AudioPlayService` | 桌面端需另找 TTS 后端，独立工作量 |
| 漫画阅读 | `ui/book/manga` | `bookSourceType=2` 图片源，另一套渲染 |
| AI 相关 | `ui/ai`, `data/entities/Ai*.kt`（8 个实体） | `plan.md` §1 已列为「暂时不做」 |
| 段评 | `ReviewRule`, `BookChapterReview` | 依赖账号体系 |
| 局域网 Web 服务 | `web/KtorServer.kt` | 等桌面端本体稳定 |
| 字体反爬 | `queryTTF` / `replaceFont` | 单独立项，工作量大 |
| UMD / MOBI / PDF | `localBook/{Umd,Mobi,Pdf}File.kt` | Step 5 之后按需求评估 |
| 词典 | `ui/dict`, `DictRule` | 低频 |
| WebView 登录 | `ui/login`, `JsExtensions.webView*` | Step 2/3 先做认证与 Cookie 回写最小闭环；复杂自动化挑战 Step 7 后评估 |

---

## 8. 里程碑与版本

| 版本 | 对应 Step | 内容 | 状态 |
| --- | --- | --- | --- |
| v0.1.0 | 现状 + Step 0 | 本地阅读闭环 + 干净架构（内部版，不发布） | ✅ **已达成**（`package.json` 现为 0.1.0） |
| v0.2.0 | Step 1 | 在线阅读可用（CSS 书源） | 🟡 代码就绪，卡在 §3.3 手工验收 |
| v0.2.1 | Step 2/3 | 协议认证 + WebView Cookie 回写（Cloudflare/Turnstile 人工认证） | ⬜ |
| v0.3.0 | Step 2 + 3 | 完整规则引擎 + 调试器 —— **真正的「Legado 桌面版」起点** | ⬜ 未开始 |
| v0.4.0 | Step 4 | 下载 / 缓存 | ⬜ |
| v0.5.0 | Step 5 | 阅读排版引擎 | ⬜ |
| v0.6.0 | Step 6 | RSS / 备份 / 设置 | ⬜ |
| v1.0.0 | Step 7 | 性能 / 稳定性 / 打包发布 | ⬜ |

---

## 9. 风险登记

| 风险 | 影响 | 缓解 |
| --- | --- | --- |
| Step 2 规则引擎工期超预期 | 阻塞后续全部在线功能 | 严格按 4.2 顺序；`RuleAnalyzer` 单独验收后再往下；调试器并行以缩短反馈环 |
| `rquickjs` 引入 C 工具链，影响跨平台构建 | Linux/macOS 构建失败 | Step 2 开始前先做 spike：在 Windows + Linux 各构建一次 hello-world |
| XPath crate 与 HTML 容错解析不匹配 | XPath 规则大面积失败 | spike 阶段用真实 HTML 验证 `scraper` → XML DOM 的转换质量；不行则退回 `libxml` 绑定 |
| Java 正则与 Rust `regex` 语义差异 | 部分书源正则规则失效 | 建立差异清单；不支持的语法（反向引用等）明确降级并在调试器提示 |
| Cloudflare / JS challenge | 部分站点完全不可用 | Step 2/3 先做 WebView 认证 + Cookie 回写 spike；复杂挑战与自动化绕过仍列入 Step 7 评估 |
| ~~架构重构引入回归~~ | ~~Step 0 拖长~~ | **已关闭** —— Step 0 与 08-31 的 `SourceService` 拆分均以小步提交 + 每步全绿完成，未发生回归 |
| **业务在 service 层重新聚团** | 每个 Step 结束都会产生新的巨型文件 | **新增（2026-08-31）** —— Step 0 后 `SourceService` 涨到 617 行。约定：每个 Step 收尾时复查最大文件，非测试行 > 250 即拆 |
| **真实 HTML 语料规模不足** | 当前 fixture 只覆盖三种人工整理的站点结构，真实站点容错仍未知 | **升级（2026-09-01）** —— 已补 `source_c/`（legado 原生语法）。但发现 `source_a/source.json` 原本是**照着导入器行为写的、并非 legado 真实写法**，说明自造夹具会迁就实现。公开语料从「补量」升级为**验收前置** |
| **规则引擎覆盖率无法自测** | §4.4 的「≥300 源，可搜索 ≥ 60%」在本机无从度量 | **新增（2026-09-01）** —— 引擎已能跑，但不支持的语法只在遇到时报错。需要公开书源集跑一次统计，才能知道 Default/XPath/JSON 三种模式的真实缺口在哪 |

---

## 10. 当前进度（2026-08-31 复核后重写）

> 本节是**唯一权威的进度来源**。上一版本此处曾 11 项全部打勾、与实测不符；
> 本次复核逐条回到代码验证，并把**验证不通过的三条如实降级**（见 §10.5）。
> 规则：这里的每一条都必须能用一条命令或一个文件路径证明。

### 10.1 代码规模现状（2026-08-31 实测）

| 层 | 规模（2026-09-01 实测） |
| --- | --- |
| Rust | 6 492 行，57 个文件；最大 `service/reader_service.rs` 220 非测试行 |
| 前端 | 1 545 行，31 个文件；最大 `features/source/useSources.ts` 193 行 |
| DB | `migrations/001..014` 共 14 个文件（本轮无新增 migration） |
| 测试 | **94 个库单测 + 12 个集成测试**（规则分析器 85 条夹具 + 引擎端到端 6 条 + 管线 3 条 + Step 0 回归 2 条） |

对照 §0.1 的起点：`command.rs` 1 226 行 + `source.rs` 484 行 + `App.vue` 264 行这三座大山已全部拆散，
单文件最大值从 1 226 降到 208。

模块目录与 §2.1 规划**完全一致**（`app` / `command` / `domain` / `service` / `repository` / `source_engine` / `infrastructure`），仅 `scheduler/` 按计划留到 Step 4。

`cargo clippy --all-targets -- -D warnings` 通过，且**全项目零 `allow(dead_code)` / `allow(unused)`**。

### 10.2 Step 0 · 已完成

| 项 | 证据 |
| --- | --- |
| B1 正文缓存 | `chapter_contents` 表 + 011 migration；`fetch_online_content` 命中/未命中分支；2 条回归测试 |
| B2 目录刷新 | `refresh_catalog` command；`list_chapters` 改为纯本地读；发 `chapter-updated` |
| B3 proxy_url | `repository/source.rs` upsert 含该列 + `upsert_round_trips_proxy_url` |
| B4 sign_script | 签名逻辑移出 `header` 分支；`infrastructure/http/request.rs` 5 条测试 |
| B5 migration 漂移 | 012 空操作迁移 + 回归测试（**仅文本断言，见 §10.5**） |
| AppError | `map_err(\|e\| e.to_string())` 为 0；**全部 26 个 command（`command/` 25 个 + `lib.rs` 的 `health_check`）返回 `AppError`** |
| command/ 边界 | 7 个文件共 417 行，最大 153 行；`command/` 下无任何 `sqlx::query` |
| ebook 解析下沉 | `infrastructure/ebook/{mod,txt,epub}.rs`，纯函数、无 DB 依赖，23 条测试 |
| 导入编排 | `BookService::{import_txt, import_epub}`；书+章节单事务写入；不再用 `list_books()` 全表扫描查单本 |
| 前端 feature 拆分 | 每 feature 一个 composable + 一个组件；无文件超过 200 行且不靠压行 |
| 死代码清理 | 删除 `legacy_command.rs`、`NewBook`/`BookRepository::create`、无用 pinia store、`lib.rs` 的 blanket `#[allow(dead_code)]` |
| 工具链 | 原生 CSS + Design Tokens；ESLint / Prettier / vue-tsc 全绿 |
| 日志 | 15 处 `tracing`，覆盖 `book`(3) / `reader`(3) / `source`(4) / `network`(3) / `database`(2) 五个 target |
| 事件桥接 | `events.ts` 真实 `listen`/`emit`，无 Tauri 时降级内存 bus；前端已订阅 `chapter-updated` |
| 文档校正 | `AppShell` 版本号改读 `package.json`；`ARCHITECTURE.md` 模块清单与实际目录对齐 |

### 10.3 Step 1 · 代码项已完成

- **1.1 正文缓存** — 进程级 50 章 LRU → SQLite `chapter_contents` 二级缓存
- **1.2 目录刷新** — 事件包含本次识别出的新增章节数（按章节序号 diff）
- **1.3 `nextTocUrl` 分页目录** — 最多 50 页，绝对 URL 归一化并做环检测
- **1.4 `nextContentUrl` 分页正文** — 最多 20 页，拼接后统一落入正文缓存并做环检测
- **1.5 书籍详情页** — `fetch_book_info` command；简介/作者/封面 URL 写回书籍并在阅读器展示
- **1.6 封面本地缓存** — Rust 侧按书源请求封面并存为 data URL；书架优先展示本地缓存
- **1.7 换源** — 按书名重搜切换书源，切换后清理章节、正文缓存与阅读进度
- **1.8 搜索去重** — 按 (标题, 作者) 归并跨源结果，同源重复 URL 丢弃；8 条测试
- **1.9 搜索失败可见性** — `search_books` 返回 `{ groups, failures, searched_sources }`，UI 折叠显示每个失败源及原因
- **3.1 导入警告** — `SourceImportReport.partial` + UI 提示（未加 `import_warnings` 列，但目的达到）
- **额外修复** — 新加入的在线书目录为空（`list_chapters` 本地化的副作用），`useReader.openBook` 现在自动拉一次目录

### 10.4 Step 2 前置准备：拆分 `SourceService`（2026-08-31）

Step 0 拆干净了 `command.rs`，但业务随后在 `SourceService` 里重新聚成 **617 行** —— 全项目最大文件，且 Step 2 要重写的导入与规则检测路径正落在其中，故先行拆分：

- `service/search_service/` —— `SearchService` 接管并发搜索、结果归并与单源探测；`grouping.rs` 单独承载去重归并与 8 条测试
- `source_engine/compat.rs` —— 不支持规则检测从 service 层下沉到规则引擎层，Step 2 将直接取代该模块
- `domain/source.rs` —— 新增 `BookSource::from_import`，取代原 23 行字段拷贝；用具名构造器而非 `From`，因该转换会伪造 `id: 0` 并丢弃会话凭据
- `infrastructure/http/url.rs` —— `resolve_url` 三份重复实现合并为一处，错误文案统一为 `{label} 无效`（顺带修两处空格笔误）
- `infrastructure/http/client.rs` —— 抽出 `base_builder` + `build_shared_client`；`import_url` 的客户端刻意保留手写（cookie/redirect 语义不同）

结果：`SourceService` 降至 230 行（非测试 202），只余书源 CRUD、legado 导入与鉴权。鉴权按 §0.6 冻结决定原地保留。**纯重构：零行为变化、零 serde 形状变化、前端零改动**（`git status src/` 为空）。库单测 48 → 52。

### 10.5 复核中发现的名不副实项（已如实降级）

| # | 原声明 | 实测 | 处置 |
| --- | --- | --- | --- |
| 1 | 「`Result<_, String>` 均为 0」 | `source_engine/` 仍有 **7 个函数**返回 `Result<_, String>` | **2026-09-01 部分关闭：**新的 `pipeline.rs` 四个入口返回 `AppError`，三个 service 调用点的 `.map_err(AppError::parse)` 已删除。`selector.rs` 作为兜底后端仍返回 `String`，`parse_sources_json` 亦然 —— 待兜底路径删除时一并清理。 |
| 2 | 「`command/` 6 个文件均 < 100 行」 | 7 个文件，最大 `command/source.rs` **153 行** | §2.8 的口径是 200 行，达标。表述已按实测改写。 |
| 3 | 「B1–B5 各有一条回归测试」 | B1–B4 是真实行为测试；**B5 的两条集成测试是文本断言**（`include_str!` 后查字符串），不验证 migration 行为 | 保留（migration 空操作本身难以行为化断言），但已在 §10.2 标注，不再宣称为行为回归。 |
| 4 | 「vue-router 提供三个可直接访问的路由」 | 三条路由 `/` `/search` `/settings` **全部指向同一个 `AppShell`**，页面切换实际靠组件内部状态 | 字面成立（hash 地址可进入），但不是页面级路由。已在 §0.5 注明，真正拆分留到 Step 5 阅读器独立成页时。 |

### 10.6 下一步（认证优先级调整后）

**Step 1 的纯 CSS 验收仍需人工提供 3 个真实书源，但不再是后续开发的唯一阻塞。**
认证相关工作按新的 Step 2/3 轨道推进；手工验收分为“无需浏览器认证”和“需浏览器认证”两组：

下一项代码顺序：协议认证补齐（JS `java.*` 网络 API、会话刷新与过期状态）→ WebView
认证 spike（Cloudflare/Turnstile Cookie 回写）→ 调试器展示认证请求与失败步骤。

1. 搜索 → 详情 → 加书架 → 目录（含分页）→ 正文（含分页）→ 换源 → 刷新目录出新章
2. 同一章第二次打开零网络请求
3. 导入 ≥100 源的真实 legado JSON，核对「完全支持 / 部分支持 / 不支持」的判定准确性

通过无需浏览器认证的链路即可发 **v0.2.0**；Cloudflare/WebView 认证闭环单独作为 **v0.2.1** 发布门槛。

**Step 2：规则引擎已接入真实管线（2026-09-01）。**

> 上一版此处写「代码侧无待办」，**不属实**。复核发现 analyzer 与三个 evaluator 在生产路径上零调用，
> `014` 写入的原始规则只写不读，最主流的 Default 模式还直接报错 —— 引擎当时是一段悬空代码。
> 详见 §4.2.2b。这已按实修正，本节不再复述。

本轮可验证证据：

- `cargo test`：**94 个库单测 + 12 个集成测试**全部通过
- `cargo clippy --all-targets -- -D warnings`：通过，零 `allow` 属性
- `cargo fmt -- --check` / `npm run lint` / `format:check` / `build`：全绿
- `tests/rule_engine.rs`（新增 6 条）：`fixtures/source_c/` 是一份**用 legado 原生语法书写**的书源
  （私有 JSoup、`@XPath:`、`||`、`##`、`text.`）。其 CSS 投影列**故意设成匹配不到的选择器**，
  因此断言通过就只可能是引擎路径产出的，兜底冒充不了
- `tests/source_pipeline.rs` 新增双路径一致性断言：`source_a` 同一份 HTML 用「CSS 投影」与
  「legado 原始规则」两种描述分别跑，四个阶段逐项相等 —— 这是「接入引擎不会让原本能用的书源回归」的核心保证
- `source_service` 测试改为断言 `list()` 读回的 `BookSource.raw_rules` 内容，闭合「只写不读」
- 单文件非测试行最大 220（`reader_service.rs`），新增文件均 < 250

**Step 2 剩余项（均需外部环境或后续开发）：**

本轮已完成协议认证基础链路：QuickJS 的 java.* 网络 API 共享 Cookie 会话，并携带未过期的
token/cookie/header/sign；书源管理界面已接入会话刷新，过期状态也会在 JS 和普通请求路径中跳过凭据。

1. **JS Runtime（4.2.3）+ `java.*` 兼容层（4.2.4）** —— 基础 QuickJS Runtime 已接入 `Js` 规则：
   `result`/`url`/变量上下文、`java.get`/`put`、Base64、URI 编码、日志、内存上限与异步超时均已覆盖，
   并有运行时及链式规则回归测试。仍待 `WebJs`、完整网络型 `java.*` API、Linux 构建验证；
2. **协议认证补齐** —— 将现有登录/token/cookie/sign 能力接入 JS `java.*` 网络 API，增加会话刷新、
   过期处理和调试器中的请求/认证状态展示；这是下一项代码优先级，完成后再扩大真实书源验收。
3. **WebView 认证 spike** —— 在 Step 2/3 期间验证 Cloudflare/Turnstile 页面认证后 Cookie 回写；
   只做用户可控的浏览器认证，不实现无头绕过。
4. **公开真实书源语料** —— 当前 85 条语法夹具与 3 份 HTML fixture 都是人工整理的，
   §4.4 的「≥300 源导入，可搜索 ≥ 60%」无法在本机度量。需要外部语料
3. **`fixtures/source_a/source.json` 此前不忠实** —— 它是照着导入器的 `attr()` 补全行为写的
   （`bookUrl: "h2 a"` 靠导入器补 `::attr(href)`），并非 legado 真实写法。本轮已改为 `h2 a@href`。
   这提示：**人工造的夹具会悄悄迁就实现**，公开语料的必要性不只是「量」，也是「不被自己的实现污染」

### 10.7 验收命令

```bash
cargo test   --manifest-path src-tauri/Cargo.toml --target-dir src-tauri/.cargo-target
cargo clippy --manifest-path src-tauri/Cargo.toml --target-dir src-tauri/.cargo-target              --all-targets -- -D warnings
npm run lint && npm run format:check && npm run build
```

当前实测（2026-09-01）：**94 库单测 + 12 集成测试全过**，clippy 零告警且无任何 allow 属性，
`cargo fmt -- --check` 干净，前端三项全绿。

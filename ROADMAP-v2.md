# Reader Desktop 路线图 v2

> **与 v1 的关系。** `ROADMAP.md`（v1）是从**自身代码审计**出发写的，回答「欠了什么债、下一个修哪个」。
> v2 是从**目标项目 `D:\Code\chatting\legado-with-MD3` 的能力面**反向出发写的，回答「离能被称作
> Legado 桌面版还差什么、按什么顺序补」。
>
> v1 的 §0–§10 作为**历史记录与已完成项的证据**保留，不废弃；**排期与优先级一律以本文件 §2 为准**。
> 两者冲突时以 v2 为准。
>
> 编写日期：2026-09-02。两侧数据均为当日实测，命令见 §11。
> **2026-09-02 修订：** 970 源真实语料已到位（`f3f55c6e-…json`），§0.3 起的所有优先级
> **已按实测数据重排**，不再是预估。初稿中基于猜测的排序（先 XPath、后 JSoup 细节）被证明是错的。

**一句话现状：架构已达标、规则引擎骨架已通、认证链路已闭环 —— 但真实语料实测覆盖率只有 31.3%，而阅读器本体几乎为零。**

---

## 0. 与目标项目的量化对照

### 0.1 体量

| 维度 | legado-with-MD3 | reader-desktop | 比值 |
| --- | --- | --- | ---: |
| 主代码 | 290,943 行 Kotlin / 1,631 文件 | 8,012 行 Rust / 59 文件 + 1,652 行 TS·Vue / 31 文件 | 3.3% |
| 数据实体 | `data/entities/` 47 个 | 5 张业务表 + `chapter_contents` | — |
| **规则引擎** | `model/analyzeRule/` 3,377 行 / 10 文件 | `source_engine/` 3,744 行 / 17 文件 | 111% |
| **抓取编排** | `model/webBook/` 1,560 行 / 5 文件 | `pipeline.rs` 185 行 + 3 个 service | ~40% |
| **排版引擎** | `read/page/{provider,entities}` 5,227 行 / 16 文件 | `ReaderPane.vue` 59 行 | ~1% |
| JS 扩展面 | `JsExtensions.kt` 100 个方法 | `java.*` 19 个 | 19% |
| 书源调试器 | `model/Debug.kt` 422 行 | 骨架 350 行（未提交，见 §5） | ~30% |
| 阅读器 UI | `ui/book/read/` 139 文件 | 1 个组件 + 1 个 composable | — |

> 行数比值会误导：legado 800 个 `ui/` 文件里大量是 Android 特有的 Activity / Fragment / 自定义 View，
> 不可直接换算成桌面端工作量。真正决定「是不是 Legado 桌面版」的只有三块：
> **规则引擎、抓取编排（AnalyzeUrl + webBook）、排版引擎**。
> 第一块行数已超过参考实现，**但实测只能完整执行 31.3% 的真实书源**（§0.3）；第二块缺一半；第三块基本为零。
> **行数不是进度。** 这张表最有价值的一行是「规则引擎 111%」与「覆盖率 31.3%」的并置。

### 0.2 能力矩阵

图例：✅ 达标 · 🟡 部分 · ⬜ 未开始 · ⛔ 明确不做

**A 层 · 规则求值（`analyzeRule` 对位）**

| 能力 | legado | 现状 | 缺口 |
| --- | --- | :---: | --- |
| 规则串词法切分 `\|\| && %% - ##` | `RuleAnalyzer.kt` 377 | ✅ | `analyzer.rs` 225 + `scanner.rs` 155，85 条夹具快照 |
| 模板 `{{}}` / `@get:` / `@put:` | `AnalyzeRule.kt` 971 | ✅ | — |
| Default(JSoup) 私有语法 | `AnalyzeByJSoup.kt` 524 | ✅ | 排除、索引列表/区间、`@@`、allInOne 及文本边界已对齐；位置筛选拆至 `position.rs` |
| XPath | `AnalyzeByXPath.kt` 155 | 🟡 | `xpath.rs` 165 手写译码器，无轴/函数/复合谓词。**实测仅 6% 源使用** |
| JSONPath | `AnalyzeByJSonPath.kt` 172 | 🟡 | 缺递归下降 `..`（8%）、过滤 `?(@.x)`（4%） |
| Regex | `AnalyzeByRegex.kt` 57 | ✅ | Java 正则差异需差异清单 |
| JS 规则 | Rhino（`modules/rhino`） | 🟡 | QuickJS，`js_runtime.rs` 847 行；超时/内存上限已有。**Rhino 的 JVM 包访问无法支持（1% 源）** |
| WebJs | ✅ | ⛔ | **实测 970 源零出现，降级为不做** |
| **URL 构造 `AnalyzeUrl`** | **1,007 行，六阶段共用** | 🟡 | **仅 search 阶段的子集（29% 源受影响，边际收益最高）—— 最大结构性缺口，见 §1.2** |
| `jsLib` 书源级 JS 库 | `SharedJsScope.kt` | ⬜ | 代码库 0 处引用；实测仅 6 源（1%）使用 |
| `concurrentRate` 限速 | ✅ | ⬜ | 只有全局 `Semaphore(8)`；实测 57 源（6%）配置 |

**B 层 · 抓取编排（`webBook` 对位）**

| 能力 | legado | 现状 |
| --- | --- | :---: |
| 搜索 / 详情 / 目录 / 正文 四阶段 | `BookList` `BookInfo` `BookChapterList` `BookContent` | ✅ |
| 目录分页 `nextTocUrl` | ✅ | ✅ 上限 50 页 + 环检测 |
| 正文分页 `nextContentUrl` | ✅ | ✅ 上限 20 页 + 环检测 |
| 章节去重 / 卷识别 `isVolume` | ✅ | ⬜ |
| `preciseSearch` / `checkKeyWord` | ✅ | ⬜ |
| 内容后处理 `ContentProcessor` | `help/book/ContentProcessor.kt` | ⬜ 一条都没有 |
| **发现页 Explore** | `exploreUrl` + `ruleExplore` | ⬜ 代码库 0 处引用；**实测 67% / 46% 填充率**，已提前到 §4.6 |
| 封面兜底规则 `coverRule` | `defaultData/coverRule.json` | ⬜ |

**C 层 · 书源模型（`BookSource.kt` 27 字段 + 6 个 Rule 类）**

| Rule 类 | legado 字段 | 现有 | 缺 |
| --- | ---: | ---: | --- |
| 顶层 `BookSource` | 27 | ~12 | `bookSourceGroup` `bookSourceType` `bookUrlPattern` `customOrder` `weight` `enabledExplore` `exploreUrl` `exploreScreen` `jsLib` `enabledCookieJar` `concurrentRate` `loginUi` `loginCheckJs` `coverDecodeJs` `respondTime` `lastUpdateTime` `bookSourceComment` `variableComment` |
| `SearchRule` | 11 | 5 | `checkKeyWord` `intro` `kind` `lastChapter` `updateTime` `wordCount` |
| `BookInfoRule` | 13 | 6 | `init` `tocUrl` `wordCount` `canReName` `downloadUrls` `relatedBooks` `updateTime` |
| `TocRule` | 10 | 4 | `preUpdateJs` `formatJs` `isVolume` `isVip` `isPay` `updateTime` |
| `ContentRule` | 11 | 2 | `title` `subContent` `webJs` `sourceRegex` `replaceRegex` `imageStyle` `imageDecode` `payAction` `callBackJs` |
| `ExploreRule` | 10 | 0 | 全部 |
| `ReviewRule` | 10 | 0 | ⛔ 不做 |

**D 层 · 阅读器**

| 能力 | legado | 现状 |
| --- | --- | :---: |
| 分页排版 | `ChapterProvider` 611 + `TextChapterLayout` 1,823 + `ZhLayout` 277 | ⬜ 只有滚动 |
| 长章节增量渲染 | `TextPageFactory` 169 | ⬜ 全量 `<p>` 渲染 |
| 主题 / 字体 / 间距配置 | `ReadBookConfig` + `ui/book/read/config/` | 🟡 只有主题名 + 字号 |
| 书签 / 章内搜索 / 阅读时长 | `Bookmark` `searchContent` `readRecord` | ⬜ |
| 净化替换规则 | `ReplaceRule` + `ContentProcessor` | ⬜ |
| 图片 / 漫画 | `ImageProvider` / `ui/book/manga` | ⛔ |
| TTS 朗读 | 3 个 Service | ⛔ |

**E 层 · 书籍与生态**

| 能力 | legado | 现状 |
| --- | --- | :---: |
| 本地格式 | txt / epub / umd / mobi / pdf | 🟡 txt / epub |
| TXT 目录规则 | `txtTocRule.json` **27 条实测正则** | 🟡 手写启发式 `split_chapters` |
| 下载 / 缓存 / 导出 | `CacheBookService` `DownloadService` `ExportBookService` | ⬜ |
| 换源 | `ui/book/changesource` + 章节对齐 | 🟡 按书名重搜，无章节对齐 |
| 书源批量校验 | `BookSourceCheckService.kt` | 🟡 只有单源探针 |
| Cookie 持久化 | `data/entities/Cookie.kt` + `enabledCookieJar` | 🟡 仅 reqwest 内存 store |
| 备份 / 恢复 | `help/storage/` 8 文件 | ⬜ |
| WebDAV 同步 | ✅ | ⬜ |
| RSS | `ui/rss/` + `model/rss/` | ⬜ |
| 书源调试器 | `Debug.kt` 422 | 🟡 骨架已有，见 §5 |
| AI / 词典 / 翻译 / 局域网 Web / 段评 | 8 个 AI 实体等 | ⛔ |

---

### 0.3 实测覆盖率基线（970 源语料，2026-09-02）

语料：`src-tauri/tests/corpus/f3f55c6e-723b-4055-b254-124c9d88c5cb.json`，**970 个真实书源，22,220 条规则串**。
远超 v1 §4.4 要求的 300 源门槛。

#### 0.3.1 头条数字

```
仅使用当前引擎已支持语法的书源：304 / 970 = 31.3%
含至少一个不支持语法的书源：      666 / 970 = 68.7%
```

**这是本项目第一个真实的覆盖率数字。** 此前所有「引擎已完成」的表述都缺这个分母。

#### 0.3.2 阻塞项排名与边际收益

「累计」列 = 按本列顺序依次修复后，可完整执行的书源占比。

| 阻塞语法 | 影响源数 | 占比 | 修复后累计 | 边际收益 | 现状 |
| --- | ---: | ---: | ---: | ---: | --- |
| **JSoup 排除 `!n`** | 299 | 31% | 44.9% | +13.6 | ✅ |
| **URL 选项对象 `,{...}`** | 277 | 29% | 62.9% | **+17.9** | 仅 search 阶段部分支持 |
| **JSoup 区间 `.a:b`** | 146 | 15% | 73.4% | +10.5 | ✅ |
| **JSoup `@@`** | 102 | 11% | 81.8% | +8.4 | ✅ |
| JSONPath 递归下降 `..` | 78 | 8% | 89.3% | +7.5 | 未实现 |
| XPath（全部形态） | 58 | 6% | 94.6% | +5.4 | 手写译码器，部分可用 |
| JSONPath 过滤 `?()` | 43 | 4% | 99.1% | +4.4 | 未实现 |
| **Rhino JVM 包访问** | 9 | 1% | 100.0% | +0.9 | ⛔ **原理上无法支持**，见 §0.3.5 |
| WebJs | **0** | **0%** | 100.0% | +0.0 | ⛔ **语料中零出现，降级为不做** |

> **只修前四项（三个 JSoup 细节 + AnalyzeUrl），覆盖率从 31.3% 直接到 81.8%。**
> 这四项都不需要引入新依赖，也不需要换解析引擎。

#### 0.3.3 语法 token 全频次（按覆盖源数）

已支持的部分同样值得记录 —— 它们证明 analyzer 的投入是有回报的：

| token | 源数 | 占比 | 状态 |
| --- | ---: | ---: | :---: |
| `{{ 模板 }}` | 961 | 99% | ✅ |
| `##` 替换 | 826 | 85% | ✅ |
| `&&` 串联 | 435 | 45% | ✅ |
| `!n` 排除 | 299 | 31% | ✅ |
| `@js:` | 286 | 29% | ✅ |
| URL 选项 `,{...}` | 277 | 29% | 🟡 |
| `\|\|` 备选 | 265 | 27% | ✅ |
| `<js>…</js>` | 226 | 23% | ✅ |
| `java.*` 调用 | 210 | 22% | 🟡 19/100 方法 |
| 负索引 `.-1` | 184 | 19% | ✅ 已实测确认 |
| JSONPath `$.` | 155 | 16% | 🟡 |
| 区间 `.a:b` | 146 | 15% | ✅ |
| `@@` | 102 | 11% | ✅ |
| `@put:` | 89 | 9% | ✅ |
| `@get:` | 79 | 8% | ✅ |
| JSONPath `..` | 78 | 8% | ⬜ |
| XPath `//` 裸写 | 59 | 6% | 🟡 |
| `@CSS:` | 30 | 3% | ✅ |
| `-` 反向 | 22 | 2% | ✅ |
| `%%` 交叉 | 8 | 1% | ✅ |
| `@Json:` | 4 | 0% | ✅ |
| **`@XPath:` 显式前缀** | **0** | **0%** | — |

**两个反直觉结论**（初稿据此排序错了）：

1. **XPath 总共只占 6%，且 `@XPath:` 显式前缀零出现** —— 全部是 `//` 裸写形态。
   初稿 P1.3 提议「换成真 XPath 引擎（`sxd-xpath` / `libxml`）」是为 6% 的源做一次大改造，
   **性价比不成立**。改为按需扩展现有译码器。
2. **`WebJs` 在 970 源中零出现** —— 初稿 P1.5 单列一节，现降级为「不做」。

#### 0.3.4 `java.*` 方法实测频次（决定补齐顺序）

| 方法 | 源数 | 占比 | 现状 |
| --- | ---: | ---: | :---: |
| `ajax` | 100 | 10% | ✅ |
| **`getString`** | 69 | 7% | ⬜ 在 JS 里执行一条规则 |
| `put` / `get` | 62 / 58 | 6% | ✅ |
| `timeFormat` | 40 | 4% | ✅ |
| `log` | 28 | 3% | ✅ |
| **`md5Encode`** | 23 | 2% | ⬜ |
| `toast` | 21 | 2% | ⛔ Android 专有，空实现即可 |
| **`getElements` / `getElement`** | 17 / 7 | 2% | ⬜ |
| **`toNumChapter`** | 16 | 2% | ⬜ |
| `post` | 14 | 1% | ✅ |
| `base64Decode` / `Encode` | 13 / 6 | 1% | ✅ |
| `startBrowser` / `startBrowserAwait` | 8 / 4 | 1% | 🟡 已有 WebView 通道 |
| `encodeURI` / `connect` | 7 / 7 | 1% | ✅ |
| **crypto 系**（`createSymmetricCrypto` `aesBase64DecodeToString` `digestHex` `HMacHex` `des*`） | 各 ≤6 | ≤1% | ⬜ 合计约 2% |
| 其余 30+ 方法 | 各 ≤6 | ≤1% | 按需 |

**结论：`java.*` 只需再补 5 个方法（`getString` `md5Encode` `getElements` `getElement` `toNumChapter`）
即可覆盖 22% 使用 JS 的书源里的绝大多数。** 100 个方法全做是彻底的浪费。

#### 0.3.5 无法支持的一类（新发现）

**9 个源（1%）通过 Rhino 直接访问 JVM 类** —— `java.lang.*`、`java.util.*`、`java.security.*`、
`java.io.*`、`java.text.*`。legado 用的是 Rhino，脚本可以直接 `new java.util.HashMap()`；
**QuickJS 原理上做不到**，除非为每个 JVM 类写模拟层。

处置：**明确列为不支持**，导入时标注、调试器中给出明确原因，不静默失败。代价 0.9% 覆盖率，可接受。
这也是选 QuickJS 而非移植 `modules/rhino` 必须付的账 —— 之前没人算过这笔账。

#### 0.3.6 规则字段填充率（决定 §4.6 补齐顺序）

| 字段 | 填充率 | 现状 |
| --- | ---: | :---: |
| `ruleSearch.{bookList,name,bookUrl}` · `searchUrl` | 100% | ✅ |
| `ruleToc.{chapterList,chapterName,chapterUrl}` · `ruleContent.content` | 97–99% | ✅ |
| `ruleSearch.author` · `ruleBookInfo.intro` | 88–92% | ✅ |
| `ruleSearch.coverUrl` · `ruleBookInfo.{name,author,coverUrl,kind,lastChapter}` | 75–82% | ✅ |
| `ruleSearch.kind` | 70% | ⬜ |
| **`exploreUrl`** | **67%** | ⬜ |
| `ruleSearch.lastChapter` | 66% | ⬜ |
| **`ruleExplore.{bookUrl,name,bookList}`** | **46%** | ⬜ |
| **`ruleBookInfo.tocUrl`** | **41%** | ⬜ |
| `ruleSearch.intro` | 41% | ⬜ |
| `ruleBookInfo.wordCount` · `ruleSearch.wordCount` | 33% / 29% | ⬜ |
| `ruleContent.nextContentUrl` | 27% | ✅ |
| **`ruleContent.replaceRegex`** | **25%** | ⬜ |
| `ruleContent.imageStyle` | 18% | ⬜ |
| `ruleSearch.checkKeyWord` | 17% | ⬜ |
| `ruleToc.nextTocUrl` | 15% | ✅ |
| `ruleBookInfo.init` | 14% | ⬜ |
| `ruleToc.updateTime` | 11% | ⬜ |
| `ruleToc.isVip` | 6% | ⬜ |

**最大的单项发现：`exploreUrl` 67% + `ruleExplore.*` 46% —— 发现页是近半数书源主动配置的能力，
而本项目代码库对它零引用。** 初稿把它排在 P5，**过晚，现提前到 P1**（见 §4.6）。

---

## 1. v2 相对 v1 调整的三处（附理由）

### 1.1 「公开语料」不是 Step 2 的剩余项，而是它的第一项

v1 把语料列在 §10.6 剩余项第 4 条，同时又在 §9 把它标为最高风险 —— **排期与风险评级自相矛盾**。
结果是：`source_engine/` 已经写了 3,744 行，**行数超过参考实现的 `analyzeRule/`（3,377）**，
但覆盖率、`UnsupportedJsoup` / `UnsupportedMode` 的实际分布、哪个 token 卡住最多书源，一概不知。

更糟的是 v1 §9 自己记录的教训：`fixtures/source_a/source.json` 原本是**照着导入器行为写的**，
不是 legado 真实写法。自造夹具会悄悄迁就实现，所以「85 条夹具全绿」不能证明任何覆盖率。

**结论：先建度量，再补功能。** 否则补的顺序只能靠猜，而猜错的代价是几百行白写的 evaluator。

> **本条已被自身验证。** 语料到位后（§0.3），初稿里两条基于猜测的排序当场被推翻：
> XPath 实际只占 6%（初稿列为 P1.3 大改造），WebJs 占 0%（初稿单列 P1.5 一节）；
> 而初稿只当作「JSoup 细节」一笔带过的 `!n` / `.a:b` / `@@`，合计卡住 **43%** 的书源。
> 如果按初稿顺序开工，第一周的工作量会花在 6% 的收益上。

> **目标项目里可立即取用的三份真实语料**（今天就能用，不必等外部输入）：
>
> | 文件 | 内容 | 用途 |
> | --- | --- | --- |
> | `assets/defaultData/bookSources.json` | 1 个真实书源，含全部 21 个顶层字段（`loginUi` / `loginCheckJs` / `ruleExplore` / `respondTime` …） | 替换自造的 `source_c`，做**不被自身实现污染**的字段级夹具 |
> | `assets/defaultData/txtTocRule.json` | **27 条**实测 TXT 目录正则（含 `volumeRule` 卷识别、中文数字、否定环视） | 直接移植，取代手写 `split_chapters` 启发式 |
> | `assets/defaultData/coverRule.json` | `data:;base64,{{java.base64Encode(key)}},{"type":"lyc"}` + `@js:` 里调 `java.hexDecodeToString` | **AnalyzeUrl 的天然压力测试**：data URL + 模板 + 选项对象 + JS 规则四合一 |
>
> 这三份都不足以充当 300 源覆盖率语料，但足以让 P0 的 harness 在拿到外部语料前就跑起来。

### 1.2 `AnalyzeUrl` 缺失是比 selector 更大的结构性缺口（v1 完全遗漏）

legado 的 `AnalyzeUrl.kt` 有 **1,007 行**，是 search / bookInfo / toc / content / explore / cover
**六个阶段共用**的请求构造器，负责：URL 模板展开、`,{...}` 选项对象（**14 个键**）、
`<js>` 与 `bodyJs` 求值、字符集、重试、并发限速、Cookie 注入、WebView 回退。

reader-desktop 目前的对应物是 `search_service::build_search_request` —— 只在**搜索**阶段
处理了 `method` / `body` / `charset` 三个键，其余五个阶段的 URL 一律当作裸 GET 发出。

v1 §4.2 的实施顺序表（4.2.1 analyzer → 4.2.2 evaluator → 4.2.3 JS → 4.2.5 导入器）**没有这一项**。
这解释了一个现象：引擎能正确解析规则，却仍可能在真实书源上失败 —— 因为**请求本身就发错了**，
selector 再准也没用。

> **语料实测确认了这条判断：`,{...}` 选项对象出现在 277 个源（29%）里，是全部阻塞项中
> 边际收益最高的一项（+17.9pp，见 §0.3.2）。** 这是初稿唯一一条被数据完全证实的推断。

| legado `UrlOption` 键 | 作用 | 现状 |
| --- | --- | :---: |
| `method` `body` `charset` | 请求方法 / 体 / 编码 | 🟡 仅 search |
| `headers` | URL 级请求头（叠加书源 header） | ⬜ |
| `retry` | 失败重试次数 | ⬜ |
| `origin` | Referer 覆盖 | ⬜ |
| `type` | 响应类型（图片 / 文件 / 文本） | ⬜ |
| `js` `bodyJs` | 请求前 JS 求值 | 🟡 仅 search 的 `<js>…</js>` |
| `webView` `webJs` `webViewDelayTime` | WebView 取源 | ⬜（P2 认证窗口已有通道基础） |
| `dnsIp` `serverID` | DoH / 多服务器 | ⛔ 不做 |
| 书源级 `concurrentRate` | 按源限速（如 `1/1000`） | ⬜ |

### 1.3 阅读器本体应当提前到下载之前

v1 把排版引擎排在 Step 5（下载之后）。但 v1 §3.3 / §10.6 的三条手工验收**已经悬置多日未执行**，
而它们是 v0.2.0 的发布门槛。

根因不是没时间，是**当前的阅读器不足以让人愿意用它读完一本书**：`ReaderPane.vue` 59 行，
`content.split('\n')` 全量渲染，无分页、无书签、无净化、无字体行距配置，10 万字章节会卡死
（v1 §6 自己点名了这一点，却把修复排在一个月后）。

dogfooding 是发现真实书源问题最高效的手段。把阅读器排在下载之后，等于把最重要的反馈来源推迟一个月，
而下载功能本身**又依赖尚未验证的抓取链路**。顺序应当反过来。

---

## 2. 排期总览

```
P0  度量先行：语料 + 覆盖率仪表盘         1 周    ◀ 当前入口
      ↓
P1  规则引擎补齐（对齐 analyzeRule）      3 周
P2  书源调试器                           1 周   ┘ 与 P1 并行
      ↓
P3  阅读体验可用化                        2 周    ◀ 从 v1 Step 5 提前
      ↓
P4  下载 / 缓存 / 导出                    2 周
P5  书源生态（发现页 / 批量校验 / 换源）    2 周    ◀ v1 未单列
P6  RSS / 备份 / 设置                     2 周
P7  性能 / 稳定性 / 发布                  2 周
```

沿用 v1 的两条纪律：

1. **每个 P 收尾时复查最大文件**，非测试行 > 250 即拆（v1 §9 的教训：Step 0 拆完 `command.rs`，
   业务又在 `SourceService` 里聚成 617 行）。当前最大 `js_runtime.rs` 847 行，**已越线，P1 内必须拆**。
2. **每一条验收都必须能用一条命令或一个文件路径证明**（v1 §10 的规则，v1 §10.5 抓出过 4 条名不副实项）。

---

## 3. P0 · 度量先行（1 周）

**目标：把「引擎覆盖率」从形容词变成一个每次提交都会更新的数字。本步不新增任何用户可见功能。**

### 3.1 语料

| 来源 | 状态 | 说明 |
| --- | --- | --- |
| **`src-tauri/tests/corpus/f3f55c6e-723b-4055-b254-124c9d88c5cb.json`** | ✅ **已到位（2026-09-02）** | **970 源 / 22,220 条规则串**，超出 300 源门槛 3 倍。§0.3 的全部数据出自它 |
| `legado-with-MD3/assets/defaultData/*.json` | ✅ 本地即有 | 见 §1.1 表：1 个全字段源 + 27 条 TXT 目录正则 + coverRule 压测样本 |
| 调试器导出的失败源 | 由 P2 产出 | 每个真实失败自动沉淀为夹具 |

**✅ 已落位（2026-09-02）：** 语料移至 `src-tauri/tests/corpus/`，`.gitignore` 忽略目录内容但保留
`README.md`（登记来源、规模与消费方式）。此前它躺在仓库根目录且未被忽略 —— 5MB JSON 一旦误提交
很难从 git 历史里摘干净。

### 3.2 静态覆盖率 harness（无网络，可进 CI）

新增 `src-tauri/src/bin/rule-audit.rs`：对语料里每一条规则串跑 `split_rule` + 干跑 evaluator，产出
§0.3 那四张表。**原型已用 Node 写过一版并跑出结果**（`.scratch/corpus-audit.cjs`、`.scratch/coverage.cjs`），
本步是把它移植成 Rust bin，好处是**用真实的 `split_rule` / evaluator 判定支持与否，而不是正则近似**：

- 语法 token 频次直方图 + 每个 token 的真实支持状态（✅ / `UnsupportedMode` / `UnsupportedJsoup`）
- JS 脚本里的 `java.xxx` 调用频次（决定 §4.4 顺序）
- 「若支持 token X，则可多解锁 N 个书源」的边际收益排序
- **单一头条数字：可完整执行的书源占比**（当前正则近似口径为 31.3%，Rust 口径可能略有出入）

> 原型用正则近似判定，会有误差：例如它把所有 `//` 开头当 XPath、把 `!数字` 一律当排除语法。
> Rust 版必须以引擎的真实返回为准，两者的差值本身就是一个值得看的数字。

### 3.3 在线可用率 harness（需网络，不进 CI）

`cargo run --bin source-audit -- --corpus <path> --keyword 剑来 --concurrency 8`

每源输出：请求 URL / HTTP 状态 / 解析条数 / 失败阶段 / 失败原因分类（网络 / 认证 / 规则不支持 / 解析为空）。
落 CSV。这是 v1 §4.4「≥300 源，可搜索 ≥60%」**唯一可能的度量方式**。

### 3.4 兜底路径的可见化与删除闸门

`pipeline.rs` 现在「引擎优先 → CSS 投影兜底」，且**吞掉引擎错误**（`tracing::debug` 后返回空 Vec）。
这让覆盖率天然不可见 —— 一个源可能全靠兜底在跑，而报告显示"正常"。

- 新增 `READER_STRICT_ENGINE=1`：置位时引擎错误直接冒泡，不兜底
- 两个 harness 一律在 strict 模式下跑
- **删除闸门**：当语料中 legado 原生源的引擎路径成功率 ≥ 95% 时，删除 `selector.rs` 兜底，
  并清掉 v1 §10.5 第 1 条遗留的 `Result<_, String>`（`selector.rs` / `parse_sources_json`）

### 3.5 验收

- [x] 一条命令产出覆盖率报告，落 `docs/coverage/rule-audit.md`，每完成一项 P1 任务后重跑并 diff

> P0 进度（本轮）：`READER_STRICT_ENGINE` 已接入规则管线；默认模式保留 CSS 兼容兜底，置为 `1`/`true` 时规则执行错误会以 `AppError::Parse` 返回，不再静默回退。
> 2026-09-03 用 Rust evaluator 重跑后的严格口径为 **191 / 970 = 19.7%**；31.3% 保留为早期 token 正则近似基线，两者不再混用。
> 2026-09-03 完成 §4.2 后严格口径升至 **330 / 970 = 34.0%**，相对上一严格基线增加 **139 源 / 14.3pp**。
- [ ] 报告能直接回答：「当前 N 个源里，M 个所有规则均可执行；剩余 N−M 个的阻塞 token TOP10 是 …」
- [x] `txtTocRule.json` 27 条规则移植进 `infrastructure/ebook/txt.rs`，替换手写启发式，附对照测试

> 2026-09-03：27 条默认规则已作为 `src-tauri/assets/defaultData/txtTocRule.json` 随程序打包；
> TXT 导入会从 12 条启用规则中按 legado 的稀疏命中策略选择最佳规则，并兼容 Java 可变长度后向断言。
> 中文数字、英文 Chapter、数字标题、长行误判和卷规则均有对照测试。
- [ ] `defaultData/bookSources.json` 进 `tests/fixtures/`，作为字段级导入回归

---

## 4. P1 · 规则引擎补齐（3 周）

**顺序已按 §0.3.2 的实测边际收益排定，不再是预估。** 每完成一项重跑 harness，把覆盖率写进 commit message。

| 序 | 任务 | 影响源数 | 覆盖率目标 | 节 |
| ---: | --- | ---: | ---: | --- |
| — | 起点 | — | **31.3%** | — |
| 1 | JSoup 排除 `!n` | 299 | 44.9% | §4.2 |
| 2 | `AnalyzeUrl` 选项对象 + 六阶段统一 | 277 | **62.9%** | §4.1 |
| 3 | JSoup 区间 `.a:b` | 146 | 73.4% | §4.2 |
| 4 | JSoup `@@` | 102 | **81.8%** | §4.2 |
| 5 | JSONPath 递归下降 `..` | 78 | 89.3% | §4.3 |
| 6 | XPath 译码器按需扩展 | 58 | 94.6% | §4.3 |
| 7 | JSONPath 过滤 `?()` | 43 | **99.1%** | §4.3 |
| — | Rhino JVM 包访问 | 9 | ⛔ 不做，见 §0.3.5 | — |

> **1 和 2 谁先做？** 数据上 `!n` 覆盖更多源（299 vs 277），但 `AnalyzeUrl` 边际收益更高
> （+17.9 vs +13.6）且是**结构性重构**，越晚做迁移成本越大（每多一个阶段接旧路径就多一处要改）。
> **建议 2 先做**，理由不是覆盖率，是架构：§4.1 完成后 §4.2 的三项都只是 `jsoup.rs` 内的局部改动。

### 4.1 `AnalyzeUrl` 提取与补齐（结构性，实测边际收益 +17.9pp 最高）

新建 `source_engine/url/`，成为六阶段共用的请求构造器：

1. 把 `search_service::build_search_request` 的逻辑**原样搬过来**（含现有测试）
2. 补齐 §1.2 表中标 ⬜ 的键：`headers` `retry` `origin` `type` `js`/`bodyJs`
3. `bookInfo` / `toc` / `content` / `explore` / `cover` 五处**全部改走它**
4. 书源级 `concurrentRate` 限速：解析 `1/1000`、`5/1000` 形态，按源令牌桶

**本步不新增用户可见功能，但它是后面每一条规则能否发对请求的前提。**
验证基准：`coverRule.json` 那条 data-URL 规则能被正确构造并执行。

> 2026-09-03 进度：共享构造器已拆入 `source_engine/url/{parser,options,encoding,transport,rate_limit}.rs`，并接入当前已存在的 search / bookInfo / toc / content / cover 五条链路；`headers` / `retry` / `origin` / `type` / `js` / `bodyJs`、data URL、字符集和 `concurrentRate` 已覆盖。Explore 链路尚未存在，待 §4.6 接入后本项转为全量完成。

### 4.2 Default(JSoup) 模式补齐（三项合计覆盖 43% 书源）

`jsoup.rs` 283 行 vs `AnalyzeByJSoup.kt` 524 行。**这一节是整个 P1 里性价比最高的部分** ——
三项改动都在单个文件内，不引入依赖，合计把覆盖率从 62.9% 推到 81.8%。

| 缺口 | 源数 | 说明 |
| --- | ---: | --- |
| **排除 `!n`** | 299 (31%) | `!0` / `!1:2` / `!-1`。当前返回 `UnsupportedJsoup` |
| **区间 `.a:b`** | 146 (15%) | `.0:1:2` / `[0:3]` / 负步长 |
| **`@@`** | 102 (11%) | 强制 JSoup 模式前缀 |
| allInOne（`:` 开头） | 未单独统计 | 顺带对齐；以目标项目 `splitSourceRule(..., allInOne = true)` 的实际实现为准 |
| `children` / `textNodes` / `ownText` 边界 | — | 与 legado 逐字对齐 |

负索引 `.-1`（184 源 / 19%）**已实测确认支持**（`jsoup.rs:136` + `step.rs:58`，各有单测）。

> 2026-09-03 进度：本节完成。统一位置筛选器现支持旧式索引列表 `.0:1:2`、排除 `!0` / `!1:2` / `!-1`、方括号单索引与包含端点区间、缺省端点、负索引、反向范围和负步长；筛选按每个父节点独立应用。`@@` 与 allInOne `:` 前缀已补回归测试，并对齐 `text` 私有匹配、`textNodes`、`ownText`、`all` 的边界行为。真实语料中原有的 range/exclusion `UnsupportedJsoup` 错误已清零。

### 4.3 JSONPath / XPath（按实测降级）

- **JSONPath（121 源受影响）**：补递归下降 `..`（78 源）与过滤 `?(@.x)`（43 源）。
  两项合计 +11.9pp，**优先级高于 XPath**。若手写成本高，直接换 `jsonpath-rust`。
- **XPath（58 源 / 6%）—— 初稿的大改造方案取消。**
  实测 `@XPath:` 显式前缀**零出现**，全部是 `//` 裸写。为 6% 的源引入 `sxd-xpath` / `libxml`
  并做 HTML→XML 归一化，不划算。改为：**按语料里实际出现的 58 条 XPath 逐条扩展现有译码器**，
  跑不通的明确报错。v1 §9 登记的「XPath crate 与 HTML 容错解析不匹配」风险随之关闭 —— 不引入 crate 就没有这个风险。

> 2026-09-03 进度：JSONPath 已补递归下降（含递归/普通通配）、负索引和数组过滤，过滤支持字段存在、
> 字符串/数字/布尔/null 比较以及 `&&` / `||`；实现已从 `evaluator.rs` 拆入独立 `jsonpath.rs`。
> XPath 译码器已按真实语料补直接子代/后代、`following-sibling`、`not(@attr=...)`、
> `contains(text(), ...)`、`text()=...` 与常见 `position()/last()` 范围。剩余长尾形态继续由审计报告驱动，
> 因此本节保持进行中，不宣称 58 条已全覆盖。

### 4.4 `java.*` 补齐（按实测频次，只补 5 个）

§0.3.4 的实测结论：**只需补 5 个方法**即可覆盖 22% 使用 JS 的书源里的绝大多数。

| 方法 | 源数 | 说明 |
| --- | ---: | --- |
| `getString` | 69 (7%) | 在 JS 里执行一条规则并取值 —— 需要把引擎回调注入 JS 作用域 |
| `md5Encode` | 23 (2%) | `md-5` 已是依赖 |
| `getElements` / `getElement` | 17 / 7 (2%) | 同 `getString`，返回节点 |
| `toNumChapter` | 16 (2%) | 中文数字章节名归一化 |
| crypto 系（`createSymmetricCrypto` `aesBase64DecodeToString` `digestHex` `HMacHex` `des*`） | 合计 ~2% | 对位 hutool；按需，可延后 |

`jsLib` 实测只有 **6 个源（1%）** 使用 —— 从 P1 降级到「按需」。

⛔ 不做：`toast` / `longToast`（21 源，Android 专有 —— **给空实现**，避免脚本抛异常）；
`androidId` / `getReadBookConfig*` / `getThemeConfig*`；`queryTTF` / `replaceFont`（字体反爬）。

**同时拆 `js_runtime.rs`（847 行，全项目最大，已违反 250 行纪律）**：
按 `runtime.rs` / `bindings/{net,codec,rule,ctx}.rs` 拆开。

### 4.5 ~~WebJs~~ —— 取消

实测 970 源中 **`webJs` 零出现**。初稿在此单列一节属于对着 legado 字段表想当然。
若将来语料里出现，复用 P2 的 WebView 通道即可，届时再排期。

### 4.6 书源模型补齐 + 发现页（按实测填充率排序）

**发现页从 P5 提前到这里** —— `exploreUrl` 填充率 **67%**、`ruleExplore.*` **46%**（§0.3.6），
是仅次于搜索的第二大入口，而代码库对它**零引用**。它同时依赖 §4.1 的 `AnalyzeUrl`
（`exploreUrl` 的多行 `名称::链接` 格式 + 选项对象），放在 §4.1 之后做正好。

字段补齐顺序（填充率 ≥25% 的先做）：

| 优先 | 字段 | 填充率 |
| :---: | --- | ---: |
| P0 | `exploreUrl` + `ruleExplore.{bookList,name,bookUrl,author,coverUrl,kind,intro,lastChapter,wordCount}` | 67% / 46% |
| P0 | `ruleSearch.kind` · `ruleSearch.lastChapter` | 70% / 66% |
| P1 | `ruleBookInfo.tocUrl` · `ruleSearch.intro` | 41% |
| P1 | `ruleBookInfo.wordCount` · `ruleSearch.wordCount` | 33% / 29% |
| P1 | `ruleContent.replaceRegex`（并入 §4.7） | 25% |
| P2 | `ruleContent.imageStyle` · `ruleSearch.checkKeyWord` · `ruleBookInfo.init` | 18% / 17% / 14% |
| P2 | `ruleToc.updateTime` · `ruleToc.isVip` | 11% / 6% |
| — | `ruleReview.*` · `ruleBookInfo.relatedBooks` | ⛔ 不做 |

顶层字段按 §0.2 C 层表补齐；实测填充率参考：`bookUrlPattern` 47%、`loginUrl` 46%、`header` 34%、
`loginUi` 7%、`loginCheckJs` 6%、`concurrentRate` 6%、`coverDecodeJs` 3%、`jsLib` 1%。

旧扁平 CSS 列（`search_item_selector` 等 13 列）标记 deprecated，随 §3.4 兜底删除一并清理。

### 4.7 内容后处理（`ContentProcessor` 对位）

legado 的处理顺序：`sourceRegex` 切分 → `replaceRegex` → 全局 `ReplaceRule` → 简繁转换 → 分段。
现在**一条都没有**。

新增 `replace_rules` 表，对齐 `ReplaceRule.kt`：
`name / group / pattern / replacement / isRegex / scope / scopeTitle / scopeContent / excludeScope / order / enabled`。

### 4.8 验收（全部可度量，门槛已按实测校准）

- [ ] 静态覆盖率：970 源语料，**引擎可完整执行的源 ≥ 90%**（strict 模式）
      —— 起点 31.3%；§4.1–§4.3 全做完理论值 99.1%，留 9pp 给正则近似与真实执行的差值
- [ ] 在线可用率：**能搜出结果的源 ≥ 60%**（对齐 v1 §4.4；这一条受站点存活率影响，与静态覆盖率不可混为一谈）
- [ ] 发现页在 ≥100 个 `exploreUrl` 非空的源上可正常出图书列表
- [ ] `selector.rs` 兜底已删除；全项目 `Result<_, String>` 为 0
- [ ] 单源规则执行 P95 < 200ms（不含网络）
- [ ] JS 死循环脚本 5 秒内被终止且不影响主进程（v1 遗留）
- [ ] 最大单文件非测试行 < 250（当前 `js_runtime.rs` 847，已越线）
- [ ] Rhino JVM 包访问的 9 个源被明确标注为不支持，非静默失败

---

## 5. P2 · 书源调试器（1 周，与 P1 并行）

> **已有骨架（2026-09-02 发现，尚未提交）：** `service/source_debug_service.rs` 120 行
> （`SourceDebugStage` 四阶段 / `SourceDebugStep` / `run()` / `update_rules()` 支持不重启改规则）
> \+ `SourceDebugPage.vue` 99 行 + `useSourceDebug.ts` 131 行。v1 §5.3 第一条验收「不重启改规则重试」
> 的后端能力已具备。下面三点是**在此基础上的增量**。

沿用 v1 §5 的 UI 布局与流式推送设计（`source-test-progress` 事件名已预留），**补三点**：

1. **展示 AnalyzeUrl 的最终请求** —— URL / method / headers / body / charset / 命中的选项对象。
   这是 §4.1 的直接受益者，也是「规则对但请求错」这类问题唯一的定位手段。
2. **展示认证态** —— 会话是否过期、本次是否携带 token/cookie/sign、是否触发 Cloudflare。
   这是 v1 §10.6 明确列出的遗留项。
3. **「一键把当前失败源导出为 fixture」** —— 把响应 HTML + 书源 JSON 直接写进 `tests/fixtures/`。
   让每个真实失败自动变成回归测试，也是对 v1 §9「自造夹具会迁就实现」风险的系统性解法。

验收（沿用 v1 §5.3）：

- [ ] 能定位一个真实失败书源的失败步骤，并在不重启应用的前提下改规则重试
- [ ] `source_engine` 行覆盖率 ≥ 70%
- [ ] CI 跑全量 fixture < 30s

---

## 6. P3 · 阅读体验可用化（2 周，从 v1 Step 5 提前）

**目标不是复刻 legado 5,227 行的排版引擎，而是让开发者自己愿意用它读完一本书 —— 因为 dogfooding
是 P1 覆盖率之外唯一能发现真实书源问题的手段（理由见 §1.3）。**

必做：

- **长章节虚拟化渲染** —— 当前全量 `<p>`，10 万字章节会卡死
- **分页模式** —— 页宽/页高计算 + 键盘翻页，与滚动模式并存
- **主题与排版配置** —— Design Tokens 驱动（`plan.md` §8 要求不写死颜色）：
  Light / Dark / Sepia / Green / Black + 字体、字号、行距、页边距、首行缩进
- **阅读进度升级** —— 从 `chapter_id + offset` 升级到能在分页模式下稳定还原
- **书签 / 章内搜索 / 阅读时长** —— 对位 `Bookmark` / `searchContent` / `readRecord`
- **接入 §4.7 的净化替换规则**

不做：仿真翻页动画、竖排、字体反爬、图片/漫画渲染。

> 2026-09-04 进度：已完成第一批阅读器增量。滚动模式改为带上下占位的段落窗口，避免长章节全量渲染；新增滚动/分页模式切换、方向键翻页、字号/行距/页边距持久化，并按模式保存和恢复阅读偏移。当前章节已支持本地搜索定位与书签偏移持久化。分页重排后的精确排版进度、阅读时长统计和全局净化规则仍待后续迭代。

验收：

- [ ] 100 万字单章 TXT 打开 < 1s，滚动无掉帧
- [ ] 分页模式下改字号 → 重排后进度不丢
- [ ] 用本项目读完一本真实在线书（这一条替代 v1 §3.3 的手工验收清单）

---

## 7. P4 · 下载 / 缓存 / 导出（2 周）

沿用 v1 §6 Step 4（`DownloadManager`、任务状态机落库、三级缓存、`scheduler/` 模块），**追加**：

- 对位 `CacheBook.kt` 的**双层限流**：全局并发上限 + 按书源 `concurrentRate`（复用 §4.1）
- 导出 TXT / EPUB，对位 `ExportBookService.kt`

---

## 8. P5 · 书源生态（2 周，v1 未单列）

v1 没有这一层，但它决定「书源体系能不能自转」：

> **发现页已按实测（`exploreUrl` 67%）提前到 §4.6，不在本阶段。**

| 项 | legado 对位 | 说明 |
| --- | --- | --- |
| **书源批量校验** | `BookSourceCheckService.kt` | 全量跑四阶段，标红失效源，回写 `respondTime` / `lastUpdateTime`。P0 的 `source-audit` bin 是它的雏形，本步是把它搬进 UI |
| **换源增强** | `ui/book/changesource` | 现在只按书名重搜；补章节对齐、`bookUrlPattern` 匹配、`canReName` |
| **分组 / 排序 / 权重 / 启停** | `bookSourceGroup` `customOrder` `weight` `enabled` `enabledExplore` | 书源上百个之后没有这些就没法管理 |
| **Cookie 持久化** | `data/entities/Cookie.kt` + `enabledCookieJar` | 现在 cookie 只活在 reqwest 内存 store，重启即失 |
| **导出为 legado 兼容 JSON** | — | 互操作性：桌面端编辑的源要能回到 Android 端 |

---

## 9. P6 · RSS / 备份 / 设置（2 周）

沿用 v1 §6 Step 6，**追加一条硬要求**：

> **备份格式与 legado 兼容** —— 对位 `help/storage/{Backup,Restore,BackupAES}.kt` 的文件清单与 AES 方案，
> 使桌面端能直接吃 Android 端的备份包。**这是「Legado 桌面版」而非「另一个阅读器」的关键差异点**，
> 优先级高于 WebDAV 同步（后者仍推迟到 P7 之后）。

---

## 10. P7 · 性能 / 稳定性 / 发布（2 周）

沿用 v1 §6 Step 7，**追加**：

- `rquickjs` 的 C 工具链在 Linux / macOS 各构建一次 —— v1 §9 登记但**至今未关闭**的风险
- 异常场景矩阵（`plan.md` §30）按 P0 harness 的失败分类补齐

---

## 11. 度量与验收命令

```bash
# 既有三件套（v1 §10.7）
cargo test   --manifest-path src-tauri/Cargo.toml --target-dir src-tauri/.cargo-target
cargo clippy --manifest-path src-tauri/Cargo.toml --target-dir src-tauri/.cargo-target \
             --all-targets -- -D warnings
npm run lint && npm run format:check && npm run build

# P0 新增：静态覆盖率（无网络，进 CI）
READER_STRICT_ENGINE=1 cargo run --bin rule-audit -- \
  --corpus src-tauri/tests/corpus/ --out docs/coverage/

# P0 新增：在线可用率（需网络，不进 CI）
READER_STRICT_ENGINE=1 cargo run --bin source-audit -- \
  --corpus src-tauri/tests/corpus/sources.json --keyword 剑来 --concurrency 8 --out audit.csv

# §0.3 数据的产出方式（Node 原型，待移植为上面两个 bin）
node .scratch/corpus-audit.cjs src-tauri/tests/corpus/f3f55c6e-723b-4055-b254-124c9d88c5cb.json   # token / java.* / 字段频次
node .scratch/coverage.cjs     src-tauri/tests/corpus/f3f55c6e-723b-4055-b254-124c9d88c5cb.json   # 覆盖率基线 + 边际收益
```

**本文件所有对照数据的取数方式**（2026-09-02 实测）：

```bash
# 两侧体量
find "D:/Code/chatting/legado-with-MD3/app/src/main/java" -name "*.kt" -exec cat {} + | wc -l
find src-tauri/src -name "*.rs" -exec cat {} + | wc -l
find src -name "*.vue" -o -name "*.ts" | xargs cat | wc -l

# 各层对照
wc -l "D:/Code/chatting/legado-with-MD3/app/src/main/java/io/legado/app/model/analyzeRule/"*.kt
find src-tauri/src/source_engine -name "*.rs" -exec cat {} + | wc -l
grep -c "fun " "D:/Code/chatting/legado-with-MD3/app/src/main/java/io/legado/app/help/JsExtensions.kt"
grep -n 'java.set(' -A1 src-tauri/src/source_engine/rule/js_runtime.rs | grep -o '"[a-z A-Z_]*"' | sort -u
```

---

## 12. 版本与门槛

| 版本 | 阶段 | 门槛（每条可验证） | 状态 |
| --- | --- | --- | :---: |
| v0.1.0 | v1 Step 0 | 本地阅读闭环 + 干净架构 | ✅ |
| **v0.2.0** | **P0** | 覆盖率仪表盘上线（基线 **31.3%** 已测得）+ `txtTocRule` 移植 + v1 §10.6 三条手工验收 | 🟡 |
| v0.2.1 | — | WebView 认证闭环在真实 Cloudflare 站点验收通过 | 🟡（实现完成，待人工站点验收） |
| **v0.3.0** | **P1 + P2** | 静态覆盖率 **≥ 90%**（起点 31.3%）· 在线可用率 ≥ 60% · 发现页可用 · 兜底路径已删 · 调试器可用 —— **真正的「Legado 桌面版」起点** | ⬜ |
| v0.4.0 | P3 | 能用本项目读完一本真实在线书 | ⬜ |
| v0.5.0 | P4 | 下载 / 缓存 / 导出 | ⬜ |
| v0.6.0 | P5 | 发现页 + 批量校验 + 书源管理 | ⬜ |
| v0.7.0 | P6 | RSS + legado 兼容备份 | ⬜ |
| v1.0.0 | P7 | 性能 / 稳定性 / Windows + Linux 打包 | ⬜ |

> 与 v1 的差异：v1 把「完整规则引擎 + 调试器」定为 v0.3.0，v2 保留这一定位但**把门槛从
> 主观判断改为两个数字**；阅读器从 v0.5.0 提前到 v0.4.0；新增 v0.6.0 书源生态。

---

## 13. 风险登记（v1 §9 的增补与关闭）

| 风险 | 影响 | 状态 / 缓解 |
| --- | --- | --- |
| ~~引擎覆盖率无法自测~~ | — | **已关闭（2026-09-02）** —— 970 源语料到位，基线测得 31.3%，边际收益排序见 §0.3.2 |
| ~~真实语料规模不足~~ | — | **已关闭（2026-09-02）** —— 970 源 / 22,220 条规则串，超门槛 3 倍 |
| **实测覆盖率仅 31.3%** | 「引擎已接入真实管线」的实际含金量远低于此前表述 | **新增** —— 这是 v1 §10 系列「名不副实项」的最后一条，也是最大的一条。P1 的全部排期为它服务 |
| **Rhino JVM 包访问无法支持** | 9 源（1%）永久不可用 | **新增（§0.3.5）** —— 选 QuickJS 必须付的账。明确标注，不静默失败 |
| **`AnalyzeUrl` 缺失** | 规则解析正确但请求发错，失败原因无法归因 | **v2 新增，已被实测证实**（277 源 / +17.9pp，全部阻塞项中边际收益最高） |
| **按字段表想当然排期** | 初稿为 0% 使用率的 `WebJs` 单列一节、为 6% 的 XPath 提议换引擎 | **新增** —— 教训：legado 有某字段 ≠ 真实书源在用。**任何新排期项必须先查语料填充率** |
| **兜底路径掩盖失败** | `pipeline.rs` 吞掉引擎错误，覆盖率天然不可见 | **v2 新增** —— `READER_STRICT_ENGINE` + 95% 删除闸门 |
| **阅读器不可用导致无法 dogfood** | 手工验收长期悬置，真实问题发现不了 | **v2 新增** —— P3 提前到下载之前 |
| `js_runtime.rs` 已达 847 行 | 违反 v1 自定的 250 行纪律 | **v2 新增** —— P1 内按 `runtime` / `bindings/*` 拆分 |
| ~~XPath crate 与 HTML 容错解析不匹配~~ | — | **已关闭（2026-09-02）** —— 实测 XPath 仅占 6%，§4.3 决定不引入 crate，风险自然消失 |
| Java 正则与 Rust `regex` 语义差异 | 部分正则规则失效 | 保持登记；P0 harness 统计实际出现的不兼容语法，建差异清单 |
| `rquickjs` C 工具链跨平台 | Linux / macOS 构建失败 | **v1 登记但未关闭** —— 顺延至 P7，建议提前到 P1 结束时做一次 spike |
| Cloudflare / JS challenge | 部分站点完全不可用 | WebView 认证窗口已落地，待真实站点验收；不承诺无头绕过 |
| 业务在 service 层重新聚团 | 每个阶段末产生新巨型文件 | v1 纪律保留：每个 P 收尾复查最大文件，非测试行 > 250 即拆 |
| ~~架构重构引入回归~~ | — | **已关闭**（v1 Step 0 小步提交完成，无回归） |

---

## 14. 明确不做（在 v1 §7 基础上更新）

| 功能 | legado 位置 | 不做的理由 |
| --- | --- | --- |
| TTS / 朗读 / 有声书 | `TTSReadAloudService` `HttpReadAloudService` `AudioPlayService` | 桌面端需另找 TTS 后端 |
| 漫画阅读 | `ui/book/manga` | `bookSourceType=2` 图片源，另一套渲染 |
| AI 相关 | `ui/ai` + 8 个 `Ai*` 实体 | `plan.md` §1 已列为暂不做 |
| 段评 | `ReviewRule` `BookChapterReview` | 依赖账号体系 |
| 局域网 Web 服务 | `web/KtorServer.kt` `modules/web` | 等桌面端本体稳定 |
| 字体反爬 | `queryTTF` / `replaceFont` | 单独立项，工作量大 |
| UMD / MOBI / PDF | `model/localBook/{Umd,Mobi,Pdf}File.kt` | P3 之后按需求评估 |
| 词典 / 翻译 | `ui/dict` `model/translation` `DictRule` | 低频 |
| 高亮 / 标签规则 | `ui/highlightTagRule` `ui/tagGroupRule` | 低频 |
| Rhino JS 引擎移植 | `modules/rhino` | 用 QuickJS 替代。**代价已量化：9 个源（1%）通过 `java.lang.*` / `java.util.*` 等直接访问 JVM 类，将永久不可用**（§0.3.5） |
| `WebJs` / `ContentRule.webJs` | `AnalyzeUrl` + `ContentRule` | **实测 970 源中零出现**，初稿的排期项已取消 |
| `dnsIp` / `serverID` | `AnalyzeUrl` UrlOption | DoH 与多服务器调度，桌面端价值低 |

---

## 15. 下一步（可立即开工）

**外部阻塞项已清零。** 按依赖顺序：

| # | 任务 | 产出 | 节 | 状态 |
| ---: | --- | --- | --- | :---: |
| 0 | **把语料挪进 `src-tauri/tests/corpus/` 并加 `.gitignore`** | 避免 5MB JSON 误入 git 历史 | §3.1 | ✅ |
| 1 | **加 `READER_STRICT_ENGINE` 开关**，让 `pipeline.rs` 停止吞掉引擎错误 | 覆盖率可测的前提 | §3.4 | ✅
| 2 | **`rule-audit` bin**：把 Node 原型移植成 Rust，用真实 `split_rule`/evaluator 判定 | 31.3% 这个数字有了权威口径 | §3.2 | ✅
| 3 | **提取 `source_engine/url/`**，`build_search_request` 搬过去并推广到六阶段 | 31.3% → 62.9% | §4.1 | 🟡（现有五阶段已接入，Explore 待 §4.6） |
| 4 | **JSoup 三项：`!n` / `.a:b` / `@@`** | 62.9% → **81.8%** | §4.2 | ✅ |
| 5 | **移植 `txtTocRule.json` 27 条规则**替换手写启发式（可与 3/4 并行） | TXT 目录识别对齐 legado | §3.5 | ✅ |
| 6 | JSONPath `..` + `?()`，XPath 按 58 条实例逐条扩展 | 81.8% → 99.1% | §4.3 | 🟡 |
| 7 | 发现页 + 字段补齐 | `exploreUrl` 67% 的源解锁第二入口 | §4.6 | 🟡（后端链路与前端入口已完成，分页/长尾字段待后续） |
| 8 | **P3 阅读器第一批**：长章节窗口、分页/滚动、阅读配置、章内搜索、书签 | 阅读正文可持续使用，降低长章节卡顿 | §6 | 🟡（基础能力已落地，验收中的百万字性能与真实在线书仍待实测） |

### v0.2.1 人工验收步骤（实现已具备）

1. 在桌面端导入一个配置了 Cloudflare challenge 的真实书源，并确认其 `base_url` / `login_url` 可访问。
2. 点击“浏览器认证”，在 `source-auth-*` 窗口完成 Cloudflare/Turnstile 验证。
3. 点击“读取浏览器会话”，确认书源状态变为“已认证”，且会话 Cookie 已持久化。
4. 点击“测试”，确认探针不再报告 Cloudflare challenge 并能解析出结果；若站点仍返回 challenge，状态保持可重试且不会误标为“已过期”。

本地无网络回归由 `cargo test --manifest-path src-tauri/Cargo.toml --target-dir src-tauri/.cargo-target` 覆盖：Cloudflare 响应头识别、challenge 与普通 403 的分类、过期会话被浏览器 Cookie 清除等路径。

**第 3、4 项做完就有 81.8%** —— 都是不引入新依赖的局部改动，是整个路线图里投入产出比最高的一段。

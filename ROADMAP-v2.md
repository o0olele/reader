# Reader Desktop 路线图 v2

> 从**目标项目 `D:\Code\chatting\legado-with-MD3` 的能力面**反向出发，回答「离能被称作 Legado 桌面版
> 还差什么、按什么顺序补」。`ROADMAP.md`（v1）作为历史记录保留，**排期与优先级一律以本文件为准**。
>
> 初版 2026-09-02。**本次修订 2026-09-05**：两侧体量、`docs/coverage/rule-audit.md` 的 1,811 条失败
> 全量归类、`r.txt` 的 280 条线上失败，均为当日实测，取数命令见 §11。

**一句话现状：2026-09-02 排出的七项阻塞语法已基本做完，静态覆盖率却只从 19.7% 升到 36.6% —— 因为
真正卡住书源的不是那七项，而是「引擎比 legado 严格」和「Java 生态语义差异」这两类此前完全没有登记的问题。
本次修订把 615 个受阻书源的失败原因逐条归类，并据此重排 P1。**

---

## 0. 与目标项目的量化对照

### 0.1 体量（2026-09-05 实测）

| 维度 | legado-with-MD3 | reader-desktop | 比值 |
| --- | --- | --- | ---: |
| 主代码 | 290,943 行 Kotlin / 1,631 文件 | 13,985 行 Rust / 73 文件 + 2,436 行 TS·Vue / 34 文件 | 5.6% |
| 数据实体 | `data/entities/` 47 个 | 5 张业务表 + `chapter_contents`（17 个迁移） | — |
| **规则引擎** | `model/analyzeRule/` 3,377 行 / 10 文件 | `source_engine/` 7,397 行 / 27 文件 | **219%** |
| **抓取编排** | `model/webBook/` 1,560 行 / 5 文件 | `pipeline.rs` 281 + `url/` 5 文件 + 4 个 service | ~70% |
| **排版引擎** | `read/page/{provider,entities}` 5,227 行 / 16 文件 | `ReaderPane.vue` 225 + `useReader.ts` 181 | ~8% |
| JS 扩展面 | `JsExtensions.kt` 100 个方法 | `js_runtime.rs` 约 30 个 | 30% |
| 书源调试器 | `model/Debug.kt` 422 行 | `source_debug_service.rs` 310 + 2 个前端文件 269 | ~130% |

> **规则引擎行数已是参考实现的 2.19 倍，覆盖率仍只有 36.6%。** 这一行是全表最重要的信息：
> 问题不再是「写得不够多」，而是**写的东西和 legado 的实际语义对不上**。§0.3 给出了具体在哪对不上。

### 0.2 能力矩阵

图例：✅ 达标 · 🟡 部分 · ⬜ 未开始 · ⛔ 明确不做

**A 层 · 规则求值（`analyzeRule` 对位）**

| 能力 | 现状 | 说明 |
| --- | :---: | --- |
| 规则串词法切分 `\|\| && %% - ##` | ✅ | `analyzer.rs` + `scanner.rs`，但**比 legado 严格**，见 §0.3.3 |
| 模板 `{{}}` / `@get:` / `@put:` | 🟡 | 90 条规则因 `template is missing }}` 失败（legado 容忍） |
| Default(JSoup) 私有语法 | 🟡 | 排除 `!n`、区间 `.a:b`、`@@`、allInOne、负索引已对齐；**JSoup 的宽松属性选择器与伪类未对齐**，见 §0.3.3 |
| XPath | 🟡 | `xpath.rs` 299 行手写译码器；实测仅 6% 源使用，剩余失败 12 条 |
| JSONPath | ✅ | 递归下降 `..`、过滤 `?()`、负索引均已实现（`jsonpath/`） |
| Regex | 🟡 | **Java 正则与 Rust `regex` 不兼容是当前第二大失败源（263 条）** |
| JS 规则 | 🟡 | QuickJS；`js_runtime.rs` **2,080 行（全项目最大，两倍于 250 行纪律）**。与 Rhino 的语义差异见 §0.3.3 |
| **URL 构造 `AnalyzeUrl`** | ✅ | `source_engine/url/` 五文件，六阶段共用；选项对象 14 键、data URL、字符集、`concurrentRate` 均已覆盖 |
| `jsLib` 书源级 JS 库 | ⬜ | 实测 6 源（1%），按需 |

**B 层 · 抓取编排（`webBook` 对位）**

| 能力 | 现状 |
| --- | :---: |
| 搜索 / 详情 / 目录 / 正文 四阶段 | ✅ |
| 目录分页 `nextTocUrl` / 正文分页 `nextContentUrl` | ✅ 均带上限与环检测 |
| **发现页 Explore** | ✅ `explore_service.rs` + `ExplorePage.vue` + 迁移 016（分页与长尾字段待补） |
| 章节去重 / 卷识别 `isVolume` | ⬜ |
| `preciseSearch` / `checkKeyWord` | ⬜ |
| 内容后处理 `ContentProcessor` | ⬜ **代码库 0 处引用** |
| 封面兜底规则 `coverRule` | ⬜ |

**C 层 · 书源模型**（顶层 `BookSource` 27 字段 / 现有约 18）

| Rule 类 | legado | 现有 | 主要缺口 |
| --- | ---: | ---: | --- |
| `SearchRule` | 11 | 5 | `checkKeyWord` `intro` `kind` `lastChapter` `updateTime` `wordCount` |
| `BookInfoRule` | 13 | 6 | `init` `tocUrl` `wordCount` `canReName` `downloadUrls` `updateTime` |
| `TocRule` | 10 | 4 | `preUpdateJs` `formatJs` `isVolume` `isVip` `isPay` `updateTime` |
| `ContentRule` | 11 | 2 | `title` `replaceRegex` `sourceRegex` `imageStyle` `imageDecode` `payAction` |
| `ExploreRule` | 10 | 部分 | 后端链路已通，字段未补齐 |
| `ReviewRule` | 10 | — | ⛔ 不做 |

**D 层 · 阅读器**

| 能力 | 现状 |
| --- | :---: |
| 长章节窗口渲染 / 滚动·分页双模式 / 字号·行距·页边距 | ✅ 第一批已落地（2026-09-04） |
| 分页排版（对位 `ChapterProvider` + `TextChapterLayout`） | 🟡 无精确页宽页高计算，改字号后进度靠近似还原 |
| 主题（Light/Dark/Sepia/Green/Black） | 🟡 只有主题名 |
| 书签 | 🟡 **仅前端 `ReaderPane.vue`，无数据表、无持久化** |
| 章内搜索 | ✅ |
| 阅读时长 `readRecord` | ⬜ |
| 净化替换规则 | ⬜ 依赖 C 层 `ContentProcessor` |
| 图片 / 漫画 / TTS | ⛔ |

**E 层 · 书籍与生态**

| 能力 | 现状 |
| --- | :---: |
| 本地格式 txt / epub | 🟡（umd / mobi / pdf ⛔） |
| TXT 目录规则（27 条 legado 正则） | ✅ 已随程序打包 |
| 下载 / 缓存 / 导出 | ⬜ `scheduler/` 不存在 |
| 换源 | 🟡 按书名重搜，无章节对齐 |
| 书源批量校验 | 🟡 `source-audit` bin 已有，未进 UI |
| Cookie 持久化 | 🟡 仅 reqwest 内存 store |
| 备份 / 恢复 · WebDAV · RSS | ⬜ |
| 书源调试器 | ✅ 四阶段 + 不重启改规则 |
| AI / 词典 / 翻译 / 局域网 Web / 段评 | ⛔ |

---

### 0.3 静态覆盖率归因（本次修订的核心）

语料 `src-tauri/tests/corpus/f3f55c6e-….json`，**970 源 / 22,220 条规则串**。
报告 `docs/coverage/rule-audit.md`：**可完整执行 355 / 970 = 36.6%，受阻 615 源，1,811 条规则失败。**

#### 0.3.1 2026-09-02 的预测已全部兑现，覆盖率却没有兑现

| 当时排出的阻塞项 | 预测修完累计 | 实际状态 |
| --- | ---: | --- |
| JSoup 排除 `!n` · 区间 `.a:b` · `@@` | 81.8% | ✅ 三项全做完 |
| `AnalyzeUrl` 选项对象 + 阶段统一 | — | ✅ 已做完 |
| JSONPath `..` + `?()` | 99.1% | ✅ 已做完 |
| XPath 译码器扩展 | — | ✅ 剩余失败仅 12 条 |
| **实际严格口径** | **99.1%** | **36.6%** |

**结论：那张边际收益表是用正则近似 token 算出来的，它衡量的是「规则里有没有出现某个符号」，
而不是「引擎能不能正确执行这条规则」。做完全部七项，60 个百分点的预测收益没有出现。**
这是比 v1「按字段表想当然」更隐蔽的一次估算失败 —— 有数据，但口径错了。
**本节起，任何排期一律以 `rule-audit` 的真实失败归类为依据，不再使用 token 频次做预测。**

#### 0.3.2 1,811 条失败的全量归类

按 `docs/coverage/rule-audit.md` 的 351 行错误逐条归类（命令见 §11）：

| 类别 | 条数 | 占比 | 性质 |
| --- | ---: | ---: | --- |
| **harness 假输入**（JSON 源被喂 HTML dummy） | **745** | **41.1%** | ⚠️ 度量缺陷，非引擎缺口 |
| 引擎比 legado 严格（`rule is empty` 298 + analyzer 40） | 338 | 18.7% | 真实缺口 |
| **Java 正则 → Rust `regex` 不兼容** | 263 | 14.5% | 真实缺口 |
| **JSoup 宽松属性选择器 / 伪类** | 202 | 11.2% | 真实缺口 |
| JS 运行时与 Rhino 的语义差异 | 160 | 8.8% | 真实缺口 |
| 模板 `}}` 缺失被判失败 | 90 | 5.0% | 真实缺口 |
| XPath / JSONPath 解析残余 | 12 | 0.7% | 真实缺口 |
| 其他 CSS | 1 | 0.1% | — |

**两个必须先接受的事实：**

1. **41% 的「失败」是 harness 自己造成的。** `rule-audit.rs:154-162` 只在规则以 `$.` / `$[` / `@json:`
   开头时喂 `{}`，否则一律喂 `<html><body>audit</body></html>`。于是 JSON 书源里的
   `data.list[*]`、`http://api.x.com/{$.id}`、`{$.score}分` 全被当成 CSS 选择器解析并报错。
   **36.6% 是下界，不是真实能力。** 修 harness 之前，任何「静态覆盖率 ≥ N%」的门槛都不可信。
2. **真实缺口的头两名（Java 正则、JSoup 属性选择器）从未出现在任何一版路线图里。**
   它们不是「legado 有而我们没做的功能」，而是「我们做了但语义和 Java 生态对不上的地方」——
   照着字段表和 token 表都看不见，只有跑真实语料才会暴露。

#### 0.3.3 真实缺口明细（P1 的输入）

**① 引擎比 legado 严格（338 条）—— 最便宜的一项**

legado 遇到空规则、空分支、未闭合引号一律**返回空结果**，本项目**返回 `Err` 并使整源判定为受阻**。

| 错误 | 条数 | legado 行为 |
| --- | ---: | --- |
| `rule is empty`（`evaluator.rs:89`，空 Regex 规则） | 298 | 返回空列表 |
| `rule contains an unclosed quote` | 22 | 按字面量处理 |
| `unclosed balanced group` / `empty branch around && \|\|` | 18 | 忽略空分支 |

改法：把这三类从 `Err` 降级为「空结果 + 一条 `tracing::debug`」。**预计一次性解锁上百个源。**

**② Java 正则 → Rust `regex`（263 条）**

`##` 替换指令里的正则直接交给 `regex` crate 编译。Java 允许转义任意字符，Rust 不允许：

```
\（.*\|\(.*\|免…      → error: unrecognized escape sequence
\《\|\》              → 同上
\【\|\】              → 同上
{{chapter.title}}…    → 模板未先展开就当正则编译
```

改法：编译前做一层 Java→Rust 正则归一化（剥离对非元字符的多余转义、`\d` 等保留、
先展开 `{{}}` 再编译），编译仍失败的降级为字面量替换。**这是 v1 就登记、三版路线图都没关闭的风险，现在有了数字。**

**③ JSoup 宽松属性选择器与伪类（202 条）**

JSoup 接受未加引号、含 `:` `|` 空格 正则的属性值，`scraper`/`cssparser` 不接受：

| 形态 | 条数 | 例 |
| --- | ---: | --- |
| `[property~=a\|b\|c]` | 约 65 | `[property~=category\|status\|update_time]`（单条 53 次） |
| `[property=og:xxx]` | 约 68 | `[property=og:novel:update_time]` |
| 属性值含空格 / 正则 / `?` | 约 30 | `[class=s6 wid6]` `[href*=/renwu/]` `[property~=las?test_chapter_name]` |
| `:contains()` / `:eq()` | 16 | `span:contains(作者：)` `li:eq(1)` |
| `src\|class.red` 等命名空间误判 | 约 23 | — |

改法：在交给 `scraper` 之前做属性选择器归一化（自动加引号、`~=a|b` 拆成 `:is()` 或多选择器），
`:contains()` / `:eq()` 用后置过滤自行实现。**不引入新依赖。**

**④ JS 运行时与 Rhino 差异（160 条）**

| 现象 | 条数 | 说明 |
| --- | ---: | --- |
| `xxx is not defined`（`src` `title` `org` `run` `i` `nx`） | 42 | 上下文变量未注入；legado 注入 `src`/`title`/`result`/`baseUrl`/`book`/`chapter` 等 |
| `not a function` | 26 | 剩余 `java.*` 方法缺失 |
| `invalid keyword: with` | 24 | QuickJS 严格模式拒绝 `with`；Rhino 允许 |
| `invalid redefinition of a variable` | 6 | Rhino 容忍重复 `var`/`let` |
| 其他（解析器差异等） | 62 | 逐条看 |

另有 51 条 `cannot read property of null` 属于 harness 假输入，不计入。

**⑤ 明确不支持（不计入缺口）**：9 个源（1%）通过 Rhino 直接访问 `java.lang.*` / `java.util.*` 等 JVM 类，
QuickJS 原理上做不到。导入时标注，不静默失败。

### 0.4 在线抽样（530 源，2026-09-05）

```
上一轮：158 / 530 = 29.8%
最新：  249 / 530 = 47.0%   （+91 源，+17.2pp）
```

`r.txt` 的 280 条失败归类：

| 类别 | 条数 | 归因 |
| --- | ---: | --- |
| 连接超时 | 181 | 源站慢 / 网络 / 代理 |
| 无法连接目标站点 | 13 | DNS / 防火墙 / 代理 |
| Cloudflare JavaScript challenge | 22 | 需 WebView 认证；HTTP 客户端不绕过 |
| HTTP 403 / 404 / 400 / 451 | 9 / 9 / 5 / 5 | 其中仅 **6 条**标注「需要重新认证」 |
| 规则脚本 `source error` | 24 | QuickJS 兼容缺口 |
| 请求 `parse error` | 9 | 请求选项 / 编码格式 |

**主因是 194 条连接类失败（69.3%），不是规则引擎。**
静态覆盖率和在线可用率必须永远分开报告 —— 前者衡量引擎，后者还包含 DNS、超时、站点存活、认证与 challenge。

---

## 1. 排期总览

```
P0  度量修正：让 rule-audit 说真话          3 天    ◀ 当前入口，阻塞其余全部排期
      ↓
P1  按真实归类补齐引擎                      2 周
P2  调试器增量                             3 天   ┘ 与 P1 并行
      ↓
P3  阅读体验补完                            1.5 周
P4  下载 / 缓存 / 导出                      2 周
P5  书源生态                                2 周
P6  RSS / legado 兼容备份 / 设置            2 周
P7  性能 / 稳定性 / 发布                    2 周
```

两条纪律（沿用 v1，**其中第一条当前处于违反状态**）：

1. **每个 P 收尾复查最大文件，非测试行 > 250 即拆。**
   `js_runtime.rs` 从 847 行涨到 **2,080 行**，是纪律写进路线图后反而恶化的唯一文件。
   `search_service/mod.rs` 755、`http/request.rs` 697、`source_service.rs` 568 同样越线。
2. **每一条验收都必须能用一条命令或一个文件路径证明。**

---

## 2. P0 · 度量修正（3 天）

**没有这一步，P1 做完也不知道做对了没有 —— 因为 41% 的失败信号是假的。**

### 2.1 修 harness 的假输入（`src-tauri/src/bin/rule-audit.rs`）

当前 `rule-audit.rs:154-162` 按规则串前缀猜输入类型。改为**按书源猜**：
若 `searchUrl` / `ruleSearch.bookList` 表明这是 JSON 源（含 `$.`、`@json:`、URL 带 `api`/`json`），
整源的 dummy 输入用一份结构化 JSON 样本；否则用 HTML 样本。

- [x] 按书源识别 JSON/HTML dummy 输入，避免 JSON 规则误用 HTML 输入
- [x] 745 条 harness 失败降到 100 条以内（完整语料复跑后不再是首要阻塞类别）
- [x] 修完立刻重跑，得到**第一个可信的静态覆盖率**（970 源，746 / 970 = 76.9%，224 个受阻源）

### 2.2 报告按「源」而非「规则」归因

现在报告只有 error → 规则数，无法回答「615 个受阻源里，多少个是被 Java 正则卡住的」。
给 `Audit` 加 `blocked_by: BTreeMap<Category, BTreeSet<source_id>>`，输出一张
**「首要阻塞类别 → 受阻源数 → 修复后可解锁源数」** 表。

- [x] 报告新增「阻塞类别 → 受阻源数」表，可统计 TOP 阻塞类别

### 2.3 保留的既有能力（无需改动）

`READER_STRICT_ENGINE=1` 严格模式 ✅ · `source-audit` 在线 harness bin ✅ ·
语料已进 `tests/corpus/` 并 `.gitignore` ✅ · `txtTocRule.json` 27 条规则已移植 ✅

### 2.4 验收

- [ ] `docs/coverage/rule-audit.md` 中 harness 类失败 < 100 条
- [ ] 报告含「阻塞类别 → 受阻源数」表
- [ ] `defaultData/bookSources.json` 进 `tests/fixtures/`，作为字段级导入回归

---

## 3. P1 · 按真实归类补齐引擎（2 周）

**顺序 = §0.3.3 的条数降序。每完成一项重跑 `rule-audit`，把「受阻源数」的变化写进 commit message。**
不再预设百分比目标 —— 上一版的百分比预测错了 60 个点。

| 序 | 任务 | 失败条数 | 节 |
| ---: | --- | ---: | --- |
| 1 | 严格度对齐：空规则 / 空分支 / 未闭合引号降级为空结果 | 338 | §3.1 |
| 2 | Java 正则归一化层 | 263 | §3.2 |
| 3 | JSoup 属性选择器归一化 + `:contains()` / `:eq()` | 202 | §3.3 |
| 4 | JS 上下文变量注入 + `with` + 重复声明容忍 + 剩余 `java.*` | 160 | §3.4 |
| 5 | 模板 `}}` 缺失容错 | 90 | §3.5 |
| 6 | XPath / JSONPath 残余 | 12 | §3.6 |
| — | Rhino JVM 包访问 | 9 源 | ⛔ 明确标注不支持 |

### 3.1 严格度对齐（最便宜，先做）

状态：✅ 已完成。空 Regex 规则、空分支、未闭合引号/平衡组现在按 legado 语义记录 debug 并返回空结果。

`evaluator.rs:89` 的 `EmptyRule`、`analyzer.rs` 的 `Unclosed` / `EmptyBranch`：
从 `Err` 改为返回空结果 + `tracing::debug`。逐条对照 legado 的 `RuleAnalyzer` / `AnalyzeByRegex` 确认语义。

### 3.2 Java 正则归一化层

状态：✅ 已完成。替换规则编译前执行 Java→Rust 转义归一化，无法编译时回退为字面量替换。

新增 `source_engine/rule/regex_compat.rs`：先展开 `{{}}`，再剥离对非元字符的多余转义，
编译失败降级为字面量替换。附一张 Java↔Rust 差异清单与对照测试。
**这关闭 v1 就登记、至今未关的「Java 正则语义差异」风险。**

### 3.3 JSoup 选择器归一化

状态：✅ 已完成。支持宽松属性值、`:contains(...)` 文本过滤和 `:eq(n)` 后置索引过滤。

新增 `source_engine/rule/css_compat.rs`：属性值自动加引号、`~=a|b|c` 展开、
`:contains()` / `:eq()` 用后置过滤实现。不引入新依赖。

### 3.4 JS 运行时对齐 + **强制拆分 `js_runtime.rs`**

状态：🟡 已完成上下文别名与宽松求值对齐，并通过 `include!` 将运行时代码分文件；独立模块边界与非测试行 < 250 的要求仍未完成。

- [x] 注入 legado 的上下文变量集（新增 `src`、`title`，既有 `result` `baseUrl` `book` `chapter` `source` `cookie` 保留）
- [x] 以非严格模式求值以支持 `with` 与重复声明
- 已新增 `java.decodeURI`、`java.toInt`、`java.toBoolean`、`java.isNull`；尚未按真实失败清单逐项验证与 legado 的语义一致性，不能视为剩余 `java.*` 已补齐。
- 文件已拆到 `js_runtime/runtime.rs` 与 `bindings/{net,rule,ctx,crypto}.rs`；当前仍以 `include!` 共用作用域，`codec.rs` 仅为占位，部分文件仍超过 250 行，需继续按职责拆分。
- 验证：`cargo test --manifest-path src-tauri/Cargo.toml --lib` 在分文件后通过 200 项；新增四个方法后，`cargo test --manifest-path src-tauri/Cargo.toml --lib source_engine::rule::js_runtime` 通过 19 项既有测试，新增方法的针对性回归测试待补。

⛔ 不做：`toast` / `longToast`（给空实现）· `getReadBookConfig*` · `queryTTF` / `replaceFont`

### 3.5 模板容错

状态：✅ 已完成。`engine.rs` 遇到未闭合 `{{` 时按字面量保留剩余文本，不再让整条规则报错。

### 3.6 XPath / JSONPath 残余

仅 12 条，按报告逐条修，跑不通的明确报错。**不引入 XPath crate。**

### 3.7 书源模型字段补齐（与上表并行）

按实测填充率排序，`≥25%` 的先做：

| 优先 | 字段 | 填充率 |
| :---: | --- | ---: |
| P0 | `ruleSearch.kind` · `ruleSearch.lastChapter` | 70% / 66% |
| P0 | `ruleExplore.*` 长尾字段 + 发现页分页 | 46% |
| P1 | `ruleBookInfo.tocUrl` · `ruleSearch.intro` | 41% |
| P1 | `ruleBookInfo.wordCount` · `ruleSearch.wordCount` | 33% / 29% |
| P1 | `ruleContent.replaceRegex`（并入 §3.8） | 25% |
| P2 | `ruleContent.imageStyle` · `ruleSearch.checkKeyWord` · `ruleBookInfo.init` | 18% / 17% / 14% |
| P2 | `ruleToc.updateTime` · `ruleToc.isVip` | 11% / 6% |
| — | `ruleReview.*` · `ruleBookInfo.relatedBooks` | ⛔ 不做 |

旧扁平 CSS 列（`search_item_selector` 等）标记 deprecated，随 `selector.rs` 兜底一并清理。

### 3.8 内容后处理（`ContentProcessor` 对位）

legado 顺序：`sourceRegex` 切分 → `replaceRegex` → 全局 `ReplaceRule` → 简繁转换 → 分段。
**现在一条都没有。** 新增 `replace_rules` 表：
`name / group / pattern / replacement / isRegex / scope / scopeTitle / scopeContent / excludeScope / order / enabled`。

### 3.9 验收

- [ ] 静态覆盖率：以 P0 修正后的口径为基线，**受阻源数下降 ≥ 70%**（当前可信基线：224 个受阻源）
- [ ] 在线可用率：`source-audit` 跑 530 源，**排除连接类失败后**，规则侧失败 ≤ 5%
- [ ] `selector.rs` CSS 兜底已删除；全项目 `Result<_, String>` 归零（当前 10 处 / 5 文件）
- [ ] 单源规则执行 P95 < 200ms（不含网络）
- [ ] JS 死循环脚本 5 秒内被终止且不影响主进程
- [ ] **最大单文件非测试行 < 250**（当前 `js_runtime.rs` 2,080 / `search_service` 755 / `http/request.rs` 697 / `source_service.rs` 568）
- [ ] Rhino JVM 包访问的 9 个源被明确标注为不支持，非静默失败

---

## 4. P2 · 调试器增量（3 天，与 P1 并行）

四阶段调试、流式推送、不重启改规则已具备（`source_debug_service.rs` 310 行 + 前端 269 行）。补三点：

1. **展示 `AnalyzeUrl` 的最终请求** —— URL / method / headers / body / charset / 命中的选项对象。
   「规则对但请求错」这类问题唯一的定位手段。
2. **展示认证态** —— 会话是否过期、本次是否携带 token/cookie/sign、是否触发 Cloudflare。
3. **一键把当前失败源导出为 fixture** —— 响应 HTML + 书源 JSON 直接写进 `tests/fixtures/`，
   让每个真实失败自动变成回归测试。

验收：

- [ ] 能定位一个真实失败书源的失败步骤，并在不重启应用的前提下改规则重试
- [ ] `source_engine` 行覆盖率 ≥ 70%（当前 216 个 Rust 测试，未测行覆盖率）
- [ ] CI 跑全量 fixture < 30s

---

## 5. P3 · 阅读体验补完（1.5 周）

第一批已落地（2026-09-04）：长章节段落窗口、滚动/分页双模式、方向键翻页、
字号·行距·页边距持久化、章内搜索。**剩余：**

- **精确分页排版** —— 页宽/页高计算，改字号后重排进度不丢（当前为近似还原）
- **书签落库** —— 现在只活在 `ReaderPane.vue`，无数据表、重启即失
- **主题** —— Light / Dark / Sepia / Green / Black，Design Tokens 驱动（`plan.md` §8：不写死颜色）
- **阅读时长** —— 对位 `readRecord`
- **接入 §3.8 的净化替换规则**

不做：仿真翻页动画、竖排、字体反爬、图片/漫画渲染。

验收：

- [ ] 100 万字单章 TXT 打开 < 1s，滚动无掉帧
- [ ] 分页模式下改字号 → 重排后进度不丢
- [ ] 重启后书签仍在
- [ ] 用本项目读完一本真实在线书

---

## 6. P4 · 下载 / 缓存 / 导出（2 周）

`scheduler/` 目前不存在，本步引入：`DownloadManager`、任务状态机落库、三级缓存。追加：

- 对位 `CacheBook.kt` 的**双层限流**：全局并发上限 + 按书源 `concurrentRate`（复用 `source_engine/url/rate_limit.rs`）
- 导出 TXT / EPUB，对位 `ExportBookService.kt`

---

## 7. P5 · 书源生态（2 周）

| 项 | legado 对位 | 说明 |
| --- | --- | --- |
| **书源批量校验** | `BookSourceCheckService.kt` | 把 `source-audit` bin 搬进 UI，回写 `respondTime` / `lastUpdateTime`，标红失效源 |
| **换源增强** | `ui/book/changesource` | 补章节对齐、`bookUrlPattern` 匹配、`canReName` |
| **分组 / 排序 / 权重 / 启停** | `bookSourceGroup` `customOrder` `weight` `enabledExplore` | 书源上百个之后没有这些就没法管理 |
| **Cookie 持久化** | `Cookie.kt` + `enabledCookieJar` | 现在 cookie 只活在 reqwest 内存 store，重启即失 |
| **导出为 legado 兼容 JSON** | — | 桌面端编辑的源要能回到 Android 端 |

---

## 8. P6 · RSS / 备份 / 设置（2 周）

> **备份格式与 legado 兼容** —— 对位 `help/storage/{Backup,Restore,BackupAES}.kt` 的文件清单与 AES 方案，
> 使桌面端能直接吃 Android 端的备份包。**这是「Legado 桌面版」而非「另一个阅读器」的关键差异点**，
> 优先级高于 WebDAV 同步（后者推迟到 P7 之后）。

---

## 9. P7 · 性能 / 稳定性 / 发布（2 周）

- `rquickjs` 的 C 工具链在 Linux / macOS 各构建一次 —— **v1 登记至今未关闭**，建议提前到 P1 结束时 spike
- 异常场景矩阵（`plan.md` §30）按 P0 的失败分类补齐
- Windows + Linux 打包

---

## 10. 文档漂移（顺手修）

- `ARCHITECTURE.md` 称「使用原生 CSS 而非 Tailwind/shadcn-vue」，但 `32196a3` 已引入 Tailwind。
- `ARCHITECTURE.md` 的 `source_engine` 段落仍描述「当前实现受支持的 CSS 子集」，与已落地的完整规则引擎不符。
- `ARCHITECTURE.md` 未提及 `explore`、`source_debug`、`source_engine/url/` 三个新模块。

---

## 11. 度量与验收命令

```bash
# 三件套
cargo test   --manifest-path src-tauri/Cargo.toml --target-dir src-tauri/.cargo-target
cargo clippy --manifest-path src-tauri/Cargo.toml --target-dir src-tauri/.cargo-target \
             --all-targets -- -D warnings
npm run lint && npm run format:check && npm run build

# 静态覆盖率（无网络，进 CI）
READER_STRICT_ENGINE=1 cargo run --bin rule-audit -- \
  --corpus src-tauri/tests/corpus/ --out docs/coverage/

# 在线可用率（需网络，不进 CI）
READER_STRICT_ENGINE=1 cargo run --bin source-audit -- \
  --corpus src-tauri/tests/corpus/f3f55c6e-723b-4055-b254-124c9d88c5cb.json \
  --keyword 剑来 --concurrency 8 --out audit.csv
```

**§0.3.2 归类表的取数方式**：解析 `docs/coverage/rule-audit.md` 的 `## Execution errors` 表，
按 351 行错误文本分桶累加「Rule count」列。分桶规则即 §0.3.2 表的类别名。

**§0.1 体量的取数方式（2026-09-05 实测）**：

```bash
L="/d/Code/chatting/legado-with-MD3/app/src/main/java/io/legado/app"
wc -l $L/model/analyzeRule/*.kt $L/model/webBook/*.kt | tail -1
ls $L/data/entities/*.kt | wc -l ; grep -c "fun " $L/help/JsExtensions.kt

find src-tauri/src -name "*.rs" -exec cat {} + | wc -l
find src \( -name "*.vue" -o -name "*.ts" \) -exec cat {} + | wc -l
find src-tauri/src -name "*.rs" -exec wc -l {} + | sort -rn | head -10
```

---

## 12. 版本与门槛

| 版本 | 阶段 | 门槛（每条可验证） | 状态 |
| --- | --- | --- | :---: |
| v0.1.0 | v1 Step 0 | 本地阅读闭环 + 干净架构 | ✅ |
| **v0.2.0** | **P0** | rule-audit harness 修正 + 按源归因表 + `txtTocRule` 移植 | 🟡（代码完成，待完整语料验收） |
| v0.2.1 | — | WebView 认证闭环在真实 Cloudflare 站点验收通过 | 🟡（链路已实现，待人工验收；不承诺无头绕过） |
| **v0.3.0** | **P1 + P2** | 受阻源数较 P0 基线下降 ≥ 70% · 规则侧在线失败 ≤ 5%（排除连接类）· 兜底路径已删 · 最大文件 < 250 行 —— **真正的「Legado 桌面版」起点** | ⬜ |
| v0.4.0 | P3 | 能用本项目读完一本真实在线书 | ⬜ |
| v0.5.0 | P4 | 下载 / 缓存 / 导出 | ⬜ |
| v0.6.0 | P5 | 批量校验 + 换源对齐 + 书源管理 | ⬜ |
| v0.7.0 | P6 | RSS + legado 兼容备份 | ⬜ |
| v1.0.0 | P7 | 性能 / 稳定性 / Windows + Linux 打包 | ⬜ |

> **v0.3.0 的门槛从「静态覆盖率 ≥ 90%」改为「受阻源数下降 ≥ 70%」。**
> 理由：90% 这个数字是用错误口径（token 近似）推出来的，而当前 36.6% 又被 harness 缺陷污染。
> 在 P0 给出可信基线之前，任何绝对百分比门槛都是自欺。

---

## 13. 风险登记

| 风险 | 影响 | 状态 |
| --- | --- | --- |
| **覆盖率口径本身不可信** | 41% 的失败是 harness 假输入造成的；36.6% 既非上界也非真实能力 | **本次新增（§0.3.2）** —— P0 全部为它服务 |
| **用 token 频次预测覆盖率收益** | 2026-09-02 预测「修完七项到 99.1%」，实际 36.6%，误差 60pp | **本次新增（§0.3.1）** —— 教训：token 出现频次 ≠ 引擎能否执行。**排期只依据真实失败归类** |
| **Java 正则与 Rust `regex` 差异** | 263 条规则失败，第二大真实缺口 | ✅ 已加入兼容归一化与字面量回退 |
| **JSoup 选择器宽松度未对齐** | 202 条规则失败 | ✅ 已加入属性选择器归一化、`:contains`、`:eq` |
| **引擎比 legado 严格** | 338 条规则失败，且会让整源判定为受阻 | **本次新增** —— 最便宜的一项，P1 §3.1 |
| **`js_runtime.rs` 2,080 行** | 847 → 2,080，是纪律写进路线图后**反而恶化**的文件 | **升级** —— P1 §3.4 强制拆分，不可再顺延 |
| 线上可用率 47.0% | 端到端仍有 53% 失败 | 主因是 194 条连接类失败（69.3%），非引擎；`source-audit` 需分层统计 |
| Rhino JVM 包访问无法支持 | 9 源（1%）永久不可用 | 选 QuickJS 必须付的账。明确标注，不静默失败 |
| Cloudflare / JS challenge | 22 源不可用 | WebView 认证已落地，待真实站点验收；不承诺无头绕过 |
| `rquickjs` C 工具链跨平台 | Linux / macOS 构建失败 | **v1 登记至今未关闭** —— P1 结束时 spike |
| 兜底路径掩盖失败 | `pipeline.rs` 曾吞掉引擎错误 | `READER_STRICT_ENGINE` 已落地；`selector.rs` 兜底待 P1 删除 |
| 业务在 service 层重新聚团 | `search_service` 755 / `source_service` 568 | 每个 P 收尾复查最大文件 |
| ~~引擎覆盖率无法自测~~ · ~~真实语料不足~~ · ~~XPath crate 不匹配~~ · ~~AnalyzeUrl 缺失~~ · ~~架构重构回归~~ | — | **已关闭** |

---

## 14. 明确不做

| 功能 | legado 位置 | 理由 |
| --- | --- | --- |
| TTS / 朗读 / 有声书 | `TTSReadAloudService` 等 3 个 Service | 桌面端需另找 TTS 后端 |
| 漫画阅读 | `ui/book/manga` | `bookSourceType=2` 图片源，另一套渲染 |
| AI 相关 | `ui/ai` + 8 个 `Ai*` 实体 | `plan.md` §1 已列为暂不做 |
| 段评 `ReviewRule` | `BookChapterReview` | 依赖账号体系 |
| 局域网 Web 服务 | `web/KtorServer.kt` | 等桌面端本体稳定 |
| 字体反爬 | `queryTTF` / `replaceFont` | 单独立项，工作量大 |
| UMD / MOBI / PDF | `model/localBook/` | P3 之后按需求评估 |
| 词典 / 翻译 · 高亮 / 标签规则 | `ui/dict` `ui/highlightTagRule` | 低频 |
| Rhino JS 引擎移植 | `modules/rhino` | 用 QuickJS 替代，代价已量化：9 源（1%）永久不可用 |
| `WebJs` / `ContentRule.webJs` | `AnalyzeUrl` + `ContentRule` | **970 源实测零出现** |
| `dnsIp` / `serverID` | `AnalyzeUrl` UrlOption | DoH 与多服务器调度，桌面端价值低 |

---

## 15. 下一步（可立即开工）

| # | 任务 | 产出 | 节 |
| ---: | --- | --- | --- |
| 1 | ✅ **修 `rule-audit.rs` 的假输入**：按书源判定 JSON/HTML | 已完成，待完整语料确认收益 | §2.1 |
| 2 | ✅ **报告按源归因**：`blocked_by: Category → Set<source_id>` | 已完成 | §2.2 |
| 3 | ✅ **严格度对齐**：空规则 / 空分支 / 未闭合结构降级为空结果 | 已完成 | §3.1 |
| 4 | ✅ **Java 正则归一化层** | 已完成 | §3.2 |
| 5 | ✅ **JSoup 选择器归一化 + `:contains()` / `:eq()`** | 已完成 | §3.3 |
| 6 | **拆 `js_runtime.rs`（2,080 行）并对齐 JS 语义** | 已分文件并新增四个 `java.*` 方法；独立模块、行数限制及新增方法语义回归待完成 | §3.4 |

**第 1、2 项必须先做。** 在此之前，第 3–6 项的收益无法度量 —— 而上一轮正是因为在错误口径上排期，
把七项全做完却只换来 17 个百分点。

### v0.2.1 人工验收步骤（实现已具备）

1. 导入一个配置了 Cloudflare challenge 的真实书源，确认 `base_url` / `login_url` 可访问。
2. 点击「浏览器认证」，在 `source-auth-*` 窗口完成 Cloudflare/Turnstile 验证。
3. 点击「读取浏览器会话」，确认书源状态变为「已认证」，且会话 Cookie 已持久化。
4. 点击「测试」，确认探针不再报告 Cloudflare challenge 并能解析出结果；若站点仍返回 challenge，
   状态保持可重试且不会误标为「已过期」。

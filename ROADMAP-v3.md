# Reader Desktop 路线图 v3

> v1 按字段表排期，错了。v2 按 token 频次排期，错了 60 个百分点。v2 改用真实失败归类后，
> **后端已经基本对了 —— 但前端一直没有被度量过。**
> 本版把度量口径扩到前端：以 `desktop-ui.html`（2,798 行静态原型）为目标态，
> 逐页对照现有 43 个前端文件，并据此重排。
> `ROADMAP.md`（v1）、`ROADMAP-v2.md`（v2）作为历史保留，**优先级一律以本文件为准**。

**一句话现状：后端 50 个 IPC 命令、148 个 Rust 文件、静态覆盖率 81.2%，已经能撑起原型里
七个页面中的五个；前端却只有 4,168 行 TS·Vue（另有 1,032 行手写 CSS），
其中一个 439 行的 `AppShell.vue` 同时充当标题栏、导航栏、阅读器顶栏和页面分发器，
9 条路由全部指向同一个组件，三个页面（RSS / 历史 / 书签）渲染的是同一份占位假数据。
瓶颈已经从「引擎写得对不对」转移到「前端根本没建起来」。**

---

## 0. 校验结果：v2 声明 vs 实测

以下逐条核对 `ROADMAP-v2.md` 中打勾的条目。**大部分属实**，三处需要修正。

### 0.1 属实的（抽查确认）

| v2 声明 | 实测 |
| --- | --- |
| P0 度量修正、按源归因表 | ✅ `docs/coverage/rule-audit.md` 头部：970 源 / 22,220 规则串 / 788 可执行（81.2%）/ 受阻 182 |
| §3.1 严格度对齐 | ✅ 空规则、空分支、未闭合引号已降级为空结果 |
| §3.2 Java 正则归一化 | ✅ `source_engine/rule/regex_compat.rs`，含 `\Q...\E` 处理 |
| §3.4 `js_runtime` 拆分 | ✅ 拆成 21 个文件（含 `bindings/`），最大 199 行（`bindings/net.rs`） |
| P4 下载 / 缓存 / 导出 | ✅ `download_service.rs` 状态机 + 4 并发 + 续跑；`export.rs` TXT/EPUB；缓存配额与 LRU |
| P5 书源生态 | ✅ 批量校验、换源、分组排序权重、Cookie 持久化、legado JSON 往返 |
| P6 本地 JSON 备份 | ✅ `backup_service.rs` 单事务恢复 + 版本校验 + 100 MiB 上限 |
| 数据表 | ✅ 24 个迁移；`018_bookmarks` / `019_reading_time` / `020_replace_rules` 均在 |
| `export_source_fixture` | ✅ `command/source.rs:58`，已注册进 `lib.rs:80` |
| `selector.rs` 兜底未删 | ✅ 诚实标注未完成 —— 355 行仍在，被 `pipeline/stages.rs` 四处调用（:50 :179 :225 :251） |
| RSS / WebDAV / TTS / AI / 局域网 Web / legado AES 备份未实现 | ✅ 全库搜索无匹配，与 §14「明确不做」一致 |

### 0.2 需要修正的三处

**① §3.9「最大单文件非测试行 < 250」打了 ✅，实测未达标（但只是擦线）**

按「首个 `#[cfg(test)]` 之前的行数」口径实测，越线与逼近的生产文件：

| 文件 | 非测试行 |
| --- | ---: |
| `source_engine/pipeline/stages.rs` | **252** ← 唯一越线 |
| `source_engine/url/parser.rs` | 247 |
| `service/search_service/probe.rs` | 242 |
| `source_engine/rule/xpath.rs` | 241 |
| `source_engine/import.rs` | 233 |
| `domain/source.rs` | 232 |
| `service/source_debug_service.rs` | 229 |
| `source_engine/rule/jsoup.rs` | 225 |
| `service/search_service/search.rs` | 221 |
| `source_engine/rule/position.rs` | 219 |
| `source_engine/rule/analyzer.rs` | 214 |

比起 v2 时代 2,080 行的 `js_runtime.rs`，这是量级不同的问题 —— 超 2 行，不是超 8 倍。
但**打勾的条目必须为真**，否则纪律本身失效。改法：拆 `stages.rs`，并把 214–249 那一档记为观察区。

**② `css_compat.rs` 不存在 —— 文档漂移**

v2 §3.3 称新增 `source_engine/rule/css_compat.rs`。该文件不存在。
功能实际落在 `source_engine/rule/jsoup/legacy.rs`（80 行）：
`normalize_css_compat`（:46）、`css_contains`（:72）、`css_eq`（:77）。
**功能是真的，路径是假的。** 只需改文档。

**③ §0.1 体量数字已过时约一倍**

| 维度 | v2 记录（2026-09-05） | 实测 | 偏差 |
| --- | --- | --- | --- |
| Rust | 13,985 行 / 73 文件 | **18,949 行 / 148 文件** | +35% 行 / +103% 文件 |
| TS·Vue | 2,436 行 / 34 文件 | **4,168 行 / 41 文件**（另 `styles.css` 1,032 行） | +71% |

文件数翻倍而行数只涨 35%，说明拆分纪律确实在起作用 —— 这是好消息，只是没被记录。

### 0.3 v0.3.0 门槛：未达标，且已停滞

```
可信基线（P0 修正后）：受阻 224 源
最新实测：            受阻 182 源   —— 下降 18.8%
门槛要求：            下降 ≥ 70%（即 ≤ 67 源）
```

v2 §3.1–§3.6 六项引擎任务全部标记完成，受阻源却只降了 42 个。
**这是 v2 自己犯过的错误的第三次重演：任务清单打完勾 ≠ 指标达成。**
v3 不再假设「做完 §X 就会达标」，而是把「受阻源数」本身作为唯一验收信号（§6.2）。

---

## 1. 前端差距：本版新增的度量维度

### 1.1 现状盘点

```
src/  41 个 .ts/.vue 文件 / 4,168 行  +  styles.css 1,032 行手写 CSS
```

| 问题 | 证据 | 性质 |
| --- | --- | --- |
| **`AppShell.vue` 439 行的单体** | 同时含 titlebar、rail、阅读器顶栏、阅读设置面板、页面分发、窗口控制、字体菜单 | 与后端 250 行纪律对等的违规，且是前端唯一的"神组件" |
| **路由是装饰性的** | `router/index.ts` 9 条路由 **全部** `component: AppShell`；真实切换靠 `useAppShell` 内部的 `view` ref | 双轨状态，URL 与视图不同步，无法深链接 |
| **三个页面渲染假数据** | `LibraryTabPage.vue` 被 rss / history / bookmarks 复用，全部渲染 `props.books.slice(0, 8)` | ⚠️ **见 §1.3** |
| **死代码** | `SourceManager.vue`（215 行）在 `index.ts:2` 导出，全库无 import；`/sources` 实际渲染 `SourceDebugPage` | 书源管理 UI 事实上不可达 |
| **没有首页** | 原型 `data-page="home"` 的整个仪表盘（最近在读 / 统计 / 每日目标 / 快捷入口）无对应实现 | 缺页 |
| **设置页是平铺长条** | `SettingsPage.vue` 61 行 4 个 section；原型是左导航 + 9 个 pane | 结构不对位 |
| **Tailwind 装了但没用** | `@tailwindcss/vite` 已配（`vite.config.ts:6`），但 `@theme` 只有两个字体变量，颜色全是硬编码十六进制（`#f5f7fb` `#24334d` `#c7d2e2`），实际样式来自 1,032 行手写 class | 无设计令牌系统 |
| **无组件库** | `node_modules` 中无 shadcn-vue / reka-ui / cva / clsx / tailwind-merge | 每个控件都手搓 |
| **深色模式是 class 覆盖** | `.app-theme-dark` 整体覆写，非令牌切换 | 无法支持原型的多主题（预设主题 / 种子色） |

### 1.2 目标态盘点（`desktop-ui.html`）

7 个页面、55 个图标、可折叠导航栏、9 组快捷键、右键菜单、toast、批量操作条。

> **关键发现：原型有两套 CSS 变量。** 第一套 M3 + 玻璃拟态（:11–105），
> 第二套在 :1045 起原样重声明并覆盖，注释写着
> `Tailwind / shadcn-style visual layer — replace the Material/Glass look`。
> **实际渲染的是第二套 slate 配色、`.25rem`–`.75rem` 圆角、无 backdrop-filter 的 shadcn 皮肤。**
> 第一套是死 CSS，不要移植。这也意味着"改用 shadcn-vue"不是换风格，而是**回到原型本来的风格**。

| 页面 | 结构 |
| --- | --- |
| home | 双栏 `1fr / 336px`；主栏 hero 最近在读 + 3 格统计 + 最近阅读网格；侧栏 每日目标环 + WebDAV 卡 + 快捷入口 |
| bookshelf | 双栏；左分组侧栏，右 sticky 工具条（搜索 / 排序 / 筛选 / 刷新 / 网格列表切换）+ 书卡网格 + 多选浮动条 + 右键菜单 |
| explore | 双栏；左分类 chip + 书源列表（状态圆点），右 sticky 书源栏 + 分类 chip + 书卡列表 |
| rss | **三栏** `220 / 300 / 1fr`：订阅源 → 文章列表 → 正文 |
| read | 见 §4 |
| my | 多列自适应；Web 服务卡 + 规则入口组 + 其他入口组 |
| settings | 双栏；左 9 项导航，右 pane 切换 |

### 1.3 一条必须写进纪律的教训

v2 最贵的两次失误本质相同：
**harness 喂假输入，导致 41% 的失败是假的；token 频次预测，导致 60pp 的收益是假的。**

`LibraryTabPage.vue` 是同一个错误在前端的版本 —— 它在「RSS 订阅」标题下渲染书架里的书，
在「书签」标题下也渲染同样的书。看起来功能存在，实际零后端。
**任何人（包括我们自己）看这个 UI 都会得出错误的完成度判断。**

> **纪律 F0：不渲染似是而非的数据。**
> 后端未接入的模块，一律显示明确的「未接入」空状态，标注所需能力，
> 不得用其他数据源的内容填充。占位可以，伪装不行。

---

## 2. 后端能力 → 原型页面的映射

50 个已注册 IPC 命令能覆盖多少原型？逐块对照：

| 原型模块 | 后端状态 | 可用命令 / 缺口 |
| --- | :---: | --- |
| 书架 · 列表 / 分组 / 删除 | ✅ | `list_books` `list_groups` `create_group` `move_book_to_group` `delete_book` |
| 书架 · 批量下载 / 导出 | ✅ | `start_download` `export_book` —— 批量条是纯前端编排 |
| 书架 · 网格列表切换 / 多选 / 右键菜单 / 排序筛选 | ✅ 纯前端 | 无需后端 |
| 书架 · 未读数角标 / 状态徽标 | 🟡 | `chapter_count` 有，已读章数需从进度推导；「更新 / 完结」需新增字段 |
| 发现 | ✅ | `list_explore_categories` `explore_books` |
| 搜索 | ✅ | `search_books` |
| 书源管理 | ✅ | 16 个 `*_book_source*` 命令，含登录 / 浏览器认证 / 批量校验 / 导入导出 |
| 书源调试 | ✅ | `debug_source_stage` `update_book_source_rules` `export_source_fixture` |
| 下载 / 缓存 | ✅ | `list_download_tasks` + 暂停/继续/取消 + `get_cache_stats` `set_cache_quota` |
| 阅读器 · 正文 / 目录 / 进度 / 书签 / 阅读时长 | ✅ | `read_chapter` `list_chapters` `*_reading_progress` `*_bookmark` `add_reading_time` |
| 阅读器 · 净化替换 | ✅ | `list_replace_rules` 等 4 个 |
| 阅读器 · 双栏分页 / 沉浸模式 / 正文搜索 / 排版控件 | ✅ 纯前端 | 无需后端 |
| 设置 · 备份恢复（本地 JSON） | ✅ | `export_backup` `restore_backup` |
| 设置 · 代理 / UA | ✅ | `get_app_settings` `save_app_settings` |
| **首页 · 统计 / 每日目标** | 🟡 | `get_reading_record` 有原始时长，缺聚合命令 |
| **书籍详情页** | 🟡 | `fetch_book_info` 有，无独立页面 |
| **TXT 目录规则 / 封面规则管理** | 🟡 | `txtTocRule.json` 已打包，无 CRUD 命令 |
| **高亮 / 批注** | ⬜ | 零后端（书签有，高亮没有） |
| **托盘 / 开机自启 / 启动页** | ⬜ | 零后端 |
| **RSS** | ⬜ | 零后端 —— 整个三栏页面 |
| **TTS 听书** | ⬜ | 零后端 —— 面板 tab + 底部 ttsbar |
| **AI** | ⬜ | 零后端 —— 面板 tab + 我的页入口 + 设置 pane |
| **翻译** | ⬜ | 零后端 —— 工具按钮 + 设置 pane |
| **WebDAV** | ⬜ | 零后端 —— 首页卡片 + 设置 pane |
| **局域网 Web 服务** | ⬜ | 零后端 —— 我的页卡片 |
| **legado 兼容 ZIP/AES 备份** | ⬜ | 现有仅自定义 JSON 格式 |

**结论：原型七页里，book­shelf / explore / read / settings / my 五页的核心链路后端已就绪，
现在就能建。home 差一个统计聚合命令。rss 整页需要新子系统。**
阅读器里的 TTS / AI / 翻译三块按纪律 F0 显示未接入。

---

## 3. 前端架构决策

### 3.1 技术栈

在现有 Tailwind v4 之上引入 **shadcn-vue（Reka UI 版）**：

```
新增依赖：reka-ui  class-variance-authority  clsx  tailwind-merge
dev：     tw-animate-css
保留：    lucide-vue-next（shadcn 默认图标库，已在用）
配置：    components.json · @/ alias（vite.config.ts + tsconfig.json）· style=new-york
```

shadcn-vue 当前版本原生支持 Tailwind v4 的 `@theme` / `@theme inline`，颜色用 OKLCH，
每个原语带 `data-slot` 属性 —— 与本项目已有的 `@tailwindcss/vite` 配置直接兼容，
不需要降级或双装 Tailwind。

### 3.2 设计令牌迁移

**来源：`desktop-ui.html:1045–1121`（生效的那套），不是 `:11–105`（死 CSS）。**

原型的 slate 配色与 shadcn 默认主题几乎同源，映射直接：

| 原型变量 | 浅色值 | shadcn 令牌 |
| --- | --- | --- |
| `--md-surface` / `--md-surface-container-lowest` | `#ffffff` | `--card` `--popover` |
| `--page-bg` | `#f8fafc` | `--background` |
| `--md-on-surface` | `#0f172a` | `--foreground` `--primary` |
| `--md-on-primary` | `#ffffff` | `--primary-foreground` |
| `--md-surface-container` | `#f1f5f9` | `--muted` `--secondary` |
| `--md-on-surface-variant` | `#64748b` | `--muted-foreground` |
| `--md-outline-variant` | `#e2e8f0` | `--border` `--input` |
| `--md-outline` | `#94a3b8` | `--ring` |
| `--md-error` | `#dc2626` | `--destructive` |
| `--r-lg` / `--r-xl` | `.75rem` | `--radius` |
| `--reader-paper` / `--reader-ink` / `--reader-ink-soft` | `#fcfbf7` / `#292524` / `#78716c` | **阅读器专属，不并入 shadcn 令牌** |

深色主题同表（`html[data-theme="dark"]`，:1085–1121）。
布局尺寸 `--rail-w:56px` `--rail-w-ext:220px` `--titlebar-h:40px` 与动效 `--dur:.16s` 单独保留。

现有 `styles.css` 的 1,032 行手写 class 随各页面重建逐步删除，**不保留兼容层** ——
兜底路径掩盖失败是 v2 已经登记过的风险（`selector.rs` 至今没删就是先例）。

### 3.3 组件映射

| 原型 class | shadcn-vue 组件 |
| --- | --- |
| `.rail`（可折叠 + Ctrl+B） | `Sidebar`（内置图标折叠态与 Ctrl+B，直接对位） |
| `.btn--filled / --tonal / --outline / --text` | `Button` variant `default / secondary / outline / ghost` |
| `.icon-btn` | `Button` `size="icon" variant="ghost"` |
| `.card` `.card__head` `.card__body` | `Card` / `CardHeader` / `CardContent` |
| `.seg` | `ToggleGroup` |
| `.tabs` | `Tabs` |
| `.chip` | `Badge` / `Toggle` |
| `.switch` | `Switch` |
| `.field` | `Input` |
| 滑块（字号 / 行距 / 亮度…） | `Slider` |
| `.ctx` 右键菜单 | `ContextMenu` |
| `.hint` toast | `Sonner` |
| Ctrl+K 全局搜索 | `Command`（command palette） |
| 三栏 RSS / 发现分栏 | `Resizable` |
| 下拉（排序 / 更多操作） | `DropdownMenu` |
| 侧滑面板 / 弹窗 | `Sheet` / `Dialog` |
| `.row` `.setting-row` `.group` | 自建复合组件 `SettingRow` / `SettingGroup`（无对应原语） |

图标：原型 55 个 sprite id 映射到 lucide-vue-next；lucide 缺失的少数几个再手绘为本地 SVG 组件。

### 3.4 阅读器例外（按需求 3）

`src/features/reader/**` **不使用 shadcn 组件**，理由是具体的而非偏好：

- 双栏分页（`columns:2`）+ 响应式退化单栏 + 首字下沉 + 沉浸模式折叠，都是排版引擎问题，
  shadcn 原语（基于 Reka UI 的无障碍行为原语）在这里不提供任何东西；
- v2 P3 遗留的**精确分页排版**需要稳定、可测量的盒模型，
  工具类叠加与 `data-slot` 注入会让页宽页高计算难以复现；
- 阅读器有独立的令牌集（`--reader-paper` / `--reader-ink` / `--reader-ink-soft` + 五种阅读主题），
  与应用 chrome 的 shadcn 令牌是两套语义。

**边界定义**：阅读器的**外壳**（右侧面板里的 Tabs、Switch、Slider、Select、下拉菜单）
仍然用 shadcn —— 那是普通表单控件。不用 shadcn 的是**正文渲染区、目录列表、分页/进度层、沉浸模式布局**。

### 3.5 前端纪律

沿用后端两条，新增一条：

1. **单个 `.vue` SFC ≤ 200 行**（template + script 合计，不含 `<style>`）。
   `AppShell.vue` 439 行是当前唯一严重越线者，P0 必须拆。
2. **每条验收都能用一条命令或一个文件路径证明。**
3. **纪律 F0（§1.3）：不渲染似是而非的数据。**

---

## 4. 阅读器目标态（原型 `page--reader` 逐项）

```
.reader
├─ topbar    返回 · 书名/章节/百分比 · 正文搜索框(命中数) · 目录T · 书签 · 沉浸F · 更多
├─ body
│  ├─ toc     标题+章节搜索 · 按卷分组 · 章节状态(已读/百分比) · 当前章高亮 · 宽度 0↔ 过渡折叠
│  ├─ stage   双栏分栏排版 · 卷名 eyebrow · h2 · 首字下沉 lead · 对话行 · mark 高亮 · 批注角标
│  │          └─ pager  上/下一页 · 第 N/M 页 · 本章百分比
│  │          └─ 响应式：视口高 <820px 或容器宽 <760px 退回单栏滚动
│  └─ panel  (320px 折叠) Tabs: 阅读样式 / 听书 / AI
│       样式 → 字体三选 · 字号 · 行距 · 段距 · 页边距 · 首行缩进 · 两端对齐
│              · 翻页动画 3×2 · 背景四色 · 亮度 · 护眼 · 跟随系统亮度
│       听书 → 语速 · 音色 · 定时停止 · 预下载 · 角色音色分配
│       AI   → 本章总结 · 人物关系 · 知识卡片 · 事件时间线
└─ bottombar
   ├─ ttsbar     迷你封面 · 朗读进度 · 波形动画 · 上/暂停/下/关
   ├─ progress   上一章 · 第N/M章 · 可拖动轨道 · 百分比 · 下一章
   └─ tools      正文搜索 自动翻页 目录 听书 阅读样式 加书签
                 日夜间 护眼 翻译 AI总结 文本处理 更多  (12 个)
```

**当前 `ReaderPane.vue` 只有 233 行，覆盖：正文渲染、目录、滚动/分页双模式、章内搜索。**
缺：双栏排版、精确分页、右侧面板三 tab、底部三层工具栏、沉浸模式、拖动进度、批注/高亮、
翻页动画、亮度/护眼。

按纪律 F0：**听书 tab、AI tab、翻译工具、文本处理工具 —— 渲染为标注了缺失能力的未接入态。**

---

## 5. 排期

两条轨道并行。前端轨是本版重心；引擎轨是 v2 未关闭项的收尾。

```
前端轨                                引擎/后端轨
F0 地基（阻塞全部前端工作）    ┐
     ↓                        │      E0 v0.3.0 门槛收尾（与 F 并行）
F1 壳层与导航                  │           ↓
     ↓                        │      E1 前端所需的新后端能力
F2 已有后端能力的页面重建      ┘
     ↓
F3 阅读器重建
     ↓
     └──────────┬──────────┘
                ↓
        S 独立子系统（各自可单独立项）
                ↓
        R 性能 / 稳定性 / 发布
```

### F0 · 前端地基

**没有这一步，后续每个页面都会继续往 `AppShell.vue` 里堆。**

- [x] 装 shadcn-vue：`reka-ui` `cva` `clsx` `tailwind-merge` `tw-animate-css`，
      `components.json`，`@/` alias 进 `vite.config.ts` + `tsconfig.json`，`lib/utils.ts` 的 `cn()`
- [x] 令牌迁移：按 §3.2 把 `desktop-ui.html:1045–1121` 写成 `@theme inline` 的 light/dark 两套
- [x] 拆 `AppShell.vue`（439 行）→ `AppLayout` / `AppTitlebar` / `AppRail` / `AppBreadcrumb` / `<RouterView>`；
      阅读器顶栏与阅读设置面板移入 `features/reader/`（`ReaderPage` / `ReaderTopbar` / `ReaderSettingsPanel` / `ReaderCatalog`）
- [x] **路由真实化**：13 条路由各自指向真实组件，删除 `useAppShell.view` 双轨状态；
      支持 `#/read/<bookId>?toc=0&panel=1` 形式的深链接（原型 :2778–2794 已定义）
- [x] 处置死代码：`SourceManager.vue` 删除，重建为 `SourceManagerPage.vue` 接回 `/sources`
- [x] **删除 `LibraryTabPage.vue` 的假数据三页**，换成 `NotConnected.vue` 未接入空状态（纪律 F0）
- [x] 生成首批组件：`button card input textarea tabs badge separator switch slider select
      dropdown-menu context-menu dialog sheet tooltip toggle-group scroll-area
      sonner progress skeleton sidebar command resizable`（144 个文件，`src/components/ui/`）

验收：
- [x] `npm run build` 通过；`grep -c 'component: AppShell' src/router/index.ts` 为 0
- [x] 全部 `.vue` ≤ 200 行：`find src -name '*.vue' -exec wc -l {} + | sort -rn | head -3`
- [x] `grep -rn 'props.books.slice' src/features` 无结果

**F0 实测（2026-09-09）**：`npm run lint` / `npm run format:check` / `npm run build` 三者全绿；
最大 SFC 190 行（`ReaderPane.vue` 拆分后）；`src/styles.css` 1,032 → **319 行、硬编码十六进制 0**；
`features/*/index.ts` 五个 barrel 全部无人 import，已删除（§6.1 #3）。

### F1 · 壳层与导航

- [x] 可折叠 rail：分组标签「浏览 / 检索 / 阅读」、角标、底部用户卡、Ctrl+B
- [x] titlebar：品牌 + 面包屑 + 主题切换 + 真实 Tauri 窗口按钮（`windowAction` 已可用）
- [x] 快捷键层：`Esc` `Ctrl+B` `Ctrl+K` `D` `1–5`，阅读器内 `→/Space` `←` `T` `F`；
      输入框内跳过（原型 :2757–2776）
- [x] `Ctrl+K` 全局搜索 → shadcn `Command` palette，接 `search_books`
- [x] `error-banner` → `Sonner` toast

验收：
- [ ] 九组快捷键逐一手测通过 —— **未手测**（本机无 GUI 会话，只有静态构建）
- [x] rail 折叠态宽度 56px、展开态 220px

> **F1 偏差登记**：`.rail` 没有直接用 shadcn `Sidebar` 的定位/宽度实现 —— 它的折叠态是 48px
> （`3rem`）、展开态 256px（`16rem`），与本版验收要求的 56px / 220px 不符，且 `fixed inset-y-0`
> 会盖住 titlebar。改为：宽度用原型令牌 `--rail-w` / `--rail-w-ext` 自己控制，内部菜单项复用
> shadcn `SidebarHeader/Content/Group/GroupLabel/GroupContent/Menu/MenuItem/MenuBadge/Footer`
> 与 `sidebarMenuButtonVariants`。§3.3 的「直接对位」按**结构对位、宽度按原型**落实。

### F2 · 已有后端能力的页面重建

按后端就绪度排序，全部只用 §2 中标 ✅ 的命令：

1. [x] **书架页** —— 分组侧栏 / sticky 工具条（搜索·排序·筛选·刷新·网格列表切换）/
   书卡（封面·徽标·hover 三按钮）/ 多选 + 浮动批量条 / 右键菜单七项
   （原型九项中的「详情」「换源」需要 E1 的书籍详情命令与换源入口，暂缺）
2. [x] **书源页** —— 重建 `SourceManagerPage`：列表、分组、自定义排序、权重、启停、
   批量校验、导入导出、登录、浏览器认证
3. [x] **书源调试页** —— 四阶段 + 最终请求展示 + 认证态展示 + 一键导出 fixture
   （v2 §4 遗留按钮已接，`DebugOutput.vue` 用 Tabs 分「中间步骤 / 最终 JSON / 原始 HTML / 请求响应头」）
4. [x] **发现页** —— 书源侧栏（状态圆点）+ 分类 chip + 书卡列表
5. [x] **搜索页** —— 沿用现有分层排序逻辑，套新组件（`useSearchView.ts` + `SearchResultCard.vue`）
6. [x] **下载 / 缓存页** —— 任务列表 + 暂停/继续/取消 + 缓存占用与配额
7. [x] **设置页** —— 左导航 + **10 个 pane**；已填 **主题 / 阅读 / 净化替换 / 下载缓存 / 备份恢复 / 其他（代理·UA）** 六个；
   封面 / AI / 翻译 / 实验室四个按纪律 F0 显示未接入
8. [x] **「我的」页** —— 规则入口组 + 其他入口组；Web 服务卡未接入态
9. [x] **首页** —— 最近在读接真实 `list_books`；统计 / 每日目标 / WebDAV 按纪律 F0 显示未接入（等 E1）

验收：
- [ ] 每页与 `desktop-ui.html` 对应 section 并排截图比对 —— **未做**（本机无法截图比对）
- [x] 无一处渲染非本模块数据：`grep -rn 'props.books.slice' src/features` 无结果；
      未接入模块一律走 `NotConnected.vue` 并列出所需能力


### F3 · 阅读器重建（不使用 shadcn 正文层，§3.4）

- [ ] 四区骨架：topbar / toc / stage / panel + bottombar 三层
- [ ] 双栏分栏排版 + `<820px` / `<760px` 响应式退回单栏
- [ ] **精确分页排版**（v2 P3 唯一未完成项）：页宽页高计算，改字号后重排进度不丢
- [ ] 目录侧栏：按卷分组、章节搜索、已读/百分比状态、当前章高亮
- [ ] 样式面板全量控件（字体/字号/行距/段距/页边距/首行缩进/两端对齐/翻页动画/背景/亮度/护眼）
- [ ] 底部：可拖动章节进度轨道 + 12 个工具按钮
- [ ] 沉浸模式（顶底栏 height→0 折叠，非 display:none）
- [ ] 接入已有能力：书签、正文搜索、净化替换规则、阅读时长
- [ ] 听书 / AI / 翻译 / 文本处理：未接入态（纪律 F0）

验收：
- 100 万字单章 TXT 打开 < 1s，滚动无掉帧（v2 遗留未验收）
- 分页模式改字号 → 进度不丢（v2 已实现段落锚点，需在双栏下重验）
- **用本项目读完一本真实在线书**（v2 遗留未验收）

### E0 · v0.3.0 门槛收尾（与 F 轨并行）

- [ ] **受阻源 182 → 门槛见 §6.2**。每次改动重跑 `rule-audit`，把受阻源变化写进 commit message
- [ ] 删 `selector.rs` CSS 兜底（355 行）+ 扁平列迁移到 `raw_rules`；
      先迁移 `pipeline/stages.rs` 四个调用点（:50 :179 :225 :251）
- [ ] `ContentRule` / `ExploreRule` 结构化 —— 目前无独立 struct，靠 `raw_rules` 原始 JSON 懒解析
- [ ] 补 `SearchRule`（5/11）· `InfoRule`（7/13）· `CatalogRule`（4/10）缺口字段，按 v2 §3.7 填充率排序
- [ ] 拆 `pipeline/stages.rs`（252 行，唯一越线）
- [ ] 在线可用率：排除连接类失败后规则侧失败 ≤ 5%（v2 遗留未验收）
- [ ] 单源规则执行 P95 < 200ms（不含网络，v2 遗留未验收）
- [ ] `source_engine` 行覆盖率 ≥ 70%（v2 遗留未验收）
- [ ] `rquickjs` C 工具链在 Linux / macOS 各构建一次 —— **v1 登记至今三版未关闭**

### E1 · 前端所需的新后端能力

按 F 轨的实际阻塞顺序做，不提前：

- [ ] **阅读统计聚合命令** —— 累计读完本数 / 累计时长 / 今日分钟 / 每日目标 / 连续天数（首页需要）
- [ ] **书籍状态字段** —— 未读章数、「更新 / 完结 / 音频」徽标（书架卡片需要）
- [ ] **书籍详情页命令** —— 现有 `fetch_book_info` 之上补简介/分类/字数/更新时间
- [ ] **高亮 / 批注** —— 新表 + CRUD（正文 `<mark>` 与批注角标需要）
- [ ] **TXT 目录规则 / 封面规则 CRUD** —— `txtTocRule.json` 已打包，缺管理命令
- [ ] **托盘 / 开机自启 / 启动页** —— 设置「其他」pane 需要

### S · 独立子系统（各自可单独立项，无内部顺序）

| 子系统 | 涉及原型位置 | 备注 |
| --- | --- | --- |
| **RSS** | 整个 rss 三栏页 + rail 入口 + 角标 | 订阅模型 / 刷新 / 正文 / 收藏 / 规则订阅 |
| **legado 兼容 ZIP·AES 备份** | 设置「备份恢复」 | 对位 `Backup/Restore/BackupAES.kt`；**这是「Legado 桌面版」而非「另一个阅读器」的关键差异点** |
| **WebDAV 同步** | 首页卡 + 设置 pane | 依赖上一项 |
| **TTS 听书** | 面板 tab + ttsbar + 首页入口 | 需选桌面 TTS 后端 |
| **翻译** | 工具按钮 + RSS 正文 + 设置 pane | |
| **AI** | 面板 tab + 我的页 + 设置 pane + 实验室 | v1 `plan.md` §1 曾列为暂不做，原型已纳入 —— 需重新决策 |
| **局域网 Web 服务** | 我的页卡片 | 对位 `web/KtorServer.kt` |

### R · 性能 / 稳定性 / 发布

- 异常场景矩阵（`plan.md` §30）按真实失败分类补齐
- Windows + Linux 打包

---

## 6. 验收门槛

### 6.1 前端（本版新增）

| # | 门槛 | 证明方式 |
| ---: | --- | --- |
| 1 | 无 `.vue` 文件 > 200 行 | `find src -name '*.vue' -exec wc -l {} + \| sort -rn \| head -3` |
| 2 | 路由不再全指向 `AppShell` | `grep -c 'component: AppShell' src/router/index.ts` == 0 |
| 3 | 无死代码导出 | 每个 `features/*/index.ts` 的导出都有 import |
| 4 | 无似是而非的数据（纪律 F0） | 逐页人工核对：未接入模块显示未接入态 |
| 5 | 令牌化完成 | `src/styles.css` 中硬编码十六进制颜色数 == 0 |
| 6 | 手写 CSS 收缩 | `styles.css` 从 1,032 行降至阅读器专属部分 |
| 7 | 七页与原型对位 | home / bookshelf / explore / rss / read / my / settings 逐页截图比对 |
| 8 | 九组快捷键可用 | 手测清单 |

### 6.2 引擎（重设 v0.3.0 门槛）

v2 的「受阻源下降 ≥ 70%」定得过于乐观：六项全做完只降 18.8%。
**继续用一个做不到的数字，等于没有门槛。** 重设为分级：

| 级别 | 受阻源 | 相对 224 基线 |
| --- | ---: | ---: |
| 当前 | 182 | −18.8% |
| **v0.8.0 门槛** | **≤ 140** | −37.5% |
| **v1.0.0 门槛** | **≤ 90** | −60% |

同时**必须**同步报告：在线可用率（分层：连接类 / 认证类 / 规则类）。
静态覆盖率衡量引擎，在线可用率还包含 DNS、超时、站点存活、Cloudflare —— 永远分开报。

### 6.3 每次 P 收尾复查

1. Rust 最大非测试行 < 250；Vue SFC < 200 行
2. 每条验收能用一条命令或一个文件路径证明

---

## 7. 版本与门槛

| 版本 | 阶段 | 门槛 | 状态 |
| --- | --- | --- | :---: |
| v0.1.0 | v1 Step 0 | 本地阅读闭环 + 干净架构 | ✅ |
| v0.2.0 | v2 P0 | rule-audit 度量修正 + 按源归因 | ✅ |
| v0.5.0 | v2 P4 | 下载 / 缓存 / 导出 | ✅ |
| v0.6.0 | v2 P5 | 批量校验 + 换源 + 书源管理 | ✅ |
| ~~v0.3.0~~ | — | 受阻源 −70% —— **门槛不可达，作废，见 §6.2** | ⛔ |
| **v0.8.0** | **F0 + F1 + F2** | shadcn-vue 地基 + 真实路由 + 六页对位原型 + 无假数据 + SFC < 200 行 | ⬜ |
| **v0.9.0** | **F3 + E0** | 阅读器七区全建 + 精确分页 + 读完一本在线书 · 受阻源 ≤ 140 + `selector.rs` 兜底已删 | ⬜ |
| v0.9.5 | E1 | 首页仪表盘数据 + 高亮批注 + 书籍详情页 | ⬜ |
| v1.0.0 | S + R | legado 兼容备份 + RSS + 受阻源 ≤ 90 + Windows/Linux 打包 | ⬜ |

TTS / AI / 翻译 / 局域网 Web 不进 v1.0.0 门槛，各自独立立项。

---

## 8. 风险登记

| 风险 | 影响 | 状态 |
| --- | --- | --- |
| **前端从未被度量** | 三版路线图 0 条前端验收；结果是 439 行单体、装饰性路由、三个假页面 | **本版新增** —— §6.1 建立八条前端门槛 |
| **纪律条目打勾但不为真** | v2 §3.9「最大文件 < 250」打 ✅，实测 `stages.rs` 252 | **本版新增** —— 打勾前必须跑一次 §6.3 的命令 |
| **门槛定得不可达就等于没门槛** | v2 的 −70% 六项做完只到 −18.8%，之后停滞 | **本版新增** —— §6.2 改为 −37.5% / −60% 分级 |
| **重写期间前后端同时在动** | 页面重建与 E1 新命令交错，容易互相阻塞 | F2 只用已就绪命令；E1 按 F 轨实际阻塞顺序做，不提前 |
| 兜底路径掩盖失败 | `selector.rs` 355 行三版未删，仍被 `stages.rs` 四处调用 | E0 关闭；令牌迁移同理**不留 CSS 兼容层** |
| 线上可用率 47.0% | 端到端 53% 失败 | 主因 194 条连接类（69.3%），非引擎；须分层报告 |
| `rquickjs` C 工具链跨平台 | Linux / macOS 构建失败 | **v1 登记至今三版未关闭** —— E0 内 spike |
| Rhino JVM 包访问 | 9 源（1%）永久不可用 | 选 QuickJS 必须付的账，已明确标注非静默失败 |
| Cloudflare / JS challenge | 22 源不可用 | WebView 认证已落地，待真实站点验收 |
| 文档漂移 | v2 §3.3 称的 `css_compat.rs` 不存在（实为 `jsoup/legacy.rs`）；体量数字过时一倍 | §0.2 已修正，同步更新 `ARCHITECTURE.md` |
| ~~覆盖率口径不可信~~ · ~~Java 正则差异~~ · ~~JSoup 宽松度~~ · ~~引擎过严~~ · ~~js_runtime 2,080 行~~ | — | **v2 已关闭** |

---

## 9. 明确不做

沿用 v2 §14，并按原型做两处调整：

| 功能 | 理由 |
| --- | --- |
| 漫画阅读 | `bookSourceType=2` 图片源，另一套渲染 |
| 段评 `ReviewRule` | 依赖账号体系 |
| 字体反爬 `queryTTF` / `replaceFont` | 单独立项，工作量大 |
| UMD / MOBI / PDF | 按需求评估 |
| 词典 / 高亮标签规则 | 低频（原型「我的」页有入口，显示未接入态） |
| Rhino JS 引擎移植 | 用 QuickJS 替代，代价已量化：9 源永久不可用 |
| `WebJs` / `ContentRule.webJs` | 970 源实测零出现 |
| `dnsIp` / `serverID` | 桌面端价值低 |
| 仿真翻页动画 / 竖排 | 原型「翻页动画」六选里的「仿真」降级为「滑动」 |
| **主题引擎 Miuix / 动态取色 / 种子色** | 原型设置页有此 UI；shadcn 令牌体系下改为预设主题四选，不做可插拔主题引擎 |
| **多窗口阅读 / 全局快捷键 / AI 人物关系图** | 原型「实验室」pane 三项，默认关闭，不排期 |

---

## 10. 度量与验收命令

```bash
# 三件套
cargo test   --manifest-path src-tauri/Cargo.toml --target-dir src-tauri/.cargo-target
cargo clippy --manifest-path src-tauri/Cargo.toml --target-dir src-tauri/.cargo-target \
             --all-targets -- -D warnings
npm run lint && npm run format:check && npm run build

# 静态覆盖率（无网络，进 CI）—— 只看「受阻源数」
READER_STRICT_ENGINE=1 cargo run --bin rule-audit -- \
  --corpus src-tauri/tests/corpus/ --out docs/coverage/

# 在线可用率（需网络，不进 CI）—— 必须分层报告
READER_STRICT_ENGINE=1 cargo run --bin source-audit -- \
  --corpus src-tauri/tests/corpus/f3f55c6e-723b-4055-b254-124c9d88c5cb.json \
  --keyword 剑来 --concurrency 8 --out audit.csv

# 后端文件行数纪律（非测试行 = 首个 #[cfg(test)] 之前）
cd src-tauri/src && for f in $(find . -name "*.rs"); do
  t=$(grep -n "#\[cfg(test)\]" "$f" | head -1 | cut -d: -f1)
  [ -z "$t" ] && n=$(wc -l < "$f") || n=$((t-1)); echo "$n $f"
done | sort -rn | head -15

# 前端门槛
find src -name '*.vue' -exec wc -l {} + | sort -rn | head -5     # ≤ 200
grep -c 'component: AppShell' src/router/index.ts                 # == 0
grep -rn 'props.books.slice' src/features                         # 无结果
grep -cE '#[0-9a-fA-F]{3,8}\b' src/styles.css                     # 硬编码颜色 → 0

# 体量
find src-tauri/src -name "*.rs" | wc -l
find src-tauri/src -name "*.rs" -exec cat {} + | wc -l
find src \( -name "*.vue" -o -name "*.ts" \) -exec cat {} + | wc -l
```

---

## 11. 下一步（可立即开工）

| # | 任务 | 阻塞关系 | 节 |
| ---: | --- | --- | --- |
| 1 | 装 shadcn-vue + `@/` alias + `components.json` | 阻塞全部前端 | F0 |
| 2 | 令牌迁移（`desktop-ui.html:1045–1121` → `@theme inline`） | 阻塞全部前端 | F0 / §3.2 |
| 3 | 拆 `AppShell.vue`（439 行）+ 路由真实化 | 阻塞全部页面重建 | F0 |
| 4 | 删 `LibraryTabPage` 假数据三页；处置 `SourceManager` 死代码 | 独立，可并行 | F0 / §1.3 |
| 5 | 修文档漂移：`css_compat.rs` 路径 · 体量数字 · shadcn 依赖声明 | 独立，可并行 | §0.2 |
| 6 | 拆 `pipeline/stages.rs`（252 行，唯一越线） | 独立，可并行 | E0 |
| 7 | 删 `selector.rs` 兜底 + 扁平列迁移 | 独立，E0 关键项 | E0 |

**第 1–3 项必须先做。** 在此之前每写一个页面，都是在往那个 439 行的单体上继续堆。
这正是 v2 在 `js_runtime.rs` 上犯过的错 —— 纪律写进了路线图，文件却从 847 行涨到 2,080 行。

### 需要同步修改的文档

`ARCHITECTURE.md` 当前写着「使用 Tailwind 工具类 + 共享设计令牌，**不依赖 shadcn-vue**」——
v2 §10 刚把这句从「不用 Tailwind/shadcn」改成现在这样。
F0 完成后需第三次修改为「Tailwind v4 + shadcn-vue（Reka UI）+ CSS 变量令牌；阅读器正文层例外」。

# 失败归类工作清单（2026-09-10）

数据：`docs/coverage/rule-audit.md`（970 源 / 22,220 规则串 / 无错误 805 = 83.0% / **受阻 165**）
方法：把该报告的「Execution errors」与「Failed rule examples」两张表与语料
`src-tauri/tests/corpus/f3f55c6e-723b-4055-b254-124c9d88c5cb.json` 联表，按源 id 归并。
分类口径与 `src-tauri/src/bin/rule_audit/input.rs::error_category` 完全一致（脚本复算出的
160 / 125 / 24 / 18 / 13 与报告逐字吻合，证明口径对齐）。

「独占源」= 只被该类挡住的源（等价于报告里的 `Unblocked by fixing only this category`）。

---

## 1. js runtime：160 条规则 / 106 源受阻 / 64 独占源

| 子簇 | 规则 | 覆盖源（上界） | 独占源（贪心） | 性质 |
| --- | ---: | ---: | ---: | --- |
| `cannot read property ... of undefined/null` | 56 | 52 | **35** | **基本是假输入产物**：dummy 缺 `attributes` / `info` / `volumes` / `chapter` / `chapterNameList` / `user` 等容器，规则一读就抛 |
| `not a function` | 41 | 41 | 12 | 混合：`java.HMacHex`（3 源）、`java.getStringList(...).toArray()/.size()/.get()` 链、`Jsoup` 相关链式调用 |
| JS 语法（`Unexpected token` / `unexpected token in expression`） | 35 | 35 | 7 | `@js:` 里的 `{{$.x}}` 未先求值（`if({{$.isFinished}}==1)`）；折行声明等 |
| `Error converting from js 'Rule' into type 'String'` | 17 | 14 | 6 | `java.getString('$..x')` 的 JSONPath 在某些输入下不可执行 |
| `cannot read property 'Jsoup' of undefined` | 9 | 9 | 4 | **真实缺口**：`org.jsoup.Jsoup.parse(html).select(...)` 未暴露绑定 |
| `Error calling function with 0 argument(s) while 1 where expected` | 2 | 2 | 1 | **真实缺口**：绑定签名与 legado 不一致 |

**结论（修正 v3 的判断）**：v3 §E0 写「E0 的真正主战场是 JS 运行时」。更准确的说法是
**js runtime 桶里最大的一块不是引擎缺口**，而是确定性假载荷缺字段造成的假失败
（35/64 独占源）。真正的引擎缺口是绑定层（`org.jsoup.Jsoup`、`java.HMacHex`、
`getStringList` 返回值的 Java 容器语义、`java.getString` 的 JSONPath）。

> 注意：往 dummy 里继续加字段可以把这个数字刷下来，但那是 v2「harness 喂假输入」的翻版
> ——规则要的键是逐源特异的（`info.Datas`、`volumes`、`chapterNameList`…），加不完。
> 因此**不建议**用扩充假载荷的方式降这个数；要么修绑定，要么明确把这类失败标成假输入产物。

---

## 2. css compatibility：125 条规则 / 77 源受阻 / 41 独占源

41 个独占源一共 61 条失败规则，按修因归类：

| 修因 | 规则 | 源 | 例子 |
| --- | ---: | ---: | --- |
| 显示模板被当 CSS 选择器 | 11 | 7 | `{$.score}分` · `第{$.chapterNum}章` · `{baseUrl}` · `<br>{$.introduction}` |
| legado 私有选择器方言残留 | 9 | 4 | `@css:.listmain a[href~=/[^/]+/\d+.htm]`（JSoup 的 `~=` 正则属性选择器） |
| 前导 `+` 未剥离 | 5 | 4 | `+@css:.bookbox` / `+@js:org.jsoup...` |
| `*Js` 字段被当 CSS | 3 | 3 | `ruleToc.preUpdateJs = book.canUpdate=false` · `java.refreshTocUrl()` |
| 纯正则捕获组引用 `$N` | 4 | 2 | `ruleToc.chapterUrl = $1`（依赖同名字段的正则捕获组，见 §3） |
| 非规则元数据被当规则 | 3 | 3 | `checkKeyWord`（关键词表）· `imageStyle = 0.0`（数值） |
| CSS 方言宽松度 | 4 | 2 | `div[class =d_post_content j_d_post_content]`（Jsoup 宽松 / `scraper` 严格） |
| 其余私有方言 | 22 | ~16 | `class. Gap_size-…@tag.a@href` · `span.0:-text` · `td.-1:0-2@text` |

**这一桶才是当前最可打的一桶**：修因清晰、单点小、绝大多数是真实语义缺口。

---

## 3. 跨类发现：正则捕获组引用 `$N`（63 条规则 / 42 源）

规则的**值本身就是** `$1`、`$2`、`$5,$7,$10`，共 63 条、涉及 42 源。它不是选择器，而是
**同名字段列表规则所用正则的捕获组引用**。典型（`快眼小说`）：

```
ruleToc.chapterList = ":正文卷[\s\S]*?/dl&&href =\"([^\"]+)\">([^<]+)"
ruleToc.chapterUrl  = "$1"
ruleToc.chapterName = "$2##[\(（【].*?[求更谢乐发订合补加].*?[】）\)]"
```

引擎（`source_engine/pipeline/stages/search.rs` 逐字段对 item 求值）不实现这套语义；
审计又是逐条规则干跑，`$1` 单独必然报 "not a CSS selector"。

**两侧都要动，缺一不可**：
- 引擎：列表规则为正则时，把捕获组带进 item 上下文，`$N` / `$N##替换` 从中取值；
- 审计：把「纯引用型规则」判为上下文相关，不计入独立失败 —— 这**不是**静默兜底，
  必须由引擎单测（小 fixture 端到端）证明该能力存在，并在报告里单独计数。

其中只有 **2 个源**是「只被这一类挡住」，另外 40 个源同时还被 js runtime 挡住。

---

## 4. 复现

```bash
# 重新生成报告（165 受阻源）
READER_STRICT_ENGINE=1 cargo run --bin rule-audit -- \
  --corpus src-tauri/tests/corpus/ --out docs/coverage/

# 注意：本机沙箱下 cargo / npm run build 需要更宽权限（见 ROADMAP-v3 §8 风险表），
# 否则 cargo 打不开 src-tauri/.cargo-target/debug/.cargo-lock。
```

联表脚本是一次性的（`.tmp-extract.cjs`，已删）。若需要长期复现，应把它变成
`rule-audit` 的第二个子命令（`--worklist`），输出本表的机器可读版本 —— 这本身是一条待办。

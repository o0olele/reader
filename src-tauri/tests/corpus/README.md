# 书源语料

本目录存放用于度量规则引擎覆盖率的**真实 legado 书源语料**。
内容不入 git（见根 `.gitignore`）—— 体积大、会过期，且并非本项目产物。

## 当前语料

| 文件 | 来源 | 规模 | 加入日期 |
| --- | --- | --- | --- |
| `f3f55c6e-723b-4055-b254-124c9d88c5cb.json` | 人工提供 | 970 源 / 22,220 条规则串 | 2026-09-02 |

## 用途

`ROADMAP-v2.md` §0.3 的全部静态覆盖率数据出自这里。当前可运行的审计器是
`src-tauri/src/bin/rule-audit.rs`（§3.2）；端到端在线审计器 `source-audit.rs`（§3.3）
尚未落地，路线图中的在线命令只是目标接口：

```bash
READER_STRICT_ENGINE=1 cargo run --manifest-path src-tauri/Cargo.toml \
  --bin rule-audit -- --corpus src-tauri/tests/corpus/<file>.json --out docs/coverage/

# 历史 Node 原型（仅用于复核旧口径）
node .scratch/corpus-audit.cjs src-tauri/tests/corpus/<file>.json   # token / java.* / 字段频次
node .scratch/coverage.cjs     src-tauri/tests/corpus/<file>.json   # 覆盖率基线 + 边际收益
```

## 补充语料时

1. 放进本目录，在上表登记来源与规模
2. 重跑上面两个脚本，把新的覆盖率数字写进 `docs/coverage/YYYY-MM-DD.md`
3. **不要**把语料里的书源当作夹具直接抄进 `tests/fixtures/` —— 夹具需要配套的 HTML 响应，
   由调试器的「导出为 fixture」功能产出（`ROADMAP-v2.md` §5）

## 另有两份可直接取用的真实资产

位于参考项目 `legado-with-MD3/app/src/main/assets/defaultData/`：

- `bookSources.json` —— 1 个含全部 21 个顶层字段的真实书源，适合做字段级导入回归
- `txtTocRule.json` —— 27 条实测 TXT 目录正则，待移植替换 `split_chapters` 手写启发式
- `coverRule.json` —— data URL + `{{模板}}` + 选项对象 + `@js:` 四合一，AnalyzeUrl 的天然压测样本

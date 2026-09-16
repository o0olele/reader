# 书源审计语料

真实 Legado 书源语料不入 git（见根 .gitignore）。2026-09-16 用户已恢复原始 JSON 到本目录。

历史报告 docs/coverage/rule-audit.md 使用人工提供的 f3f55c6e-723b-4055-b254-124c9d88c5cb.json：970 源、22,220 规则串，登记于 2026-09-02。
2026-09-16 全量重测：970 源 / 22,220 规则串 / 131 受阻 / 839 无执行错误（86.5%），与历史报告无差异。
恢复文件 SHA256：`7854ED6202E75659CE40F297E364B53C8D0409ADFDE319D2477BE63CB16A778B`。
这是本次输入指纹；历史未登记哈希，不能据此证明历史文件逐字节相同。
执行记录见 `docs/e0-rule-authority-2026-09-16.md`，结果见 `docs/coverage/rule-audit.md`。

同日 Jsoup 绑定修复后再次重测：受阻 **131 → 127**，843 无执行错误（86.9%）；
分母与输入哈希不变，正式报告已更新，细节及 hook 统计边界见 `docs/e0-jsoup-2026-09-16.md`。

## 重测命令（仓库根目录，PowerShell）

```powershell
$env:READER_STRICT_ENGINE = '1'
cargo run --locked --manifest-path src-tauri/Cargo.toml --bin rule-audit -- --corpus src-tauri/tests/corpus/f3f55c6e-723b-4055-b254-124c9d88c5cb.json --out docs/coverage/
cargo run --locked --manifest-path src-tauri/Cargo.toml --bin source-audit -- --corpus src-tauri/tests/corpus/f3f55c6e-723b-4055-b254-124c9d88c5cb.json --keyword 剑来 --concurrency 8 --out audit.csv
```

rule-audit 与 source-audit 均已有实现。前者使用 dummy 输入，不访问网站；空匹配不证明提取正确。后者需要网络，应分层报告连接、认证、规则失败。
使用新语料时应登记来源、日期、源数和规则数；不要将不同分母的结果直接当作同一基线的改善。

## CI 与回归夹具

CI 运行 cargo test，其中包含审计器单测和 tests/fixtures 下已提交的配套 JSON/HTML 回归夹具。
不将 fixtures 或参考项目的单源 bookSources.json 冒充 970 源语料，不报告全量覆盖率通过。
从真实网站添加 fixture 时，用调试器导出配套响应，并检查敏感会话信息。

参考项目 app/src/main/assets/defaultData 下的 bookSources.json、txtTocRule.json、coverRule.json 可用于各自的字段回归；当前 TXT 解析器已使用默认目录规则，管理 CRUD 仍未实现。

import assert from 'node:assert/strict'
import { test } from 'node:test'
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { checkStructure, lineCount, productionLines, vueLines } from './check-structure.mjs'

test('line counts agree for LF and CRLF without counting a final newline twice', () => {
  assert.equal(lineCount('a\nb\n'), 2)
  assert.equal(lineCount('a\r\nb\r\n'), 2)
  assert.equal(lineCount('a\nb'), 2)
  assert.equal(lineCount(''), 0)
})

test('Rust inline tests and dedicated test modules are excluded', () => {
  assert.equal(productionLines('source.rs', 'fn production() {}\n\n#[cfg(test)]\nmod tests {\n}\n'), 2)
  assert.equal(productionLines('bookmark_tests.rs', 'test code'), 0)
  assert.equal(productionLines('tests.rs', 'test code'), 0)
})

test('an external test module does not hide later production code', () => {
  const text = '#[cfg(test)]\n#[path = "rule_audit/tests.rs"]\nmod tests;\nfn main() {}\n'
  assert.equal(productionLines('rule-audit.rs', text), 4)
})

test('a cfg(test) helper does not truncate subsequent production code', () => {
  const text = '#[cfg(test)]\nfn helper() {}\nfn production() {}\n'
  assert.equal(productionLines('request.rs', text), 3)
})

test('Vue style blocks are excluded while script and template remain counted', () => {
  const text = '<script setup>\n</script>\n<template>hello</template>\n<style scoped>\na {}\n</style>\n'
  assert.equal(vueLines(text), 3)
})

test('repository checks reject threshold violations and style/route regressions', () => {
  const root = mkdtempSync(join(tmpdir(), 'reader-structure-'))
  try {
    mkdirSync(join(root, 'src-tauri/src'), { recursive: true })
    mkdirSync(join(root, 'src/router'), { recursive: true })
    const write = (path, text) => writeFileSync(join(root, path), text)
    write('src-tauri/src/lib.rs', '// production\n'.repeat(249))
    write('src/Page.vue', '<!-- line -->\n'.repeat(200))
    write('src/styles.css', ':root { color: var(--foreground); }')
    write('src/router/index.ts', 'component: AppLayout')
    assert.deepEqual(checkStructure(root).failures, [])
    write('src-tauri/src/lib.rs', '// production\n'.repeat(250))
    write('src/Page.vue', '<!-- line -->\n'.repeat(201))
    write('src/styles.css', ':root { color: #ffffff; }')
    write('src/router/index.ts', 'component: AppShell')
    const failures = checkStructure(root).failures
    assert.equal(failures.length, 4)
    assert.ok(failures.some((failure) => failure.includes('250 production lines')))
    assert.ok(failures.some((failure) => failure.includes('201 SFC lines')))
    assert.ok(failures.some((failure) => failure.includes('hex color')))
    assert.ok(failures.some((failure) => failure.includes('AppShell')))
  } finally {
    rmSync(root, { recursive: true, force: true })
  }
})

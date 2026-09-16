import { readdirSync, readFileSync } from 'node:fs'
import { basename, join, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import process from 'node:process'

export function lineCount(text) {
  const normalized = text.replace(/\r\n/g, '\n')
  return normalized ? normalized.replace(/\n$/, '').split('\n').length : 0
}

export function productionLines(path, text) {
  // Dedicated test modules follow the repository's tests.rs / *_tests.rs convention.
  if (/^(?:tests|.*_tests)\.rs$/.test(basename(path))) return 0
  // Only a trailing inline test module ends production code. External cfg(test)
  // module declarations (notably rule-audit.rs) must not hide the rest of a file.
  const marker = /^#\[cfg\(test\)\]\r?\nmod tests\s*\{/m.exec(text)
  return lineCount(marker ? text.slice(0, marker.index) : text)
}

export function vueLines(text) {
  return lineCount(text.replace(/<style\b[^>]*>[\s\S]*?<\/style>\s*/g, ''))
}

function files(root) {
  return readdirSync(root, { withFileTypes: true }).flatMap((entry) => {
    const path = join(root, entry.name)
    return entry.isDirectory() ? files(path) : [path]
  })
}

export function checkStructure(root) {
  const failures = []
  let rustMax = 0
  let vueMax = 0
  for (const path of files(join(root, 'src-tauri/src')).filter((path) => path.endsWith('.rs'))) {
    const lines = productionLines(path, readFileSync(path, 'utf8'))
    rustMax = Math.max(rustMax, lines)
    if (lines >= 250) failures.push(`${path}: ${lines} production lines (must be <250)`)
  }
  for (const path of files(join(root, 'src')).filter((path) => path.endsWith('.vue'))) {
    const lines = vueLines(readFileSync(path, 'utf8'))
    vueMax = Math.max(vueMax, lines)
    if (lines > 200) failures.push(`${path}: ${lines} SFC lines excluding style (must be <=200)`)
  }
  const styles = readFileSync(join(root, 'src/styles.css'), 'utf8')
  if (/#[\da-f]{3,8}\b/i.test(styles)) failures.push('src/styles.css contains a hard-coded hex color')
  const routes = readFileSync(join(root, 'src/router/index.ts'), 'utf8')
  if (/component:\s*AppShell\b/.test(routes)) failures.push('Routes must not point to AppShell')
  return { failures, rustMax, vueMax }
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const root = fileURLToPath(new URL('../', import.meta.url))
  const { failures, rustMax, vueMax } = checkStructure(root)
  process.stdout.write(`Maximum Rust production lines: ${rustMax}; Vue SFC lines: ${vueMax}\n`)
  for (const failure of failures) process.stderr.write(`${failure}\n`)
  process.exitCode = failures.length ? 1 : 0
}

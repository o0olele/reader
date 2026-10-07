import type { BookSource, SourceTestResult } from '@/services/api'

export type SourceFilter = 'all' | 'enabled' | 'disabled' | 'attention'
export type SourceSort = 'default' | 'name' | 'response'

export function probeNeedsAttention(result?: SourceTestResult, error?: string) {
  return (
    !!error ||
    (!!result &&
      (result.status < 200 ||
        result.status >= 400 ||
        result.auth_required ||
        result.cloudflare_challenge ||
        result.result_count === 0))
  )
}

export function probeLabel(result?: SourceTestResult, error?: string) {
  if (error) return '测试失败'
  if (!result) return '未测试'
  if (result.cloudflare_challenge) return '需要浏览器验证'
  if (result.auth_required) return '需要登录'
  if (!result.status) return '请求失败'
  if (result.status < 200 || result.status >= 400) return `HTTP ${result.status}`
  if (!result.result_count) return '未找到结果'
  return `找到 ${result.result_count} 条结果`
}

export function sessionLabel(source: BookSource) {
  if (!source.access_token && !source.session_cookie) return '无登录会话'
  if (source.session_expires_at) {
    const raw = source.session_expires_at
    const expiry = /^\d+$/.test(raw) ? Number(raw) * 1000 : Date.parse(raw)
    if (Number.isFinite(expiry) && expiry <= Date.now()) return '登录已过期'
  }
  return '已登录'
}

export function arrangeSources(
  sources: BookSource[],
  results: Record<number, SourceTestResult>,
  options: { keyword: string; filter: SourceFilter; group: string; sort: SourceSort },
  errors: Record<number, string> = {},
) {
  const needle = options.keyword.trim().toLocaleLowerCase()
  const filtered = sources.filter((source) => {
    if (
      needle &&
      !`${source.name} ${source.base_url} ${source.source_group ?? ''}`.toLocaleLowerCase().includes(needle)
    )
      return false
    if (options.filter === 'enabled' && !source.enabled) return false
    if (options.filter === 'disabled' && source.enabled) return false
    if (options.filter === 'attention' && !probeNeedsAttention(results[source.id], errors[source.id])) return false
    if (options.group === 'ungrouped' && source.source_group?.trim()) return false
    if (options.group.startsWith('group:') && source.source_group?.trim() !== options.group.slice(6)) return false
    return true
  })
  return filtered.sort((a, b) => {
    if (options.sort === 'response') {
      const difference =
        (results[a.id]?.duration_ms ?? a.respond_time ?? Infinity) -
        (results[b.id]?.duration_ms ?? b.respond_time ?? Infinity)
      if (difference) return difference
    }
    if (options.sort === 'default') {
      const difference = a.custom_order - b.custom_order || b.weight - a.weight
      if (difference) return difference
    }
    return a.name.localeCompare(b.name, 'zh-CN') || a.id - b.id
  })
}

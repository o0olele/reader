import type { ReadingHistoryEntry } from '../services/api'

/**
 * Display helpers for reading records, shared by the 历史 page and the home
 * dashboard's 最近在读 card so both read one row the same way.
 */

/** SQLite writes `updated_at` in UTC without a zone marker. */
export function parseStamp(value: string): Date | undefined {
  const date = new Date(`${value.replace(' ', 'T')}Z`)
  return Number.isNaN(date.getTime()) ? undefined : date
}

export type HistoryBucket = '今天' | '昨天' | '更早'

/** Local midnight, so a bucket follows the calendar day the reader saw. */
function dayStart(date: Date): number {
  return new Date(date.getFullYear(), date.getMonth(), date.getDate()).getTime()
}

/**
 * Which day section a row belongs to. Days are compared from local midnight
 * rather than by "24 hours ago", so yesterday at 23:00 is 昨天, not 今天.
 */
export function historyBucket(value: string, now = new Date()): HistoryBucket {
  const date = parseStamp(value)
  if (!date) return '更早'
  const days = Math.round((dayStart(now) - dayStart(date)) / 86_400_000)
  if (days <= 0) return '今天'
  return days === 1 ? '昨天' : '更早'
}

/** `20:31` for today and yesterday; `9月14日 08:00` further back. */
export function readAt(value: string, now = new Date()): string {
  const date = parseStamp(value)
  if (!date) return value
  const time = date.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' })
  const day = date.toLocaleDateString('zh-CN', { month: 'numeric', day: 'numeric' })
  return historyBucket(value, now) === '更早' ? `${day} ${time}` : time
}

/**
 * Progress by chapter position, or `null` when the row has no position or the
 * catalog is unknown — the page then shows no percentage instead of inventing
 * one from a book that has never been downloaded.
 */
export function readPercent(entry: ReadingHistoryEntry): number | null {
  if (entry.chapter_id === null || entry.chapter_count <= 0) return null
  const ratio = ((entry.chapter_number ?? 0) + 1) / entry.chapter_count
  return Math.min(100, Math.max(0, Math.round(ratio * 100)))
}

/**
 * Under a minute reads as 不足 1 分钟: the timer flushes whole seconds, so a
 * short session would otherwise claim a flat "0 分钟".
 */
export function formatDuration(seconds: number): string {
  if (seconds < 60) return '不足 1 分钟'
  const minutes = Math.round(seconds / 60)
  if (minutes < 60) return `${minutes} 分钟`
  const hours = Math.floor(minutes / 60)
  const rest = minutes % 60
  return rest ? `${hours} 小时 ${rest} 分钟` : `${hours} 小时`
}

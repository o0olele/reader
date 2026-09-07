import { invoke } from '@tauri-apps/api/core'
import type { CacheStats, DownloadTask, ExportResult } from './types'

export const listDownloadTasks = () => invoke<DownloadTask[]>('list_download_tasks')
export const startDownload = (bookId: number, chapterIds?: number[]) =>
  invoke<number>('start_download', { bookId, chapterIds })
export const pauseDownload = (taskId: number) => invoke<void>('pause_download', { taskId })
export const resumeDownload = (taskId: number) => invoke<void>('resume_download', { taskId })
export const cancelDownload = (taskId: number) => invoke<void>('cancel_download', { taskId })
export const exportBook = (bookId: number, format: 'txt' | 'epub', targetPath: string) =>
  invoke<ExportResult>('export_book', { bookId, format, targetPath })
export const getCacheStats = () => invoke<CacheStats>('get_cache_stats')
export const setCacheQuota = (megabytes: number) => invoke<CacheStats>('set_cache_quota', { megabytes })
export const clearChapterCache = () => invoke<CacheStats>('clear_chapter_cache')

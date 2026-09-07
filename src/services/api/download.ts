import { invoke } from '@tauri-apps/api/core'
import type { DownloadTask } from './types'

export const listDownloadTasks = () => invoke<DownloadTask[]>('list_download_tasks')
export const startDownload = (bookId: number, chapterIds?: number[]) =>
  invoke<number>('start_download', { bookId, chapterIds })
export const pauseDownload = (taskId: number) => invoke<void>('pause_download', { taskId })
export const resumeDownload = (taskId: number) => invoke<void>('resume_download', { taskId })
export const cancelDownload = (taskId: number) => invoke<void>('cancel_download', { taskId })

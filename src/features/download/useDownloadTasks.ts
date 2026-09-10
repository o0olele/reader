import { onBeforeUnmount, onMounted, ref } from 'vue'
import {
  cancelDownload,
  clearChapterCache,
  getCacheStats,
  getErrorMessage,
  listDownloadTasks,
  pauseDownload,
  resumeDownload,
  setCacheQuota,
  startDownload,
  type CacheStats,
  type DownloadTask,
} from '@/services/api'
import { notifyError, notifySuccess } from '@/app/useToast'

/** Polls the persisted download queue and the chapter cache quota. */
export function useDownloadTasks() {
  const tasks = ref<DownloadTask[]>([])
  const cache = ref<CacheStats>()
  const quotaMb = ref(1024)
  const busy = ref<number>()
  const cacheBusy = ref(false)
  let timer: ReturnType<typeof setInterval> | undefined
  let ticks = 0

  async function refreshTasks() {
    try {
      tasks.value = await listDownloadTasks()
    } catch (cause) {
      notifyError(getErrorMessage(cause))
    }
  }

  async function refreshCache() {
    try {
      const next = await getCacheStats()
      if (!cache.value) quotaMb.value = Math.round(next.quota_bytes / 1024 / 1024)
      cache.value = next
    } catch (cause) {
      notifyError(getErrorMessage(cause))
    }
  }

  async function refresh() {
    await Promise.all([refreshTasks(), refreshCache()])
  }

  async function updateCache(action: () => Promise<CacheStats>) {
    cacheBusy.value = true
    try {
      cache.value = await action()
      quotaMb.value = Math.round(cache.value.quota_bytes / 1024 / 1024)
    } catch (cause) {
      notifyError(getErrorMessage(cause))
    } finally {
      cacheBusy.value = false
    }
  }

  async function act(taskId: number, action: () => Promise<unknown>) {
    busy.value = taskId
    try {
      await action()
      await refresh()
    } catch (cause) {
      notifyError(getErrorMessage(cause))
    } finally {
      busy.value = undefined
    }
  }

  async function start(bookId: number) {
    try {
      await startDownload(bookId)
      notifySuccess('下载任务已创建')
      await refresh()
    } catch (cause) {
      notifyError(getErrorMessage(cause))
    }
  }

  onMounted(() => {
    void refresh()
    timer = setInterval(() => {
      void refreshTasks()
      ticks += 1
      if (ticks % 5 === 0) void refreshCache()
    }, 1000)
  })
  onBeforeUnmount(() => {
    if (timer) clearInterval(timer)
  })

  return {
    tasks,
    cache,
    quotaMb,
    busy,
    cacheBusy,
    refresh,
    start,
    act,
    setQuota: () => updateCache(() => setCacheQuota(quotaMb.value)),
    clearCache: () => updateCache(clearChapterCache),
    pause: (id: number) => act(id, () => pauseDownload(id)),
    resume: (id: number) => act(id, () => resumeDownload(id)),
    cancel: (id: number) => act(id, () => cancelDownload(id)),
  }
}

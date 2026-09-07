<script setup lang="ts">
/* global setInterval, clearInterval */
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import { save } from '@tauri-apps/plugin-dialog'
import { BookOpen, Database, Download, FileText, Pause, Play, RotateCcw, Trash2, X } from 'lucide-vue-next'
import {
  cancelDownload,
  clearChapterCache,
  exportBook,
  getCacheStats,
  getErrorMessage,
  listDownloadTasks,
  pauseDownload,
  resumeDownload,
  setCacheQuota,
  startDownload,
  type Book,
  type CacheStats,
  type DownloadTask,
} from '../../services/api'

const props = defineProps<{ books: Book[] }>()
const tasks = ref<DownloadTask[]>([])
const error = ref('')
const busy = ref<number>()
const cache = ref<CacheStats>()
const quotaMb = ref(1024)
const cacheBusy = ref(false)
const onlineBooks = computed(() => props.books.filter((book) => book.source_id))
let timer: ReturnType<typeof setInterval> | undefined
let refreshTicks = 0

async function refreshTasks() {
  try {
    tasks.value = await listDownloadTasks()
  } catch (cause) {
    error.value = getErrorMessage(cause)
  }
}
async function refreshCache() {
  try {
    const nextCache = await getCacheStats()
    if (!cache.value) quotaMb.value = Math.round(nextCache.quota_bytes / 1024 / 1024)
    cache.value = nextCache
  } catch (cause) {
    error.value = getErrorMessage(cause)
  }
}
async function refresh() {
  await Promise.all([refreshTasks(), refreshCache()])
}
async function updateCache(action: () => Promise<CacheStats>) {
  cacheBusy.value = true
  error.value = ''
  try {
    cache.value = await action()
    quotaMb.value = Math.round(cache.value.quota_bytes / 1024 / 1024)
  } catch (cause) {
    error.value = getErrorMessage(cause)
  } finally {
    cacheBusy.value = false
  }
}
function formatBytes(bytes: number) {
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / 1024 / 1024).toFixed(1)} MB`
}
async function act(taskId: number, action: () => Promise<unknown>) {
  busy.value = taskId
  error.value = ''
  try {
    await action()
    await refresh()
  } catch (cause) {
    error.value = getErrorMessage(cause)
  } finally {
    busy.value = undefined
  }
}
async function start(bookId: number) {
  busy.value = -bookId
  error.value = ''
  try {
    await startDownload(bookId)
    await refresh()
  } catch (cause) {
    error.value = getErrorMessage(cause)
  } finally {
    busy.value = undefined
  }
}
function safeFilename(value: string) {
  return (
    value
      .replace(/[<>:"/\\|?*]/g, '_')
      .replace(/\s+/g, ' ')
      .trim() || '未命名书籍'
  )
}
async function exportTask(task: DownloadTask, format: 'txt' | 'epub') {
  const target = await save({
    defaultPath: `${safeFilename(task.book_title)}.${format}`,
    filters: [{ name: format.toUpperCase(), extensions: [format] }],
  })
  if (!target) return
  await act(task.id, () => exportBook(task.book_id, format, target))
}
function selectBook(event: Event) {
  const select = event.target as HTMLSelectElement
  const bookId = Number(select.value)
  select.value = ''
  if (bookId) void start(bookId)
}
const percent = (task: DownloadTask) =>
  task.total_chapters ? Math.round((task.completed_chapters * 100) / task.total_chapters) : 0
const statusText = (status: DownloadTask['status']) =>
  ({
    pending: '等待中',
    running: '下载中',
    paused: '已暂停',
    completed: '已完成',
    failed: '失败',
    cancelled: '已取消',
  })[status]

onMounted(() => {
  void refresh()
  timer = setInterval(() => {
    void refreshTasks()
    refreshTicks += 1
    if (refreshTicks % 5 === 0) void refreshCache()
  }, 1000)
})
onBeforeUnmount(() => {
  if (timer) clearInterval(timer)
})
</script>

<template>
  <section class="download-page">
    <div class="download-start-card">
      <div>
        <p>离线缓存</p>
        <h2>下载整本书</h2>
        <span>目录和正文会持久化，重启后任务自动续跑。</span>
      </div>
      <select :disabled="!onlineBooks.length" @change="selectBook">
        <option value="">选择在线书籍…</option>
        <option v-for="book in onlineBooks" :key="book.id" :value="book.id">{{ book.title }}</option>
      </select>
    </div>
    <div class="cache-card">
      <div class="download-task-icon"><Database :size="20" /></div>
      <div class="cache-summary">
        <strong>正文缓存</strong>
        <span v-if="cache">{{ formatBytes(cache.used_bytes) }} · {{ cache.cached_chapters }} 章</span>
      </div>
      <label>上限 <input v-model.number="quotaMb" type="number" min="64" max="102400" /> MB</label>
      <button :disabled="cacheBusy" @click="updateCache(() => setCacheQuota(quotaMb))">保存</button>
      <button class="danger" :disabled="cacheBusy || !cache?.cached_chapters" @click="updateCache(clearChapterCache)">
        <Trash2 :size="15" /> 清空
      </button>
    </div>
    <p v-if="error" class="download-error" role="alert">{{ error }}</p>
    <div v-if="tasks.length" class="download-task-list">
      <article v-for="task in tasks" :key="task.id" class="download-task">
        <div class="download-task-icon"><Download :size="20" /></div>
        <div class="download-task-main">
          <div class="download-task-title">
            <strong>{{ task.book_title }}</strong
            ><span :class="`status-${task.status}`">{{ statusText(task.status) }}</span>
          </div>
          <div class="download-progress"><i :style="{ width: `${percent(task)}%` }"></i></div>
          <small>{{ task.completed_chapters }} / {{ task.total_chapters }} 章 · {{ percent(task) }}%</small>
          <em v-if="task.error">{{ task.error }}</em>
        </div>
        <div class="download-actions">
          <button
            v-if="task.status === 'completed'"
            :disabled="busy === task.id"
            title="导出 TXT"
            @click="exportTask(task, 'txt')"
          >
            <FileText :size="16" />
          </button>
          <button
            v-if="task.status === 'completed'"
            :disabled="busy === task.id"
            title="导出 EPUB"
            @click="exportTask(task, 'epub')"
          >
            <BookOpen :size="16" />
          </button>
          <button
            v-if="task.status === 'running'"
            :disabled="busy === task.id"
            title="暂停"
            @click="act(task.id, () => pauseDownload(task.id))"
          >
            <Pause :size="16" />
          </button>
          <button
            v-if="task.status === 'paused'"
            :disabled="busy === task.id"
            title="继续"
            @click="act(task.id, () => resumeDownload(task.id))"
          >
            <Play :size="16" />
          </button>
          <button
            v-if="task.status === 'failed'"
            :disabled="busy === task.id"
            title="重试"
            @click="act(task.id, () => resumeDownload(task.id))"
          >
            <RotateCcw :size="16" />
          </button>
          <button
            v-if="task.status === 'pending' || task.status === 'running' || task.status === 'paused'"
            :disabled="busy === task.id"
            title="取消"
            @click="act(task.id, () => cancelDownload(task.id))"
          >
            <X :size="16" />
          </button>
        </div>
      </article>
    </div>
    <div v-else class="library-empty">
      <Download :size="40" stroke-width="1.5" />
      <h2>暂无下载任务</h2>
      <p>先把一本在线书加入书架，再从上方开始下载。</p>
    </div>
  </section>
</template>

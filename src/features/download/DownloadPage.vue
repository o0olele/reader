<script setup lang="ts">
import { computed } from 'vue'
import { save } from '@tauri-apps/plugin-dialog'
import { Database, Download, Trash2 } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import DownloadTaskRow from './DownloadTaskRow.vue'
import PageBody from '@/components/PageBody.vue'
import PageHeader from '@/components/PageHeader.vue'
import { useDownloadTasks } from './useDownloadTasks'
import { exportBook, getErrorMessage, type DownloadTask } from '@/services/api'
import { notifyError, notifySuccess } from '@/app/useToast'
import { useShellContext } from '@/app/shellKeys'

const { bookshelf } = useShellContext()
const { tasks, cache, quotaMb, busy, cacheBusy, start, pause, resume, cancel, setQuota, clearCache } =
  useDownloadTasks()

const onlineBooks = computed(() => bookshelf.books.filter((book) => book.source_id))

function formatBytes(bytes: number) {
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / 1024 / 1024).toFixed(1)} MB`
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
  try {
    const target = await save({
      defaultPath: `${safeFilename(task.book_title)}.${format}`,
      filters: [{ name: format.toUpperCase(), extensions: [format] }],
    })
    if (!target) return
    const result = await exportBook(task.book_id, format, target)
    notifySuccess(`已导出 ${result.bytes_written} 字节`)
  } catch (cause) {
    notifyError(getErrorMessage(cause))
  }
}
</script>

<template>
  <div class="flex h-full flex-col">
    <PageHeader title="下载与缓存" :subtitle="`${tasks.length} 个任务 · 目录与正文持久化，重启后自动续跑`">
      <Select :model-value="''" @update:model-value="start(Number($event))">
        <SelectTrigger class="h-8 w-52" :disabled="!onlineBooks.length">
          <SelectValue placeholder="选择在线书籍…" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem v-for="book in onlineBooks" :key="book.id" :value="String(book.id)">{{ book.title }}</SelectItem>
        </SelectContent>
      </Select>
    </PageHeader>

    <PageBody>
      <div class="mb-4 flex flex-wrap items-center gap-3 rounded-md border bg-card p-3">
        <Database :size="18" class="text-muted-foreground" />
        <div class="min-w-0 flex-1">
          <strong class="text-sm">正文缓存</strong>
          <span v-if="cache" class="ml-2 text-xs text-muted-foreground">
            {{ formatBytes(cache.used_bytes) }} · {{ cache.cached_chapters }} 章 / 上限
            {{ formatBytes(cache.quota_bytes) }}
          </span>
        </div>
        <label class="flex items-center gap-1.5 text-xs text-muted-foreground">
          上限
          <Input
            v-model.number="quotaMb"
            class="h-8 w-24"
            type="number"
            min="64"
            max="102400"
            aria-label="缓存上限 MB"
          />
          MB
        </label>
        <Button variant="outline" size="sm" :disabled="cacheBusy" @click="setQuota">保存配额</Button>
        <Button
          variant="outline"
          size="sm"
          class="text-destructive"
          :disabled="cacheBusy || !cache?.cached_chapters"
          @click="clearCache"
        >
          <Trash2 /> 清空缓存
        </Button>
      </div>

      <div v-if="tasks.length" class="grid gap-2">
        <DownloadTaskRow
          v-for="task in tasks"
          :key="task.id"
          :task="task"
          :busy="busy === task.id"
          @pause="pause(task.id)"
          @resume="resume(task.id)"
          @cancel="cancel(task.id)"
          @export="exportTask(task, $event)"
        />
      </div>
      <div v-else class="grid place-items-center rounded-lg border border-dashed px-8 py-16 text-center">
        <Download :size="32" class="mb-3 text-muted-foreground" />
        <h2 class="text-sm font-semibold">暂无下载任务</h2>
        <p class="mt-1 text-xs text-muted-foreground">
          {{ onlineBooks.length ? '从右上角选择一本在线书开始离线缓存。' : '先把一本在线书加入书架。' }}
        </p>
      </div>
    </PageBody>
  </div>
</template>

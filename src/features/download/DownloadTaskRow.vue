<script setup lang="ts">
import { computed } from 'vue'
import { BookOpen, FileText, Pause, Play, RotateCcw, X } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Progress } from '@/components/ui/progress'
import type { DownloadTask } from '@/services/api'

const props = defineProps<{
  task: DownloadTask
  busy: boolean
}>()

const emit = defineEmits<{
  pause: []
  resume: []
  cancel: []
  export: [format: 'txt' | 'epub']
}>()

const percent = computed(() =>
  props.task.total_chapters ? Math.round((props.task.completed_chapters * 100) / props.task.total_chapters) : 0,
)
const statusText = computed(
  () =>
    ({
      pending: '等待中',
      running: '下载中',
      paused: '已暂停',
      completed: '已完成',
      failed: '失败',
      cancelled: '已取消',
    })[props.task.status],
)
</script>

<template>
  <article class="flex items-center gap-3 rounded-md border bg-card p-3">
    <div class="min-w-0 flex-1">
      <div class="flex items-center gap-2">
        <strong class="truncate text-sm">{{ task.book_title }}</strong>
        <span class="rounded bg-muted px-1.5 text-[10px] text-muted-foreground">{{ statusText }}</span>
      </div>
      <Progress :model-value="percent" class="mt-2 h-1.5" />
      <small class="mt-1 block text-[11px] text-muted-foreground">
        {{ task.completed_chapters }} / {{ task.total_chapters }} 章 · {{ percent }}%
      </small>
      <em v-if="task.error" class="mt-1 block text-[11px] text-destructive">{{ task.error }}</em>
    </div>
    <div class="flex shrink-0 items-center gap-0.5">
      <Button
        v-if="task.status === 'completed'"
        variant="ghost"
        size="icon-sm"
        title="导出 TXT"
        :disabled="busy"
        @click="emit('export', 'txt')"
      >
        <FileText />
      </Button>
      <Button
        v-if="task.status === 'completed'"
        variant="ghost"
        size="icon-sm"
        title="导出 EPUB"
        :disabled="busy"
        @click="emit('export', 'epub')"
      >
        <BookOpen />
      </Button>
      <Button
        v-if="task.status === 'running'"
        variant="ghost"
        size="icon-sm"
        title="暂停"
        :disabled="busy"
        @click="emit('pause')"
      >
        <Pause />
      </Button>
      <Button
        v-if="task.status === 'paused' || task.status === 'failed'"
        variant="ghost"
        size="icon-sm"
        :title="task.status === 'failed' ? '重试' : '继续'"
        :disabled="busy"
        @click="emit('resume')"
      >
        <RotateCcw v-if="task.status === 'failed'" /><Play v-else />
      </Button>
      <Button
        v-if="task.status === 'pending' || task.status === 'running' || task.status === 'paused'"
        variant="ghost"
        size="icon-sm"
        title="取消"
        :disabled="busy"
        @click="emit('cancel')"
      >
        <X />
      </Button>
    </div>
  </article>
</template>

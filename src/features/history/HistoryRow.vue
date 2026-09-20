<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { BookOpen, Trash2 } from 'lucide-vue-next'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import type { ReadingHistoryEntry } from '@/services/api'
import { formatDuration, readAt, readPercent } from '@/lib/readingHistory'

const props = defineProps<{ entry: ReadingHistoryEntry }>()
defineEmits<{ clear: [] }>()

const router = useRouter()
const percent = computed(() => readPercent(props.entry))
const position = computed(() => {
  const { chapter_id, chapter_number, chapter_title, has_progress } = props.entry
  // The three chapter fields are set or unset together; `has_progress` tells the
  // two empty cases apart, so a book read without a saved position is not blamed
  // on a catalog change it never saw.
  if (chapter_id === null) return has_progress ? '阅读位置已失效（目录已更新）' : '尚未保存阅读位置'
  if (chapter_number === null) return '尚未建立目录'
  return `第 ${chapter_number + 1} 章 · ${chapter_title}`
})

/**
 * Opens the book with no location: the reader restores the saved position —
 * paragraph anchor included — which is more exact than any number this page
 * could pass back through the URL.
 */
function resume() {
  void router.push({ name: 'read', params: { bookId: String(props.entry.book_id) } })
}
</script>

<template>
  <li class="flex items-center gap-3 rounded-md border bg-card px-3 py-2.5">
    <span
      class="grid size-10 shrink-0 place-items-center overflow-hidden rounded-md bg-secondary text-sm text-muted-foreground"
    >
      <img v-if="entry.cover_data" :src="entry.cover_data" :alt="entry.book_title" class="h-full w-full object-cover" />
      <template v-else>{{ entry.book_title.slice(0, 1) }}</template>
    </span>
    <div class="min-w-0 flex-1">
      <div class="flex items-center gap-2">
        <span class="truncate text-sm font-medium">{{ entry.book_title }}</span>
        <span class="shrink-0 text-xs text-muted-foreground">{{ entry.book_author || '未知作者' }}</span>
        <Badge v-if="percent === 100" variant="secondary" class="shrink-0">已读完</Badge>
        <Badge v-else-if="percent !== null" variant="outline" class="shrink-0">进度 {{ percent }}%</Badge>
      </div>
      <p class="mt-0.5 truncate text-xs text-muted-foreground">
        {{ position }} · 累计 {{ formatDuration(entry.duration_seconds) }} · 最后阅读
        {{ readAt(entry.last_read_at) }}
      </p>
    </div>
    <Button size="sm" variant="outline" @click="resume"><BookOpen /> 继续阅读</Button>
    <Button
      size="icon-sm"
      variant="ghost"
      class="text-destructive"
      :aria-label="`清除《${entry.book_title}》的阅读记录`"
      @click="$emit('clear')"
    >
      <Trash2 />
    </Button>
  </li>
</template>

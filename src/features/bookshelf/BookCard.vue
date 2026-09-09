<script setup lang="ts">
import { computed } from 'vue'
import { save } from '@tauri-apps/plugin-dialog'
import { BookOpen, Download, FileText, MoreHorizontal, Trash2 } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuSeparator,
  ContextMenuTrigger,
} from '@/components/ui/context-menu'
import { exportBook, refreshCatalog, startDownload, type Book } from '@/services/api'
import { notifyError, notifySuccess } from '@/app/useToast'

const props = defineProps<{ book: Book; selected: boolean; mode: 'grid' | 'list' }>()
const emit = defineEmits<{ open: []; select: []; remove: [] }>()

const badge = computed(() => (props.book.source_id ? '在线' : '本地'))

async function exportAs(format: 'txt' | 'epub') {
  try {
    const path = await save({
      defaultPath: `${props.book.title}.${format}`,
      filters: [{ name: format.toUpperCase(), extensions: [format] }],
    })
    if (!path) return
    const result = await exportBook(props.book.id, format, path)
    notifySuccess(`已导出 ${result.bytes_written} 字节`)
  } catch (cause) {
    notifyError(String(cause))
  }
}

async function download() {
  try {
    await startDownload(props.book.id)
    notifySuccess('下载任务已创建')
  } catch (cause) {
    notifyError(String(cause))
  }
}

async function refresh() {
  try {
    await refreshCatalog(props.book.id)
    notifySuccess('目录已刷新')
  } catch (cause) {
    notifyError(String(cause))
  }
}
</script>

<template>
  <ContextMenu>
    <ContextMenuTrigger as-child>
      <article
        :class="[
          'group relative cursor-pointer rounded-lg border bg-card transition-shadow hover:shadow-sm',
          selected ? 'ring-2 ring-ring' : '',
          mode === 'grid' ? 'flex flex-col p-3' : 'flex items-center gap-3 p-2.5',
        ]"
        tabindex="0"
        @click="emit('open')"
        @keydown.enter="emit('open')"
        @contextmenu.prevent
      >
        <div
          :class="[
            'grid shrink-0 place-items-center overflow-hidden rounded-md bg-secondary text-xs text-muted-foreground',
            mode === 'grid' ? 'mb-2.5 h-[132px] w-full' : 'h-14 w-10',
          ]"
        >
          <img v-if="book.cover_data" :src="book.cover_data" :alt="book.title" class="h-full w-full object-cover" />
          <BookOpen v-else :size="mode === 'grid' ? 22 : 14" />
        </div>
        <div class="min-w-0 flex-1">
          <h3 class="truncate text-sm font-medium">{{ book.title }}</h3>
          <p class="mt-0.5 truncate text-xs text-muted-foreground">
            {{ book.author || '未知作者' }} · {{ book.chapter_count }} 章
          </p>
          <div class="mt-1.5 flex items-center gap-1.5">
            <span class="rounded bg-muted px-1.5 text-[10px] text-muted-foreground">{{ badge }}</span>
            <span v-if="book.latest_chapter" class="truncate text-[10px] text-muted-foreground">
              {{ book.latest_chapter }}
            </span>
          </div>
        </div>
        <div
          class="absolute right-2 top-2 flex gap-0.5 opacity-0 transition-opacity group-hover:opacity-100 group-focus-within:opacity-100"
        >
          <Button
            variant="secondary"
            size="icon-sm"
            :aria-label="`下载 ${book.title}`"
            title="下载"
            @click.stop="download"
          >
            <Download />
          </Button>
          <Button
            variant="secondary"
            size="icon-sm"
            :aria-label="`导出 ${book.title}`"
            title="导出 TXT"
            @click.stop="exportAs('txt')"
          >
            <FileText />
          </Button>
          <Button
            variant="secondary"
            size="icon-sm"
            :aria-label="`选择 ${book.title}`"
            title="多选"
            @click.stop="emit('select')"
          >
            <MoreHorizontal />
          </Button>
        </div>
      </article>
    </ContextMenuTrigger>
    <ContextMenuContent class="w-48">
      <ContextMenuItem @select="emit('open')">开始阅读</ContextMenuItem>
      <ContextMenuItem @select="refresh">刷新目录</ContextMenuItem>
      <ContextMenuItem @select="download">加入下载队列</ContextMenuItem>
      <ContextMenuSeparator />
      <ContextMenuItem @select="exportAs('txt')">导出为 TXT</ContextMenuItem>
      <ContextMenuItem @select="exportAs('epub')">导出为 EPUB</ContextMenuItem>
      <ContextMenuSeparator />
      <ContextMenuItem @select="emit('select')">多选</ContextMenuItem>
      <ContextMenuItem class="text-destructive" @select="emit('remove')"> <Trash2 /> 删除书籍 </ContextMenuItem>
    </ContextMenuContent>
  </ContextMenu>
</template>

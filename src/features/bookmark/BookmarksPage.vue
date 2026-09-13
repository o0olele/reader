<script setup lang="ts">
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { Bookmark, RefreshCw, Trash2 } from 'lucide-vue-next'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import PageBody from '@/components/PageBody.vue'
import PageHeader from '@/components/PageHeader.vue'
import type { BookmarkEntry } from '@/services/api'
import { useBookmarks } from './useBookmarks'

const router = useRouter()
const bookmarks = useBookmarks()

/** SQLite writes `updated_at` in UTC; show it in the reader's own timezone. */
function savedAt(value: string): string {
  const date = new Date(`${value.replace(' ', 'T')}Z`)
  if (Number.isNaN(date.getTime())) return value
  return date.toLocaleString('zh-CN', { month: 'numeric', day: 'numeric', hour: '2-digit', minute: '2-digit' })
}

/** The reader deep link restores the exact scroll offset the bookmark holds. */
function open(entry: BookmarkEntry) {
  void router.push({
    name: 'read',
    params: { bookId: String(entry.book_id) },
    query: { chapter: String(entry.chapter_id), offset: String(entry.offset), mode: entry.mode },
  })
}

onMounted(bookmarks.refresh)
</script>

<template>
  <div class="flex h-full flex-col">
    <PageHeader title="书签" :subtitle="`${bookmarks.items.length} 条书签 · 跨书汇总，按保存时间排序`">
      <Button variant="outline" size="sm" :disabled="bookmarks.loading" @click="bookmarks.refresh()">
        <RefreshCw :class="bookmarks.loading ? 'animate-spin' : ''" /> 刷新
      </Button>
    </PageHeader>

    <PageBody>
      <div class="flex items-center gap-2">
        <Input
          v-model="bookmarks.keyword"
          class="h-8 max-w-xs"
          placeholder="搜索书名、作者或章节"
          aria-label="搜索书签"
        />
        <span v-if="bookmarks.keyword" class="text-xs text-muted-foreground">
          匹配 {{ bookmarks.visible.length }} / {{ bookmarks.items.length }} 条
        </span>
      </div>

      <p v-if="bookmarks.error" role="alert" class="mt-4 rounded-md border border-dashed p-4 text-xs text-destructive">
        {{ bookmarks.error }}
      </p>

      <ul v-else-if="bookmarks.visible.length" class="mt-4 grid gap-2">
        <li
          v-for="entry in bookmarks.visible"
          :key="entry.chapter_id"
          class="flex items-center gap-3 rounded-md border bg-card px-3 py-2.5"
        >
          <span
            class="grid size-9 shrink-0 place-items-center overflow-hidden rounded-md bg-secondary text-sm text-muted-foreground"
          >
            {{ entry.book_title.slice(0, 1) }}
          </span>
          <div class="min-w-0 flex-1">
            <div class="flex items-center gap-2">
              <span class="truncate text-sm font-medium">{{ entry.book_title }}</span>
              <span class="shrink-0 text-xs text-muted-foreground">{{ entry.book_author || '未知作者' }}</span>
              <Badge variant="secondary" class="shrink-0">{{ entry.mode === 'paged' ? '翻页位置' : '滚动位置' }}</Badge>
            </div>
            <p class="mt-0.5 truncate text-xs text-muted-foreground">
              第 {{ entry.chapter_number + 1 }} 章 · {{ entry.chapter_title }} · {{ savedAt(entry.updated_at) }}
            </p>
          </div>
          <Button size="sm" variant="outline" @click="open(entry)"><Bookmark /> 打开</Button>
          <Button
            size="icon-sm"
            variant="ghost"
            class="text-destructive"
            :aria-label="`删除《${entry.book_title}》的书签`"
            @click="bookmarks.remove(entry)"
          >
            <Trash2 />
          </Button>
        </li>
      </ul>

      <p
        v-else-if="bookmarks.loading"
        class="mt-4 rounded-md border border-dashed p-6 text-center text-xs text-muted-foreground"
      >
        正在读取书签…
      </p>

      <div
        v-else
        class="mt-4 grid place-items-center rounded-lg border border-dashed px-8 py-12 text-center text-muted-foreground"
      >
        <Bookmark :size="28" class="mb-3" />
        <h2 class="text-sm font-semibold text-foreground">
          {{ bookmarks.keyword ? '没有匹配的书签' : '还没有书签' }}
        </h2>
        <p class="mt-1.5 max-w-md text-xs leading-relaxed">
          {{
            bookmarks.keyword
              ? '换个书名或章节关键词试试。'
              : '在阅读器底部点「加书签」记录当前位置，所有书签都会汇总到这里。'
          }}
        </p>
      </div>
    </PageBody>
  </div>
</template>

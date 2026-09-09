<script setup lang="ts">
import { computed, ref } from 'vue'
import { useRouter } from 'vue-router'
import { Download, FolderPlus, Grid3x3, List, Plus, RefreshCw, Trash2, X } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { ToggleGroup, ToggleGroupItem } from '@/components/ui/toggle-group'
import BookCard from './BookCard.vue'
import PageHeader from '@/components/PageHeader.vue'
import { startDownload, type Book } from '@/services/api'
import { notifyError, notifySuccess } from '@/app/useToast'
import { useShellContext } from '@/app/shellKeys'

type SortKey = 'updated' | 'title' | 'author' | 'chapters'
type FilterKey = 'all' | 'local' | 'online'

const router = useRouter()
const { bookshelf, openBook } = useShellContext()
const fileInput = ref<HTMLInputElement>()
const keyword = ref('')
const sort = ref<SortKey>('updated')
const filter = ref<FilterKey>('all')
const mode = ref<'grid' | 'list'>('grid')
const selected = ref<number[]>([])

const visible = computed(() => {
  const needle = keyword.value.trim().toLowerCase()
  let list = bookshelf.visibleBooks.filter((book) =>
    needle ? `${book.title} ${book.author ?? ''}`.toLowerCase().includes(needle) : true,
  )
  if (filter.value === 'local') list = list.filter((book) => !book.source_id)
  if (filter.value === 'online') list = list.filter((book) => Boolean(book.source_id))
  const sorted = [...list]
  if (sort.value === 'title') sorted.sort((a, b) => a.title.localeCompare(b.title, 'zh-Hans-CN'))
  else if (sort.value === 'author') sorted.sort((a, b) => (a.author ?? '').localeCompare(b.author ?? '', 'zh-Hans-CN'))
  else if (sort.value === 'chapters') sorted.sort((a, b) => b.chapter_count - a.chapter_count)
  else sorted.sort((a, b) => b.updated_at.localeCompare(a.updated_at))
  return sorted
})

function toggleSelect(book: Book) {
  selected.value = selected.value.includes(book.id)
    ? selected.value.filter((id) => id !== book.id)
    : [...selected.value, book.id]
}

async function open(book: Book) {
  await openBook(book)
  await router.push({ name: 'read', params: { bookId: String(book.id) } })
}

async function downloadSelected() {
  try {
    await Promise.all(selected.value.map((id) => startDownload(id)))
    notifySuccess(`已创建 ${selected.value.length} 个下载任务`)
    selected.value = []
  } catch (cause) {
    notifyError(String(cause))
  }
}

async function removeSelected() {
  const books = bookshelf.books.filter((book) => selected.value.includes(book.id))
  if (!window.confirm(`确定删除选中的 ${books.length} 本书吗？`)) return
  for (const book of books) await bookshelf.removeBook(book)
  selected.value = []
}
</script>

<template>
  <div class="flex h-full flex-col">
    <PageHeader title="书架" :subtitle="`${bookshelf.books.length} 本 · ${bookshelf.groups.length} 个分组`" />

    <div class="flex min-h-0 flex-1">
      <aside class="w-52 shrink-0 overflow-y-auto border-r bg-card p-2">
        <button
          type="button"
          class="mb-0.5 flex w-full items-center justify-between rounded-md px-2.5 py-1.5 text-sm hover:bg-accent"
          :class="bookshelf.activeGroup === null ? 'bg-accent font-medium' : ''"
          @click="bookshelf.activeGroup = null"
        >
          <span>全部书籍</span><span class="text-xs text-muted-foreground">{{ bookshelf.books.length }}</span>
        </button>
        <button
          v-for="group in bookshelf.groups"
          :key="group.id"
          type="button"
          class="mb-0.5 flex w-full items-center justify-between rounded-md px-2.5 py-1.5 text-sm hover:bg-accent"
          :class="bookshelf.activeGroup === group.id ? 'bg-accent font-medium' : ''"
          @click="bookshelf.activeGroup = group.id"
        >
          <span class="truncate">{{ group.name }}</span>
          <span class="text-xs text-muted-foreground">{{ group.book_count }}</span>
        </button>
        <Button variant="ghost" size="sm" class="mt-1 w-full justify-start" @click="bookshelf.addGroup()">
          <FolderPlus /> 新建分组
        </Button>
      </aside>

      <div class="flex min-w-0 flex-1 flex-col">
        <div class="flex shrink-0 items-center gap-2 border-b bg-card px-4 py-2">
          <Input v-model="keyword" class="h-8 w-56" placeholder="搜索书名、作者" aria-label="搜索书架" />
          <Select v-model="sort">
            <SelectTrigger class="h-8 w-32"><SelectValue /></SelectTrigger>
            <SelectContent>
              <SelectItem value="updated">最近更新</SelectItem>
              <SelectItem value="title">按标题</SelectItem>
              <SelectItem value="author">按作者</SelectItem>
              <SelectItem value="chapters">按章节数</SelectItem>
            </SelectContent>
          </Select>
          <Select v-model="filter">
            <SelectTrigger class="h-8 w-28"><SelectValue /></SelectTrigger>
            <SelectContent>
              <SelectItem value="all">全部来源</SelectItem>
              <SelectItem value="local">仅本地</SelectItem>
              <SelectItem value="online">仅在线</SelectItem>
            </SelectContent>
          </Select>
          <Button variant="outline" size="sm" @click="bookshelf.refresh()"><RefreshCw /> 刷新</Button>
          <ToggleGroup
            type="single"
            :model-value="mode"
            variant="outline"
            size="sm"
            class="ml-auto"
            @update:model-value="mode = ($event as 'grid' | 'list') || mode"
          >
            <ToggleGroupItem value="grid" aria-label="网格视图"><Grid3x3 /></ToggleGroupItem>
            <ToggleGroupItem value="list" aria-label="列表视图"><List /></ToggleGroupItem>
          </ToggleGroup>
          <Button size="sm" @click="fileInput?.click()"><Plus /> 导入书籍</Button>
          <input
            ref="fileInput"
            class="hidden"
            type="file"
            accept=".txt,.epub,text/plain,application/epub+zip"
            @change="bookshelf.handleFile"
          />
        </div>

        <div class="min-h-0 flex-1 overflow-y-auto p-4">
          <div
            v-if="visible.length"
            :class="mode === 'grid' ? 'grid grid-cols-[repeat(auto-fill,minmax(168px,1fr))] gap-3' : 'grid gap-2'"
          >
            <BookCard
              v-for="book in visible"
              :key="book.id"
              :book="book"
              :mode="mode"
              :selected="selected.includes(book.id)"
              @open="open(book)"
              @select="toggleSelect(book)"
              @remove="bookshelf.removeBook(book)"
            />
          </div>
          <div v-else class="grid place-items-center rounded-lg border border-dashed px-8 py-16 text-center">
            <h2 class="text-sm font-semibold">书架还是空的</h2>
            <p class="mt-1 text-xs text-muted-foreground">导入 TXT 或 EPUB，或从发现页加入一本在线书。</p>
            <Button class="mt-4" size="sm" @click="fileInput?.click()"><Plus /> 选择文件</Button>
          </div>
        </div>

        <div v-if="selected.length" class="flex shrink-0 items-center gap-3 border-t bg-card px-4 py-2 text-sm">
          <span>已选 {{ selected.length }} 本</span>
          <Button variant="outline" size="sm" @click="downloadSelected"><Download /> 批量下载</Button>
          <Button variant="outline" size="sm" class="text-destructive" @click="removeSelected">
            <Trash2 /> 批量删除
          </Button>
          <Button variant="ghost" size="sm" class="ml-auto" @click="selected = []"><X /> 取消选择</Button>
        </div>
      </div>
    </div>
  </div>
</template>

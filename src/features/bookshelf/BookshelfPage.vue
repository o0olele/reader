<script setup lang="ts">
import { computed, ref } from 'vue'
import { useRouter } from 'vue-router'
import { Download, FolderInput, Plus, Trash2, X } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import BookCard from './BookCard.vue'
import BookshelfSidebar from './BookshelfSidebar.vue'
import BookshelfToolbar from './BookshelfToolbar.vue'
import MoveToGroupDialog from './MoveToGroupDialog.vue'
import PageHeader from '@/components/PageHeader.vue'
import { startDownload, type Book } from '@/services/api'
import { notifyError, notifySuccess } from '@/app/useToast'
import { useShellContext } from '@/app/shellKeys'
import { arrangeBooks, type FilterKey, type SortKey, type ViewMode } from './shelfView'
import { writeDraggedBookIds } from './shelfDrag'

const router = useRouter()
const { bookshelf, openBook } = useShellContext()
const fileInput = ref<HTMLInputElement>()
const keyword = ref('')
const sort = ref<SortKey>('updated')
const filter = ref<FilterKey>('all')
const mode = ref<ViewMode>('grid')
const selected = ref<number[]>([])
const moveTargets = ref<number[]>([])
const moveOpen = ref(false)

const visible = computed(() =>
  arrangeBooks(bookshelf.visibleBooks, { keyword: keyword.value, sort: sort.value, filter: filter.value }),
)

function toggleSelect(book: Book) {
  selected.value = selected.value.includes(book.id)
    ? selected.value.filter((id) => id !== book.id)
    : [...selected.value, book.id]
}

async function open(book: Book) {
  await openBook(book)
  await router.push({ name: 'read', params: { bookId: String(book.id) } })
}

/** Dragging a selected card carries the whole selection; otherwise just that book. */
function startDrag(book: Book, event: DragEvent) {
  writeDraggedBookIds(event, selected.value.includes(book.id) ? selected.value : [book.id])
}

function requestMove(book?: Book) {
  moveTargets.value = book ? [book.id] : [...selected.value]
  if (moveTargets.value.length) moveOpen.value = true
}

async function moveBook(book: Book, groupId: number) {
  const group = bookshelf.groups.find((item) => item.id === groupId)
  const moved = await bookshelf.moveBooks([book.id], groupId)
  if (moved && group) notifySuccess(`已将《${book.title}》移动到「${group.name}」`)
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
      <BookshelfSidebar />

      <div class="flex min-w-0 flex-1 flex-col">
        <BookshelfToolbar
          v-model:keyword="keyword"
          v-model:sort="sort"
          v-model:filter="filter"
          v-model:mode="mode"
          @import="fileInput?.click()"
        />
        <input
          ref="fileInput"
          class="hidden"
          type="file"
          accept=".txt,.epub,text/plain,application/epub+zip"
          @change="bookshelf.handleFile"
        />

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
              :groups="bookshelf.groups"
              :selected="selected.includes(book.id)"
              @open="open(book)"
              @select="toggleSelect(book)"
              @remove="bookshelf.removeBook(book)"
              @move="moveBook(book, $event)"
              @move-request="requestMove(book)"
              @dragstart="startDrag(book, $event)"
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
          <Button variant="outline" size="sm" @click="requestMove()"><FolderInput /> 移动到分组</Button>
          <Button variant="outline" size="sm" @click="downloadSelected"><Download /> 批量下载</Button>
          <Button variant="outline" size="sm" class="text-destructive" @click="removeSelected">
            <Trash2 /> 批量删除
          </Button>
          <Button variant="ghost" size="sm" class="ml-auto" @click="selected = []"><X /> 取消选择</Button>
        </div>
      </div>
    </div>

    <MoveToGroupDialog v-model:open="moveOpen" :book-ids="moveTargets" @moved="selected = []" />
  </div>
</template>

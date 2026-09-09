<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { BookOpen } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import NotConnected from '@/components/NotConnected.vue'
import ReaderBottomBar from './ReaderBottomBar.vue'
import ReaderPane from './ReaderPane.vue'
import ReaderSettingsPanel from './ReaderSettingsPanel.vue'
import ReaderTopbar from './ReaderTopbar.vue'
import { useShellContext } from '@/app/shellKeys'
import type { Chapter } from '@/services/api'

const route = useRoute()
const router = useRouter()
const { reader, bookshelf, openBook } = useShellContext()

// Deep link: `#/read/<bookId>?toc=0&panel=1` (prototype :2778–2794).
const chapterListOpen = ref(route.query.toc !== '0')
const settingsOpen = ref(route.query.panel === '1')
const immersive = ref(false)
const autoPage = ref(false)
const pane = ref<InstanceType<typeof ReaderPane>>()
const missing = ref<{ title: string; description: string; capabilities: string[] }>()

const chapterIndex = computed(() => reader.chapters.findIndex((chapter) => chapter.id === reader.selectedChapter?.id))

function selectChapter(chapter: Chapter) {
  void reader.selectChapter(chapter)
}

function selectRelativeChapter(offset: number) {
  const chapter = reader.chapters[chapterIndex.value + offset]
  if (chapter) selectChapter(chapter)
}

async function closeReader() {
  await reader.closeBook()
  await router.push({ name: 'bookshelf' })
}

async function openFromShelf(book: (typeof bookshelf.books)[number]) {
  await openBook(book)
  await router.push({ name: 'read', params: { bookId: String(book.id) } })
}

function showMissing(title: string, description: string, capabilities: string[]) {
  missing.value = { title, description, capabilities }
}

watch(
  [() => route.params.bookId, () => bookshelf.books.length],
  async () => {
    const id = Number(route.params.bookId)
    if (!id || reader.selectedBook?.id === id) return
    const book = bookshelf.books.find((item) => item.id === id)
    if (book) await openBook(book)
  },
  { immediate: true },
)

let autoPageTimer: ReturnType<typeof setInterval> | undefined
watch(autoPage, (on) => {
  if (autoPageTimer) clearInterval(autoPageTimer)
  if (on) autoPageTimer = setInterval(() => pane.value?.advance(), 12_000)
})

function isTyping(target: EventTarget | null) {
  const element = target as HTMLElement | null
  const tag = element?.tagName?.toLowerCase() ?? ''
  return tag === 'input' || tag === 'textarea' || element?.isContentEditable === true
}

// Prototype reader keys: `T` toggles the catalog, `F` toggles immersive mode.
// Page turning (`→` / `Space` / `←`) is owned by ReaderPane's paged mode.
function onKeydown(event: KeyboardEvent) {
  if (isTyping(event.target)) return
  if (event.key === 't' || event.key === 'T') chapterListOpen.value = !chapterListOpen.value
  else if (event.key === 'f' || event.key === 'F') immersive.value = !immersive.value
}

onMounted(() => window.addEventListener('keydown', onKeydown))
onBeforeUnmount(() => {
  window.removeEventListener('keydown', onKeydown)
  if (autoPageTimer) clearInterval(autoPageTimer)
})
</script>

<template>
  <div class="flex h-full flex-col">
    <template v-if="reader.selectedBook">
      <ReaderTopbar
        :collapsed="immersive"
        :chapter-list-open="chapterListOpen"
        :settings-open="settingsOpen"
        @prev="selectRelativeChapter(-1)"
        @next="selectRelativeChapter(1)"
        @close="closeReader"
        @toggle-toc="chapterListOpen = !chapterListOpen"
        @toggle-panel="settingsOpen = !settingsOpen"
      />
      <div class="flex min-h-0 flex-1">
        <ReaderPane
          ref="pane"
          :chapters="reader.chapters"
          :selected-chapter="reader.selectedChapter"
          :theme="reader.theme"
          :font-size="reader.fontSize"
          :font-family="reader.fontFamily"
          :line-height="reader.lineHeight"
          :page-margin="reader.pageMargin"
          :paragraph-spacing="reader.paragraphSpacing"
          :text-indent="reader.textIndent"
          :justify="reader.justify"
          :page-animation="reader.pageAnimation"
          :brightness="reader.brightness"
          :eye-care="reader.eyeCare"
          :reader-mode="reader.readerMode"
          :chapter-list-open="chapterListOpen"
          :loading="reader.loadingChapter"
          :book="reader.selectedBook"
          @select-chapter="selectChapter"
          @scroll="reader.scheduleProgressSave"
          @reader-content="reader.readerContent = $event"
          @reader-mode="reader.readerMode = $event"
        />
        <ReaderSettingsPanel v-if="settingsOpen" @close="settingsOpen = false" />
      </div>
      <ReaderBottomBar
        :class="immersive ? 'h-0 overflow-hidden border-t-0' : ''"
        class="transition-[height] duration-150"
        :chapter-count="reader.chapters.length"
        :chapter-index="chapterIndex"
        :has-bookmark="Boolean(pane?.bookmark)"
        :auto-page="autoPage"
        :eye-care="reader.eyeCare"
        @prev="selectRelativeChapter(-1)"
        @next="selectRelativeChapter(1)"
        @goto="reader.chapters[$event] && selectChapter(reader.chapters[$event])"
        @search="chapterListOpen = false"
        @auto-page="autoPage = !autoPage"
        @toc="chapterListOpen = !chapterListOpen"
        @style="settingsOpen = !settingsOpen"
        @bookmark="pane?.toggleBookmark()"
        @theme="reader.theme = reader.theme === 'light' ? 'dark' : 'light'"
        @eye-care="reader.eyeCare = !reader.eyeCare"
        @tts="showMissing('听书', '需要桌面 TTS 后端。', ['桌面 TTS 后端选型与朗读命令（ROADMAP-v3 S）'])"
        @translate="showMissing('翻译', '需要翻译服务接入。', ['翻译服务接入（ROADMAP-v3 S）'])"
        @ai="showMissing('AI 总结', '需要 AI 子系统。', ['AI 子系统与模型配置（ROADMAP-v3 S）'])"
        @text-process="showMissing('文本处理', '尚未实现。', ['文本处理工具集'])"
        @more="showMissing('更多', '多窗口阅读 / 全局快捷键等属于实验室，§9 明确不排期。', ['§9 明确不做'])"
      />
    </template>

    <div v-else class="flex min-h-0 flex-1 items-center justify-center p-10">
      <div class="w-full max-w-md text-center">
        <BookOpen :size="34" class="mx-auto mb-3 text-muted-foreground" />
        <h2 class="text-sm font-semibold">还没有打开的书</h2>
        <p class="mt-1 text-xs text-muted-foreground">从书架选一本开始阅读，URL 里的 bookId 会被记录为深链接。</p>
        <div v-if="bookshelf.books.length" class="mt-4 grid gap-1.5 text-left">
          <Button
            v-for="book in bookshelf.books.slice(0, 6)"
            :key="book.id"
            variant="outline"
            size="sm"
            class="justify-start"
            @click="openFromShelf(book)"
          >
            <BookOpen />{{ book.title }}
          </Button>
        </div>
        <Button v-else class="mt-4" size="sm" @click="router.push({ name: 'bookshelf' })">去书架导入</Button>
      </div>
    </div>

    <Dialog :open="Boolean(missing)" @update:open="missing = undefined">
      <DialogContent class="sm:max-w-md">
        <DialogHeader><DialogTitle>未接入</DialogTitle></DialogHeader>
        <NotConnected
          v-if="missing"
          :title="missing.title"
          :description="missing.description"
          :capabilities="missing.capabilities"
        />
      </DialogContent>
    </Dialog>
  </div>
</template>

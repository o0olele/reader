<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import ReaderBookPanel from './ReaderBookPanel.vue'
import ReaderBottomBar from './ReaderBottomBar.vue'
import ReaderEmptyState from './ReaderEmptyState.vue'
import ReaderPane from './ReaderPane.vue'
import ReaderSearchPanel from './ReaderSearchPanel.vue'
import ReaderSettingsPanel from './ReaderSettingsPanel.vue'
import ReaderTopbar from './ReaderTopbar.vue'
import ReaderUnavailableDialog from './ReaderUnavailableDialog.vue'
import { useContentSearch } from './useContentSearch'
import { useReaderDeepLink } from './useReaderDeepLink'
import { useShellContext } from '@/app/shellKeys'
import type { Chapter, SearchContentHit } from '@/services/api'

const route = useRoute()
const router = useRouter()
const { reader } = useShellContext()

// Deep link (prototype :2778–2794): `toc` / `panel` are read here, while the
// book itself (plus a bookmark's `chapter` / `offset` / `mode`) is owned by
// `useReaderDeepLink`.
useReaderDeepLink()

const chapterListOpen = ref(route.query.toc !== '0')
/** 右侧只有一个 320px 槽位（ROADMAP-v3 §4 panel），同一时刻最多一块面板。 */
type ReaderPanel = 'book' | 'search' | 'settings'
const panel = ref<ReaderPanel | undefined>(route.query.panel === '1' ? 'settings' : undefined)
const bookOpen = computed(() => panel.value === 'book')
const searchOpen = computed(() => panel.value === 'search')
const settingsOpen = computed(() => panel.value === 'settings')
const immersive = ref(false)
const autoPage = ref(false)
const pane = ref<InstanceType<typeof ReaderPane>>()
const missing = ref<{ title: string; description: string; capabilities: string[] }>()

const contentSearch = useContentSearch(
  () => reader.selectedBook?.id,
  () => reader.selectedChapter?.id,
)

const chapterIndex = computed(() => reader.chapters.findIndex((chapter) => chapter.id === reader.selectedChapter?.id))

function selectChapter(chapter: Chapter) {
  void reader.selectChapter(chapter)
}

function selectRelativeChapter(offset: number) {
  const chapter = reader.chapters[chapterIndex.value + offset]
  if (chapter) selectChapter(chapter)
}

/** 正文搜索的结果行：跳章 + 定位 + 高亮，面板留着方便继续点下一条。 */
function openSearchHit(hit: SearchContentHit) {
  void pane.value?.jumpToHit(hit)
}

/** 再点同一颗按钮就收起；换一块面板时其余两块自动让位。 */
function togglePanel(target: ReaderPanel) {
  panel.value = panel.value === target ? undefined : target
}

function toggleSearch() {
  togglePanel('search')
  if (searchOpen.value) chapterListOpen.value = false
}

async function closeReader() {
  await reader.closeBook()
  await router.push({ name: 'bookshelf' })
}

function showMissing(title: string, description: string, capabilities: string[]) {
  missing.value = { title, description, capabilities }
}

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

// Prototype reader keys: `T` toggles the catalog, `F` toggles immersive mode,
// `Esc` closes the 正文搜索 panel (`desktop-ui.html:2762`).
// Page turning (`→` / `Space` / `←`) is owned by ReaderPane's paged mode.
function onKeydown(event: KeyboardEvent) {
  if (isTyping(event.target)) return
  if (event.key === 'Escape' && searchOpen.value) panel.value = undefined
  else if (event.key === 't' || event.key === 'T') chapterListOpen.value = !chapterListOpen.value
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
        :book-open="bookOpen"
        @prev="selectRelativeChapter(-1)"
        @next="selectRelativeChapter(1)"
        @close="closeReader"
        @toggle-toc="chapterListOpen = !chapterListOpen"
        @toggle-panel="togglePanel('settings')"
        @toggle-book="togglePanel('book')"
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
        <ReaderSettingsPanel v-if="settingsOpen" @close="panel = undefined" />
        <ReaderBookPanel
          v-if="bookOpen && reader.selectedBook"
          :book="reader.selectedBook"
          :chapter-index="chapterIndex"
          :chapter-count="reader.chapters.length"
          @close="panel = undefined"
        />
        <!-- 搜索面板按一个状态对象转发：它是阅读器内部组合，不必逐字段拆成 props。 -->
        <ReaderSearchPanel
          v-if="searchOpen"
          :search="contentSearch"
          :current-chapter-id="reader.selectedChapter?.id"
          @update:query="contentSearch.query = $event"
          @update:regex="contentSearch.regex = $event"
          @update:scope="contentSearch.scope = $event"
          @jump="openSearchHit"
          @stop="contentSearch.stop()"
          @close="panel = undefined"
        />
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
        @search="toggleSearch"
        @auto-page="autoPage = !autoPage"
        @toc="chapterListOpen = !chapterListOpen"
        @style="togglePanel('settings')"
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

    <ReaderEmptyState v-else />

    <ReaderUnavailableDialog :tool="missing" @close="missing = undefined" />
  </div>
</template>

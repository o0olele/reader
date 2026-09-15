<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import ReaderBottomBar from './ReaderBottomBar.vue'
import ReaderEmptyState from './ReaderEmptyState.vue'
import ReaderPane from './ReaderPane.vue'
import ReaderPanels from './ReaderPanels.vue'
import ReaderTopbar from './ReaderTopbar.vue'
import ReaderUnavailableDialog from './ReaderUnavailableDialog.vue'
import { useChangeSource } from './useChangeSource'
import { useContentSearch } from './useContentSearch'
import { useReaderDeepLink } from './useReaderDeepLink'
import { useReaderSidePanels } from './useReaderSidePanels'
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
const immersive = ref(false)
const autoPage = ref(false)
const pane = ref<InstanceType<typeof ReaderPane>>()
const missing = ref<{ title: string; description: string; capabilities: string[] }>()

const contentSearch = useContentSearch(
  () => reader.selectedBook?.id,
  () => reader.selectedChapter?.id,
)

/** 换源面板（`useChangeSource`）自己持有开关，好让换源搜索能跨面板开关继续跑。 */
const changeSource = useChangeSource()
const panels = useReaderSidePanels(changeSource, route.query.panel === '1' ? 'settings' : undefined)

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

function toggleSearch() {
  panels.toggle('search')
  if (panels.searchOpen) chapterListOpen.value = false
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
  if (event.key === 'Escape' && panels.searchOpen) panels.close()
  else if (event.key === 'Escape' && panels.sourceOpen) changeSource.close()
  else if (event.key === 't' || event.key === 'T') chapterListOpen.value = !chapterListOpen.value
  else if (event.key === 'f' || event.key === 'F') immersive.value = !immersive.value
}

onMounted(() => window.addEventListener('keydown', onKeydown))
onBeforeUnmount(() => {
  window.removeEventListener('keydown', onKeydown)
  if (autoPageTimer) clearInterval(autoPageTimer)
  // 离开阅读器时停掉还在跑的换源搜索，不让它再往已卸载的组件里写结果。
  changeSource.close()
})
</script>

<template>
  <div class="flex h-full flex-col">
    <template v-if="reader.selectedBook">
      <ReaderTopbar
        :collapsed="immersive"
        :chapter-list-open="chapterListOpen"
        :settings-open="panels.settingsOpen"
        :book-open="panels.bookOpen"
        :switching-source="reader.switchingSource"
        :source-open="panels.sourceOpen"
        @prev="selectRelativeChapter(-1)"
        @next="selectRelativeChapter(1)"
        @close="closeReader"
        @toggle-toc="chapterListOpen = !chapterListOpen"
        @toggle-panel="panels.toggle('settings')"
        @toggle-book="panels.toggle('book')"
        @change-source="panels.toggleSource()"
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
        <ReaderPanels
          :panel="panels.panel"
          :change-source="changeSource"
          :search="contentSearch"
          :book="reader.selectedBook"
          :chapter-index="chapterIndex"
          :chapter-count="reader.chapters.length"
          :selected-chapter-id="reader.selectedChapter?.id"
          @close="panels.close()"
          @jump="openSearchHit"
          @stop-search="contentSearch.stop()"
          @update:query="contentSearch.query = $event"
          @update:regex="contentSearch.regex = $event"
          @update:scope="contentSearch.scope = $event"
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
        @style="panels.toggle('settings')"
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

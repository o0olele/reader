<script setup lang="ts">
import ReaderBookPanel from './ReaderBookPanel.vue'
import ReaderChangeSourcePanel from './ReaderChangeSourcePanel.vue'
import ReaderSearchPanel from './ReaderSearchPanel.vue'
import ReaderSettingsPanel from './ReaderSettingsPanel.vue'
import type { Book, SearchContentHit } from '@/services/api'
import type { useChangeSource } from './useChangeSource'
import type { useContentSearch } from './useContentSearch'
import type { ReaderPanel } from './useReaderSidePanels'

/**
 * 右侧 320px 槽位里的四块面板：书籍信息 / 正文搜索 / 阅读设置 / 换源。
 *
 * 纯展示：面板开合由 `useReaderSidePanels` 与 `useChangeSource` 决定，
 * 这里只按 `panel` 渲染其中一块（换源面板认自己的 `open`），并原样转发事件。
 */
defineProps<{
  panel?: ReaderPanel
  changeSource: ReturnType<typeof useChangeSource>
  search: ReturnType<typeof useContentSearch>
  book?: Book
  chapterIndex: number
  chapterCount: number
  selectedChapterId?: number
}>()

const emit = defineEmits<{
  close: []
  jump: [hit: SearchContentHit]
  stopSearch: []
  'update:query': [value: string]
  'update:regex': [value: boolean]
  'update:scope': [value: 'book' | 'chapter']
}>()
</script>

<template>
  <ReaderSettingsPanel v-if="panel === 'settings'" @close="emit('close')" />
  <ReaderBookPanel
    v-if="panel === 'book' && book"
    :book="book"
    :chapter-index="chapterIndex"
    :chapter-count="chapterCount"
    @close="emit('close')"
  />
  <!-- 搜索面板按一个状态对象转发：它是阅读器内部组合，不必逐字段拆成 props。 -->
  <ReaderSearchPanel
    v-if="panel === 'search'"
    :search="search"
    :current-chapter-id="selectedChapterId"
    @update:query="emit('update:query', $event)"
    @update:regex="emit('update:regex', $event)"
    @update:scope="emit('update:scope', $event)"
    @jump="emit('jump', $event)"
    @stop="emit('stopSearch')"
    @close="emit('close')"
  />
  <ReaderChangeSourcePanel v-if="changeSource.open" :change="changeSource" @close="changeSource.close()" />
</template>

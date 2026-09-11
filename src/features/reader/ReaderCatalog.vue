<script setup lang="ts">
import { computed, ref } from 'vue'
import { Bookmark, List } from 'lucide-vue-next'
import { Input } from '@/components/ui/input'
import { volumeLabel } from './readerTypography'
import type { Chapter } from '@/services/api'

const props = defineProps<{
  chapters: Chapter[]
  selectedChapter?: Chapter
  hasBookmark: boolean
  /** Persisted reading position — everything before it is 已读 (prototype :1978–1981). */
  readUpToChapterId?: number
  /** Live in-chapter progress for the chapter being read. */
  currentPercent: number
}>()

const emit = defineEmits<{ selectChapter: [chapter: Chapter]; jump: [] }>()

const activeTab = ref<'catalog' | 'bookmarks'>('catalog')
const keyword = ref('')

const readUpToIndex = computed(() => props.chapters.findIndex((chapter) => chapter.id === props.readUpToChapterId))

function status(index: number): string {
  const chapter = props.chapters[index]
  if (!chapter) return ''
  if (chapter.id === props.selectedChapter?.id) return `${props.currentPercent}%`
  return readUpToIndex.value >= 0 && index < readUpToIndex.value ? '已读' : ''
}

/** Group by the 卷/部/篇 prefix when a title carries one; otherwise 「正文」. */
const groups = computed(() => {
  const needle = keyword.value.trim().toLowerCase()
  const buckets = new Map<string, { index: number; chapter: Chapter }[]>()
  props.chapters.forEach((chapter, index) => {
    if (needle && !chapter.title.toLowerCase().includes(needle)) return
    const label = volumeLabel(chapter.title)
    const bucket = buckets.get(label)
    if (bucket) bucket.push({ index, chapter })
    else buckets.set(label, [{ index, chapter }])
  })
  return [...buckets.entries()].map(([label, items]) => ({ label, items }))
})
</script>

<template>
  <aside class="chapter-list">
    <div class="reader-side-tabs">
      <button type="button" :class="{ active: activeTab === 'catalog' }" @click="activeTab = 'catalog'">
        <List :size="15" />目录
      </button>
      <button type="button" :class="{ active: activeTab === 'bookmarks' }" @click="activeTab = 'bookmarks'">
        <Bookmark :size="15" />书签
      </button>
    </div>

    <template v-if="activeTab === 'catalog'">
      <Input v-model="keyword" class="mb-3 h-8" placeholder="搜索章节" aria-label="搜索章节" />
      <div v-for="group in groups" :key="group.label" class="mb-3">
        <div class="reader-toc-volume">{{ group.label }}</div>
        <button
          v-for="item in group.items"
          :key="item.chapter.id"
          type="button"
          :class="['chapter-button', { selected: selectedChapter?.id === item.chapter.id }]"
          @click="emit('selectChapter', item.chapter)"
        >
          <span>{{ item.chapter.title }}</span>
          <span v-if="status(item.index)" class="chapter-status">{{ status(item.index) }}</span>
        </button>
      </div>
      <p v-if="!groups.length" class="px-2 py-6 text-center text-xs text-muted-foreground">没有匹配的章节。</p>
    </template>

    <div v-else class="bookmark-panel">
      <button v-if="hasBookmark" type="button" class="bookmark-item" @click="emit('jump')">
        <Bookmark :size="16" />{{ selectedChapter?.title }}<span>跳转</span>
      </button>
      <p v-else class="bookmark-empty">当前章节还没有书签。</p>
    </div>
  </aside>
</template>

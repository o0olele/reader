<script setup lang="ts">
import { computed, ref } from 'vue'
import { Bookmark, List } from 'lucide-vue-next'
import { Input } from '@/components/ui/input'
import type { Chapter } from '@/services/api'

const props = defineProps<{
  chapters: Chapter[]
  selectedChapter?: Chapter
  hasBookmark: boolean
}>()

const emit = defineEmits<{ selectChapter: [chapter: Chapter]; jump: [] }>()

const activeTab = ref<'catalog' | 'bookmarks'>('catalog')
const keyword = ref('')

/** Group by the 卷/部/篇 prefix when a title carries one; otherwise 「正文」. */
const groups = computed(() => {
  const needle = keyword.value.trim().toLowerCase()
  const filtered = needle
    ? props.chapters.filter((chapter) => chapter.title.toLowerCase().includes(needle))
    : props.chapters
  const buckets = new Map<string, Chapter[]>()
  for (const chapter of filtered) {
    const match = /^(第[零一二三四五六七八九十百千万0-9]+[卷部篇])/.exec(chapter.title.trim())
    const label = match ? match[1] : '正文'
    const bucket = buckets.get(label)
    if (bucket) bucket.push(chapter)
    else buckets.set(label, [chapter])
  }
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
        <div class="mb-1 px-1 text-[11px] font-medium tracking-wide text-muted-foreground">{{ group.label }}</div>
        <button
          v-for="chapter in group.items"
          :key="chapter.id"
          type="button"
          :class="['chapter-button', { selected: selectedChapter?.id === chapter.id }]"
          @click="emit('selectChapter', chapter)"
        >
          {{ chapter.title }}
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

<script setup lang="ts">
import { computed } from 'vue'
import { Regex, Search, Square, X } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Progress } from '@/components/ui/progress'
import type { SearchContentHit } from '@/services/api'
import SearchContentHitRow from './SearchContentHitRow.vue'
import type { useContentSearch } from './useContentSearch'

const props = defineProps<{
  /** The reader's search state, read-only here; edits go back out as events. */
  search: ReturnType<typeof useContentSearch>
  currentChapterId?: number
}>()

const emit = defineEmits<{
  'update:query': [value: string]
  'update:regex': [value: boolean]
  'update:scope': [value: 'book' | 'chapter']
  jump: [hit: SearchContentHit]
  stop: []
  close: []
}>()

const percent = computed(() => {
  const total = props.search.progress?.total ?? 0
  return total ? Math.round(((props.search.progress?.scanned ?? 0) / total) * 100) : 0
})

/** Read-only alias so the template stays readable. */
const state = props.search
</script>

<template>
  <aside class="reader-search">
    <div class="reader-search-head">
      <strong class="text-sm">正文搜索</strong>
      <Button variant="ghost" size="icon-sm" aria-label="关闭正文搜索" @click="emit('close')"><X /></Button>
    </div>

    <div class="reader-search-box">
      <Search :size="15" class="shrink-0 text-muted-foreground" />
      <input
        class="reader-search-input"
        type="search"
        :value="state.query"
        placeholder="搜索本书正文"
        aria-label="搜索本书正文"
        autofocus
        @input="emit('update:query', ($event.target as HTMLInputElement).value)"
        @keydown.escape="emit('close')"
      />
      <Button v-if="state.searching" variant="ghost" size="icon-sm" title="停止搜索" @click="emit('stop')">
        <Square />
      </Button>
      <Button v-else-if="state.query" variant="ghost" size="icon-sm" title="清空" @click="emit('update:query', '')">
        <X />
      </Button>
    </div>

    <div class="reader-search-meta">
      <span class="reader-search-count">
        <template v-if="state.searching">
          已扫描 {{ state.progress?.scanned ?? 0 }}/{{ state.progress?.total ?? 0 }} 章
        </template>
        <template v-else-if="state.query.trim()">共 {{ state.visibleHits.length }} 条结果</template>
        <template v-else>输入关键词后自动搜索</template>
      </span>
      <button
        type="button"
        :class="['search-toggle', { active: state.regex }]"
        title="正则表达式"
        @click="emit('update:regex', !state.regex)"
      >
        <Regex :size="13" />正则
      </button>
      <button
        type="button"
        :class="['search-toggle', { active: state.scope === 'chapter' }]"
        :disabled="currentChapterId === undefined"
        title="只看当前章节"
        @click="emit('update:scope', state.scope === 'chapter' ? 'book' : 'chapter')"
      >
        仅本章 {{ state.currentChapterHits }}
      </button>
    </div>

    <Progress v-if="state.searching" :model-value="percent" class="mx-4 h-1" />

    <p v-if="state.error" role="alert" class="px-4 py-2 text-xs text-destructive">{{ state.error }}</p>
    <p v-else-if="state.truncated" class="px-4 py-2 text-[11px] text-muted-foreground">
      命中过多，只列出前 {{ state.visibleHits.length }} 条。
    </p>

    <p v-if="!state.query.trim()" class="reader-search-empty">搜索本书已缓存正文的章节，点击结果即可跳转并高亮。</p>
    <p v-else-if="!state.searching && !state.visibleHits.length && !state.error" class="reader-search-empty">
      没有找到相关内容。只有已缓存正文的章节会被搜索。
    </p>

    <div class="reader-search-list">
      <SearchContentHitRow
        v-for="hit in state.visibleHits"
        :key="`${hit.chapter_id}-${hit.result_index}`"
        :hit="hit"
        :current="hit.chapter_id === currentChapterId"
        @jump="emit('jump', $event)"
      />
    </div>

    <p v-if="state.query.trim()" class="reader-search-foot">
      已扫描 {{ state.searchedChapters }}/{{ state.totalChapters }} 章，其中 {{ state.visibleHits.length }} 条命中
    </p>
  </aside>
</template>

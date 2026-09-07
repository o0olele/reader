<script setup lang="ts">
import { computed, inject, ref } from 'vue'
import { searchKey, sourcesKey } from '../../app/shellKeys'
import type { SearchResultGroup } from '../../services/api'
import { Check, Filter, LayoutList, Pause, Play, Search, SlidersHorizontal, X } from 'lucide-vue-next'

const search = inject(searchKey)!
const sources = inject(sourcesKey)!
const layout = ref<'list' | 'compact'>('list')
const showSettings = ref(false)
const sourceFilterOpen = ref(false)
type SearchType = 'all' | 'novel' | 'comic' | 'audio'
const typeFilter = ref<SearchType>('all')
const sourceGroup = ref('全部书源')

search.setSourceIdsProvider(() =>
  sources.sources
    .filter(
      (source) =>
        source.enabled && (sourceGroup.value === '全部书源' || (source.source_group || '未分组') === sourceGroup.value),
    )
    .map((source) => source.id),
)

const sourceGroups = computed(() => {
  const groups = new Set(
    sources.sources.filter((source) => source.enabled).map((source) => source.source_group || '未分组'),
  )
  return ['全部书源', ...groups]
})
function selectSourceGroup(group: string) {
  sourceGroup.value = group
  search.selectedSourceIds = null
}
const enabledSourceIds = computed(() => sources.sources.filter((source) => source.enabled).map((source) => source.id))
const selectedSourceCount = computed(() =>
  search.selectedSourceIds === null ? enabledSourceIds.value.length : search.selectedSourceIds.length,
)
const matchesType = (kind: string | undefined, type: SearchType) => {
  if (type === 'all') return true
  const value = (kind || '').toLowerCase()
  if (type === 'comic') return value.includes('漫画') || value.includes('comic') || value.includes('manhua')
  if (type === 'audio') return value.includes('音频') || value.includes('有声') || value.includes('audio')
  return (
    !value || value.includes('小说') || value.includes('novel') || (!value.includes('漫画') && !value.includes('音频'))
  )
}
const visibleGroups = computed(() =>
  search.groups.filter((group) => {
    if (!group.sources.some((source) => matchesType(source.kind, typeFilter.value))) return false
    if (
      sourceGroup.value !== '全部书源' &&
      !group.sources.some(
        (source) => sources.sources.find((item) => item.id === source.source_id)?.source_group === sourceGroup.value,
      )
    )
      return false
    return true
  }),
)

const isAdding = (group: SearchResultGroup) => group.sources.some((source) => source.url === search.addingResult)

const canOpenBrowserAuth = (reason: string, authRequired: boolean) =>
  authRequired || reason.includes('Cloudflare challenge') || reason.includes('需要浏览器执行 JavaScript 验证')

function openBrowserAuth(sourceId: number) {
  const source = sources.sources.find((item) => item.id === sourceId)
  if (source) void sources.browserAuth(source)
}
</script>

<template>
  <div class="search-results">
    <form class="search-page-form" @submit.prevent="search.run()">
      <div class="search-page-input">
        <Search :size="17" /><input
          v-model="search.query"
          placeholder="搜索书名、作者或关键词"
          aria-label="搜索书籍"
        /><button
          v-if="search.query"
          type="button"
          class="search-clear"
          aria-label="清空搜索"
          @click="search.query = ''"
        >
          <X :size="15" /></button
        ><button type="submit">{{ search.searching ? '重新搜索' : '搜索' }}</button>
      </div>
    </form>
    <div class="search-toolbar">
      <div class="search-progress" v-if="search.searching || search.hasSearched">
        <span
          >结果 {{ search.groups.length }} · 已搜索 {{ search.completedSources }}/{{
            search.totalSources || enabledSourceIds.length
          }}
          个书源</span
        >
        <i
          ><b
            :style="{ width: `${search.totalSources ? (search.completedSources / search.totalSources) * 100 : 0}%` }"
          ></b
        ></i>
        <button
          v-if="search.searching"
          type="button"
          class="icon-button"
          :title="search.paused ? '继续' : '暂停'"
          @click="search.paused ? search.resume() : search.pause()"
        >
          <Play v-if="search.paused" :size="15" /><Pause v-else :size="15" />
        </button>
      </div>
      <button type="button" class="toolbar-button" @click="showSettings = !showSettings">
        <SlidersHorizontal :size="15" />设置
      </button>
      <button type="button" class="toolbar-button" @click="sourceFilterOpen = !sourceFilterOpen">
        <Filter :size="15" />书源筛选（{{ selectedSourceCount }}）
      </button>
      <div class="layout-switch">
        <button :class="{ active: layout === 'list' }" @click="layout = 'list'"><LayoutList :size="15" /></button
        ><button :class="{ active: layout === 'compact' }" @click="layout = 'compact'"><Check :size="15" /></button>
      </div>
    </div>
    <div v-if="showSettings" class="search-settings-panel">
      <strong>搜索类型</strong>
      <button
        v-for="item in [
          ['all', '全部'],
          ['novel', '小说'],
          ['comic', '漫画'],
          ['audio', '音频'],
        ]"
        :key="item[0]"
        :class="{ active: typeFilter === item[0] }"
        @click="typeFilter = item[0] as 'all' | 'novel' | 'comic' | 'audio'"
      >
        {{ item[1] }}
      </button>
    </div>
    <div v-if="sourceFilterOpen" class="search-settings-panel">
      <strong>选择书源分组</strong>
      <button
        v-for="group in sourceGroups"
        :key="group"
        :class="{ active: sourceGroup === group }"
        @click="selectSourceGroup(group)"
      >
        {{ group }}
      </button>
      <span class="search-settings-divider"></span>
      <strong>书源</strong>
      <button
        type="button"
        :class="{ active: search.selectedSourceIds === null }"
        @click="search.selectedSourceIds = null"
      >
        全部
      </button>
      <button
        v-for="source in sources.sources.filter((item) => item.enabled)"
        :key="source.id"
        type="button"
        :class="{ active: search.selectedSourceIds === null || search.selectedSourceIds.includes(source.id) }"
        @click="search.toggleSource(source.id, enabledSourceIds)"
      >
        {{ source.name }}
      </button>
    </div>
    <details v-if="search.failures.length" class="source-failures">
      <summary>
        {{ search.searchedSources - search.failures.length }} / {{ search.searchedSources }} 个书源返回结果，{{
          search.failures.length
        }}
        个失败
      </summary>
      <ul>
        <li v-for="failure in search.failures" :key="failure.source_id">
          <strong>{{ failure.source_name }}</strong
          >：{{ failure.reason }}<span v-if="failure.auth_required">（需要重新认证）</span>
          <button
            v-if="
              canOpenBrowserAuth(failure.reason, failure.auth_required) &&
              sources.sources.some((item) => item.id === failure.source_id)
            "
            type="button"
            class="secondary"
            @click="openBrowserAuth(failure.source_id)"
          >
            打开认证窗口
          </button>
        </li>
      </ul>
    </details>

    <div v-if="search.searching && !search.groups.length" class="search-empty">正在搜索，结果会即时显示...</div>
    <div v-else-if="!search.searching && !search.hasSearched" class="search-empty">输入关键词后开始搜索</div>
    <div v-else-if="!visibleGroups.length" class="search-empty">没有符合当前筛选条件的结果</div>

    <article
      v-for="group in visibleGroups"
      :key="`${group.title}-${group.author ?? ''}`"
      :class="['search-result', { 'search-result-compact': layout === 'compact' }]"
    >
      <div class="book-cover">
        <img v-if="group.cover" :src="group.cover" :alt="group.title" loading="lazy" />
        <template v-else>{{ group.title.slice(0, 1) }}</template>
      </div>
      <div class="search-result-meta">
        <h2>{{ group.title }}</h2>
        <p>
          {{ group.author || '作者未知' }}
          <span v-if="group.sources.length > 1"> · {{ group.sources.length }} 个书源</span>
          <span v-else> · {{ group.sources[0].source_name }}</span>
        </p>
        <p
          v-if="group.sources[0].latest_chapter || group.sources[0].word_count || group.sources[0].kind"
          class="search-result-tags"
        >
          <span v-if="group.sources[0].kind">{{ group.sources[0].kind }}</span>
          <span v-if="group.sources[0].word_count">{{ group.sources[0].word_count }}</span>
          <span v-if="group.sources[0].latest_chapter">最新：{{ group.sources[0].latest_chapter }}</span>
        </p>
        <p v-if="group.sources[0].intro" class="search-result-intro">{{ group.sources[0].intro }}</p>
        <details v-if="group.sources.length > 1" class="search-result-sources">
          <summary>按书源选择</summary>
          <button
            v-for="source in group.sources"
            :key="`${source.source_id}-${source.url}`"
            type="button"
            class="secondary"
            :disabled="search.addingResult === source.url"
            @click="search.addToShelf(source)"
          >
            {{ source.source_name }}
          </button>
        </details>
        <button
          type="button"
          class="secondary"
          :disabled="isAdding(group)"
          @click="search.addToShelf(group.sources[0])"
        >
          {{ isAdding(group) ? '加入中...' : '加入书架' }}
        </button>
      </div>
    </article>
  </div>
</template>

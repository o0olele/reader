<script setup lang="ts">
import { inject } from 'vue'
import { searchKey, sourcesKey } from '../../app/shellKeys'
import type { SearchResultGroup } from '../../services/api'
import SourceManager from '../source/SourceManager.vue'

const search = inject(searchKey)!
const sources = inject(sourcesKey)!

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
    <SourceManager :query="search.query" />

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
            v-if="canOpenBrowserAuth(failure.reason, failure.auth_required) && sources.sources.some((item) => item.id === failure.source_id)"
            type="button"
            class="secondary"
            @click="openBrowserAuth(failure.source_id)"
          >
            打开认证窗口
          </button>
        </li>
      </ul>
    </details>

    <div v-if="search.searching" class="search-empty">正在搜索...</div>
    <div v-else-if="!search.hasSearched" class="search-empty">输入关键词后开始搜索</div>
    <div v-else-if="!search.groups.length" class="search-empty">没有搜到结果</div>

    <article v-for="group in search.groups" :key="`${group.title}-${group.author ?? ''}`" class="search-result">
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

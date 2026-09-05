<script setup lang="ts">
import { inject } from 'vue'
import { searchKey } from '../../app/shellKeys'

const search = inject(searchKey)!
</script>

<template>
  <section class="search-results explore-panel">
    <div class="explore-heading">
      <p class="explore-hint">选择书源分类浏览书籍</p>
      <button type="button" class="secondary" @click="search.loadExplore()">刷新分类</button>
    </div>

    <div v-if="search.exploreCategories.length" class="explore-categories">
      <button
        v-for="category in search.exploreCategories"
        :key="`${category.source_id}-${category.url}`"
        type="button"
        class="secondary"
        :class="{
          active:
            search.selectedExplore?.url === category.url && search.selectedExplore?.source_id === category.source_id,
        }"
        @click="search.runExplore(category)"
      >
        {{ category.source_name }} · {{ category.title }}
      </button>
    </div>

    <div v-else class="search-empty">暂无可用的发现分类</div>
    <div v-if="search.exploring" class="search-empty">正在加载发现页...</div>
    <div v-else-if="search.selectedExplore && !search.exploreResults.length" class="search-empty">该分类没有结果</div>

    <article v-for="result in search.exploreResults" :key="`${result.source_id}-${result.url}`" class="search-result">
      <div class="book-cover">
        <img v-if="result.cover" :src="result.cover" :alt="result.title" loading="lazy" />
        <template v-else>{{ result.title.slice(0, 1) }}</template>
      </div>
      <div class="search-result-meta">
        <h2>{{ result.title }}</h2>
        <p>
          {{ result.author || '作者未知' }} · {{ result.source_name }}
          <span v-if="result.kind"> · {{ result.kind }}</span>
          <span v-if="result.latest_chapter"> · {{ result.latest_chapter }}</span>
        </p>
        <p v-if="result.intro" class="search-result-intro">{{ result.intro }}</p>
        <button
          type="button"
          class="secondary"
          :disabled="search.addingResult === result.url"
          @click="search.addToShelf(result)"
        >
          {{ search.addingResult === result.url ? '加入中...' : '加入书架' }}
        </button>
      </div>
    </article>
  </section>
</template>

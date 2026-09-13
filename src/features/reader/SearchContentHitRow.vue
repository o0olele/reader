<script setup lang="ts">
import { computed } from 'vue'
import type { SearchContentHit } from '@/services/api'

const props = defineProps<{ hit: SearchContentHit; current: boolean }>()
const emit = defineEmits<{ jump: [hit: SearchContentHit] }>()

/** The snippet with the match cut out, so only the hit gets a `<mark>`. */
const parts = computed(() => {
  const { snippet, snippet_offset: offset, snippet_length: length } = props.hit
  const start = Math.max(0, Math.min(offset, snippet.length))
  const end = Math.max(start, Math.min(start + length, snippet.length))
  if (end <= start) return undefined
  return [snippet.slice(0, start), snippet.slice(start, end), snippet.slice(end)]
})
</script>

<template>
  <button type="button" :class="['search-hit', { current }]" @click="emit('jump', hit)">
    <span class="search-hit-head">
      <span class="search-hit-title">{{ hit.chapter_title }}</span>
      <span v-if="hit.in_title" class="search-hit-tag">标题</span>
      <span v-if="current" class="search-hit-tag">当前章节</span>
      <span class="search-hit-percent">{{ hit.progress_percent.toFixed(1) }}%</span>
    </span>
    <span class="search-hit-snippet">
      <template v-if="parts"
        >{{ parts[0] }}<mark>{{ parts[1] }}</mark
        >{{ parts[2] }}</template
      >
      <template v-else>{{ hit.snippet }}</template>
    </span>
  </button>
</template>

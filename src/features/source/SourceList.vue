<script setup lang="ts">
import SourceRow from './SourceRow.vue'
import { useWindowedList } from '@/lib/useWindowedList'
import type { BookSource } from '@/services/api'

const props = defineProps<{ sources: BookSource[]; query: string }>()

/**
 * 500+ sources used to mount 500+ rows at once — every row carries three inputs,
 * a switch and a Reka dropdown, so the first paint cost thousands of nodes.
 * `useWindowedList` mounts only the rows around the viewport.
 */
const { viewport, trackHeight, visible, offset, syncScroll } = useWindowedList(() => props.sources, {
  rowSelector: '[data-source-row]',
  fallbackRowHeight: 56,
})
</script>

<template>
  <div class="flex min-h-0 flex-1 flex-col">
    <div v-if="sources.length" ref="viewport" class="min-h-0 flex-1 overflow-y-auto" @scroll.passive="syncScroll">
      <div class="relative" :style="{ height: `${trackHeight}px` }">
        <div class="absolute inset-x-0 top-0 grid gap-2" :style="{ transform: `translateY(${offset}px)` }">
          <SourceRow v-for="source in visible" :key="source.id" :source="source" :query="query" />
        </div>
      </div>
    </div>
    <p v-else class="rounded-md border border-dashed p-8 text-center text-xs text-muted-foreground">
      还没有书源。导入 legado JSON 或手动新增一个。
    </p>
  </div>
</template>

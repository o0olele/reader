<script setup lang="ts">
import { computed, nextTick, ref, watch } from 'vue'
import SourceRow from './SourceRow.vue'
import type { BookSource } from '@/services/api'

const props = defineProps<{ sources: BookSource[]; query: string }>()

/**
 * 500+ sources used to mount 500+ rows at once — every row carries three inputs,
 * a switch and a Reka dropdown, so the first paint cost thousands of nodes.
 * Only the rows around the viewport are mounted now.
 *
 * Rows share one box model, which makes the window pure arithmetic: one measured
 * row height plus the grid gap, no per-row measurement bookkeeping.
 */
const GAP = 8
const OVERSCAN = 8
const FALLBACK_ROW_HEIGHT = 56
const ROW_SELECTOR = '[data-source-row]'

const viewport = ref<HTMLElement>()
const scrollTop = ref(0)
const viewportHeight = ref(0)
const rowHeight = ref(FALLBACK_ROW_HEIGHT)

const stride = computed(() => rowHeight.value + GAP)
const trackHeight = computed(() => (props.sources.length ? props.sources.length * stride.value - GAP : 0))
const firstIndex = computed(() => Math.max(0, Math.floor(scrollTop.value / stride.value) - OVERSCAN))
const lastIndex = computed(() =>
  Math.min(props.sources.length, Math.ceil((scrollTop.value + viewportHeight.value) / stride.value) + OVERSCAN),
)
const visible = computed(() => props.sources.slice(firstIndex.value, lastIndex.value))
const offset = computed(() => firstIndex.value * stride.value)

/** Scroll handler: the hot path, so it only reads the one value the window needs. */
function syncScroll() {
  const element = viewport.value
  if (element) scrollTop.value = element.scrollTop
}

/** A mounted row is the ruler for every row, and the box decides how many fit. */
function measure() {
  const element = viewport.value
  if (!element) return
  viewportHeight.value = element.clientHeight
  const row = element.querySelector<HTMLElement>(ROW_SELECTOR)
  if (row?.offsetHeight) rowHeight.value = row.offsetHeight
}

// The scroller only exists once there is something to scroll, which can happen
// long after mount (the first refresh resolving), so bind on the ref instead.
watch(
  viewport,
  (element, _previous, onCleanup) => {
    if (!element) return
    syncScroll()
    measure()
    const observer = new ResizeObserver(measure)
    observer.observe(element)
    onCleanup(() => observer.disconnect())
  },
  { flush: 'post' },
)

// A new result set (filter, import, refresh) starts at the top — otherwise the
// window can sit past the end of a shorter list.
watch(
  () => props.sources,
  () => {
    scrollTop.value = 0
    if (viewport.value) viewport.value.scrollTop = 0
    void nextTick(measure)
  },
)
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

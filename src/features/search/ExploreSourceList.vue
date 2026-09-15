<script setup lang="ts">
import { computed, ref } from 'vue'
import { Input } from '@/components/ui/input'
import { useWindowedList } from '@/lib/useWindowedList'
import type { BookSource } from '@/services/api'

const props = defineProps<{ sources: BookSource[]; activeId: number | null }>()
const emit = defineEmits<{ select: [source: BookSource] }>()

/**
 * The aside used to mount a button per discover-capable source, which at several
 * hundred sources is the whole first-paint cost of the page. The list is windowed
 * now, and the removed 全部书源 entry is replaced by this filter box.
 */
const keyword = ref('')
const matches = computed(() => {
  const needle = keyword.value.trim().toLowerCase()
  if (!needle) return props.sources
  return props.sources.filter((source) =>
    `${source.name} ${source.source_group ?? ''} ${source.base_url}`.toLowerCase().includes(needle),
  )
})

const { viewport, trackHeight, visible, offset, syncScroll } = useWindowedList(() => matches.value, {
  rowSelector: '[data-explore-source]',
  gap: 2,
  overscan: 6,
  fallbackRowHeight: 32,
})
</script>

<template>
  <div class="flex min-h-0 flex-1 flex-col gap-2 p-2">
    <Input v-model="keyword" class="h-8 text-xs" placeholder="搜索书源" aria-label="搜索书源" />

    <div v-if="matches.length" ref="viewport" class="min-h-0 flex-1 overflow-y-auto" @scroll.passive="syncScroll">
      <div class="relative" :style="{ height: `${trackHeight}px` }">
        <div class="absolute inset-x-0 top-0 grid gap-0.5" :style="{ transform: `translateY(${offset}px)` }">
          <button
            v-for="source in visible"
            :key="source.id"
            data-explore-source
            type="button"
            class="flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-left text-sm hover:bg-accent"
            :class="activeId === source.id ? 'bg-accent font-medium' : ''"
            @click="emit('select', source)"
          >
            <span class="size-2 shrink-0 rounded-full" :class="source.enabled ? 'bg-primary' : 'bg-muted-foreground'" />
            <span class="truncate">{{ source.name }}</span>
          </button>
        </div>
      </div>
    </div>

    <p v-else class="p-2 text-xs text-muted-foreground">
      {{ sources.length ? '没有匹配的书源。' : '还没有启用「发现」的书源。' }}
    </p>
  </div>
</template>

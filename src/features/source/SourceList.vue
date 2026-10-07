<script setup lang="ts">
import { Library, SearchX, Upload } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import SourceRow from './SourceRow.vue'
import { useWindowedList } from '@/lib/useWindowedList'
import type { BookSource } from '@/services/api'

const props = defineProps<{ sources: BookSource[]; query: string; hasSources: boolean }>()
const emit = defineEmits<{ import: []; reset: []; manage: [source: BookSource] }>()
const { viewport, trackHeight, visible, offset, syncScroll } = useWindowedList(() => props.sources, {
  rowSelector: '[data-source-row]',
  fallbackRowHeight: 76,
  gap: 0,
})
</script>

<template>
  <div class="flex min-h-0 flex-1 flex-col overflow-hidden rounded-xl border bg-card shadow-sm">
    <template v-if="sources.length">
      <div
        class="source-grid shrink-0 border-b bg-muted/30 px-4 py-3 text-[11px] font-medium text-muted-foreground"
        aria-hidden="true"
      >
        <span>书源名称 / 网址</span><span>分组</span><span>最近测试</span><span>启用状态</span
        ><span class="text-right">操作</span>
      </div>
      <div
        ref="viewport"
        class="min-h-0 flex-1 overflow-y-auto [scrollbar-gutter:stable]"
        aria-label="书源列表"
        @scroll.passive="syncScroll"
      >
        <div class="relative" :style="{ height: `${trackHeight}px` }">
          <div class="absolute inset-x-0 top-0" :style="{ transform: `translateY(${offset}px)` }">
            <SourceRow
              v-for="source in visible"
              :key="source.id"
              :source="source"
              :query="query"
              @manage="emit('manage', $event)"
            />
          </div>
        </div>
      </div>
    </template>
    <div v-else class="flex flex-1 flex-col items-center justify-center px-6 py-10 text-center">
      <span class="mb-4 grid size-14 place-items-center rounded-2xl bg-muted text-muted-foreground"
        ><SearchX v-if="hasSources" class="size-6" /><Library v-else class="size-6"
      /></span>
      <h2 class="text-sm font-semibold">{{ hasSources ? '没有符合条件的书源' : '添加你的第一个书源' }}</h2>
      <p class="mt-2 text-xs text-muted-foreground">
        {{
          hasSources
            ? '试试其他关键词、分组或状态，也可以清除所有筛选。'
            : '导入书源后，即可搜索书籍、浏览发现并开始阅读。'
        }}
      </p>
      <Button v-if="hasSources" class="mt-5" variant="outline" size="sm" @click="emit('reset')">清除筛选</Button>
      <Button v-else class="mt-5" size="sm" @click="emit('import')"><Upload />导入书源</Button>
    </div>
  </div>
</template>

<style>
.source-grid {
  display: grid;
  grid-template-columns: minmax(180px, 1fr) 110px 140px 90px 122px;
  gap: 16px;
}
@media (max-width: 1100px) {
  .source-grid {
    grid-template-columns: minmax(130px, 1fr) 70px 100px 78px 112px;
    gap: 8px;
  }
}
</style>

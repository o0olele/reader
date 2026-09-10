<script setup lang="ts">
import { computed } from 'vue'
import { Loader2, Plus } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import type { BookSearchResult, SearchResultGroup } from '@/services/api'

const props = defineProps<{
  group: SearchResultGroup
  layout: 'list' | 'compact'
  adding: boolean
  busyUrl?: string
}>()

const emit = defineEmits<{ add: [result: BookSearchResult] }>()

const primary = computed(() => props.group.sources[0])
const tags = computed(() => [primary.value?.kind, primary.value?.word_count].filter(Boolean) as string[])
</script>

<template>
  <article
    class="flex gap-3 rounded-lg border bg-card p-3"
    :class="layout === 'compact' ? 'items-center' : 'items-start'"
  >
    <div
      class="grid shrink-0 place-items-center overflow-hidden rounded-md bg-secondary text-xs text-muted-foreground"
      :class="layout === 'compact' ? 'h-12 w-9' : 'h-[104px] w-[72px]'"
    >
      <img v-if="group.cover" :src="group.cover" :alt="group.title" loading="lazy" class="h-full w-full object-cover" />
      <span v-else>{{ group.title.slice(0, 1) }}</span>
    </div>

    <div class="min-w-0 flex-1">
      <h2 class="truncate text-sm font-semibold">{{ group.title }}</h2>
      <p class="mt-0.5 truncate text-xs text-muted-foreground">
        {{ group.author || '作者未知' }} ·
        {{ group.sources.length > 1 ? `${group.sources.length} 个书源` : primary?.source_name }}
      </p>
      <p v-if="tags.length || primary?.latest_chapter" class="mt-1.5 flex flex-wrap gap-1.5 text-[10px]">
        <span v-for="tag in tags" :key="tag" class="rounded bg-muted px-1.5 py-0.5 text-muted-foreground">{{
          tag
        }}</span>
        <span v-if="primary?.latest_chapter" class="rounded bg-muted px-1.5 py-0.5 text-muted-foreground">
          最新：{{ primary.latest_chapter }}
        </span>
      </p>
      <p v-if="layout === 'list' && primary?.intro" class="mt-1.5 line-clamp-2 text-xs text-muted-foreground">
        {{ primary.intro }}
      </p>

      <div class="mt-2 flex flex-wrap items-center gap-2">
        <Button size="sm" :disabled="adding" @click="emit('add', primary)">
          <Loader2 v-if="adding" class="animate-spin" /><Plus v-else />{{ adding ? '加入中…' : '加入书架' }}
        </Button>
        <details v-if="group.sources.length > 1" class="text-xs">
          <summary class="cursor-pointer rounded-md px-2 py-1.5 hover:bg-accent">按书源选择</summary>
          <div class="mt-2 flex flex-wrap gap-1.5">
            <Button
              v-for="source in group.sources"
              :key="`${source.source_id}-${source.url}`"
              variant="outline"
              size="sm"
              :disabled="busyUrl === source.url"
              @click="emit('add', source)"
            >
              {{ source.source_name }}
            </Button>
          </div>
        </details>
      </div>
    </div>
  </article>
</template>

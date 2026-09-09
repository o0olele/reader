<script setup lang="ts">
import { computed, ref } from 'vue'
import { Filter, LayoutList, Pause, Play, Rows3, Search, SlidersHorizontal, X } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Progress } from '@/components/ui/progress'
import { ToggleGroup, ToggleGroupItem } from '@/components/ui/toggle-group'
import PageBody from '@/components/PageBody.vue'
import PageHeader from '@/components/PageHeader.vue'
import SearchResultCard from './SearchResultCard.vue'
import { useSearchView } from './useSearchView'

const {
  search,
  sources,
  typeFilter,
  sourceGroup,
  sourceGroups,
  enabledSourceIds,
  selectedSourceCount,
  rankedGroups,
  selectSourceGroup,
  canOpenBrowserAuth,
  openBrowserAuth,
} = useSearchView()

const layout = ref<'list' | 'compact'>('list')
const showTypeFilter = ref(false)
const showSourceFilter = ref(false)
const TYPES = [
  { value: 'all', label: '全部' },
  { value: 'novel', label: '小说' },
  { value: 'comic', label: '漫画' },
  { value: 'audio', label: '音频' },
] as const

const progress = computed(() =>
  search.totalSources ? Math.round((search.completedSources / search.totalSources) * 100) : 0,
)
</script>

<template>
  <div class="flex h-full flex-col">
    <PageHeader title="搜索" :subtitle="`已启用 ${enabledSourceIds.length} 个书源`">
      <ToggleGroup
        type="single"
        :model-value="layout"
        variant="outline"
        size="sm"
        @update:model-value="layout = ($event as 'list' | 'compact') || layout"
      >
        <ToggleGroupItem value="list" aria-label="详细列表"><LayoutList /></ToggleGroupItem>
        <ToggleGroupItem value="compact" aria-label="紧凑列表"><Rows3 /></ToggleGroupItem>
      </ToggleGroup>
    </PageHeader>

    <form class="flex shrink-0 items-center gap-2 border-b bg-card px-6 py-2.5" @submit.prevent="search.run()">
      <Input v-model="search.query" class="h-9 max-w-xl" placeholder="搜索书名、作者或关键词" aria-label="搜索书籍" />
      <Button type="submit" size="sm">{{ search.searching ? '重新搜索' : '搜索' }}</Button>
      <Button v-if="search.query" variant="ghost" size="sm" @click="search.query = ''"><X /> 清空</Button>
    </form>

    <div class="flex shrink-0 flex-wrap items-center gap-2 border-b bg-card px-6 py-2">
      <div v-if="search.searching || search.hasSearched" class="flex items-center gap-2 text-xs text-muted-foreground">
        <span>
          结果 {{ search.groups.length }} · 已搜索 {{ search.completedSources }}/{{
            search.totalSources || enabledSourceIds.length
          }}
        </span>
        <Progress :model-value="progress" class="h-1 w-28" />
        <Button
          v-if="search.searching"
          variant="ghost"
          size="icon-sm"
          :title="search.paused ? '继续' : '暂停'"
          @click="search.paused ? search.resume() : search.pause()"
        >
          <Play v-if="search.paused" /><Pause v-else />
        </Button>
      </div>
      <Button variant="outline" size="sm" @click="showTypeFilter = !showTypeFilter">
        <SlidersHorizontal /> 搜索类型
      </Button>
      <Button variant="outline" size="sm" @click="showSourceFilter = !showSourceFilter">
        <Filter /> 书源筛选（{{ selectedSourceCount }}）
      </Button>
    </div>

    <div v-if="showTypeFilter" class="flex shrink-0 items-center gap-1.5 border-b bg-card px-6 py-2">
      <Button
        v-for="item in TYPES"
        :key="item.value"
        :variant="typeFilter === item.value ? 'default' : 'outline'"
        size="sm"
        @click="typeFilter = item.value"
      >
        {{ item.label }}
      </Button>
    </div>

    <div v-if="showSourceFilter" class="flex shrink-0 flex-wrap items-center gap-1.5 border-b bg-card px-6 py-2">
      <Button
        v-for="group in sourceGroups"
        :key="group"
        :variant="sourceGroup === group ? 'default' : 'outline'"
        size="sm"
        @click="selectSourceGroup(group)"
      >
        {{ group }}
      </Button>
      <span class="mx-1 h-5 w-px bg-border" />
      <Button
        :variant="search.selectedSourceIds === null ? 'default' : 'outline'"
        size="sm"
        @click="search.selectedSourceIds = null"
      >
        全部
      </Button>
      <Button
        v-for="source in sources.sources.filter((item) => item.enabled)"
        :key="source.id"
        :variant="
          search.selectedSourceIds === null || search.selectedSourceIds.includes(source.id) ? 'default' : 'outline'
        "
        size="sm"
        @click="search.toggleSource(source.id, enabledSourceIds)"
      >
        {{ source.name }}
      </Button>
    </div>

    <PageBody>
      <details v-if="search.failures.length" class="mb-4 rounded-md border bg-card p-3 text-xs">
        <summary class="cursor-pointer">
          {{ search.searchedSources - search.failures.length }} / {{ search.searchedSources }} 个书源返回结果，{{
            search.failures.length
          }}
          个失败
        </summary>
        <ul class="mt-2 space-y-1">
          <li v-for="failure in search.failures" :key="failure.source_id" class="flex items-center gap-2">
            <strong>{{ failure.source_name }}</strong>
            <span class="text-muted-foreground">{{ failure.reason }}</span>
            <Button
              v-if="
                canOpenBrowserAuth(failure.reason, failure.auth_required) &&
                sources.sources.some((item) => item.id === failure.source_id)
              "
              variant="outline"
              size="sm"
              @click="openBrowserAuth(failure.source_id)"
            >
              打开认证窗口
            </Button>
          </li>
        </ul>
      </details>

      <div v-if="search.searching && !search.groups.length" class="py-12 text-center text-xs text-muted-foreground">
        正在搜索，结果会即时显示…
      </div>
      <div v-else-if="!search.searching && !search.hasSearched" class="py-12 text-center text-xs text-muted-foreground">
        <Search :size="28" class="mx-auto mb-2" />
        输入关键词后开始搜索，或用 Ctrl+K 直接搜索
      </div>
      <div v-else-if="!rankedGroups.length" class="py-12 text-center text-xs text-muted-foreground">
        没有符合当前筛选条件的结果
      </div>

      <div v-else class="grid gap-2">
        <SearchResultCard
          v-for="group in rankedGroups"
          :key="`${group.title}-${group.author ?? ''}`"
          :group="group"
          :layout="layout"
          :adding="group.sources.some((source) => source.url === search.addingResult)"
          :busy-url="search.addingResult"
          @add="search.addToShelf($event)"
        />
      </div>
    </PageBody>
  </div>
</template>

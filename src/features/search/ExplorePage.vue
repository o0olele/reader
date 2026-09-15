<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { Loader2, Plus, RefreshCw } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import PageBody from '@/components/PageBody.vue'
import PageHeader from '@/components/PageHeader.vue'
import ExploreSourceList from './ExploreSourceList.vue'
import { useShellContext } from '@/app/shellKeys'

const { search, sources } = useShellContext()
const activeSourceId = ref<number | null>(null)

/**
 * A source only participates in 发现 when it actually carries an 发现页 URL:
 * legado imports default `enabledExplore` to true, so the flag alone would list
 * sources whose category bar can only ever be empty (`ExploreService::categories`
 * skips them for the same reason). Keeping them out also makes the subtitle and
 * the first-source default honest.
 */
const exploreSources = computed(() =>
  sources.sources.filter((source) => source.enabled && source.enabled_explore && !!source.explore_url?.trim()),
)
const categories = computed(() =>
  activeSourceId.value === null
    ? []
    : search.exploreCategories.filter((category) => category.source_id === activeSourceId.value),
)
const activeCategoryKey = computed(() =>
  search.selectedExplore ? `${search.selectedExplore.source_id}-${search.selectedExplore.url}` : '',
)
/** Switching source leaves the previous selection behind, so an empty result set
 *  only means “该分类没有结果” while the selection still belongs to this source. */
const pickedCategory = computed(() =>
  categories.value.some((category) => `${category.source_id}-${category.url}` === activeCategoryKey.value),
)
const results = computed(() =>
  activeSourceId.value === null
    ? []
    : search.exploreResults.filter((result) => result.source_id === activeSourceId.value),
)

/**
 * The page browses exactly one source now that 全部书源 is gone. That is also what
 * keeps it cheap: the category bar can no longer fan out into every category of
 * every source, which is where the first paint used to stall at 500+ sources.
 * The first available source is selected as soon as one exists.
 */
watch(
  exploreSources,
  (list) => {
    if (!list.length) {
      activeSourceId.value = null
      return
    }
    if (!list.some((source) => source.id === activeSourceId.value)) activeSourceId.value = list[0].id
  },
  { immediate: true },
)
</script>

<template>
  <div class="flex h-full flex-col">
    <PageHeader title="发现" :subtitle="`${exploreSources.length} 个书源参与发现`">
      <Button variant="outline" size="sm" @click="search.loadExplore()"><RefreshCw /> 刷新分类</Button>
    </PageHeader>

    <div class="flex min-h-0 flex-1">
      <aside class="flex w-56 shrink-0 flex-col border-r bg-card">
        <ExploreSourceList :sources="exploreSources" :active-id="activeSourceId" @select="activeSourceId = $event.id" />
      </aside>

      <div class="flex min-w-0 flex-1 flex-col">
        <div class="flex shrink-0 flex-wrap items-center gap-1.5 border-b bg-card px-4 py-2">
          <Button
            v-for="category in categories"
            :key="`${category.source_id}-${category.url}`"
            :variant="activeCategoryKey === `${category.source_id}-${category.url}` ? 'default' : 'outline'"
            size="sm"
            @click="search.runExplore(category)"
          >
            {{ category.title }}
          </Button>
          <span v-if="!categories.length" class="text-xs text-muted-foreground">暂无可用的发现分类</span>
        </div>

        <PageBody>
          <div v-if="search.exploring" class="py-12 text-center text-xs text-muted-foreground">正在加载发现页…</div>
          <div v-else-if="pickedCategory && !results.length" class="py-12 text-center text-xs text-muted-foreground">
            该分类没有结果
          </div>
          <div v-else-if="!results.length" class="py-12 text-center text-xs text-muted-foreground">
            从上方选择一个分类开始浏览
          </div>

          <div v-else class="grid gap-2">
            <article
              v-for="result in results"
              :key="`${result.source_id}-${result.url}`"
              class="flex items-start gap-3 rounded-lg border bg-card p-3"
            >
              <div
                class="grid h-[104px] w-[72px] shrink-0 place-items-center overflow-hidden rounded-md bg-secondary text-xs text-muted-foreground"
              >
                <img
                  v-if="result.cover"
                  :src="result.cover"
                  :alt="result.title"
                  loading="lazy"
                  class="h-full w-full object-cover"
                />
                <span v-else>{{ result.title.slice(0, 1) }}</span>
              </div>
              <div class="min-w-0 flex-1">
                <h2 class="truncate text-sm font-semibold">{{ result.title }}</h2>
                <p class="mt-0.5 truncate text-xs text-muted-foreground">
                  {{ result.author || '作者未知' }} · {{ result.source_name }}
                  <span v-if="result.kind"> · {{ result.kind }}</span>
                  <span v-if="result.latest_chapter"> · {{ result.latest_chapter }}</span>
                </p>
                <p v-if="result.intro" class="mt-1.5 line-clamp-2 text-xs text-muted-foreground">{{ result.intro }}</p>
                <Button
                  class="mt-2"
                  size="sm"
                  :disabled="search.addingResult === result.url"
                  @click="search.addToShelf(result)"
                >
                  <Loader2 v-if="search.addingResult === result.url" class="animate-spin" /><Plus v-else />
                  {{ search.addingResult === result.url ? '加入中…' : '加入书架' }}
                </Button>
              </div>
            </article>
          </div>
        </PageBody>
      </div>
    </div>
  </div>
</template>

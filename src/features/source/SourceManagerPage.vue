<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useRoute } from 'vue-router'
import { save } from '@tauri-apps/plugin-dialog'
import { Download, FlaskConical, Loader2, Plus, Upload } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover'
import PageBody from '@/components/PageBody.vue'
import PageHeader from '@/components/PageHeader.vue'
import SourceEditorDialog from './SourceEditorDialog.vue'
import SourceImportDialog from './SourceImportDialog.vue'
import SourceManagementDialog from './SourceManagementDialog.vue'
import SourceTestSummary from './SourceTestSummary.vue'
import SourceToolbar from './SourceToolbar.vue'
import SourceList from './SourceList.vue'
import { arrangeSources, probeNeedsAttention, type SourceFilter, type SourceSort } from './sourceView'
import { useShellContext } from '@/app/shellKeys'
import { notifyError } from '@/app/useToast'
import { getErrorMessage, type BookSource } from '@/services/api'

const route = useRoute()
const { sources } = useShellContext()
const editorOpen = ref(false)
const importOpen = ref(false)
const managementOpen = ref(false)
const managedSource = ref<BookSource>()
const keyword = ref(typeof route.query.q === 'string' ? route.query.q : '')
const filter = ref<SourceFilter>('all')
const group = ref('all')
const sort = ref<SourceSort>('default')
const probeQuery = ref('剑来')
const probeOpen = ref(false)
const counts = computed(() => ({
  all: sources.sources.length,
  enabled: sources.sources.filter((source) => source.enabled).length,
  disabled: sources.sources.filter((source) => !source.enabled).length,
  attention: sources.sources.filter((source) =>
    probeNeedsAttention(sources.testResults[source.id], sources.testErrors[source.id]),
  ).length,
}))
const groups = computed(() =>
  [
    ...new Set(sources.sources.map((source) => source.source_group?.trim()).filter((name): name is string => !!name)),
  ].sort((a, b) => a.localeCompare(b, 'zh-CN')),
)
const filtered = computed(() =>
  arrangeSources(
    sources.sources,
    sources.testResults,
    {
      keyword: keyword.value,
      filter: filter.value,
      group: group.value,
      sort: sort.value,
    },
    sources.testErrors,
  ),
)

function resetFilters() {
  keyword.value = ''
  filter.value = 'all'
  group.value = 'all'
}
// A command-palette deep link must not be hidden by an earlier status/group filter.
watch(
  () => route.query.q,
  (value) => {
    resetFilters()
    keyword.value = typeof value === 'string' ? value : ''
  },
)
function manage(source: BookSource) {
  managedSource.value = source
  managementOpen.value = true
}
function showAttention() {
  resetFilters()
  filter.value = 'attention'
}
function batchTest() {
  probeOpen.value = false
  void sources.batchTest(probeQuery.value.trim())
}
async function exportSources() {
  try {
    const target = await save({
      defaultPath: 'bookSources.json',
      filters: [{ name: 'Legado 书源', extensions: ['json'] }],
    })
    if (target) await sources.exportTo(target)
  } catch (cause) {
    notifyError(getErrorMessage(cause))
  }
}
</script>

<template>
  <div class="flex h-full flex-col">
    <PageHeader title="书源管理" :subtitle="`${counts.all} 个书源 · ${counts.enabled} 个已启用，随时发现下一本好书`">
      <Button variant="ghost" size="sm" :disabled="sources.exporting || !counts.all" @click="exportSources"
        ><Download />{{ sources.exporting ? '导出中…' : '导出' }}</Button
      >
      <Button variant="outline" size="sm" @click="editorOpen = true"><Plus />新增书源</Button>
      <Button size="sm" :disabled="sources.importing" @click="importOpen = true"
        ><Upload />{{ sources.importing ? '导入中…' : '导入书源' }}</Button
      >
    </PageHeader>
    <PageBody :scroll="false">
      <SourceToolbar
        v-model:keyword="keyword"
        v-model:filter="filter"
        v-model:group="group"
        v-model:sort="sort"
        :counts="counts"
        :groups="groups"
        :filtered-count="filtered.length"
      />
      <SourceTestSummary @attention="showAttention" />
      <SourceList
        :sources="filtered"
        :query="probeQuery.trim()"
        :has-sources="counts.all > 0"
        @import="importOpen = true"
        @reset="resetFilters"
        @manage="manage"
      />
      <div class="mt-3 flex shrink-0 items-center justify-between gap-3 text-xs text-muted-foreground">
        <span>启用的书源会参与搜索；测试结果仅供参考。</span>
        <Popover v-model:open="probeOpen">
          <PopoverTrigger as-child>
            <Button
              variant="outline"
              size="sm"
              :disabled="sources.batchTesting || sources.testing != null || !counts.all"
              ><Loader2 v-if="sources.batchTesting" class="animate-spin" /><FlaskConical v-else />{{
                sources.batchTesting ? '验证中…' : '批量验证'
              }}</Button
            >
          </PopoverTrigger>
          <PopoverContent align="end" class="w-80">
            <form class="space-y-3" @submit.prevent="batchTest">
              <div>
                <h2 class="text-sm font-semibold">测试书源搜索</h2>
                <p class="mt-1 text-xs leading-relaxed text-muted-foreground">
                  验证全部 {{ counts.enabled }} 个已启用书源，不受当前筛选影响。关键词也用于列表中的单项测试。
                </p>
              </div>
              <label class="grid gap-2 text-xs font-medium"
                >测试关键词<Input v-model="probeQuery" required placeholder="输入书名，例如：剑来"
              /></label>
              <Button type="submit" size="sm" class="w-full" :disabled="!probeQuery.trim() || !counts.enabled"
                >开始验证</Button
              >
            </form>
          </PopoverContent>
        </Popover>
      </div>
    </PageBody>
    <SourceImportDialog v-model:open="importOpen" />
    <SourceEditorDialog v-model:open="editorOpen" />
    <SourceManagementDialog v-if="managedSource" v-model:open="managementOpen" :source="managedSource" />
  </div>
</template>

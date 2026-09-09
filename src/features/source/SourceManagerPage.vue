<script setup lang="ts">
import { computed, ref } from 'vue'
import { save } from '@tauri-apps/plugin-dialog'
import { Download, Plus, RefreshCw, Upload } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import PageBody from '@/components/PageBody.vue'
import PageHeader from '@/components/PageHeader.vue'
import SourceEditorDialog from './SourceEditorDialog.vue'
import SourceRow from './SourceRow.vue'
import { useShellContext } from '@/app/shellKeys'

const { sources } = useShellContext()
const fileInput = ref<HTMLInputElement>()
const editorOpen = ref(false)
const keyword = ref('')
const probeQuery = ref('剑来')

const filtered = computed(() => {
  const needle = keyword.value.trim().toLowerCase()
  if (!needle) return sources.sources
  return sources.sources.filter((source) =>
    `${source.name} ${source.base_url} ${source.source_group ?? ''}`.toLowerCase().includes(needle),
  )
})
const badBatchResults = computed(
  () =>
    sources.batchResults.filter(
      (result) => result.status < 200 || result.status >= 400 || result.auth_required || result.cloudflare_challenge,
    ).length,
)

async function exportSources() {
  const target = await save({
    defaultPath: 'bookSources.json',
    filters: [{ name: 'Legado 书源', extensions: ['json'] }],
  })
  if (target) await sources.exportTo(target)
}
</script>

<template>
  <div class="flex h-full flex-col">
    <PageHeader
      title="书源管理"
      :subtitle="`${sources.sources.length} 个书源 · 批量校验 / 导入导出 / 登录 / 浏览器认证`"
    >
      <Button variant="outline" size="sm" :disabled="sources.importing" @click="fileInput?.click()">
        <Upload /> 导入 JSON
      </Button>
      <Button variant="outline" size="sm" :disabled="sources.exporting" @click="exportSources">
        <Download /> 导出 Legado
      </Button>
      <Button
        variant="outline"
        size="sm"
        :disabled="sources.batchTesting || !sources.sources.length"
        @click="sources.batchTest(probeQuery)"
      >
        <RefreshCw :class="sources.batchTesting ? 'animate-spin' : ''" />
        {{ sources.batchTesting ? '批量验证中…' : '批量验证' }}
      </Button>
      <Button size="sm" @click="editorOpen = true"><Plus /> 新增书源</Button>
    </PageHeader>

    <input
      ref="fileInput"
      class="hidden"
      type="file"
      accept=".json,application/json"
      @change="sources.importFromFile"
    />

    <PageBody>
      <div class="mb-4 flex flex-wrap items-center gap-2">
        <Input v-model="keyword" class="h-8 w-56" placeholder="筛选书源" aria-label="筛选书源" />
        <Input v-model="probeQuery" class="h-8 w-40" placeholder="探测关键词" aria-label="探测关键词" />
        <form class="flex items-center gap-2" @submit.prevent="sources.importFromUrl()">
          <Input
            v-model="sources.sourceUrl"
            class="h-8 w-72"
            placeholder="从 URL 导入书源 JSON"
            aria-label="书源 JSON URL"
          />
          <Button type="submit" variant="outline" size="sm" :disabled="sources.importing">从 URL 导入</Button>
        </form>
      </div>

      <div v-if="sources.lastProbe" class="mb-4 rounded-md border bg-card p-3 text-xs">
        <div class="mb-1 font-semibold">{{ sources.lastProbe.source_name }} 最近一次探测</div>
        <div class="flex flex-wrap gap-x-4 gap-y-1 text-muted-foreground">
          <span>HTTP {{ sources.lastProbe.status }}</span>
          <span>会话：{{ sources.lastProbe.session_state }}</span>
          <span>Cookie：{{ sources.lastProbe.has_cookie ? '已携带' : '未携带' }}</span>
          <span>Token：{{ sources.lastProbe.has_token ? '已携带' : '未携带' }}</span>
          <span>耗时：{{ sources.lastProbe.duration_ms }} ms</span>
          <span>解析：{{ sources.lastProbe.result_count }} 条</span>
          <span v-if="sources.lastProbe.cloudflare_challenge" class="text-destructive">需要完成 JavaScript 验证</span>
          <span v-else-if="sources.lastProbe.auth_required" class="text-destructive">需要重新认证</span>
        </div>
        <code class="mt-1 block truncate text-[11px] text-muted-foreground">{{ sources.lastProbe.request_url }}</code>
      </div>

      <div v-if="sources.batchResults.length" class="mb-4 rounded-md border bg-card p-3 text-xs">
        <div class="mb-1 font-semibold">
          批量验证：{{ sources.batchResults.length - badBatchResults }} 个可用 · {{ badBatchResults }} 个需处理
        </div>
        <div class="flex flex-wrap gap-x-3 gap-y-1">
          <span
            v-for="result in sources.batchResults"
            :key="result.source_id"
            :class="
              result.status >= 200 && result.status < 400 && !result.auth_required && !result.cloudflare_challenge
                ? 'text-muted-foreground'
                : 'text-destructive'
            "
          >
            {{ result.source_name }}：{{
              result.status ? `HTTP ${result.status} · ${result.result_count} 条` : '请求失败'
            }}
          </span>
        </div>
      </div>

      <div v-if="filtered.length" class="grid gap-2">
        <SourceRow v-for="source in filtered" :key="source.id" :source="source" :query="probeQuery" />
      </div>
      <p v-else class="rounded-md border border-dashed p-8 text-center text-xs text-muted-foreground">
        还没有书源。导入 legado JSON 或手动新增一个。
      </p>
    </PageBody>

    <SourceEditorDialog v-model:open="editorOpen" />
  </div>
</template>

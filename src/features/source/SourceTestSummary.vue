<script setup lang="ts">
import { computed } from 'vue'
import { CircleAlert, FlaskConical, Loader2, X } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { useShellContext } from '@/app/shellKeys'
import { probeLabel, probeNeedsAttention } from './sourceView'

defineEmits<{ attention: [] }>()
const { sources } = useShellContext()
const attentionCount = computed(
  () =>
    sources.sources.filter((source) =>
      probeNeedsAttention(sources.testResults[source.id], sources.testErrors[source.id]),
    ).length,
)
</script>

<template>
  <div
    v-if="sources.batchTesting"
    class="mb-4 flex shrink-0 items-center gap-3 rounded-lg border bg-card px-4 py-3 text-xs"
    role="status"
  >
    <Loader2 class="size-4 animate-spin" />
    <div>
      <p class="font-medium">正在验证全部已启用书源…</p>
      <p class="mt-1 text-muted-foreground">完成后将更新每个书源的测试结果，书源较多时可能需要一些时间。</p>
    </div>
  </div>
  <div
    v-else-if="sources.batchResults.length"
    class="mb-4 flex shrink-0 items-center gap-3 rounded-lg border bg-card px-4 py-3 text-xs"
    role="status"
  >
    <FlaskConical class="size-4 text-muted-foreground" />
    <div class="flex-1">
      <span class="font-medium">已完成 {{ sources.batchResults.length }} 个书源的批量验证</span
      ><span class="ml-3 text-muted-foreground">当前 {{ attentionCount }} 个需关注</span>
    </div>
    <Button v-if="attentionCount" variant="ghost" size="sm" @click="$emit('attention')">查看需关注</Button>
    <Button variant="ghost" size="icon-sm" aria-label="收起批量验证摘要" @click="sources.batchResults = []"
      ><X
    /></Button>
  </div>
  <details v-if="sources.lastProbe" class="mb-4 shrink-0 rounded-lg border bg-card text-xs">
    <summary class="cursor-pointer px-4 py-3">
      <span class="ml-1 inline-flex max-w-[90%] items-center gap-2 align-middle">
        <CircleAlert v-if="probeNeedsAttention(sources.lastProbe)" class="size-3.5 shrink-0 text-destructive" />
        <span class="truncate font-medium">{{ sources.lastProbe.source_name }}</span>
        <span class="shrink-0 text-muted-foreground"
          >{{ probeLabel(sources.lastProbe) }} · {{ sources.lastProbe.duration_ms }} ms</span
        >
      </span>
    </summary>
    <div class="border-t px-4 py-3 text-muted-foreground">
      <div class="flex flex-wrap gap-x-4 gap-y-1">
        <span>HTTP {{ sources.lastProbe.status }}</span
        ><span>会话：{{ sources.lastProbe.session_state }}</span
        ><span>Cookie：{{ sources.lastProbe.has_cookie ? '已携带' : '未携带' }}</span
        ><span>Token：{{ sources.lastProbe.has_token ? '已携带' : '未携带' }}</span>
      </div>
      <p class="mt-2 truncate font-mono text-[11px]" :title="sources.lastProbe.request_url">
        {{ sources.lastProbe.request_url }}
      </p>
      <p v-if="sources.lastProbe.result_count === 0" class="mt-2">
        没有搜索结果不一定代表书源失效，可换个关键词再次测试。
      </p>
    </div>
  </details>
</template>

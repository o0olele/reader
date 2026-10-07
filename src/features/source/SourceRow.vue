<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { CheckCircle2, CircleAlert, Globe, Loader2, MoreHorizontal, Settings2 } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Switch } from '@/components/ui/switch'
import { useShellContext } from '@/app/shellKeys'
import type { BookSource } from '@/services/api'
import { probeLabel, probeNeedsAttention, sessionLabel } from './sourceView'

const props = defineProps<{ source: BookSource; query: string }>()
const emit = defineEmits<{ manage: [source: BookSource] }>()
const { sources } = useShellContext()
const router = useRouter()
const result = computed(() => sources.testResults[props.source.id])
const error = computed(() => sources.testErrors[props.source.id])
const attention = computed(() => probeNeedsAttention(result.value, error.value))
const duration = computed(() => result.value?.duration_ms ?? props.source.respond_time)
</script>

<template>
  <div data-source-row class="source-grid h-[76px] items-center border-b px-4 transition-colors hover:bg-muted/40">
    <div class="flex min-w-0 items-center gap-3">
      <span class="grid size-9 shrink-0 place-items-center rounded-lg border bg-muted/40 text-muted-foreground"
        ><Globe class="size-4"
      /></span>
      <div class="min-w-0">
        <div class="flex items-center gap-2">
          <strong
            class="truncate text-sm font-medium"
            :class="source.enabled ? '' : 'text-muted-foreground'"
            :title="source.name"
            >{{ source.name }}</strong
          >
          <span
            v-if="source.enabled_explore && source.explore_url"
            class="shrink-0 rounded border px-1 text-[10px] text-muted-foreground"
            >发现</span
          >
        </div>
        <p class="mt-1 truncate text-xs text-muted-foreground" :title="source.base_url">{{ source.base_url }}</p>
      </div>
    </div>
    <div class="min-w-0">
      <span
        class="inline-block max-w-full truncate rounded-md bg-muted px-2 py-1 text-[11px] text-muted-foreground"
        :title="source.source_group || '未分组'"
        >{{ source.source_group || '未分组' }}</span
      >
    </div>
    <div class="min-w-0 text-xs">
      <div v-if="sources.testing === source.id" class="flex items-center gap-1.5 text-muted-foreground">
        <Loader2 class="size-3.5 animate-spin" />测试中…
      </div>
      <div v-else class="flex items-center gap-1.5" :class="attention ? 'text-destructive' : 'text-muted-foreground'">
        <CircleAlert v-if="attention" class="size-3.5 shrink-0" /><CheckCircle2
          v-else-if="result"
          class="size-3.5 shrink-0"
        />
        <span class="truncate" :title="error || probeLabel(result)">{{ probeLabel(result, error) }}</span>
      </div>
      <p class="mt-1 text-[11px] text-muted-foreground" :title="sessionLabel(source)">
        {{ error ? '请重试' : duration == null ? sessionLabel(source) : `${duration} ms` }}
      </p>
    </div>
    <div class="flex items-center gap-2">
      <Switch
        :model-value="source.enabled"
        :aria-label="`启用 ${source.name}`"
        :disabled="sources.toggling.has(source.id)"
        @update:model-value="sources.toggle(source)"
      />
      <span class="text-[11px] text-muted-foreground">{{ source.enabled ? '启用' : '停用' }}</span>
    </div>
    <div class="flex items-center justify-end gap-1">
      <Button
        variant="ghost"
        size="sm"
        :disabled="sources.testing != null || sources.batchTesting"
        :aria-label="`测试 ${source.name}`"
        :title="`测试关键词：${query || '测试'}`"
        @click="sources.test(source, query)"
        >测试</Button
      >
      <Button
        variant="ghost"
        size="icon-sm"
        :aria-label="`管理 ${source.name}`"
        title="管理分组与排序"
        @click="emit('manage', source)"
        ><Settings2
      /></Button>
      <DropdownMenu>
        <DropdownMenuTrigger as-child
          ><Button variant="ghost" size="icon-sm" :aria-label="`${source.name} 更多操作`"><MoreHorizontal /></Button
        ></DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          <DropdownMenuItem @select="router.push({ name: 'source-debug', query: { source: String(source.id) } })"
            >打开书源调试</DropdownMenuItem
          >
          <DropdownMenuSeparator />
          <DropdownMenuItem @select="sources.browserAuth(source)">浏览器认证</DropdownMenuItem>
          <DropdownMenuItem @select="sources.saveBrowserSession(source)">读取浏览器会话</DropdownMenuItem>
          <DropdownMenuItem
            :disabled="!source.access_token && !source.session_cookie"
            @select="sources.clearSession(source)"
            >清除会话</DropdownMenuItem
          >
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  </div>
</template>

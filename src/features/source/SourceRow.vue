<script setup lang="ts">
import { useRouter } from 'vue-router'
import { Globe, Loader2, MoreHorizontal } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Input } from '@/components/ui/input'
import { Switch } from '@/components/ui/switch'
import { useShellContext } from '@/app/shellKeys'
import type { BookSource } from '@/services/api'

const props = defineProps<{ source: BookSource; query: string }>()
const { sources } = useShellContext()
const router = useRouter()

// The draft lives in the store, not the row: the list only keeps the rows near
// the viewport mounted, so scrolling away must not drop an unsaved edit.
const draft = sources.managementDraft(props.source)

function sessionLabel(source: BookSource) {
  if (!source.access_token && !source.session_cookie) return '未认证'
  if (source.session_expires_at) {
    const raw = source.session_expires_at
    const expiry = /^\d+$/.test(raw) ? Number(raw) * 1000 : Date.parse(raw)
    if (Number.isFinite(expiry) && expiry <= Date.now()) return '已过期'
  }
  return '已认证'
}

function saveManagement() {
  return sources.saveManagement(props.source)
}

async function openDebug() {
  await router.push({ name: 'source-debug', query: { source: String(props.source.id) } })
}
</script>

<template>
  <div
    data-source-row
    class="grid grid-cols-[minmax(180px,1fr)_120px_64px_64px_auto_auto] items-center gap-2 rounded-md border bg-card px-3 py-2"
  >
    <div class="min-w-0">
      <div class="flex items-center gap-2">
        <span class="size-2 shrink-0 rounded-full" :class="source.enabled ? 'bg-primary' : 'bg-muted-foreground'" />
        <strong class="truncate text-sm">{{ source.name }}</strong>
        <span class="shrink-0 rounded bg-muted px-1.5 text-[10px] text-muted-foreground">{{
          sessionLabel(source)
        }}</span>
      </div>
      <div class="mt-0.5 flex items-center gap-1.5 truncate text-[11px] text-muted-foreground">
        <Globe :size="11" />{{ source.base_url }}
        <template v-if="source.respond_time != null">· {{ source.respond_time }} ms</template>
      </div>
    </div>
    <Input v-model="draft.group" class="h-8 text-xs" placeholder="分组" aria-label="书源分组" />
    <Input v-model.number="draft.order" class="h-8 text-xs" type="number" aria-label="排序" />
    <Input v-model.number="draft.weight" class="h-8 text-xs" type="number" aria-label="权重" />
    <div class="flex items-center gap-1.5">
      <Switch :model-value="source.enabled" aria-label="启用书源" @update:model-value="sources.toggle(source)" />
      <Button
        variant="outline"
        size="sm"
        :disabled="sources.testing === source.id"
        @click="sources.test(source, query)"
      >
        <Loader2 v-if="sources.testing === source.id" class="animate-spin" />
        {{ sources.testing === source.id ? '' : '测试' }}
      </Button>
    </div>
    <DropdownMenu>
      <DropdownMenuTrigger as-child>
        <Button variant="ghost" size="icon-sm" :aria-label="`${source.name} 更多操作`"><MoreHorizontal /></Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuItem @select="saveManagement">保存分组与排序</DropdownMenuItem>
        <DropdownMenuItem @select="sources.browserAuth(source)">浏览器认证</DropdownMenuItem>
        <DropdownMenuItem @select="sources.saveBrowserSession(source)">读取浏览器会话</DropdownMenuItem>
        <DropdownMenuItem @select="sources.clearSession(source)">清除会话</DropdownMenuItem>
        <DropdownMenuSeparator />
        <DropdownMenuItem @select="openDebug">打开书源调试</DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  </div>
</template>

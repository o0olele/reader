<script setup lang="ts">
import { Search, X } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import type { SourceFilter, SourceSort } from './sourceView'

defineProps<{ counts: Record<SourceFilter, number>; groups: string[]; filteredCount: number }>()
const keyword = defineModel<string>('keyword', { required: true })
const filter = defineModel<SourceFilter>('filter', { required: true })
const group = defineModel<string>('group', { required: true })
const sort = defineModel<SourceSort>('sort', { required: true })
const filters = [
  { value: 'all', label: '全部书源' },
  { value: 'enabled', label: '已启用' },
  { value: 'disabled', label: '已停用' },
  { value: 'attention', label: '需关注' },
] as const
</script>

<template>
  <div class="shrink-0 space-y-4 pb-4">
    <div class="flex items-center justify-between gap-3">
      <div class="flex gap-1 rounded-lg bg-muted/70 p-1" role="group" aria-label="按书源状态筛选">
        <button
          v-for="item in filters"
          :key="item.value"
          type="button"
          class="flex items-center gap-2 rounded-md px-3 py-1.5 text-xs font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
          :class="
            filter === item.value ? 'bg-card text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'
          "
          :aria-pressed="filter === item.value"
          @click="filter = item.value"
        >
          {{ item.label }}
          <span
            class="rounded px-1.5 py-0.5 text-[10px] tabular-nums"
            :class="filter === item.value ? 'bg-muted' : ''"
            >{{ counts[item.value] }}</span
          >
        </button>
      </div>
      <span class="text-xs text-muted-foreground" role="status"
        >显示 {{ filteredCount }} / {{ counts.all }} 个书源</span
      >
    </div>
    <div class="flex flex-wrap items-center gap-2">
      <div class="relative min-w-48 flex-1">
        <Search class="pointer-events-none absolute top-2.5 left-3 size-4 text-muted-foreground" />
        <Input
          v-model="keyword"
          class="h-9 bg-card pr-9 pl-9"
          placeholder="搜索名称、网址或分组…"
          aria-label="搜索书源"
        />
        <Button
          v-if="keyword"
          variant="ghost"
          size="icon-sm"
          class="absolute top-0.5 right-0.5"
          aria-label="清空搜索"
          @click="keyword = ''"
          ><X
        /></Button>
      </div>
      <Select v-model="group">
        <SelectTrigger class="w-40 bg-card" aria-label="筛选分组"><SelectValue /></SelectTrigger>
        <SelectContent>
          <SelectItem value="all">全部分组</SelectItem>
          <SelectItem value="ungrouped">未分组</SelectItem>
          <SelectItem v-for="name in groups" :key="name" :value="`group:${name}`">{{ name }}</SelectItem>
        </SelectContent>
      </Select>
      <Select v-model="sort">
        <SelectTrigger class="w-36 bg-card" aria-label="书源排序"><SelectValue /></SelectTrigger>
        <SelectContent>
          <SelectItem value="default">默认排序</SelectItem>
          <SelectItem value="name">名称排序</SelectItem>
          <SelectItem value="response">响应最快</SelectItem>
        </SelectContent>
      </Select>
    </div>
  </div>
</template>

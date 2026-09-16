<script setup lang="ts">
import { Grid3x3, List, Plus, RefreshCw } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { ToggleGroup, ToggleGroupItem } from '@/components/ui/toggle-group'
import { useShellContext } from '@/app/shellKeys'
import type { FilterKey, SortKey, ViewMode } from './shelfView'

const emit = defineEmits<{ import: [] }>()
const { bookshelf } = useShellContext()
const keyword = defineModel<string>('keyword', { required: true })
const sort = defineModel<SortKey>('sort', { required: true })
const filter = defineModel<FilterKey>('filter', { required: true })
const mode = defineModel<ViewMode>('mode', { required: true })
</script>

<template>
  <div class="flex shrink-0 items-center gap-2 border-b bg-card px-4 py-2">
    <Input v-model="keyword" class="h-8 w-56" placeholder="搜索书名、作者" aria-label="搜索书架" />
    <Select v-model="sort">
      <SelectTrigger class="h-8 w-32"><SelectValue /></SelectTrigger>
      <SelectContent>
        <SelectItem value="updated">最近更新</SelectItem>
        <SelectItem value="title">按标题</SelectItem>
        <SelectItem value="author">按作者</SelectItem>
        <SelectItem value="chapters">按章节数</SelectItem>
      </SelectContent>
    </Select>
    <Select v-model="filter">
      <SelectTrigger class="h-8 w-28"><SelectValue /></SelectTrigger>
      <SelectContent>
        <SelectItem value="all">全部来源</SelectItem>
        <SelectItem value="local">仅本地</SelectItem>
        <SelectItem value="online">仅在线</SelectItem>
      </SelectContent>
    </Select>
    <Button variant="outline" size="sm" @click="bookshelf.refresh()"><RefreshCw /> 刷新</Button>
    <ToggleGroup
      type="single"
      :model-value="mode"
      variant="outline"
      size="sm"
      class="ml-auto"
      @update:model-value="mode = ($event as ViewMode) || mode"
    >
      <ToggleGroupItem value="grid" aria-label="网格视图"><Grid3x3 /></ToggleGroupItem>
      <ToggleGroupItem value="list" aria-label="列表视图"><List /></ToggleGroupItem>
    </ToggleGroup>
    <Button size="sm" @click="emit('import')"><Plus /> 导入书籍</Button>
  </div>
</template>

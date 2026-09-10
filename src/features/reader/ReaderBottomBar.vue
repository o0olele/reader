<script setup lang="ts">
import { computed } from 'vue'
import {
  ArrowLeft,
  ArrowRight,
  Bookmark,
  Headphones,
  Languages,
  List,
  MoreHorizontal,
  Play,
  Search,
  SlidersHorizontal,
  Sparkles,
  Sun,
  Type,
} from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Slider } from '@/components/ui/slider'

const props = defineProps<{
  chapterCount: number
  chapterIndex: number
  hasBookmark: boolean
  autoPage: boolean
  eyeCare: boolean
}>()

const emit = defineEmits<{
  prev: []
  next: []
  goto: [index: number]
  search: []
  autoPage: []
  toc: []
  tts: []
  style: []
  bookmark: []
  theme: []
  eyeCare: []
  translate: []
  ai: []
  textProcess: []
  more: []
}>()

const percent = computed(() =>
  props.chapterCount ? Math.round(((props.chapterIndex + 1) / props.chapterCount) * 100) : 0,
)
</script>

<template>
  <div class="shrink-0 border-t bg-card">
    <div class="flex items-center gap-3 px-4 py-1.5">
      <Button variant="ghost" size="sm" :disabled="chapterIndex <= 0" @click="emit('prev')"
        ><ArrowLeft /> 上一章</Button
      >
      <span class="w-28 shrink-0 text-center text-xs text-muted-foreground">
        第 {{ chapterIndex + 1 }}/{{ chapterCount }} 章
      </span>
      <Slider
        class="flex-1"
        :model-value="[chapterIndex]"
        :min="0"
        :max="Math.max(0, chapterCount - 1)"
        :step="1"
        @update:model-value="emit('goto', $event?.[0] ?? chapterIndex)"
      />
      <span class="w-12 shrink-0 text-right text-xs text-muted-foreground">{{ percent }}%</span>
      <Button variant="ghost" size="sm" :disabled="chapterIndex >= chapterCount - 1" @click="emit('next')">
        下一章 <ArrowRight />
      </Button>
    </div>

    <div class="flex items-center gap-0.5 border-t px-3 py-1">
      <Button variant="ghost" size="sm" title="正文搜索" @click="emit('search')"><Search /> 正文搜索</Button>
      <Button variant="ghost" size="sm" :class="autoPage ? 'bg-accent' : ''" title="自动翻页" @click="emit('autoPage')">
        <Play /> 自动翻页
      </Button>
      <Button variant="ghost" size="sm" title="目录 (T)" @click="emit('toc')"><List /> 目录</Button>
      <Button variant="ghost" size="sm" title="听书（未接入）" @click="emit('tts')"><Headphones /> 听书</Button>
      <Button variant="ghost" size="sm" title="阅读样式" @click="emit('style')"><SlidersHorizontal /> 阅读样式</Button>
      <Button
        variant="ghost"
        size="sm"
        :class="hasBookmark ? 'bg-accent' : ''"
        title="加书签"
        @click="emit('bookmark')"
      >
        <Bookmark /> 加书签
      </Button>
      <Button variant="ghost" size="sm" title="日夜间" @click="emit('theme')"><Sun /> 日夜间</Button>
      <Button variant="ghost" size="sm" :class="eyeCare ? 'bg-accent' : ''" title="护眼" @click="emit('eyeCare')">
        <Type /> 护眼
      </Button>
      <Button variant="ghost" size="sm" title="翻译（未接入）" @click="emit('translate')"><Languages /> 翻译</Button>
      <Button variant="ghost" size="sm" title="AI 总结（未接入）" @click="emit('ai')"><Sparkles /> AI 总结</Button>
      <Button variant="ghost" size="sm" title="文本处理（未接入）" @click="emit('textProcess')"
        ><Type /> 文本处理</Button
      >
      <Button variant="ghost" size="icon-sm" class="ml-auto" title="更多" @click="emit('more')">
        <MoreHorizontal />
      </Button>
    </div>
  </div>
</template>

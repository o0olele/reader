<script setup lang="ts">
import { computed, ref } from 'vue'
import { useElementSize } from '@vueuse/core'
import {
  Bookmark,
  Headphones,
  Languages,
  List,
  MoreHorizontal,
  Search,
  SlidersHorizontal,
  Sparkles,
  Type,
} from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import ReaderThemePopover from './ReaderThemePopover.vue'
import ReaderAutoPageControl from './ReaderAutoPageControl.vue'
import ReaderChapterProgress from './ReaderChapterProgress.vue'

defineProps<{
  chapterCount: number
  chapterIndex: number
  hasBookmark: boolean
  autoPage: boolean
  autoPageSeconds: number
  autoPageInterval: number
  autoPageReady: boolean
  /** 当前背景主题：底栏主题按钮直接显示它。 */
  theme: string
  /** 目录是否展开：展开时目录按钮保持选中态。 */
  tocOpen: boolean
}>()

const emit = defineEmits<{
  prev: []
  next: []
  goto: [index: number]
  search: []
  startAutoPage: [seconds: number]
  pauseAutoPage: []
  toc: []
  tts: []
  style: []
  bookmark: []
  theme: [theme: string]
  translate: []
  ai: []
  textProcess: []
  more: []
}>()

const bar = ref<HTMLElement | null>(null)
const { width } = useElementSize(bar)
// 按底栏实际宽度适配侧栏开合；阈值也为自动翻页的最长状态文案留出空间。
const compact = computed(() => width.value < 800)
const overflow = computed(() => width.value < 1100)
const buttonSize = computed(() => (compact.value ? 'icon-sm' : 'sm'))
const secondaryTools = [
  { label: '听书', icon: Headphones, run: () => emit('tts') },
  { label: '翻译', icon: Languages, run: () => emit('translate') },
  { label: 'AI 总结', icon: Sparkles, run: () => emit('ai') },
  { label: '文本处理', icon: Type, run: () => emit('textProcess') },
]
</script>

<template>
  <div ref="bar" class="min-w-0 shrink-0 border-t bg-card">
    <ReaderChapterProgress
      :chapter-count="chapterCount"
      :chapter-index="chapterIndex"
      :compact="compact"
      @prev="emit('prev')"
      @next="emit('next')"
      @goto="emit('goto', $event)"
    />
    <div class="flex items-center gap-0.5 border-t py-1 [&>button]:shrink-0" :class="compact ? 'px-2' : 'px-3'">
      <Button variant="ghost" :size="buttonSize" aria-label="正文搜索" title="正文搜索" @click="emit('search')">
        <Search /><span v-if="!compact">正文搜索</span>
      </Button>
      <ReaderAutoPageControl
        :active="autoPage"
        :seconds-left="autoPageSeconds"
        :interval="autoPageInterval"
        :ready="autoPageReady"
        :compact="compact"
        @start="emit('startAutoPage', $event)"
        @pause="emit('pauseAutoPage')"
      />
      <Button
        variant="ghost"
        :size="buttonSize"
        :class="tocOpen ? 'bg-accent' : ''"
        :aria-pressed="tocOpen"
        aria-label="目录"
        title="目录 (T)"
        @click="emit('toc')"
      >
        <List /><span v-if="!compact">目录</span>
      </Button>
      <Button variant="ghost" :size="buttonSize" aria-label="阅读样式" title="阅读样式" @click="emit('style')">
        <SlidersHorizontal /><span v-if="!compact">阅读样式</span>
      </Button>
      <Button
        variant="ghost"
        :size="buttonSize"
        :class="hasBookmark ? 'bg-accent' : ''"
        :aria-pressed="hasBookmark"
        :aria-label="hasBookmark ? '移除书签' : '加书签'"
        :title="hasBookmark ? '移除书签' : '加书签'"
        @click="emit('bookmark')"
      >
        <Bookmark /><span v-if="!compact">{{ hasBookmark ? '已加书签' : '加书签' }}</span>
      </Button>
      <ReaderThemePopover :theme="theme" :compact="compact" @select="emit('theme', $event)" />
      <template v-if="!overflow">
        <Button
          v-for="tool in secondaryTools"
          :key="tool.label"
          variant="ghost"
          size="sm"
          :title="`${tool.label}（未接入）`"
          @click="tool.run"
        >
          <component :is="tool.icon" />{{ tool.label }}
        </Button>
      </template>
      <DropdownMenu>
        <DropdownMenuTrigger as-child>
          <Button variant="ghost" size="icon-sm" class="ml-auto shrink-0" aria-label="更多" title="更多">
            <MoreHorizontal />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent side="top" align="end" class="w-44">
          <template v-if="overflow">
            <DropdownMenuItem
              v-for="tool in secondaryTools"
              :key="tool.label"
              :title="`${tool.label}（未接入）`"
              @select="tool.run"
            >
              <component :is="tool.icon" />{{ tool.label }}
            </DropdownMenuItem>
            <DropdownMenuSeparator />
          </template>
          <DropdownMenuItem @select="emit('more')"><MoreHorizontal />更多功能</DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { ALargeSmall, ArrowLeft, ArrowRight, List, MoreHorizontal, Sun, Type, X } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { useShellContext } from '@/app/shellKeys'

defineProps<{ chapterListOpen: boolean; settingsOpen: boolean; collapsed?: boolean }>()
const emit = defineEmits<{ prev: []; next: []; close: []; toggleToc: []; togglePanel: [] }>()

const { reader } = useShellContext()
const FONTS = ['思源宋体', '霞鹜文楷', '系统默认']
const SIZES = [14, 16, 17, 18, 20, 22, 24]
const chapterIndex = computed(() => reader.chapters.findIndex((chapter) => chapter.id === reader.selectedChapter?.id))
const chapterLabel = computed(() =>
  chapterIndex.value < 0 ? '正文' : `第${chapterIndex.value + 1}章 ${reader.selectedChapter?.title ?? ''}`,
)
</script>

<template>
  <header
    class="flex shrink-0 items-center gap-3 overflow-hidden border-b bg-card px-4 transition-[height] duration-150 ease-out"
    :class="collapsed ? 'h-0 border-b-0' : 'h-14'"
  >
    <div class="flex items-center gap-1">
      <Button variant="ghost" size="icon-sm" aria-label="上一章" :disabled="chapterIndex <= 0" @click="emit('prev')">
        <ArrowLeft />
      </Button>
      <Button
        variant="ghost"
        size="icon-sm"
        aria-label="下一章"
        :disabled="chapterIndex < 0 || chapterIndex >= reader.chapters.length - 1"
        @click="emit('next')"
      >
        <ArrowRight />
      </Button>
      <Button variant="ghost" size="icon-sm" aria-label="关闭阅读" title="关闭阅读" @click="emit('close')">
        <X />
      </Button>
    </div>

    <strong class="min-w-0 flex-1 truncate text-sm">{{ chapterLabel }}</strong>

    <div class="flex items-center gap-1">
      <DropdownMenu>
        <DropdownMenuTrigger as-child>
          <Button variant="ghost" size="sm" class="gap-1.5" aria-label="选择字体">
            <Type />{{ reader.fontFamily }}
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          <DropdownMenuLabel>字体</DropdownMenuLabel>
          <DropdownMenuItem v-for="font in FONTS" :key="font" @select="reader.fontFamily = font">
            <span :class="reader.fontFamily === font ? 'font-semibold' : ''">{{ font }}</span>
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>

      <DropdownMenu>
        <DropdownMenuTrigger as-child>
          <Button variant="ghost" size="sm" class="gap-1.5" aria-label="选择字号">
            <ALargeSmall />{{ reader.fontSize }}px
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          <DropdownMenuLabel>字号</DropdownMenuLabel>
          <DropdownMenuItem v-for="size in SIZES" :key="size" @select="reader.fontSize = size">
            <span :class="reader.fontSize === size ? 'font-semibold' : ''">{{ size }} px</span>
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>

      <Button
        variant="ghost"
        size="icon-sm"
        aria-label="切换阅读主题"
        title="切换阅读主题"
        @click="reader.theme = reader.theme === 'light' ? 'dark' : 'light'"
      >
        <Sun />
      </Button>
      <Button
        variant="ghost"
        size="icon-sm"
        :class="chapterListOpen ? 'bg-accent' : ''"
        aria-label="显示或隐藏目录"
        title="显示或隐藏目录 (T)"
        @click="emit('toggleToc')"
      >
        <List />
      </Button>
      <Button
        variant="ghost"
        size="icon-sm"
        :class="settingsOpen ? 'bg-accent' : ''"
        aria-label="显示或隐藏阅读设置"
        title="显示或隐藏阅读设置"
        @click="emit('togglePanel')"
      >
        <MoreHorizontal />
      </Button>
    </div>
  </header>
</template>

<script setup lang="ts">
import { computed, ref, type Component } from 'vue'
import { Check, Eye, Moon, MoonStar, Sun } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover'
import { READER_THEMES, readerTheme } from './readerThemes'

const props = defineProps<{ theme: string }>()

const emit = defineEmits<{ select: [theme: string] }>()

/** 底栏按钮图标跟着当前主题走，一眼看出是日间还是夜间。 */
const ICONS: Record<string, Component> = {
  light: Sun,
  sepia: Eye,
  dark: Moon,
  black: MoonStar,
}

const open = ref(false)
const current = computed(() => readerTheme(props.theme))
const currentIcon = computed(() => ICONS[current.value.value] ?? Sun)

function select(value: string) {
  emit('select', value)
  open.value = false
}
</script>

<template>
  <Popover v-model:open="open">
    <PopoverTrigger as-child>
      <Button variant="ghost" size="sm" :title="`背景主题：${current.label}`">
        <component :is="currentIcon" /> {{ current.label }}
      </Button>
    </PopoverTrigger>
    <!-- 底栏贴着窗口下沿，明确向上弹，别依赖碰撞翻转。 -->
    <PopoverContent side="top" align="start" class="w-52 p-2">
      <p class="px-1 pb-1.5 text-xs text-muted-foreground">背景主题</p>
      <div class="grid gap-0.5">
        <button
          v-for="option in READER_THEMES"
          :key="option.value"
          type="button"
          class="flex items-center gap-2 rounded-sm px-2 py-1.5 text-sm hover:bg-accent hover:text-accent-foreground"
          :class="option.value === current.value ? 'bg-accent/60 font-medium' : ''"
          :aria-pressed="option.value === current.value"
          @click="select(option.value)"
        >
          <span class="size-5 shrink-0 rounded-sm border" :class="option.swatch" />
          <span class="flex-1 text-left">{{ option.label }}</span>
          <Check v-if="option.value === current.value" class="size-4" />
        </button>
      </div>
    </PopoverContent>
  </Popover>
</template>

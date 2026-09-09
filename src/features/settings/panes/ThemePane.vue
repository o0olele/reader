<script setup lang="ts">
import { Moon, Sun } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { useShellContext } from '@/app/shellKeys'
import { useTheme } from '@/app/useTheme'

const { reader } = useShellContext()
const { isDark, setTheme } = useTheme()
const READER_THEMES = [
  { value: 'light', label: '浅色', swatch: 'bg-[oklch(0.99_0.003_106)]' },
  { value: 'sepia', label: '护眼', swatch: 'bg-[oklch(0.95_0.03_85)]' },
  { value: 'dark', label: '深色', swatch: 'bg-[oklch(0.27_0.02_265)]' },
  { value: 'black', label: '黑夜', swatch: 'bg-black' },
] as const
</script>

<template>
  <div class="grid max-w-2xl gap-6">
    <section class="grid gap-2">
      <h2 class="text-sm font-semibold">应用外观</h2>
      <p class="text-xs text-muted-foreground">整窗配色跟随 shadcn 令牌；快捷键 D 也可切换。</p>
      <div class="flex gap-2">
        <Button :variant="isDark ? 'outline' : 'default'" size="sm" @click="setTheme('light')"> <Sun /> 浅色 </Button>
        <Button :variant="isDark ? 'default' : 'outline'" size="sm" @click="setTheme('dark')"> <Moon /> 深色 </Button>
      </div>
    </section>

    <section class="grid gap-2">
      <h2 class="text-sm font-semibold">阅读主题</h2>
      <p class="text-xs text-muted-foreground">阅读器正文使用独立的纸张/墨色令牌，与应用 chrome 分离。</p>
      <div class="flex gap-2">
        <button
          v-for="option in READER_THEMES"
          :key="option.value"
          type="button"
          class="flex w-20 flex-col items-center gap-1 rounded-md border p-1.5 text-[11px]"
          :class="reader.theme === option.value ? 'border-primary ring-1 ring-primary' : ''"
          @click="reader.theme = option.value"
        >
          <span class="h-8 w-full rounded-sm border" :class="option.swatch" />
          {{ option.label }}
        </button>
      </div>
    </section>
  </div>
</template>

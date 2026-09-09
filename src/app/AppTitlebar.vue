<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window'
import { BookOpen, Maximize2, Minus, Moon, Sun, X } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import AppBreadcrumb from './AppBreadcrumb.vue'
import { useTheme } from './useTheme'

const { isDark, toggleTheme } = useTheme()
const win = getCurrentWindow()

async function windowAction(action: 'minimize' | 'maximize' | 'close') {
  try {
    if (action === 'minimize') await win.minimize()
    else if (action === 'maximize') await win.toggleMaximize()
    else await win.close()
  } catch {
    /* browser preview */
  }
}

async function dragWindow(event: MouseEvent) {
  if (event.button !== 0 || (event.target as HTMLElement).closest('button, input, a, form')) return
  try {
    await win.startDragging()
  } catch {
    /* browser preview */
  }
}
</script>

<template>
  <header
    data-tauri-drag-region
    class="flex h-(--titlebar-h) shrink-0 items-center gap-2.5 border-b bg-card px-3 select-none"
    @mousedown="dragWindow"
    @dblclick="windowAction('maximize')"
  >
    <span data-tauri-drag-region class="flex items-center gap-2 text-xs font-semibold">
      <BookOpen :size="15" />
      Legado · 桌面端
    </span>
    <AppBreadcrumb />
    <div data-tauri-drag-region class="flex-1" />
    <Button
      variant="ghost"
      size="icon-sm"
      :aria-label="isDark ? '切换到浅色主题' : '切换到深色主题'"
      :title="isDark ? '切换浅色主题 (D)' : '切换深色主题 (D)'"
      @click.stop="toggleTheme"
    >
      <Sun v-if="isDark" /><Moon v-else />
    </Button>
    <div class="ml-1 flex items-center gap-0.5">
      <Button variant="ghost" size="icon-sm" aria-label="最小化" @click.stop="windowAction('minimize')">
        <Minus />
      </Button>
      <Button variant="ghost" size="icon-sm" aria-label="最大化" @click.stop="windowAction('maximize')">
        <Maximize2 :size="14" />
      </Button>
      <Button
        variant="ghost"
        size="icon-sm"
        aria-label="关闭"
        class="hover:bg-destructive hover:text-destructive-foreground"
        @click.stop="windowAction('close')"
      >
        <X />
      </Button>
    </div>
  </header>
</template>

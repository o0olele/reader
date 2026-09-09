<script setup lang="ts">
import { computed } from 'vue'
import { RouterLink, useRoute } from 'vue-router'
import { Menu, Search } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import {
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuBadge,
  SidebarMenuItem,
} from '@/components/ui/sidebar'
import SidebarMenuButtonChild from '@/components/ui/sidebar/SidebarMenuButtonChild.vue'
import { cn } from '@/lib/utils'
import { shellFooterNav, shellNavGroups, type ShellNavItem } from './shellNav'
import { useCommandPalette } from './useCommandPalette'
import { useRail } from './useRail'
import { useShellContext } from './shellKeys'

const route = useRoute()
const { extended, toggle } = useRail()
const { openPalette } = useCommandPalette()
const { bookshelf, appVersion } = useShellContext()

const badgeCount = (item: ShellNavItem) => (item.badge === 'books' ? bookshelf.books.length : 0)
const isActive = (item: ShellNavItem) => route.path === item.to
const itemClass = computed(() => cn('h-9', !extended.value && 'justify-center'))
</script>

<template>
  <aside
    class="flex shrink-0 flex-col border-r bg-sidebar text-sidebar-foreground transition-[width] duration-150 ease-out"
    :style="{ width: extended ? 'var(--rail-w-ext)' : 'var(--rail-w)' }"
  >
    <SidebarHeader class="gap-2 p-2">
      <div class="flex h-9 items-center gap-1.5">
        <Button
          variant="ghost"
          size="icon-sm"
          title="展开 / 收起导航 (Ctrl+B)"
          aria-label="展开或收起导航"
          @click="toggle"
        >
          <Menu />
        </Button>
        <span v-if="extended" class="truncate text-[13px] font-semibold">Legado</span>
      </div>
      <button
        v-if="extended"
        type="button"
        class="flex h-9 items-center gap-2 rounded-md bg-muted px-2.5 text-left text-xs hover:bg-accent"
        @click="openPalette"
      >
        <Search class="size-4 shrink-0" />
        <span class="flex-1 truncate font-semibold text-foreground">搜索书籍、书源</span>
        <kbd class="rounded border bg-background px-1 text-[10px] text-muted-foreground">Ctrl K</kbd>
      </button>
      <Button
        v-else
        variant="ghost"
        size="icon-sm"
        aria-label="全局搜索"
        title="全局搜索 (Ctrl+K)"
        @click="openPalette"
      >
        <Search />
      </Button>
    </SidebarHeader>

    <SidebarContent>
      <SidebarGroup v-for="group in shellNavGroups" :key="group.label" class="p-1">
        <SidebarGroupLabel v-if="extended">{{ group.label }}</SidebarGroupLabel>
        <SidebarGroupContent>
          <SidebarMenu>
            <SidebarMenuItem v-for="item in group.items" :key="item.to">
              <SidebarMenuButtonChild as-child :is-active="isActive(item)" :class="itemClass">
                <RouterLink :to="item.to" :title="item.label">
                  <component :is="item.icon" />
                  <span v-if="extended" class="truncate">{{ item.label }}</span>
                </RouterLink>
              </SidebarMenuButtonChild>
              <SidebarMenuBadge v-if="extended && badgeCount(item)">{{ badgeCount(item) }}</SidebarMenuBadge>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarGroupContent>
      </SidebarGroup>
    </SidebarContent>

    <SidebarFooter class="gap-1 p-1">
      <SidebarMenu>
        <SidebarMenuItem v-for="item in shellFooterNav" :key="item.to">
          <SidebarMenuButtonChild as-child :is-active="isActive(item)" :class="itemClass">
            <RouterLink :to="item.to" :title="item.label">
              <component :is="item.icon" />
              <span v-if="extended" class="truncate">{{ item.label }}</span>
            </RouterLink>
          </SidebarMenuButtonChild>
        </SidebarMenuItem>
      </SidebarMenu>
      <div v-if="extended" class="rounded-md border bg-card px-2 py-1.5 text-[11px] text-muted-foreground">
        <div class="font-semibold text-foreground">本地书友</div>
        <div>{{ bookshelf.books.length }} 本 · v{{ appVersion }}</div>
      </div>
    </SidebarFooter>
  </aside>
</template>

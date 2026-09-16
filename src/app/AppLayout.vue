<script setup lang="ts">
import { RouterView } from 'vue-router'
import { Toaster } from '@/components/ui/sonner'
import { TooltipProvider } from '@/components/ui/tooltip'
// vue-sonner v2 ships its base stylesheet separately; without it the toasts fall
// back to an unstyled full-width block at the bottom of the page.
import 'vue-sonner/style.css'
import AppRail from './AppRail.vue'
import AppTitlebar from './AppTitlebar.vue'
import CommandPalette from './CommandPalette.vue'
import { useGlobalKeys } from './useGlobalKeys'
import { useShell } from './useShell'
import { useTheme } from './useTheme'

const { isDark } = useTheme()
useShell()
useGlobalKeys()
</script>

<template>
  <!-- `delay-duration="0"` matches the shadcn sidebar: rail hints appear on
       hover without the 700ms default lag. -->
  <TooltipProvider :delay-duration="0">
    <div
      :class="['flex h-screen flex-col overflow-hidden bg-background text-foreground', { 'app-theme-dark': isDark }]"
    >
      <AppTitlebar />
      <div class="flex min-h-0 flex-1">
        <AppRail />
        <main class="min-w-0 flex-1 overflow-hidden bg-background">
          <RouterView v-slot="{ Component }">
            <component :is="Component" />
          </RouterView>
        </main>
      </div>
      <CommandPalette />
      <Toaster position="bottom-right" :duration="3200" :theme="isDark ? 'dark' : 'light'" />
    </div>
  </TooltipProvider>
</template>

<script setup lang="ts">
import type { LucideIcon } from 'lucide-vue-next'

/**
 * Empty state for a page that has a real data source but no rows yet. It is
 * deliberately not `NotConnected`: that one marks a missing capability (纪律 F0),
 * this one marks "nothing here yet, here is how to fill it".
 */
defineProps<{ icon: LucideIcon; title: string; description?: string }>()
</script>

<template>
  <div class="grid place-items-center rounded-lg border border-dashed px-8 py-12 text-center text-muted-foreground">
    <component :is="icon" :size="28" class="mb-3" />
    <h2 class="text-sm font-semibold text-foreground">{{ title }}</h2>
    <p v-if="description" class="mt-1.5 max-w-md text-xs leading-relaxed">{{ description }}</p>
    <slot />
  </div>
</template>

<script setup lang="ts">
defineProps<{ title: string; subtitle?: string }>()
</script>

<template>
  <header class="flex shrink-0 items-start justify-between gap-4 border-b bg-card px-6 py-3.5">
    <div class="min-w-0">
      <h1 class="truncate text-base font-semibold">{{ title }}</h1>
      <p v-if="subtitle" class="mt-0.5 truncate text-xs text-muted-foreground">{{ subtitle }}</p>
    </div>
    <div class="flex shrink-0 items-center gap-2">
      <slot />
    </div>
  </header>
</template>

<script setup lang="ts">
import { CircleSlash } from 'lucide-vue-next'

/**
 * ROADMAP-v3 discipline F0: a module the backend does not serve yet says so,
 * and names the capability it is waiting for. It never borrows another
 * module's data to look finished.
 */
defineProps<{ title: string; description?: string; capabilities?: string[] }>()
</script>

<template>
  <div class="grid place-items-center rounded-lg border border-dashed px-8 py-12 text-center">
    <CircleSlash :size="32" class="mb-3 text-muted-foreground" />
    <h2 class="text-sm font-semibold">未接入 · {{ title }}</h2>
    <p v-if="description" class="mt-1.5 max-w-md text-xs leading-relaxed text-muted-foreground">
      {{ description }}
    </p>
    <ul v-if="capabilities?.length" class="mt-3 space-y-1 text-left text-xs text-muted-foreground">
      <li v-for="item in capabilities" :key="item">· 需要：{{ item }}</li>
    </ul>
  </div>
</template>

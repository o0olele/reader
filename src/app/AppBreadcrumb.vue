<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { routeTitles } from './shellNav'

const route = useRoute()
const title = computed(() => routeTitles[String(route.name ?? '')] ?? '')
</script>

<template>
  <span
    v-if="title"
    class="rounded bg-muted px-2 py-0.5 text-[11px] font-medium text-muted-foreground"
    aria-current="page"
  >
    {{ title }}
  </span>
</template>

<script setup lang="ts">
/**
 * Page scroll area. `scroll` stays on for ordinary pages; a page that owns its
 * own scroll box (a windowed list) turns it off and lays out as a flex column.
 */
withDefaults(defineProps<{ scroll?: boolean }>(), { scroll: true })
</script>

<template>
  <div :class="scroll ? 'min-h-0 flex-1 overflow-y-auto p-6' : 'flex min-h-0 flex-1 flex-col overflow-hidden p-6'">
    <slot />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

/**
 * Renders one reader paragraph (or chapter title) with a single highlighted
 * range. The three pieces are emitted as bare text nodes plus one `<mark>`, so
 * the paragraph keeps the exact same glyphs and line breaking as an unmarked
 * one — `white-space: pre-wrap` means even a stray space would show up.
 */
const props = defineProps<{
  text: string
  /** UTF-16 offset and length of the hit, straight from the backend. */
  offset?: number | null
  length?: number | null
}>()

const parts = computed(() => {
  const offset = props.offset
  const length = props.length
  if (typeof offset !== 'number' || !length || offset < 0 || offset >= props.text.length) return undefined
  const end = Math.min(props.text.length, offset + length)
  if (end <= offset) return undefined
  return [props.text.slice(0, offset), props.text.slice(offset, end), props.text.slice(end)]
})
</script>

<template>
  <template v-if="parts"
    >{{ parts[0] }}<mark class="reader-hit">{{ parts[1] }}</mark
    >{{ parts[2] }}</template
  >
  <template v-else>{{ text }}</template>
</template>

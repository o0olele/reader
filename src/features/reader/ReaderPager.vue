<script setup lang="ts">
import { ChevronLeft, ChevronRight } from 'lucide-vue-next'

defineProps<{ pageIndex: number; pageCount: number; chapterPercent: number }>()
const emit = defineEmits<{ turn: [direction: number] }>()
</script>

<template>
  <!-- Prototype `.reader__pager` (:887–894, markup :2013–2017) -->
  <div class="reader-pager">
    <button type="button" title="上一页" :disabled="pageIndex <= 0" @click="emit('turn', -1)">
      <ChevronLeft :size="17" />
    </button>
    <span
      >第 <b>{{ pageIndex + 1 }}</b> / {{ pageCount }} 页 · 本章 {{ chapterPercent }}%</span
    >
    <button type="button" title="下一页" :disabled="pageIndex >= pageCount - 1" @click="emit('turn', 1)">
      <ChevronRight :size="17" />
    </button>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  readerMode: 'scroll' | 'paged'
  hasBookmark: boolean
  busy: boolean
  loading: boolean
  matches: number
  modelValue: string
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
  mode: [mode: 'scroll' | 'paged']
  turn: [direction: number]
  toggleBookmark: []
}>()
</script>

<template>
  <div class="reader-toolbar">
    <div class="reader-mode-toggle" role="group" aria-label="阅读模式">
      <button type="button" :class="{ active: readerMode === 'scroll' }" @click="emit('mode', 'scroll')">滚动</button>
      <button type="button" :class="{ active: readerMode === 'paged' }" @click="emit('mode', 'paged')">分页</button>
    </div>
    <div v-if="readerMode === 'paged'" class="reader-page-actions">
      <button type="button" aria-label="上一页" @click="emit('turn', -1)">上一页</button>
      <button type="button" aria-label="下一页" @click="emit('turn', 1)">下一页</button>
    </div>
    <div class="reader-tools">
      <input
        :value="props.modelValue"
        type="search"
        placeholder="搜索本章"
        aria-label="搜索本章"
        @input="emit('update:modelValue', ($event.target as HTMLInputElement).value)"
      />
      <span v-if="props.modelValue.trim()" class="reader-search-count">{{ matches }} 处</span>
      <button type="button" :aria-pressed="hasBookmark" :disabled="busy || loading" @click="emit('toggleBookmark')">
        {{ hasBookmark ? '移除书签' : '添加书签' }}
      </button>
    </div>
  </div>
</template>

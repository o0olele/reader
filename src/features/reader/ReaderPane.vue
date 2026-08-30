<script setup lang="ts">
import type { Chapter } from '../../services/api/health'
import { onMounted, ref, watch } from 'vue'

defineProps<{
  chapters: Chapter[]
  selectedChapter?: Chapter
  theme: string
  fontSize: number
  loading: boolean
}>()

const emit = defineEmits<{
  selectChapter: [chapter: Chapter]
  scroll: []
  readerContent: [element: HTMLElement | null]
}>()

const contentRef = ref<HTMLElement | null>(null)
onMounted(() => emit('readerContent', contentRef.value))
watch(contentRef, (element) => emit('readerContent', element))
</script>

<template>
  <div class="reader-layout">
    <aside class="chapter-list">
      <h2>目录</h2>
      <button
        v-for="chapter in chapters"
        :key="chapter.id"
        type="button"
        :class="['chapter-button', { selected: selectedChapter?.id === chapter.id }]"
        @click="emit('selectChapter', chapter)"
      >
        {{ chapter.title }}
      </button>
    </aside>
    <article
      v-if="selectedChapter"
      ref="contentRef"
      :class="['reader-content', `theme-${theme}`]"
      @scroll="emit('scroll')"
    >
      <h2>{{ selectedChapter.title }}</h2>
      <p v-if="loading" class="reader-loading">正在获取正文...</p>
      <template v-else>
        <p
          v-for="(paragraph, index) in selectedChapter.content.split('\n')"
          :key="index"
          :style="{ fontSize: `${fontSize}px` }"
        >
          {{ paragraph }}
        </p>
      </template>
    </article>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { ArrowRight, BookOpen, Bookmark, Clock3, Download, Rss } from 'lucide-vue-next'
import type { Book } from '../../services/api'

const props = defineProps<{ kind: 'downloads' | 'rss' | 'history' | 'bookmarks'; books: Book[] }>()
const emit = defineEmits<{ open: [book: Book] }>()

const meta = computed(() => {
  const values = {
    downloads: {
      icon: Download,
      eyebrow: '本地传输',
      title: '下载',
      description: '管理正在导入和已完成的书籍。',
      empty: '暂无下载任务。导入 TXT 或 EPUB 后会显示在这里。',
    },
    rss: {
      icon: Rss,
      eyebrow: '订阅更新',
      title: 'RSS',
      description: '订阅书源更新，及时发现新章节。',
      empty: '还没有 RSS 订阅，先去书源页面添加一个可靠的来源。',
    },
    history: {
      icon: Clock3,
      eyebrow: '最近阅读',
      title: '历史',
      description: '继续阅读最近打开的书籍。',
      empty: '还没有阅读记录。打开一本书后，它会出现在这里。',
    },
    bookmarks: {
      icon: Bookmark,
      eyebrow: '阅读标记',
      title: '书签',
      description: '集中查看你在阅读器里保存的章节标记。',
      empty: '还没有书签。阅读时点击书签按钮即可保存当前位置。',
    },
  } as const
  return values[props.kind]
})
const visibleBooks = computed(() =>
  props.kind === 'history'
    ? [...props.books].sort((a, b) => b.updated_at.localeCompare(a.updated_at)).slice(0, 8)
    : props.books.slice(0, 8),
)
</script>

<template>
  <section class="library-page">
    <div class="library-hero">
      <div class="library-hero-icon"><component :is="meta.icon" :size="22" /></div>
      <div>
        <p>{{ meta.eyebrow }}</p>
        <h1>{{ meta.title }}</h1>
        <span>{{ meta.description }}</span>
      </div>
    </div>
    <div v-if="visibleBooks.length" class="library-list">
      <button
        v-for="book in visibleBooks"
        :key="book.id"
        class="library-book"
        type="button"
        @click="emit('open', book)"
      >
        <span class="library-book-cover"
          ><img v-if="book.cover_data" :src="book.cover_data" :alt="book.title" /><BookOpen v-else :size="18"
        /></span>
        <span class="library-book-meta"
          ><strong>{{ book.title }}</strong
          ><small>{{ book.author || '未知作者' }} · {{ book.chapter_count }} 章</small></span
        >
        <ArrowRight :size="17" class="library-book-arrow" />
      </button>
    </div>
    <div v-else class="library-empty">
      <component :is="meta.icon" :size="40" stroke-width="1.5" />
      <h2>{{ meta.empty }}</h2>
      <p>从左侧导航选择其他功能继续。</p>
    </div>
  </section>
</template>

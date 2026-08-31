<script setup lang="ts">
import type { Book, BookshelfGroup } from '../../services/api'

defineProps<{
  books: Book[]
  groups: BookshelfGroup[]
}>()

const emit = defineEmits<{
  open: [book: Book]
  move: [book: Book, event: Event]
  remove: [book: Book]
  choose: []
}>()
</script>

<template>
  <div v-if="books.length" class="book-grid">
    <article
      v-for="book in books"
      :key="book.id"
      class="book-item"
      tabindex="0"
      @click="emit('open', book)"
      @keydown.enter="emit('open', book)"
    >
      <div class="book-cover">
        <img v-if="book.cover_data" :src="book.cover_data" :alt="book.title" />
        <template v-else>{{ book.title.slice(0, 1) }}</template>
      </div>
      <div class="book-meta">
        <h2>{{ book.title }}</h2>
        <p>{{ book.chapter_count }} 个章节</p>
        <select aria-label="移动到分组" @click.stop @change="emit('move', book, $event)">
          <option :value="book.group_id ?? ''">当前分组</option>
          <option v-for="group in groups" :key="group.id" :value="group.id">{{ group.name }}</option>
        </select>
        <button type="button" class="delete-button" @click.stop="emit('remove', book)">删除</button>
      </div>
    </article>
  </div>
  <div v-else class="empty-state">
    <div class="empty-icon">+</div>
    <h2>书架还是空的</h2>
    <p>导入 TXT 或 EPUB，开始你的第一本书。</p>
    <button type="button" class="secondary" @click="emit('choose')">选择文件</button>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { X } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import type { Book } from '@/services/api'

const props = defineProps<{
  book: Book
  /** 当前章节在目录里的下标（0 基）与目录总章数；两者都没有时不渲染进度行。 */
  chapterIndex: number
  chapterCount: number
}>()

defineEmits<{ close: [] }>()

/**
 * 书籍简介来自 `books.intro`（书源 `ruleBookInfo.intro`）。它只属于书籍信息，
 * 不属于任何一章正文 —— 之前它被渲染在每章正文流顶部，于是每章都重复一次。
 * 部分书源会把更新时间/源站拼进简介，所以按行分段渲染而不是压成一段。
 */
const introLines = computed(() =>
  (props.book.intro ?? '')
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter((line) => line.length > 0),
)

/** 只列真实存在的字段：缺字段的行直接不出现，不填占位值。 */
const rows = computed(() => {
  const items: { label: string; value: string }[] = []
  if (props.book.kind) items.push({ label: '分类', value: props.book.kind })
  if (props.book.latest_chapter) items.push({ label: '最新章节', value: props.book.latest_chapter })
  if (props.book.chapter_count) items.push({ label: '章节', value: `共 ${props.book.chapter_count} 章` })
  if (props.chapterCount) {
    items.push({ label: '阅读进度', value: `第 ${props.chapterIndex + 1} / ${props.chapterCount} 章` })
  }
  return items
})
</script>

<template>
  <aside class="reader-book">
    <div class="reader-book-head">
      <strong class="text-sm">书籍信息</strong>
      <Button variant="ghost" size="icon-sm" aria-label="关闭书籍信息" @click="$emit('close')"><X /></Button>
    </div>

    <div class="reader-book-scroll">
      <div class="reader-book-hero">
        <img v-if="book.cover_data" class="reader-book-cover" :src="book.cover_data" :alt="book.title" />
        <div v-else class="reader-book-cover-empty">{{ book.title.slice(0, 1) }}</div>
        <div class="min-w-0 flex-1">
          <h3 class="reader-book-title">{{ book.title }}</h3>
          <p v-if="book.author" class="reader-book-author">{{ book.author }}</p>
        </div>
      </div>

      <dl v-if="rows.length" class="reader-book-meta">
        <div v-for="row in rows" :key="row.label">
          <dt>{{ row.label }}</dt>
          <dd>{{ row.value }}</dd>
        </div>
      </dl>

      <div class="reader-book-section">简介</div>
      <p v-for="(line, index) in introLines" :key="index" class="reader-book-intro">{{ line }}</p>
      <p v-if="!introLines.length" class="reader-book-empty">这本书还没有简介。</p>
    </div>
  </aside>
</template>

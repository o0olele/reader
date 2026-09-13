<script setup lang="ts">
import { BookOpen } from 'lucide-vue-next'
import { useRouter } from 'vue-router'
import { Button } from '@/components/ui/button'
import { useShellContext } from '@/app/shellKeys'
import type { Book } from '@/services/api'

const router = useRouter()
const { bookshelf, openBook } = useShellContext()

/** Shown when the route has no book open: pick one straight from the shelf. */
async function openFromShelf(book: Book) {
  await openBook(book)
  await router.push({ name: 'read', params: { bookId: String(book.id) } })
}
</script>

<template>
  <div class="flex min-h-0 flex-1 items-center justify-center p-10">
    <div class="w-full max-w-md text-center">
      <BookOpen :size="34" class="mx-auto mb-3 text-muted-foreground" />
      <h2 class="text-sm font-semibold">还没有打开的书</h2>
      <p class="mt-1 text-xs text-muted-foreground">从书架选一本开始阅读，URL 里的 bookId 会被记录为深链接。</p>
      <div v-if="bookshelf.books.length" class="mt-4 grid gap-1.5 text-left">
        <Button
          v-for="book in bookshelf.books.slice(0, 6)"
          :key="book.id"
          variant="outline"
          size="sm"
          class="justify-start"
          @click="openFromShelf(book)"
        >
          <BookOpen />{{ book.title }}
        </Button>
      </div>
      <Button v-else class="mt-4" size="sm" @click="router.push({ name: 'bookshelf' })">去书架导入</Button>
    </div>
  </div>
</template>

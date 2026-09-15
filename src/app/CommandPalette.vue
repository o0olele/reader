<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useRouter } from 'vue-router'
import { BookOpen, Database, Search } from 'lucide-vue-next'
import {
  CommandDialog,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandSeparator,
} from '@/components/ui/command'
import { routeKeywords, routeTitles } from './shellNav'
import { useCommandPalette } from './useCommandPalette'
import { useShellContext } from './shellKeys'
import type { Book, BookSource } from '../services/api'

/** The shelf and the source table hold thousands of rows; the palette shows a
 *  few of each and hands the rest of the job to the online search. */
const BOOK_LIMIT = 8
const SOURCE_LIMIT = 5

const router = useRouter()
const { open, closePalette } = useCommandPalette()
const { bookshelf, sources, search, openBook } = useShellContext()

/** Mirrors the input element, which `Command` owns: the value arrives through
 *  `@input`, and the dialog drops it (with its content) on close. */
const query = ref('')
const keyword = computed(() => query.value.trim())
const needle = computed(() => keyword.value.toLowerCase())

/**
 * One haystack per row: every field the row is matched on. It is also handed to
 * `CommandItem` as `keywords`, so the list's own text filter cannot hide a row
 * this component has already decided matches — a book is listed under its
 * title, but its author and group have to match too.
 */
function haystack(parts: Array<string | number | undefined>): string {
  return parts.filter((part) => part !== undefined && part !== '').join(' ')
}

const matchedBooks = computed(() => {
  if (!needle.value) return []
  const groups = new Map(bookshelf.groups.map((group) => [group.id, group.name]))
  return bookshelf.books
    .map((book) => ({
      book,
      text: haystack([
        book.title,
        book.author,
        book.group_id === undefined ? undefined : groups.get(book.group_id),
        book.source_id ? '在线' : '本地',
        '书籍 book',
      ]),
    }))
    .filter((row) => row.text.toLowerCase().includes(needle.value))
})

const matchedSources = computed(() => {
  if (!needle.value) return []
  return sources.sources
    .map((source) => ({
      source,
      text: haystack([source.name, source.source_group, source.base_url, '书源 source']),
    }))
    .filter((row) => row.text.toLowerCase().includes(needle.value))
})

const pages = Object.entries(routeTitles).map(([name, title]) => ({
  name,
  title,
  text: haystack([title, routeKeywords[name]]),
}))

/** An empty query lists every page, which is what the palette opens on. */
const matchedPages = computed(() =>
  needle.value ? pages.filter((page) => page.text.toLowerCase().includes(needle.value)) : pages,
)

const localMiss = computed(
  () =>
    Boolean(keyword.value) && !matchedBooks.value.length && !matchedSources.value.length && !matchedPages.value.length,
)

function onInput(event: Event) {
  query.value = (event.target as HTMLInputElement).value
}

async function go(name: string) {
  closePalette()
  await router.push({ name })
}

/** Opening loads the book first, so the reader never paints its empty state. */
async function openShelfBook(book: Book) {
  closePalette()
  await openBook(book)
  await router.push({ name: 'read', params: { bookId: String(book.id) } })
}

/** Lands on 书源管理 with that source filtered out of the hundreds around it. */
async function openSource(source: BookSource) {
  closePalette()
  await router.push({ name: 'sources', query: { q: source.name } })
}

async function runSearch() {
  const value = keyword.value
  if (!value) return
  closePalette()
  await router.push({ name: 'search' })
  search.query = value
  await search.run()
}

// The dialog unmounts its content on close, so the mirrored query has to follow
// or the next open would filter on the previous keyword.
watch(open, (value) => {
  if (!value) query.value = ''
})
</script>

<template>
  <CommandDialog v-model:open="open">
    <CommandInput placeholder="搜索书架、书源，或跳转页面…" @input="onInput" />
    <CommandList>
      <CommandGroup v-if="matchedBooks.length" :heading="`书籍 · ${matchedBooks.length}`">
        <CommandItem
          v-for="row in matchedBooks.slice(0, BOOK_LIMIT)"
          :key="`book-${row.book.id}`"
          :value="`book-${row.book.id}`"
          :keywords="row.text"
          @select="openShelfBook(row.book)"
        >
          <BookOpen />
          <span class="truncate">{{ row.book.title }}</span>
          <span class="ml-auto shrink-0 text-xs text-muted-foreground">{{ row.book.author || '未知作者' }}</span>
        </CommandItem>
        <div v-if="matchedBooks.length > BOOK_LIMIT" class="px-2 py-1.5 text-xs text-muted-foreground">
          还有 {{ matchedBooks.length - BOOK_LIMIT }} 本匹配，到书架页可看全部。
        </div>
      </CommandGroup>

      <CommandGroup v-if="matchedSources.length" :heading="`书源 · ${matchedSources.length}`">
        <CommandItem
          v-for="row in matchedSources.slice(0, SOURCE_LIMIT)"
          :key="`source-${row.source.id}`"
          :value="`source-${row.source.id}`"
          :keywords="row.text"
          @select="openSource(row.source)"
        >
          <Database />
          <span class="truncate">{{ row.source.name }}</span>
          <span v-if="row.source.source_group" class="ml-auto shrink-0 text-xs text-muted-foreground">
            {{ row.source.source_group }}
          </span>
        </CommandItem>
      </CommandGroup>

      <CommandGroup v-if="matchedPages.length" heading="跳转">
        <CommandItem
          v-for="page in matchedPages"
          :key="page.name"
          :value="`page-${page.name}`"
          :keywords="page.text"
          @select="go(page.name)"
        >
          {{ page.title }}
        </CommandItem>
      </CommandGroup>

      <div v-if="localMiss" class="px-3 py-3 text-center text-xs text-muted-foreground">
        书架、书源与页面里没有匹配「{{ keyword }}」的内容。
      </div>

      <template v-if="keyword">
        <CommandSeparator />
        <CommandGroup heading="在线搜索">
          <CommandItem :value="`search-${keyword}`" :keywords="`搜索 在线书籍 ${keyword}`" @select="runSearch">
            <Search />
            <span>用「{{ keyword }}」搜索在线书籍</span>
          </CommandItem>
        </CommandGroup>
      </template>
    </CommandList>
  </CommandDialog>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { Search } from 'lucide-vue-next'
import {
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandSeparator,
} from '@/components/ui/command'
import { routeTitles } from './shellNav'
import { useCommandPalette } from './useCommandPalette'
import { useShellContext } from './shellKeys'

const router = useRouter()
const { open, closePalette } = useCommandPalette()
const { search } = useShellContext()
const query = ref('')

const pages = Object.entries(routeTitles).map(([name, title]) => ({ name, title }))

function onInput(event: Event) {
  query.value = (event.target as HTMLInputElement).value
}

async function go(name: string) {
  closePalette()
  await router.push({ name })
}

async function runSearch() {
  const keyword = query.value.trim()
  closePalette()
  await router.push({ name: 'search' })
  if (!keyword) return
  search.query = keyword
  await search.run()
}
</script>

<template>
  <CommandDialog v-model:open="open">
    <CommandInput placeholder="搜索书籍、作者、书源，或跳转页面…" @input="onInput" />
    <CommandList>
      <CommandEmpty>没有匹配的结果。</CommandEmpty>
      <CommandGroup heading="搜索">
        <CommandItem :value="`搜索 ${query}`" @select="runSearch">
          <Search />
          <span>用「{{ query || '关键词' }}」搜索在线书籍</span>
        </CommandItem>
      </CommandGroup>
      <CommandSeparator />
      <CommandGroup heading="跳转">
        <CommandItem v-for="page in pages" :key="page.name" :value="page.title" @select="go(page.name)">
          {{ page.title }}
        </CommandItem>
      </CommandGroup>
    </CommandList>
  </CommandDialog>
</template>

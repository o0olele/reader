<script setup lang="ts">
import { ref } from 'vue'
import { FolderPlus } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { useShellContext } from '@/app/shellKeys'
import { notifySuccess } from '@/app/useToast'
import type { BookshelfGroup } from '@/services/api'
import CreateGroupDialog from './CreateGroupDialog.vue'
import { readDraggedBookIds } from './shelfDrag'

const { bookshelf } = useShellContext()
const createOpen = ref(false)
const dropTarget = ref<number | null>(null)

async function drop(event: DragEvent, group: BookshelfGroup) {
  dropTarget.value = null
  const ids = readDraggedBookIds(event)
  if (!ids.length) return
  const moved = await bookshelf.moveBooks(ids, group.id)
  if (moved) notifySuccess(`已将 ${moved} 本书移动到「${group.name}」`)
}
</script>

<template>
  <aside class="w-52 shrink-0 overflow-y-auto border-r bg-card p-2">
    <button
      type="button"
      class="mb-0.5 flex w-full items-center justify-between rounded-md px-2.5 py-1.5 text-sm hover:bg-accent"
      :class="bookshelf.activeGroup === null ? 'bg-accent font-medium' : ''"
      @click="bookshelf.activeGroup = null"
    >
      <span>全部书籍</span><span class="text-xs text-muted-foreground">{{ bookshelf.books.length }}</span>
    </button>
    <button
      v-for="group in bookshelf.groups"
      :key="group.id"
      type="button"
      class="mb-0.5 flex w-full items-center justify-between rounded-md border border-transparent px-2.5 py-1.5 text-sm hover:bg-accent"
      :class="[
        bookshelf.activeGroup === group.id ? 'bg-accent font-medium' : '',
        dropTarget === group.id ? 'border-ring bg-accent ring-1 ring-ring' : '',
      ]"
      @click="bookshelf.activeGroup = group.id"
      @dragover.prevent="dropTarget = group.id"
      @dragleave="dropTarget = dropTarget === group.id ? null : dropTarget"
      @drop.prevent="drop($event, group)"
    >
      <span class="truncate">{{ group.name }}</span>
      <span class="text-xs text-muted-foreground">{{ group.book_count }}</span>
    </button>
    <Button variant="ghost" size="sm" class="mt-1 w-full justify-start" @click="createOpen = true">
      <FolderPlus /> 新建分组
    </Button>
    <CreateGroupDialog v-model:open="createOpen" />
  </aside>
</template>

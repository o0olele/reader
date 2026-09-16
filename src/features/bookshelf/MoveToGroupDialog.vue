<script setup lang="ts">
import { computed, ref } from 'vue'
import { Check, FolderInput } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { useShellContext } from '@/app/shellKeys'
import { notifySuccess } from '@/app/useToast'

const props = defineProps<{ bookIds: number[] }>()
const emit = defineEmits<{ moved: [count: number] }>()
const open = defineModel<boolean>('open', { default: false })
const { bookshelf } = useShellContext()
const moving = ref<number | null>(null)

/** Groups that already hold every selected book; moving there would be a no-op. */
const currentGroups = computed(() => {
  const ids = new Set<number>()
  for (const book of bookshelf.books) {
    if (props.bookIds.includes(book.id) && book.group_id) ids.add(book.group_id)
  }
  return ids
})

async function choose(groupId: number, groupName: string) {
  if (moving.value !== null || !props.bookIds.length) return
  moving.value = groupId
  try {
    const count = await bookshelf.moveBooks(props.bookIds, groupId)
    if (!count) return
    open.value = false
    emit('moved', count)
    notifySuccess(`已将 ${count} 本书移动到「${groupName}」`)
  } finally {
    moving.value = null
  }
}
</script>

<template>
  <Dialog v-model:open="open">
    <DialogContent class="sm:max-w-sm">
      <DialogHeader>
        <DialogTitle>移动到分组</DialogTitle>
        <DialogDescription>
          {{ bookIds.length > 1 ? `为选中的 ${bookIds.length} 本书选择目标分组。` : '为这本书选择目标分组。' }}
        </DialogDescription>
      </DialogHeader>
      <div v-if="bookshelf.groups.length" class="grid max-h-72 gap-1.5 overflow-y-auto pr-1">
        <button
          v-for="group in bookshelf.groups"
          :key="group.id"
          type="button"
          class="flex w-full items-center justify-between gap-2 rounded-md border px-3 py-2 text-left text-sm transition-colors hover:bg-accent disabled:cursor-not-allowed disabled:opacity-60"
          :disabled="moving !== null || currentGroups.has(group.id)"
          @click="choose(group.id, group.name)"
        >
          <span class="flex min-w-0 items-center gap-2">
            <FolderInput class="size-4 shrink-0 text-muted-foreground" />
            <span class="truncate">{{ group.name }}</span>
          </span>
          <Check v-if="currentGroups.has(group.id)" class="size-4 shrink-0 text-primary" />
          <span v-else class="shrink-0 text-xs text-muted-foreground">{{ group.book_count }} 本</span>
        </button>
      </div>
      <p v-else class="py-6 text-center text-sm text-muted-foreground">还没有分组，先在左侧新建一个分组。</p>
      <DialogFooter>
        <Button type="button" variant="ghost" @click="open = false">取消</Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>

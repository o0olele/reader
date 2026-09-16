<script setup lang="ts">
import { ref, watch } from 'vue'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { useShellContext } from '@/app/shellKeys'

const open = defineModel<boolean>('open', { default: false })
const { bookshelf } = useShellContext()
const name = ref('')
const creating = ref(false)

watch(open, (value) => {
  if (value) name.value = ''
})

async function submit() {
  const trimmed = name.value.trim()
  if (!trimmed || creating.value) return
  creating.value = true
  try {
    if (await bookshelf.addGroup(trimmed)) open.value = false
  } finally {
    creating.value = false
  }
}
</script>

<template>
  <Dialog v-model:open="open">
    <DialogContent class="sm:max-w-sm">
      <DialogHeader>
        <DialogTitle>新建分组</DialogTitle>
        <DialogDescription>为书架创建一个分组，便于归类整理的书籍。</DialogDescription>
      </DialogHeader>
      <form class="grid gap-4" @submit.prevent="submit">
        <label class="grid gap-1.5 text-xs">
          <span class="text-muted-foreground">分组名称</span>
          <Input v-model="name" placeholder="例如：待读" aria-label="分组名称" />
        </label>
        <DialogFooter>
          <Button type="button" variant="ghost" @click="open = false">取消</Button>
          <Button type="submit" :disabled="!name.trim() || creating">{{ creating ? '创建中…' : '创建' }}</Button>
        </DialogFooter>
      </form>
    </DialogContent>
  </Dialog>
</template>

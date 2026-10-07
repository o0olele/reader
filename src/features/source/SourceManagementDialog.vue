<script setup lang="ts">
import { ref, watch } from 'vue'
import { Loader2 } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { useShellContext } from '@/app/shellKeys'
import type { BookSource } from '@/services/api'

const props = defineProps<{ source: BookSource }>()
const open = defineModel<boolean>('open', { default: false })
const { sources } = useShellContext()
const saving = ref(false)
const draft = ref(sources.managementDraft(props.source))
watch([open, () => props.source], ([isOpen]) => {
  if (isOpen) draft.value = sources.managementDraft(props.source)
})
async function submit() {
  saving.value = true
  try {
    if (await sources.saveManagement(props.source)) open.value = false
  } finally {
    saving.value = false
  }
}
</script>

<template>
  <Dialog v-model:open="open">
    <DialogContent>
      <DialogHeader>
        <DialogTitle>管理书源</DialogTitle>
        <DialogDescription class="break-all">{{ source.name }}</DialogDescription>
      </DialogHeader>
      <form class="space-y-5" @submit.prevent="submit">
        <label class="grid gap-2 text-xs font-medium">
          分组
          <Input
            v-model="draft.group"
            maxlength="80"
            placeholder="例如：小说、漫画、常用"
            aria-label="书源分组"
            :disabled="saving"
          />
          <span class="font-normal text-muted-foreground">留空则放入「未分组」。</span>
        </label>
        <div class="grid grid-cols-2 gap-4">
          <label class="grid gap-2 text-xs font-medium">
            排序
            <Input
              v-model.number="draft.order"
              type="number"
              step="1"
              required
              aria-label="书源排序值"
              :disabled="saving"
            />
            <span class="font-normal text-muted-foreground">数值越小，排列越靠前。</span>
          </label>
          <label class="grid gap-2 text-xs font-medium">
            权重
            <Input
              v-model.number="draft.weight"
              type="number"
              step="1"
              required
              aria-label="书源权重"
              :disabled="saving"
            />
            <span class="font-normal text-muted-foreground">排序相同时，权重高的优先。</span>
          </label>
        </div>
        <DialogFooter>
          <Button type="button" variant="ghost" :disabled="saving" @click="open = false">取消</Button>
          <Button type="submit" :disabled="saving"
            ><Loader2 v-if="saving" class="animate-spin" />{{ saving ? '保存中…' : '保存设置' }}</Button
          >
        </DialogFooter>
      </form>
    </DialogContent>
  </Dialog>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { FileJson, Link, Loader2, Upload } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { useShellContext } from '@/app/shellKeys'

const open = defineModel<boolean>('open', { default: false })
const { sources } = useShellContext()
const fileInput = ref<HTMLInputElement>()
async function importFile(event: Event) {
  if (await sources.importFromFile(event)) open.value = false
}
async function importUrl() {
  if (await sources.importFromUrl()) open.value = false
}
</script>

<template>
  <Dialog v-model:open="open">
    <DialogContent>
      <DialogHeader>
        <DialogTitle>导入书源</DialogTitle>
        <DialogDescription>支持 Legado 格式的 JSON 文件，也可以粘贴书源链接。</DialogDescription>
      </DialogHeader>
      <input ref="fileInput" type="file" accept=".json,application/json" class="hidden" @change="importFile" />
      <button
        type="button"
        class="flex flex-col items-center gap-3 rounded-lg border border-dashed bg-card p-7 transition-colors hover:border-ring hover:bg-accent/50 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50"
        :disabled="sources.importing"
        @click="fileInput?.click()"
      >
        <span class="grid size-11 place-items-center rounded-xl bg-muted"><FileJson class="size-5" /></span>
        <span class="text-sm font-medium">选择 JSON 文件</span>
        <span class="text-xs text-muted-foreground">一次导入一个或多个书源</span>
      </button>
      <div class="flex items-center gap-3 text-xs text-muted-foreground">
        <span class="h-px flex-1 bg-border" />或从链接导入<span class="h-px flex-1 bg-border" />
      </div>
      <form class="space-y-3" @submit.prevent="importUrl">
        <label class="flex items-center gap-2 text-xs font-medium" for="source-import-url"
          ><Link class="size-3.5" />书源链接</label
        >
        <Input
          id="source-import-url"
          v-model="sources.sourceUrl"
          type="url"
          required
          placeholder="https://example.com/bookSources.json"
          :disabled="sources.importing"
        />
        <Button class="w-full" type="submit" :disabled="sources.importing || !sources.sourceUrl.trim()">
          <Loader2 v-if="sources.importing" class="animate-spin" /><Upload v-else />
          {{ sources.importing ? '正在导入…' : '从链接导入' }}
        </Button>
      </form>
      <p class="text-xs leading-relaxed text-muted-foreground">同名书源会更新已有配置，导入后可测试搜索结果。</p>
    </DialogContent>
  </Dialog>
</template>

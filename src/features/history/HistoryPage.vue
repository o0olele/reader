<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Clock3, RefreshCw, Trash2 } from 'lucide-vue-next'
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
import EmptyState from '@/components/EmptyState.vue'
import PageBody from '@/components/PageBody.vue'
import PageHeader from '@/components/PageHeader.vue'
import HistoryRow from './HistoryRow.vue'
import { useReadingHistory } from './useReadingHistory'

const history = useReadingHistory()
const confirming = ref(false)

async function clearAll() {
  await history.clearAll()
  confirming.value = false
}

onMounted(history.refresh)
</script>

<template>
  <div class="flex h-full flex-col">
    <PageHeader title="历史" :subtitle="`${history.items.length} 本有阅读记录 · 按最后阅读时间排序`">
      <Button variant="outline" size="sm" :disabled="history.loading" @click="history.refresh()">
        <RefreshCw :class="history.loading ? 'animate-spin' : ''" /> 刷新
      </Button>
      <Button variant="destructive" size="sm" :disabled="!history.items.length" @click="confirming = true">
        <Trash2 /> 清空历史
      </Button>
    </PageHeader>

    <PageBody>
      <div class="flex items-center gap-2">
        <Input
          v-model="history.keyword"
          class="h-8 max-w-xs"
          placeholder="搜索书名、作者或章节"
          aria-label="搜索阅读历史"
        />
        <span v-if="history.keyword" class="text-xs text-muted-foreground">
          匹配 {{ history.visible.length }} / {{ history.items.length }} 本
        </span>
      </div>

      <p v-if="history.error" role="alert" class="mt-4 rounded-md border border-dashed p-4 text-xs text-destructive">
        {{ history.error }}
      </p>

      <template v-else-if="history.groups.length">
        <section v-for="group in history.groups" :key="group.label" class="mt-4">
          <h2 class="mb-2 text-xs font-medium text-muted-foreground">
            {{ group.label }} · {{ group.entries.length }} 本
          </h2>
          <ul class="grid gap-2">
            <HistoryRow
              v-for="entry in group.entries"
              :key="entry.book_id"
              :entry="entry"
              @clear="history.remove(entry)"
            />
          </ul>
        </section>
      </template>

      <p
        v-else-if="history.loading"
        class="mt-4 rounded-md border border-dashed p-6 text-center text-xs text-muted-foreground"
      >
        正在读取阅读记录…
      </p>

      <EmptyState
        v-else
        class="mt-4"
        :icon="Clock3"
        :title="history.keyword ? '没有匹配的阅读记录' : '还没有阅读记录'"
        :description="
          history.keyword
            ? '换个书名、作者或章节关键词试试。'
            : '打开一本书读一会儿，这里会按最后阅读时间汇总进度与时长。'
        "
      />
    </PageBody>

    <Dialog v-model:open="confirming">
      <DialogContent class="sm:max-w-sm">
        <DialogHeader>
          <DialogTitle>清空阅读历史？</DialogTitle>
          <DialogDescription>
            将删除全部 {{ history.items.length }} 本书的阅读时长与阅读位置，首页的今日时长、连续天数与累计时长一并归零。
            <strong class="font-medium text-foreground">书籍本身不会被删除。</strong>
            此操作不可撤销。
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <Button variant="ghost" @click="confirming = false">取消</Button>
          <Button variant="destructive" @click="clearAll"><Trash2 /> 清空</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Database, Download, Trash2 } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import {
  clearChapterCache,
  getCacheStats,
  getErrorMessage,
  getReaderPrefetchNum,
  setCacheQuota,
  setReaderPrefetchNum,
  type CacheStats,
} from '@/services/api'
import { notifyError, notifySuccess } from '@/app/useToast'

const cache = ref<CacheStats>()
const quotaMb = ref(1024)
const prefetchNum = ref(10)
const savedPrefetchNum = ref(10)
const busy = ref(false)
const prefetchBusy = ref(false)

function formatBytes(bytes: number) {
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / 1024 / 1024).toFixed(1)} MB`
}

async function run(action: () => Promise<CacheStats>) {
  busy.value = true
  try {
    cache.value = await action()
    quotaMb.value = Math.round(cache.value.quota_bytes / 1024 / 1024)
    notifySuccess('缓存设置已更新')
  } catch (cause) {
    notifyError(getErrorMessage(cause))
  } finally {
    busy.value = false
  }
}

async function savePrefetch() {
  prefetchBusy.value = true
  try {
    prefetchNum.value = savedPrefetchNum.value = await setReaderPrefetchNum(prefetchNum.value)
    notifySuccess('预下载设置已更新')
  } catch (cause) {
    notifyError(getErrorMessage(cause))
  } finally {
    prefetchBusy.value = false
  }
}

onMounted(async () => {
  try {
    cache.value = await getCacheStats()
    quotaMb.value = Math.round(cache.value.quota_bytes / 1024 / 1024)
  } catch (cause) {
    notifyError(getErrorMessage(cause))
  }
  try {
    prefetchNum.value = savedPrefetchNum.value = await getReaderPrefetchNum()
  } catch (cause) {
    notifyError(getErrorMessage(cause))
  }
})
</script>

<template>
  <div class="grid max-w-2xl gap-4">
    <h2 class="text-sm font-semibold">正文缓存</h2>
    <div class="flex flex-wrap items-center gap-3 rounded-md border bg-card p-3">
      <Database :size="18" class="text-muted-foreground" />
      <div class="min-w-0 flex-1 text-xs">
        <template v-if="cache">
          <strong class="block text-sm">{{ formatBytes(cache.used_bytes) }}</strong>
          <span class="text-muted-foreground"
            >{{ cache.cached_chapters }} 章 · 上限 {{ formatBytes(cache.quota_bytes) }}</span
          >
        </template>
        <span v-else class="text-muted-foreground">读取中…</span>
      </div>
      <label class="flex items-center gap-1.5 text-xs text-muted-foreground">
        上限
        <Input v-model.number="quotaMb" class="h-8 w-24" type="number" min="64" max="102400" aria-label="缓存上限 MB" />
        MB
      </label>
      <Button variant="outline" size="sm" :disabled="busy" @click="run(() => setCacheQuota(quotaMb))">保存配额</Button>
      <Button
        variant="outline"
        size="sm"
        class="text-destructive"
        :disabled="busy || !cache?.cached_chapters"
        @click="run(clearChapterCache)"
      >
        <Trash2 /> 清空
      </Button>
    </div>

    <div class="flex flex-wrap items-center gap-3 rounded-md border bg-card p-3">
      <Download :size="18" class="text-muted-foreground" />
      <div class="min-w-0 flex-1 text-xs">
        <strong class="block text-sm">预下载章节数</strong>
        <span class="text-muted-foreground">
          阅读时提前缓存当前章之后的 {{ prefetchNum }} 章，以及之前的至多 5 章；翻到该章即为缓存命中，0 表示关闭。
        </span>
      </div>
      <label class="flex items-center gap-1.5 text-xs text-muted-foreground">
        <Input v-model.number="prefetchNum" class="h-8 w-24" type="number" min="0" max="50" aria-label="预下载章节数" />
        章
      </label>
      <Button
        variant="outline"
        size="sm"
        :disabled="prefetchBusy || prefetchNum === savedPrefetchNum"
        @click="savePrefetch"
      >
        保存预下载
      </Button>
    </div>

    <p class="text-xs text-muted-foreground">下载任务列表在「下载」页；这里只管理正文缓存配额与阅读时的预下载窗口。</p>
  </div>
</template>

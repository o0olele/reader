<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Database, Trash2 } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { clearChapterCache, getCacheStats, getErrorMessage, setCacheQuota, type CacheStats } from '@/services/api'
import { notifyError, notifySuccess } from '@/app/useToast'

const cache = ref<CacheStats>()
const quotaMb = ref(1024)
const busy = ref(false)

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

onMounted(async () => {
  try {
    cache.value = await getCacheStats()
    quotaMb.value = Math.round(cache.value.quota_bytes / 1024 / 1024)
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
    <p class="text-xs text-muted-foreground">下载任务列表在「下载」页；这里只管理正文缓存配额。</p>
  </div>
</template>

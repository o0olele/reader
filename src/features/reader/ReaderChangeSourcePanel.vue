<script setup lang="ts">
import { computed } from 'vue'
import {
  ArrowDownToLine,
  ArrowLeftRight,
  ArrowUpToLine,
  Ban,
  Loader2,
  MoreHorizontal,
  Pause,
  RefreshCw,
  X,
} from 'lucide-vue-next'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'
import { Progress } from '@/components/ui/progress'
import type { ChangeSourceCandidate, useChangeSource } from './useChangeSource'

const props = defineProps<{
  /** 换源状态（`useChangeSource`），与书籍信息 / 正文搜索共用右侧槽位。 */
  change: ReturnType<typeof useChangeSource>
}>()

const emit = defineEmits<{ close: [] }>()

const percent = computed(() =>
  props.change.total ? Math.round((props.change.completed / props.change.total) * 100) : 0,
)

const OPTIONS = [
  { key: 'checkAuthor', label: '校验作者', hint: '书名相同还要作者对得上，避免同名书换错源' },
  { key: 'loadInfo', label: '加载详情', hint: '换源前先读封面与简介' },
  { key: 'loadToc', label: '加载目录', hint: '换源前先取目录（同时验证该源可用）' },
] as const

function chapterLabel(candidate: ChangeSourceCandidate) {
  const latest = candidate.preview?.latest_chapter ?? candidate.result.latest_chapter
  if (latest) return `最新：${latest}`
  if (candidate.result.word_count) return candidate.result.word_count
  return '暂无最新章节信息'
}
</script>

<template>
  <aside class="reader-source">
    <div class="reader-source-head">
      <strong class="flex items-center gap-1.5 text-sm"><ArrowLeftRight :size="15" />换源</strong>
      <Button variant="ghost" size="icon-sm" aria-label="关闭换源" title="关闭换源" @click="emit('close')">
        <X />
      </Button>
    </div>

    <p class="reader-source-hint">
      《{{ change.bookTitle }}》当前：{{ change.currentSourceName }}。换源后目录会重新抓取，并回到原来的那一章。
    </p>

    <div class="reader-source-tools">
      <input
        class="reader-source-filter"
        type="search"
        :value="change.filter"
        placeholder="筛选书源名 / 最新章节"
        aria-label="筛选换源结果"
        @input="change.setFilter(($event.target as HTMLInputElement).value)"
      />
      <Button
        variant="ghost"
        size="icon-sm"
        :title="change.searching ? '停止搜索' : '重新搜索'"
        :aria-label="change.searching ? '停止搜索' : '重新搜索'"
        @click="change.searching ? change.stop() : change.run()"
      >
        <Pause v-if="change.searching" /><RefreshCw v-else />
      </Button>
    </div>

    <div class="reader-source-options">
      <button
        v-for="option in OPTIONS"
        :key="option.key"
        type="button"
        :class="['search-toggle', { active: change.settings[option.key] }]"
        :title="option.hint"
        @click="change.toggleOption(option.key)"
      >
        {{ option.label }}
      </button>
    </div>

    <div v-if="change.searching || change.total" class="reader-source-progress">
      <span class="min-w-0 flex-1 truncate">
        已搜索 {{ change.completed }}/{{ change.total }} · 结果 {{ change.candidates.length }}
        <template v-if="change.searching && change.lastSourceName"> · {{ change.lastSourceName }}</template>
      </span>
      <Progress :model-value="percent" class="h-1 w-16" />
    </div>

    <div class="reader-source-list">
      <p v-if="!change.visible.length" class="reader-source-empty">
        {{ change.searching ? '正在搜索各书源，结果会即时显示…' : '没有找到其它书源收录这本书' }}
      </p>

      <article
        v-for="candidate in change.visible"
        :key="`${candidate.result.source_id}-${candidate.result.url}`"
        class="reader-source-row"
        :class="{
          current: change.isCurrent(candidate),
          broken: Boolean(candidate.error),
        }"
      >
        <div class="min-w-0 flex-1">
          <div class="reader-source-name">
            <span>{{ candidate.result.source_name }}</span>
            <Badge v-if="change.isCurrent(candidate)" variant="secondary">当前</Badge>
          </div>
          <p class="reader-source-meta">{{ candidate.result.author || '作者未知' }} · {{ chapterLabel(candidate) }}</p>
          <p v-if="candidate.preview?.info?.intro" class="reader-source-intro">{{ candidate.preview.info.intro }}</p>
          <p v-if="candidate.preview?.info_error" class="reader-source-note">
            详情读取失败：{{ candidate.preview.info_error }}
          </p>
          <p v-if="candidate.error" class="reader-source-error">{{ candidate.error }}</p>

          <div class="reader-source-actions">
            <Badge v-if="candidate.preview?.chapters.length" variant="outline">
              {{ candidate.preview.chapters.length }} 章
            </Badge>
            <Button
              size="sm"
              :disabled="
                change.isCurrent(candidate) || Boolean(candidate.error) || change.switchingUrl === candidate.result.url
              "
              @click="change.switchTo(candidate)"
            >
              <Loader2 v-if="change.switchingUrl === candidate.result.url" class="animate-spin" />
              <ArrowLeftRight v-else />
              {{ change.isCurrent(candidate) ? '当前' : '换源' }}
            </Button>
            <DropdownMenu>
              <DropdownMenuTrigger as-child>
                <Button variant="ghost" size="icon-sm" :title="`${candidate.result.source_name} 书源操作`">
                  <MoreHorizontal />
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <DropdownMenuItem @select="change.pinCandidate(candidate, true)">
                  <ArrowUpToLine />置顶该书源
                </DropdownMenuItem>
                <DropdownMenuItem @select="change.pinCandidate(candidate, false)">
                  <ArrowDownToLine />置底该书源
                </DropdownMenuItem>
                <DropdownMenuItem @select="change.disableSource(candidate)"> <Ban />停用该书源 </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        </div>
      </article>
    </div>
  </aside>
</template>

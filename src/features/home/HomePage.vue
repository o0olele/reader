<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { BookOpen, Clock3, Flame, Info, Target } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Progress } from '@/components/ui/progress'
import NotConnected from '@/components/NotConnected.vue'
import PageBody from '@/components/PageBody.vue'
import PageHeader from '@/components/PageHeader.vue'
import { getReadingStats, type ReadingStats } from '@/services/api'
import { useShellContext } from '@/app/shellKeys'

const router = useRouter()
const { bookshelf, openBook } = useShellContext()
const stats = ref<ReadingStats>()

const current = computed(() => [...bookshelf.books].sort((a, b) => b.updated_at.localeCompare(a.updated_at))[0])
const greeting = computed(() => {
  const hour = new Date().getHours()
  if (hour < 6) return '夜深了，书友'
  if (hour < 12) return '早上好，书友'
  if (hour < 18) return '下午好，书友'
  return '晚上好，书友'
})
const today = computed(() =>
  new Date().toLocaleDateString('zh-CN', { year: 'numeric', month: 'long', day: 'numeric', weekday: 'long' }),
)
const todayMinutes = computed(() => Math.round((stats.value?.today_seconds ?? 0) / 60))
const goalPercent = computed(() => {
  const goal = stats.value?.daily_goal_minutes ?? 0
  return goal ? Math.min(100, Math.round((todayMinutes.value / goal) * 100)) : 0
})

async function continueReading() {
  const book = current.value
  if (!book) {
    await router.push({ name: 'bookshelf' })
    return
  }
  await openBook(book)
  await router.push({ name: 'read', params: { bookId: String(book.id) } })
}

onMounted(async () => {
  try {
    stats.value = await getReadingStats()
  } catch {
    /* browser preview has no backend */
  }
})
</script>

<template>
  <div class="flex h-full flex-col">
    <PageHeader
      :title="greeting"
      :subtitle="`${today}${stats?.streak_days ? ` · 已连续阅读 ${stats.streak_days} 天` : ''}`"
    >
      <Button size="sm" variant="outline" @click="router.push({ name: 'bookshelf' })"> <BookOpen /> 打开书架 </Button>
    </PageHeader>
    <PageBody>
      <div class="grid grid-cols-[1fr_336px] items-start gap-6">
        <div class="flex flex-col gap-6">
          <Card v-if="current">
            <CardHeader class="pb-2">
              <CardTitle class="text-xs font-medium text-muted-foreground">最近在读</CardTitle>
            </CardHeader>
            <CardContent class="flex gap-5">
              <div
                class="grid h-[132px] w-[92px] shrink-0 place-items-center overflow-hidden rounded-md bg-secondary text-xs text-muted-foreground"
              >
                <img
                  v-if="current.cover_data"
                  :src="current.cover_data"
                  :alt="current.title"
                  class="h-full w-full object-cover"
                />
                <span v-else>{{ current.title.slice(0, 1) }}</span>
              </div>
              <div class="flex min-w-0 flex-col">
                <h2 class="truncate text-lg font-semibold">{{ current.title }}</h2>
                <p class="mt-1 text-xs text-muted-foreground">
                  {{ current.author || '未知作者' }} · 共 {{ current.chapter_count }} 章
                </p>
                <p v-if="current.latest_chapter" class="mt-2 truncate text-xs">最新：{{ current.latest_chapter }}</p>
                <div class="mt-auto flex gap-2 pt-4">
                  <Button size="sm" @click="continueReading"><BookOpen /> 继续阅读</Button>
                  <Button size="sm" variant="outline" @click="router.push({ name: 'bookshelf' })">
                    <Info /> 书架
                  </Button>
                </div>
              </div>
            </CardContent>
          </Card>
          <NotConnected
            v-else
            title="最近在读"
            description="书架里还没有书，导入一本后首页会显示最近在读。"
            :capabilities="['书架中至少一本书（导入 TXT / EPUB 或从发现页加入）']"
          />

          <div class="grid grid-cols-3 gap-4">
            <Card>
              <CardContent class="pt-6">
                <div class="flex items-center gap-1.5 text-xs text-muted-foreground">
                  <BookOpen :size="13" />累计读完
                </div>
                <div class="mt-2 text-2xl font-semibold">
                  {{ stats?.finished_books ?? 0
                  }}<small class="ml-1 text-xs font-normal text-muted-foreground">本</small>
                </div>
                <p class="mt-1 text-[11px] text-muted-foreground">进度停在末章的书籍</p>
              </CardContent>
            </Card>
            <Card>
              <CardContent class="pt-6">
                <div class="flex items-center gap-1.5 text-xs text-muted-foreground"><Clock3 :size="13" />累计时长</div>
                <div class="mt-2 text-2xl font-semibold">
                  {{ stats ? (stats.total_seconds / 3600).toFixed(1) : '—'
                  }}<small class="ml-1 text-xs font-normal text-muted-foreground">小时</small>
                </div>
                <p class="mt-1 text-[11px] text-muted-foreground">今日 {{ todayMinutes }} 分钟</p>
              </CardContent>
            </Card>
            <Card>
              <CardContent class="pt-6">
                <div class="flex items-center gap-1.5 text-xs text-muted-foreground"><Flame :size="13" />连续天数</div>
                <div class="mt-2 text-2xl font-semibold">
                  {{ stats?.streak_days ?? 0 }}<small class="ml-1 text-xs font-normal text-muted-foreground">天</small>
                </div>
                <p class="mt-1 text-[11px] text-muted-foreground">含今天</p>
              </CardContent>
            </Card>
          </div>
        </div>

        <div class="flex flex-col gap-6">
          <Card>
            <CardHeader class="pb-2">
              <CardTitle class="flex items-center gap-1.5 text-xs font-medium text-muted-foreground">
                <Target :size="13" />每日目标
              </CardTitle>
            </CardHeader>
            <CardContent class="grid gap-3">
              <div class="flex items-end justify-between">
                <span class="text-2xl font-semibold">{{ todayMinutes }}</span>
                <span class="text-xs text-muted-foreground">/ {{ stats?.daily_goal_minutes || '未设置' }} 分钟</span>
              </div>
              <Progress :model-value="goalPercent" class="h-1.5" />
              <p class="text-[11px] text-muted-foreground">
                {{ stats?.daily_goal_minutes ? `已完成 ${goalPercent}%` : '设定每日目标后这里会显示进度。' }}
              </p>
            </CardContent>
          </Card>

          <NotConnected
            title="WebDAV 同步"
            description="原型首页的 WebDAV 卡片；同步子系统尚未立项。"
            :capabilities="['WebDAV 同步子系统（ROADMAP-v3 S）']"
          />
        </div>
      </div>
    </PageBody>
  </div>
</template>

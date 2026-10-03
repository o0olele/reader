<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { Pencil, Target } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Progress } from '@/components/ui/progress'
import { notifySuccess } from '@/app/useToast'
import { getErrorMessage, setReadingGoal, type ReadingStats } from '@/services/api'

const props = defineProps<{ stats?: ReadingStats }>()
const emit = defineEmits<{ saved: [stats: ReadingStats] }>()
const open = ref(false)
const goalMinutes = ref<string | number>(0)
const saving = ref(false)
const error = ref('')
const todayMinutes = computed(() => Math.round((props.stats?.today_seconds ?? 0) / 60))
const goalPercent = computed(() => {
  const goal = props.stats?.daily_goal_minutes ?? 0
  return goal ? Math.min(100, Math.round((todayMinutes.value / goal) * 100)) : 0
})
const validGoal = computed(() => {
  const minutes = Number(goalMinutes.value)
  return String(goalMinutes.value).trim() !== '' && Number.isInteger(minutes) && minutes >= 0 && minutes <= 1440
})

watch(open, (value) => {
  if (value) {
    goalMinutes.value = props.stats?.daily_goal_minutes ?? 0
    error.value = ''
  }
})

async function saveGoal() {
  if (!validGoal.value || saving.value) return
  saving.value = true
  error.value = ''
  try {
    const stats = await setReadingGoal(Number(goalMinutes.value))
    emit('saved', stats)
    open.value = false
    notifySuccess(stats.daily_goal_minutes ? `每日目标已设为 ${stats.daily_goal_minutes} 分钟` : '已清除每日目标')
  } catch (cause) {
    error.value = getErrorMessage(cause)
  } finally {
    saving.value = false
  }
}
</script>

<template>
  <Dialog v-model:open="open">
    <Card class="group relative transition-colors hover:border-primary/50">
      <DialogTrigger as-child>
        <button
          type="button"
          class="absolute inset-0 z-10 cursor-pointer rounded-xl focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
          aria-label="设置每日阅读目标"
          :disabled="saving"
        />
      </DialogTrigger>
      <CardHeader class="pb-2">
        <CardTitle class="flex items-center gap-1.5 text-xs font-medium text-muted-foreground">
          <Target :size="13" />每日目标
          <span class="ml-auto flex items-center gap-1 text-[11px] group-hover:text-foreground">
            <Pencil :size="12" />设置
          </span>
        </CardTitle>
      </CardHeader>
      <CardContent class="grid gap-3">
        <div class="flex items-end justify-between">
          <span class="text-2xl font-semibold">{{ todayMinutes }}</span>
          <span class="text-xs text-muted-foreground">/ {{ stats?.daily_goal_minutes || '未设置' }} 分钟</span>
        </div>
        <Progress :model-value="goalPercent" class="h-1.5" />
        <p class="text-[11px] text-muted-foreground">
          {{ stats?.daily_goal_minutes ? `已完成 ${goalPercent}%` : '点击设置每日阅读目标。' }}
        </p>
      </CardContent>
    </Card>
    <DialogContent class="sm:max-w-sm">
      <DialogHeader>
        <DialogTitle>设置每日阅读目标</DialogTitle>
        <DialogDescription>每天想读多久？设置后会按今日阅读时长计算进度，填 0 表示不设目标。</DialogDescription>
      </DialogHeader>
      <form class="grid gap-4" @submit.prevent="saveGoal">
        <label class="grid gap-1.5 text-xs">
          <span class="text-muted-foreground">每日阅读时长（分钟）</span>
          <Input
            v-model="goalMinutes"
            type="number"
            min="0"
            max="1440"
            step="1"
            required
            :disabled="saving"
            aria-label="每日阅读时长（分钟）"
          />
        </label>
        <p class="text-xs text-muted-foreground">可设置 0–1440 之间的整数。</p>
        <p v-if="error" role="alert" class="text-xs text-destructive">{{ error }}</p>
        <DialogFooter>
          <Button type="button" variant="ghost" :disabled="saving" @click="open = false">取消</Button>
          <Button type="submit" :disabled="!validGoal || saving">{{ saving ? '保存中…' : '保存目标' }}</Button>
        </DialogFooter>
      </form>
    </DialogContent>
  </Dialog>
</template>

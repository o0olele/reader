<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { Slider } from '@/components/ui/slider'
import { Switch } from '@/components/ui/switch'
import { getReadingStats, setReadingGoal } from '@/services/api'
import { notifyError, notifySuccess } from '@/app/useToast'
import { useShellContext } from '@/app/shellKeys'

const { reader } = useShellContext()
const goalMinutes = ref(0)
const savingGoal = ref(false)

async function saveGoal() {
  savingGoal.value = true
  try {
    const stats = await setReadingGoal(goalMinutes.value)
    goalMinutes.value = stats.daily_goal_minutes
    notifySuccess(stats.daily_goal_minutes ? `每日目标已设为 ${stats.daily_goal_minutes} 分钟` : '已清除每日目标')
  } catch (cause) {
    notifyError(String(cause))
  } finally {
    savingGoal.value = false
  }
}

onMounted(async () => {
  try {
    goalMinutes.value = (await getReadingStats()).daily_goal_minutes
  } catch {
    /* browser preview */
  }
})
</script>

<template>
  <div class="grid max-w-2xl gap-6">
    <section class="grid gap-2">
      <h2 class="text-sm font-semibold">排版</h2>
      <div class="grid grid-cols-2 gap-4">
        <label class="grid gap-1.5 text-xs">
          <span class="text-muted-foreground">字体</span>
          <Select v-model="reader.fontFamily">
            <SelectTrigger><SelectValue /></SelectTrigger>
            <SelectContent>
              <SelectItem value="思源宋体">思源宋体</SelectItem>
              <SelectItem value="霞鹜文楷">霞鹜文楷</SelectItem>
              <SelectItem value="系统默认">系统默认</SelectItem>
            </SelectContent>
          </Select>
        </label>
        <label class="grid gap-1.5 text-xs">
          <span class="text-muted-foreground">字号（{{ reader.fontSize }} px）</span>
          <Slider
            :model-value="[reader.fontSize]"
            :min="14"
            :max="28"
            :step="1"
            @update:model-value="reader.fontSize = $event?.[0] ?? reader.fontSize"
          />
        </label>
        <label class="grid gap-1.5 text-xs">
          <span class="text-muted-foreground">行距（{{ reader.lineHeight.toFixed(1) }}）</span>
          <Slider
            :model-value="[reader.lineHeight]"
            :min="1.4"
            :max="2.4"
            :step="0.1"
            @update:model-value="reader.lineHeight = $event?.[0] ?? reader.lineHeight"
          />
        </label>
        <label class="grid gap-1.5 text-xs">
          <span class="text-muted-foreground">页边距（{{ reader.pageMargin }} px）</span>
          <Slider
            :model-value="[reader.pageMargin]"
            :min="16"
            :max="64"
            :step="4"
            @update:model-value="reader.pageMargin = $event?.[0] ?? reader.pageMargin"
          />
        </label>
      </div>
    </section>

    <section class="grid gap-2">
      <h2 class="text-sm font-semibold">翻页</h2>
      <div class="flex items-center justify-between rounded-md border p-3 text-xs">
        <span>分页模式（关闭后为滚动）</span>
        <Switch
          :model-value="reader.readerMode === 'paged'"
          @update:model-value="reader.readerMode = $event ? 'paged' : 'scroll'"
        />
      </div>
      <p class="text-xs text-muted-foreground">
        翻页动画六选、段距、首行缩进、两端对齐属于 F3 阅读器重建范围，尚未接入。
      </p>
      <Button variant="outline" size="sm" class="w-fit" @click="reader.readerMode = 'paged'">恢复默认（分页）</Button>
    </section>

    <section class="grid gap-2">
      <h2 class="text-sm font-semibold">每日目标</h2>
      <p class="text-xs text-muted-foreground">首页仪表盘用它计算今日进度；填 0 表示不设目标。</p>
      <div class="flex items-center gap-2">
        <Input
          v-model.number="goalMinutes"
          class="h-8 w-28"
          type="number"
          min="0"
          max="1440"
          aria-label="每日目标分钟"
        />
        <span class="text-xs text-muted-foreground">分钟 / 天</span>
        <Button size="sm" :disabled="savingGoal" @click="saveGoal">{{ savingGoal ? '保存中…' : '保存目标' }}</Button>
      </div>
    </section>
  </div>
</template>

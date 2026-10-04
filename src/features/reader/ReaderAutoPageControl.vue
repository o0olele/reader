<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { Pause, Play, Settings2 } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover'

const props = defineProps<{
  active: boolean
  secondsLeft: number
  interval: number
  ready: boolean
  compact?: boolean
}>()
const emit = defineEmits<{ start: [seconds: number]; pause: [] }>()
const open = ref(false)
const draft = ref<string | number>(props.interval)
const valid = computed(() => {
  const seconds = Number(draft.value)
  return String(draft.value).trim() !== '' && Number.isInteger(seconds) && seconds >= 1 && seconds <= 300
})
watch(open, (value) => {
  if (value) draft.value = props.interval
})

function submit() {
  if (!valid.value || !props.ready) return
  emit('start', Number(draft.value))
  open.value = false
}
</script>

<template>
  <div class="flex shrink-0 items-center">
    <Button
      variant="ghost"
      :size="compact && !active ? 'icon-sm' : 'sm'"
      :class="active ? 'bg-accent' : ''"
      :aria-pressed="active"
      :aria-label="active ? '暂停自动翻页' : '设置自动翻页'"
      :title="
        active
          ? ready
            ? `暂停自动翻页（剩余 ${secondsLeft} 秒）`
            : '暂停自动翻页（等待正文）'
          : '设置翻页间隔并开启自动翻页'
      "
      @click="active ? emit('pause') : (open = true)"
    >
      <Pause v-if="active" /><Play v-else />
      <span v-if="!compact">{{
        active ? (ready ? `暂停翻页 · ${secondsLeft}秒` : '暂停翻页 · 等待正文') : '自动翻页'
      }}</span>
      <span v-else-if="active" class="min-w-9 text-center tabular-nums">{{
        ready ? `${secondsLeft}秒` : '等待中'
      }}</span>
    </Button>
    <Popover v-model:open="open">
      <PopoverTrigger as-child>
        <Button variant="ghost" size="icon-sm" aria-label="自动翻页设置" title="自动翻页设置">
          <Settings2 />
        </Button>
      </PopoverTrigger>
      <PopoverContent side="top" align="start" class="w-72">
        <form class="grid gap-4" @submit.prevent="submit">
          <div class="grid gap-1">
            <h3 class="text-sm font-semibold">自动翻页设置</h3>
            <p class="text-xs leading-relaxed text-muted-foreground">
              按当前模式翻一页或滚动一屏，章末自动进入下一章。
            </p>
          </div>
          <label class="grid gap-1.5 text-xs">
            <span>翻页间隔（秒）</span>
            <Input v-model="draft" type="number" min="1" max="300" step="1" required aria-label="翻页间隔（秒）" />
          </label>
          <div class="flex gap-2">
            <Button
              v-for="seconds in [5, 10, 15, 30]"
              :key="seconds"
              type="button"
              size="sm"
              :variant="Number(draft) === seconds ? 'secondary' : 'outline'"
              @click="draft = seconds"
            >
              {{ seconds }} 秒
            </Button>
          </div>
          <p class="text-xs text-muted-foreground">支持 1–300 秒，设置会自动记住。开启后可随时点击暂停。</p>
          <p v-if="!ready" class="text-xs text-muted-foreground">请等待正文加载完成后开启。</p>
          <Button type="submit" size="sm" :disabled="!valid || !ready">
            {{ active ? '应用设置' : '开始自动翻页' }}
          </Button>
        </form>
      </PopoverContent>
    </Popover>
  </div>
</template>

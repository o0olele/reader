<script setup lang="ts">
import { computed } from 'vue'
import { ArrowLeft, ArrowRight } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Slider } from '@/components/ui/slider'

const props = defineProps<{ chapterCount: number; chapterIndex: number; compact: boolean }>()
const emit = defineEmits<{ prev: []; next: []; goto: [index: number] }>()
const percent = computed(() =>
  props.chapterCount ? Math.round(((props.chapterIndex + 1) / props.chapterCount) * 100) : 0,
)
</script>

<template>
  <div class="flex min-w-0 items-center py-1.5" :class="compact ? 'gap-2 px-2' : 'gap-3 px-4'">
    <Button
      variant="ghost"
      :size="compact ? 'icon-sm' : 'sm'"
      class="shrink-0"
      aria-label="上一章"
      title="上一章"
      :disabled="chapterIndex <= 0"
      @click="emit('prev')"
    >
      <ArrowLeft /><span v-if="!compact">上一章</span>
    </Button>
    <span
      class="shrink-0 whitespace-nowrap text-center text-xs tabular-nums text-muted-foreground"
      :class="compact ? '' : 'min-w-28'"
      :aria-label="`第 ${chapterIndex + 1} 章，共 ${chapterCount} 章`"
    >
      <template v-if="!compact">第 </template>{{ chapterIndex + 1 }} / {{ chapterCount
      }}<template v-if="!compact"> 章</template>
    </span>
    <Slider
      class="min-w-0 flex-1"
      :model-value="[chapterIndex]"
      :min="0"
      :max="Math.max(0, chapterCount - 1)"
      :step="1"
      @update:model-value="emit('goto', $event?.[0] ?? chapterIndex)"
    />
    <span class="shrink-0 text-right text-xs tabular-nums text-muted-foreground" :class="compact ? 'w-9' : 'w-12'">
      {{ percent }}%
    </span>
    <Button
      variant="ghost"
      :size="compact ? 'icon-sm' : 'sm'"
      class="shrink-0"
      aria-label="下一章"
      title="下一章"
      :disabled="chapterIndex >= chapterCount - 1"
      @click="emit('next')"
    >
      <span v-if="!compact">下一章</span><ArrowRight />
    </Button>
  </div>
</template>

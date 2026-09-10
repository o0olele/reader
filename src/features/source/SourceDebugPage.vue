<script setup lang="ts">
import { computed, watch } from 'vue'
import { useRoute } from 'vue-router'
import { Play, Save, Upload } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { ToggleGroup, ToggleGroupItem } from '@/components/ui/toggle-group'
import DebugOutput from './DebugOutput.vue'
import PageBody from '@/components/PageBody.vue'
import PageHeader from '@/components/PageHeader.vue'
import { useShellContext } from '@/app/shellKeys'

const route = useRoute()
const { sourceDebug: debug } = useShellContext()

const stageHint = computed(() => debug.stages.find((item) => item.value === debug.stage)?.hint ?? '输入 URL 或关键词')

// `/sources/debug?source=12` deep link from the source list.
watch(
  () => route.query.source,
  (value) => {
    const id = Number(value)
    if (id && debug.sourceId !== id) debug.sourceId = id
  },
  { immediate: true },
)
</script>

<template>
  <div class="flex h-full flex-col">
    <PageHeader
      title="书源调试"
      :subtitle="debug.currentSource ? `当前：${debug.currentSource.name}` : '四阶段单步执行'"
    >
      <Button variant="outline" size="sm" :disabled="debug.savingRules || !debug.sourceId" @click="debug.saveRules()">
        <Save /> {{ debug.savingRules ? '保存中…' : '保存规则' }}
      </Button>
      <Button variant="outline" size="sm" :disabled="!debug.result" @click="debug.exportFixture()">
        <Upload /> 导出 fixture
      </Button>
    </PageHeader>

    <div class="flex shrink-0 flex-wrap items-center gap-2 border-b bg-card px-6 py-2.5">
      <Select :model-value="String(debug.sourceId ?? '')" @update:model-value="debug.sourceId = Number($event)">
        <SelectTrigger class="h-8 w-52"><SelectValue placeholder="选择书源" /></SelectTrigger>
        <SelectContent>
          <SelectItem v-for="source in debug.sourceOptions" :key="source.id" :value="String(source.id)">
            {{ source.name }}
          </SelectItem>
        </SelectContent>
      </Select>

      <ToggleGroup
        type="single"
        :model-value="debug.stage"
        variant="outline"
        size="sm"
        @update:model-value="debug.stage = ($event as typeof debug.stage) || debug.stage"
      >
        <ToggleGroupItem v-for="item in debug.stages" :key="item.value" :value="item.value">
          {{ item.label }}
        </ToggleGroupItem>
      </ToggleGroup>

      <Input v-model="debug.input" class="h-8 w-80" :placeholder="stageHint" aria-label="调试输入" />
      <Button size="sm" :disabled="debug.running || !debug.sourceId" @click="debug.run()">
        <Play /> {{ debug.running ? '执行中…' : '单步执行' }}
      </Button>
    </div>

    <div class="flex min-h-0 flex-1">
      <section class="w-80 shrink-0 overflow-y-auto border-r bg-card p-4">
        <h2 class="mb-3 text-sm font-semibold">规则编辑</h2>
        <label v-for="item in debug.stages" :key="item.value" class="mb-3 grid gap-1.5 text-xs">
          <span class="text-muted-foreground">{{ item.label }}</span>
          <textarea
            v-model="debug.rules[item.value]"
            rows="5"
            spellcheck="false"
            :aria-label="`${item.label}规则`"
            class="w-full resize-y rounded-md border bg-background px-2 py-1.5 font-mono text-xs outline-none focus-visible:ring-1 focus-visible:ring-ring"
          />
        </label>
        <p class="text-xs text-muted-foreground">先「保存规则」写回书源，再「单步执行」按新规则执行并查看结果。</p>
      </section>

      <PageBody>
        <div v-if="debug.running && !debug.result" class="py-12 text-center text-xs text-muted-foreground">
          正在执行「{{ debug.stageLabel }}」阶段（{{ debug.progressState === 'started' ? '已收到进度' : '等待进度' }}）…
        </div>
        <div v-else-if="!debug.result" class="py-12 text-center text-xs text-muted-foreground">
          选择书源和输入后点击「单步执行」
        </div>
        <DebugOutput v-else :result="debug.result" />
      </PageBody>
    </div>
  </div>
</template>

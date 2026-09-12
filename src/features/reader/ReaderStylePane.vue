<script setup lang="ts">
import { Button } from '@/components/ui/button'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { Slider } from '@/components/ui/slider'
import { Switch } from '@/components/ui/switch'
import { ToggleGroup, ToggleGroupItem } from '@/components/ui/toggle-group'
import { useShellContext } from '@/app/shellKeys'
import { defaultFullJustification } from './readerTypography'

const { reader } = useShellContext()
const THEMES = [
  { value: 'light', label: '浅色', swatch: 'bg-[oklch(0.99_0.003_106)]' },
  { value: 'sepia', label: '护眼', swatch: 'bg-[oklch(0.95_0.03_85)]' },
  { value: 'dark', label: '深色', swatch: 'bg-[oklch(0.27_0.02_265)]' },
  { value: 'black', label: '黑夜', swatch: 'bg-black' },
] as const

function reset() {
  reader.theme = 'light'
  reader.fontFamily = '思源宋体'
  reader.fontSize = 17
  reader.lineHeight = 1.8
  reader.pageMargin = 32
  reader.paragraphSpacing = 1.2
  reader.textIndent = 2
  reader.justify = defaultFullJustification(reader.selectedChapter?.content)
  reader.brightness = 1
  reader.eyeCare = false
  reader.pageAnimation = 'slide'
  reader.readerMode = 'paged'
}
</script>

<template>
  <div class="grid gap-4">
    <section class="grid gap-2">
      <span class="text-xs text-muted-foreground">主题</span>
      <div class="grid grid-cols-4 gap-2">
        <button
          v-for="option in THEMES"
          :key="option.value"
          type="button"
          class="flex flex-col items-center gap-1 rounded-md border p-1 text-[11px]"
          :class="reader.theme === option.value ? 'border-primary ring-1 ring-primary' : ''"
          @click="reader.theme = option.value"
        >
          <span class="h-8 w-full rounded-sm border" :class="option.swatch" />
          {{ option.label }}
        </button>
      </div>
    </section>

    <section class="grid gap-2">
      <span class="text-xs text-muted-foreground">字体</span>
      <Select v-model="reader.fontFamily">
        <SelectTrigger><SelectValue placeholder="选择字体" /></SelectTrigger>
        <SelectContent>
          <SelectItem value="思源宋体">思源宋体</SelectItem>
          <SelectItem value="霞鹜文楷">霞鹜文楷</SelectItem>
          <SelectItem value="系统默认">系统默认</SelectItem>
        </SelectContent>
      </Select>
    </section>

    <section class="grid gap-2">
      <span class="text-xs text-muted-foreground">字号</span>
      <div class="flex items-center gap-2">
        <Button variant="outline" size="sm" @click="reader.fontSize = Math.max(14, reader.fontSize - 1)">A-</Button>
        <span class="flex-1 text-center text-sm">{{ reader.fontSize }} px</span>
        <Button variant="outline" size="sm" @click="reader.fontSize = Math.min(28, reader.fontSize + 1)">A+</Button>
      </div>
    </section>

    <label class="grid gap-2">
      <span class="flex justify-between text-xs text-muted-foreground">
        行距 <output>{{ reader.lineHeight.toFixed(1) }}</output>
      </span>
      <Slider
        :model-value="[reader.lineHeight]"
        :min="1.4"
        :max="2.4"
        :step="0.1"
        @update:model-value="reader.lineHeight = $event?.[0] ?? reader.lineHeight"
      />
    </label>

    <label class="grid gap-2">
      <span class="flex justify-between text-xs text-muted-foreground">
        段距 <output>{{ reader.paragraphSpacing.toFixed(1) }} em</output>
      </span>
      <Slider
        :model-value="[reader.paragraphSpacing]"
        :min="0.4"
        :max="2.5"
        :step="0.1"
        @update:model-value="reader.paragraphSpacing = $event?.[0] ?? reader.paragraphSpacing"
      />
    </label>

    <label class="grid gap-2">
      <span class="flex justify-between text-xs text-muted-foreground">
        页边距 <output>{{ reader.pageMargin }} px</output>
      </span>
      <Slider
        :model-value="[reader.pageMargin]"
        :min="16"
        :max="64"
        :step="4"
        @update:model-value="reader.pageMargin = $event?.[0] ?? reader.pageMargin"
      />
    </label>

    <label class="grid gap-2">
      <span class="flex justify-between text-xs text-muted-foreground">
        首行缩进 <output>{{ reader.textIndent.toFixed(1) }} em</output>
      </span>
      <Slider
        :model-value="[reader.textIndent]"
        :min="0"
        :max="3"
        :step="0.5"
        @update:model-value="reader.textIndent = $event?.[0] ?? reader.textIndent"
      />
    </label>

    <div class="flex items-center justify-between text-xs">
      <span>两端对齐</span>
      <Switch v-model="reader.justify" />
    </div>

    <section class="grid gap-2">
      <span class="text-xs text-muted-foreground">阅读模式</span>
      <ToggleGroup
        type="single"
        :model-value="reader.readerMode"
        variant="outline"
        size="sm"
        @update:model-value="reader.readerMode = ($event as 'scroll' | 'paged') || reader.readerMode"
      >
        <ToggleGroupItem value="paged" class="flex-1">翻页</ToggleGroupItem>
        <ToggleGroupItem value="scroll" class="flex-1">滚动</ToggleGroupItem>
      </ToggleGroup>
    </section>

    <section class="grid gap-2">
      <span class="text-xs text-muted-foreground">翻页动画</span>
      <ToggleGroup
        type="single"
        :model-value="reader.pageAnimation"
        variant="outline"
        size="sm"
        @update:model-value="reader.pageAnimation = ($event as 'none' | 'slide') || reader.pageAnimation"
      >
        <ToggleGroupItem value="none" class="flex-1">无</ToggleGroupItem>
        <ToggleGroupItem value="slide" class="flex-1">滑动</ToggleGroupItem>
      </ToggleGroup>
      <p class="text-[11px] text-muted-foreground">覆盖 / 淡入 / 仿真 / 竖排 未接入（§9 明确不做仿真与竖排）。</p>
    </section>

    <label class="grid gap-2">
      <span class="flex justify-between text-xs text-muted-foreground">
        亮度 <output>{{ Math.round(reader.brightness * 100) }}%</output>
      </span>
      <Slider
        :model-value="[reader.brightness]"
        :min="0.6"
        :max="1.4"
        :step="0.05"
        @update:model-value="reader.brightness = $event?.[0] ?? reader.brightness"
      />
    </label>

    <div class="flex items-center justify-between text-xs">
      <span>护眼模式</span>
      <Switch v-model="reader.eyeCare" />
    </div>
    <p class="text-[11px] text-muted-foreground">「跟随系统亮度」需要桌面平台接口，未接入。</p>

    <Button variant="outline" size="sm" @click="reset">恢复默认</Button>
  </div>
</template>

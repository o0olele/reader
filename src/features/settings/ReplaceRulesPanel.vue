<script setup lang="ts">
import { computed } from 'vue'
import { Plus, Trash2 } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Switch } from '@/components/ui/switch'
import { Textarea } from '@/components/ui/textarea'
import { useReplaceRules } from './useReplaceRules'

const { rules, draft, busy, error, message, sampleTitle, sampleContent, preview, edit, save, remove, toggle, test } =
  useReplaceRules()

// ReplaceRule keeps these three nullable; the form edits them as plain strings.
const group = computed({ get: () => draft.value.group ?? '', set: (value: string) => (draft.value.group = value) })
const scope = computed({ get: () => draft.value.scope ?? '', set: (value: string) => (draft.value.scope = value) })
const excludeScope = computed({
  get: () => draft.value.exclude_scope ?? '',
  set: (value: string) => (draft.value.exclude_scope = value),
})
</script>

<template>
  <div class="grid max-w-3xl gap-4">
    <section class="grid gap-1">
      <h2 class="text-sm font-semibold">净化替换</h2>
      <p class="text-xs text-muted-foreground">
        去除广告或替换阅读文字。规则按顺序执行，适用于本地和在线书籍；停用后重新打开章节即可恢复原文。
      </p>
      <p v-if="error" role="alert" class="text-xs text-destructive">{{ error }}</p>
      <p v-if="message" role="status" class="text-xs text-muted-foreground">{{ message }}</p>
    </section>

    <section class="grid gap-1.5">
      <div
        v-for="rule in rules"
        :key="rule.id"
        class="flex items-center gap-2 rounded-md border bg-card px-3 py-2 text-sm"
      >
        <span class="min-w-0 flex-1 truncate">
          {{ rule.sort_order }} · {{ rule.name }}
          <small v-if="rule.group" class="text-muted-foreground">（{{ rule.group }}）</small>
        </span>
        <Button variant="outline" size="sm" :disabled="busy" :aria-pressed="rule.enabled" @click="toggle(rule)">
          {{ rule.enabled ? '已启用' : '已停用' }}
        </Button>
        <Button variant="ghost" size="sm" :disabled="busy" @click="edit(rule)">编辑</Button>
        <Button variant="ghost" size="icon-sm" class="text-destructive" :disabled="busy" @click="remove(rule)">
          <Trash2 />
        </Button>
      </div>
      <p v-if="!rules.length" class="rounded-md border border-dashed p-6 text-center text-xs text-muted-foreground">
        还没有净化规则。
      </p>
    </section>

    <form class="grid gap-3 rounded-md border p-4" @submit.prevent="save">
      <fieldset :disabled="busy" class="grid gap-3">
        <legend class="px-1 text-sm font-semibold">{{ draft.id ? '编辑规则' : '新增规则' }}</legend>
        <div class="grid grid-cols-2 gap-3">
          <label class="grid gap-1.5 text-xs">
            <span class="text-muted-foreground">名称</span>
            <Input v-model="draft.name" required />
          </label>
          <label class="grid gap-1.5 text-xs">
            <span class="text-muted-foreground">分组</span>
            <Input v-model="group" />
          </label>
          <label class="grid gap-1.5 text-xs">
            <span class="text-muted-foreground">匹配内容</span>
            <Textarea v-model="draft.pattern" required rows="3" />
          </label>
          <label class="grid gap-1.5 text-xs">
            <span class="text-muted-foreground">替换为（留空则删除）</span>
            <Textarea v-model="draft.replacement" rows="3" />
          </label>
          <label class="grid gap-1.5 text-xs">
            <span class="text-muted-foreground">作用范围</span>
            <Input v-model="scope" placeholder="书名或书源地址；留空为全部" />
          </label>
          <label class="grid gap-1.5 text-xs">
            <span class="text-muted-foreground">排除范围</span>
            <Input v-model="excludeScope" placeholder="不应用规则的书名或书源地址" />
          </label>
          <label class="grid gap-1.5 text-xs">
            <span class="text-muted-foreground">执行顺序</span>
            <Input v-model.number="draft.sort_order" type="number" step="1" required />
          </label>
        </div>
        <p class="text-xs text-muted-foreground">范围可填多个完整书名或书源地址，以换行或逗号分隔；排除范围优先。</p>
        <div class="flex flex-wrap gap-4">
          <label class="flex items-center gap-2 text-xs"><Switch v-model="draft.is_regex" /> 正则表达式</label>
          <label class="flex items-center gap-2 text-xs"><Switch v-model="draft.scope_title" /> 章节标题</label>
          <label class="flex items-center gap-2 text-xs"><Switch v-model="draft.scope_content" /> 正文</label>
          <label class="flex items-center gap-2 text-xs"><Switch v-model="draft.enabled" /> 启用</label>
        </div>
        <div class="flex gap-2">
          <Button type="submit" size="sm">保存规则</Button>
          <Button type="button" variant="outline" size="sm" @click="edit()"><Plus /> 新增规则</Button>
        </div>
      </fieldset>
    </form>

    <details class="rounded-md border p-3 text-xs">
      <summary class="cursor-pointer font-semibold">预览当前规则</summary>
      <p class="mt-2 text-muted-foreground">使用下方样本文字测试当前编辑的规则，忽略启用状态和作用范围。</p>
      <div class="mt-3 grid grid-cols-2 gap-3">
        <label class="grid gap-1.5">
          <span class="text-muted-foreground">样本标题</span>
          <Input v-model="sampleTitle" />
        </label>
        <label class="grid gap-1.5">
          <span class="text-muted-foreground">样本正文</span>
          <Textarea v-model="sampleContent" rows="5" />
        </label>
      </div>
      <Button class="mt-3" variant="outline" size="sm" :disabled="busy" @click="test">预览</Button>
      <div v-if="preview" aria-live="polite" class="mt-3">
        <h3 class="font-semibold">{{ preview.title }}</h3>
        <pre class="mt-1 overflow-auto rounded-md border bg-muted/40 p-2 whitespace-pre-wrap">{{
          preview.content || '（替换后正文为空）'
        }}</pre>
      </div>
    </details>
  </div>
</template>

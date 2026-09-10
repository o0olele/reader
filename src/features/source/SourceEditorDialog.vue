<script setup lang="ts">
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { useShellContext } from '@/app/shellKeys'

const { sources } = useShellContext()
const open = defineModel<boolean>('open', { default: false })

const BASIC = [
  { key: 'name', label: '名称' },
  { key: 'base_url', label: '基础 URL' },
  { key: 'search_url', label: '搜索 URL（使用 {{key}}）' },
  { key: 'explore_url', label: '发现页 URL' },
  { key: 'book_url_pattern', label: '详情页 URL 正则' },
] as const
const SELECTORS = [
  { key: 'item', label: '结果项 CSS' },
  { key: 'title', label: '标题 CSS' },
  { key: 'author', label: '作者 CSS' },
  { key: 'url', label: '链接 CSS' },
  { key: 'next_toc_url_selector', label: '目录下一页 CSS' },
  { key: 'next_content_url_selector', label: '正文下一页 CSS' },
] as const
const AUTH = [
  { key: 'login_url', label: '登录 URL' },
  { key: 'login_body', label: '登录 Body' },
  { key: 'token_path', label: 'Token 路径' },
  { key: 'sign_script', label: '签名表达式' },
] as const
const MISC = [
  { key: 'proxy_url', label: '代理 URL' },
  { key: 'source_group', label: '书源分组' },
  { key: 'custom_order', label: '排序' },
  { key: 'weight', label: '权重' },
] as const

async function submit() {
  await sources.save()
  if (!sources.saving) open.value = false
}
</script>

<template>
  <Dialog v-model:open="open">
    <DialogContent class="max-h-[85vh] overflow-y-auto sm:max-w-2xl">
      <DialogHeader>
        <DialogTitle>添加书源</DialogTitle>
        <DialogDescription>手写书源只保存扁平选择器字段；导入 legado JSON 会保留原始规则。</DialogDescription>
      </DialogHeader>
      <form class="grid gap-4" @submit.prevent="submit">
        <div class="grid grid-cols-2 gap-3">
          <label v-for="field in BASIC" :key="field.key" class="grid gap-1.5 text-xs">
            <span class="text-muted-foreground">{{ field.label }}</span>
            <Input v-model="sources.form[field.key]" :aria-label="field.label" />
          </label>
        </div>
        <div class="grid grid-cols-2 gap-3">
          <label v-for="field in SELECTORS" :key="field.key" class="grid gap-1.5 text-xs">
            <span class="text-muted-foreground">{{ field.label }}</span>
            <Input v-model="sources.form[field.key]" :aria-label="field.label" />
          </label>
        </div>
        <div class="grid grid-cols-2 gap-3">
          <label v-for="field in AUTH" :key="field.key" class="grid gap-1.5 text-xs">
            <span class="text-muted-foreground">{{ field.label }}</span>
            <Input v-model="sources.form[field.key]" :aria-label="field.label" />
          </label>
          <label class="grid gap-1.5 text-xs">
            <span class="text-muted-foreground">登录方法</span>
            <Select v-model="sources.form.login_method">
              <SelectTrigger><SelectValue /></SelectTrigger>
              <SelectContent>
                <SelectItem value="POST">POST</SelectItem>
                <SelectItem value="GET">GET</SelectItem>
                <SelectItem value="PUT">PUT</SelectItem>
              </SelectContent>
            </Select>
          </label>
        </div>
        <div class="grid grid-cols-4 gap-3">
          <label v-for="field in MISC" :key="field.key" class="grid gap-1.5 text-xs">
            <span class="text-muted-foreground">{{ field.label }}</span>
            <Input v-model="sources.form[field.key]" :aria-label="field.label" />
          </label>
        </div>
        <DialogFooter>
          <Button type="button" variant="ghost" @click="open = false">取消</Button>
          <Button type="submit" :disabled="sources.saving">{{ sources.saving ? '保存中…' : '保存书源' }}</Button>
        </DialogFooter>
      </form>
    </DialogContent>
  </Dialog>
</template>

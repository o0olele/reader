<script setup lang="ts">
import { useRouter } from 'vue-router'
import {
  Bookmark,
  BookOpen,
  ChevronRight,
  Clock3,
  Database,
  Download,
  Filter,
  Folder,
  Info,
  Settings,
  Sparkles,
} from 'lucide-vue-next'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import NotConnected from '@/components/NotConnected.vue'
import PageBody from '@/components/PageBody.vue'
import PageHeader from '@/components/PageHeader.vue'
import { useShellContext } from '@/app/shellKeys'

const router = useRouter()
const { appVersion, sources, bookshelf } = useShellContext()

const rows = [
  { icon: Database, title: '书源管理', to: '/sources', detail: () => `${sources.sources.length} 个书源` },
  { icon: Filter, title: '替换净化', to: '/settings/replace', detail: () => '阅读时生效的净化规则' },
  { icon: Bookmark, title: '书签', to: '/bookmarks', detail: () => '跨书书签汇总' },
  { icon: Clock3, title: '阅读记录', to: '/history', detail: () => '按时间排序的阅读历史' },
  { icon: Download, title: '缓存管理', to: '/downloads', detail: () => '下载任务与缓存配额' },
  { icon: Folder, title: '书架', to: '/bookshelf', detail: () => `${bookshelf.books.length} 本` },
  { icon: Settings, title: '设置', to: '/settings', detail: () => '主题、阅读、备份、代理' },
  { icon: Info, title: '关于', to: '/settings/about', detail: () => `版本 ${appVersion}` },
] as const
</script>

<template>
  <div class="flex h-full flex-col">
    <PageHeader title="我的" subtitle="本地书友 · 数据全部保存在本机" />
    <PageBody>
      <div class="grid grid-cols-2 items-start gap-6">
        <div class="flex flex-col gap-6">
          <NotConnected
            title="Web 服务"
            description="原型的局域网 Web 服务卡片（http://192.168.x.x:1122），对位 legado 的 web/KtorServer.kt；桌面端尚未立项。"
            :capabilities="['局域网 Web 服务子系统（ROADMAP-v3 S）']"
          />
          <Card>
            <CardHeader><CardTitle class="text-sm">规则</CardTitle></CardHeader>
            <CardContent class="p-1.5 pt-0">
              <button
                v-for="row in rows.slice(0, 4)"
                :key="row.title"
                type="button"
                class="flex w-full items-center gap-3 rounded-md px-2.5 py-2 text-left hover:bg-accent"
                @click="router.push(row.to)"
              >
                <component :is="row.icon" :size="16" class="shrink-0 text-muted-foreground" />
                <span class="min-w-0 flex-1">
                  <span class="block truncate text-sm">{{ row.title }}</span>
                  <span class="block truncate text-xs text-muted-foreground">{{ row.detail() }}</span>
                </span>
                <ChevronRight :size="15" class="shrink-0 text-muted-foreground" />
              </button>
            </CardContent>
          </Card>
        </div>
        <div class="flex flex-col gap-6">
          <NotConnected
            title="AI 对话"
            description="与本书内容对话；AI 子系统尚未立项，按纪律 F0 显示未接入。"
            :capabilities="['AI 子系统（ROADMAP-v3 S）']"
          />
          <Card>
            <CardHeader><CardTitle class="text-sm">其他</CardTitle></CardHeader>
            <CardContent class="p-1.5 pt-0">
              <button
                v-for="row in rows.slice(4)"
                :key="row.title"
                type="button"
                class="flex w-full items-center gap-3 rounded-md px-2.5 py-2 text-left hover:bg-accent"
                @click="router.push(row.to)"
              >
                <component :is="row.icon" :size="16" class="shrink-0 text-muted-foreground" />
                <span class="min-w-0 flex-1">
                  <span class="block truncate text-sm">{{ row.title }}</span>
                  <span class="block truncate text-xs text-muted-foreground">{{ row.detail() }}</span>
                </span>
                <ChevronRight :size="15" class="shrink-0 text-muted-foreground" />
              </button>
            </CardContent>
          </Card>
          <Card>
            <CardHeader><CardTitle class="text-sm">TTS 听书 / 翻译</CardTitle></CardHeader>
            <CardContent class="pt-0">
              <NotConnected
                title="听书与翻译"
                description="两个子系统都还没有后端，阅读器内按纪律 F0 显示未接入态。"
                :capabilities="['TTS 桌面后端选型', '翻译服务接入']"
              />
            </CardContent>
          </Card>
          <div class="flex items-center gap-2 text-xs text-muted-foreground">
            <BookOpen :size="14" /> Reader Desktop v{{ appVersion }} · 数据保存在本机 SQLite
            <Sparkles :size="14" />
          </div>
        </div>
      </div>
    </PageBody>
  </div>
</template>

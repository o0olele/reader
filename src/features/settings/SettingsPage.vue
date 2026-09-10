<script setup lang="ts">
import { computed, defineAsyncComponent, type Component } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { Blocks, Database, Filter, Image, Languages, Palette, Save, Sparkles, Type, Wifi } from 'lucide-vue-next'
import NotConnected from '@/components/NotConnected.vue'
import PageBody from '@/components/PageBody.vue'
import PageHeader from '@/components/PageHeader.vue'
import ReplaceRulesPanel from './ReplaceRulesPanel.vue'
import { cn } from '@/lib/utils'

interface Pane {
  key: string
  label: string
  icon: Component
  component?: Component
  missing?: { title: string; description: string; capabilities: string[] }
}

const PANES: Pane[] = [
  {
    key: 'theme',
    label: '主题',
    icon: Palette,
    component: defineAsyncComponent(() => import('./panes/ThemePane.vue')),
  },
  {
    key: 'reading',
    label: '阅读',
    icon: Type,
    component: defineAsyncComponent(() => import('./panes/ReadingPane.vue')),
  },
  { key: 'replace', label: '净化替换', icon: Filter, component: ReplaceRulesPanel },
  {
    key: 'cover',
    label: '封面',
    icon: Image,
    missing: {
      title: '封面规则',
      description: '原型设置页的封面规则管理；后端没有封面规则 CRUD 命令。',
      capabilities: ['封面规则 CRUD 命令（ROADMAP-v3 E1）'],
    },
  },
  {
    key: 'cache',
    label: '下载缓存',
    icon: Database,
    component: defineAsyncComponent(() => import('./panes/CachePane.vue')),
  },
  {
    key: 'backup',
    label: '备份恢复',
    icon: Save,
    component: defineAsyncComponent(() => import('./panes/BackupPane.vue')),
  },
  {
    key: 'network',
    label: '其他',
    icon: Wifi,
    component: defineAsyncComponent(() => import('./panes/NetworkPane.vue')),
  },
  {
    key: 'ai',
    label: 'AI',
    icon: Sparkles,
    missing: {
      title: 'AI',
      description: 'AI 对话 / 本章总结 / 人物关系 / 知识卡片 / 事件时间线都需要新的子系统。',
      capabilities: ['AI 子系统与模型配置（ROADMAP-v3 S）'],
    },
  },
  {
    key: 'translate',
    label: '翻译',
    icon: Languages,
    missing: {
      title: '翻译',
      description: '阅读器工具按钮与 RSS 正文翻译都依赖翻译服务接入。',
      capabilities: ['翻译服务接入（ROADMAP-v3 S）'],
    },
  },
  {
    key: 'lab',
    label: '实验室',
    icon: Blocks,
    missing: {
      title: '实验室',
      description: '多窗口阅读 / 全局快捷键 / AI 人物关系图 —— 路线图 §9 明确默认关闭、不排期。',
      capabilities: ['按 §9「明确不做」处理，不排期'],
    },
  },
]

const route = useRoute()
const router = useRouter()
const active = computed(() => PANES.find((pane) => pane.key === route.params.pane) ?? PANES[0])
</script>

<template>
  <div class="flex h-full flex-col">
    <PageHeader title="设置" :subtitle="active.label" />
    <div class="flex min-h-0 flex-1">
      <nav class="w-48 shrink-0 overflow-y-auto border-r bg-card p-2">
        <button
          v-for="pane in PANES"
          :key="pane.key"
          type="button"
          :class="
            cn(
              'mb-0.5 flex w-full items-center gap-2.5 rounded-md px-2.5 py-1.5 text-sm hover:bg-accent',
              active.key === pane.key && 'bg-accent font-medium',
            )
          "
          @click="router.push({ name: 'settings', params: { pane: pane.key } })"
        >
          <component :is="pane.icon" :size="15" class="shrink-0 text-muted-foreground" />
          <span class="truncate">{{ pane.label }}</span>
        </button>
      </nav>
      <PageBody>
        <component :is="active.component" v-if="active.component" />
        <NotConnected
          v-else-if="active.missing"
          :title="active.missing.title"
          :description="active.missing.description"
          :capabilities="active.missing.capabilities"
        />
      </PageBody>
    </div>
  </div>
</template>

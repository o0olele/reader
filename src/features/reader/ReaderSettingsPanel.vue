<script setup lang="ts">
import { X } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import NotConnected from '@/components/NotConnected.vue'
import ReaderStylePane from './ReaderStylePane.vue'

defineEmits<{ close: [] }>()
</script>

<template>
  <aside class="flex w-[320px] shrink-0 flex-col gap-3 overflow-hidden border-l bg-card">
    <div class="flex items-center justify-between px-4 pt-4">
      <strong class="text-sm">阅读面板</strong>
      <Button variant="ghost" size="icon-sm" aria-label="关闭阅读面板" @click="$emit('close')"><X /></Button>
    </div>

    <Tabs default-value="style" class="flex min-h-0 flex-1 flex-col">
      <TabsList class="mx-4 grid grid-cols-3">
        <TabsTrigger value="style">阅读样式</TabsTrigger>
        <TabsTrigger value="tts">听书</TabsTrigger>
        <TabsTrigger value="ai">AI</TabsTrigger>
      </TabsList>

      <TabsContent value="style" class="min-h-0 flex-1 overflow-y-auto px-4 pb-4">
        <ReaderStylePane />
      </TabsContent>

      <TabsContent value="tts" class="min-h-0 flex-1 overflow-y-auto px-4 pb-4">
        <NotConnected
          title="听书"
          description="语速 / 音色 / 定时停止 / 预下载 / 角色音色分配需要选一个桌面 TTS 后端。"
          :capabilities="['桌面 TTS 后端选型与朗读命令（ROADMAP-v3 S）']"
        />
      </TabsContent>

      <TabsContent value="ai" class="min-h-0 flex-1 overflow-y-auto px-4 pb-4">
        <NotConnected
          title="AI"
          description="本章总结 / 人物关系 / 知识卡片 / 事件时间线都依赖 AI 子系统。"
          :capabilities="['AI 子系统与模型配置（ROADMAP-v3 S）']"
        />
      </TabsContent>
    </Tabs>
  </aside>
</template>

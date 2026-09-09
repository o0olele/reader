<script setup lang="ts">
import NotConnected from '@/components/NotConnected.vue'
import PageBody from '@/components/PageBody.vue'
import PageHeader from '@/components/PageHeader.vue'
</script>

<template>
  <div class="flex h-full flex-col">
    <PageHeader title="RSS" subtitle="订阅源 · 文章列表 · 正文（三栏 220 / 300 / 1fr）" />
    <PageBody>
      <NotConnected
        title="RSS 订阅"
        description="原型的三栏 RSS 页面需要订阅模型、刷新调度、正文抽取与收藏。后端目前零实现，本页不显示任何书架数据。"
        :capabilities="['RSS 订阅模型与数据表', '订阅刷新与正文抽取 IPC 命令', 'rail 入口未读角标']"
      />
    </PageBody>
  </div>
</template>

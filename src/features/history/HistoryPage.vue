<script setup lang="ts">
import NotConnected from '@/components/NotConnected.vue'
import PageBody from '@/components/PageBody.vue'
import PageHeader from '@/components/PageHeader.vue'
</script>

<template>
  <div class="flex h-full flex-col">
    <PageHeader title="历史" subtitle="按时间排序的阅读记录" />
    <PageBody>
      <NotConnected
        title="阅读记录"
        description="单本书的阅读时长可以通过 get_reading_record 读到，但没有跨书的记录列表命令，因此本页不借用书架数据冒充历史。"
        :capabilities="['按时间排序的阅读记录聚合命令（ROADMAP-v3 E1）']"
      />
    </PageBody>
  </div>
</template>

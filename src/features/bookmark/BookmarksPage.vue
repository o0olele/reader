<script setup lang="ts">
import NotConnected from '@/components/NotConnected.vue'
import PageBody from '@/components/PageBody.vue'
import PageHeader from '@/components/PageHeader.vue'
</script>

<template>
  <div class="flex h-full flex-col">
    <PageHeader title="书签" subtitle="跨书的书签汇总" />
    <PageBody>
      <NotConnected
        title="书签列表"
        description="书签可以按「书 + 章」读写（get_bookmark / save_bookmark / delete_bookmark），但没有跨书列举命令，因此本页不显示任何书籍数据。"
        :capabilities="['list_bookmarks 跨书签列表命令（ROADMAP-v3 E1）']"
      />
    </PageBody>
  </div>
</template>

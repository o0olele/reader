<script setup lang="ts">
import { Download, Upload } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import NotConnected from '@/components/NotConnected.vue'
import { useShellContext } from '@/app/shellKeys'

const { settings } = useShellContext()
</script>

<template>
  <div class="grid max-w-2xl gap-6">
    <section class="grid gap-2">
      <h2 class="text-sm font-semibold">本地 JSON 备份</h2>
      <p class="text-xs text-muted-foreground">
        将书架、阅读进度、书源、净化规则和下载任务导出为 JSON；恢复时会覆盖当前本地数据。
        备份包含已保存的书源认证信息，请勿外传。
      </p>
      <div class="flex gap-2">
        <Button size="sm" @click="settings.backup()"><Download /> 导出备份</Button>
        <Button variant="outline" size="sm" @click="settings.restore()"><Upload /> 恢复备份</Button>
      </div>
    </section>

    <section class="grid gap-2">
      <h2 class="text-sm font-semibold">legado 兼容备份</h2>
      <NotConnected
        title="legado ZIP / AES 备份"
        description="对位 legado 的 Backup / Restore / BackupAES.kt。这是「Legado 桌面版」而非「另一个阅读器」的关键差异点，尚未立项。"
        :capabilities="['legado ZIP 备份格式读写', 'AES 加密备份（BackupAES.kt 对位）']"
      />
    </section>
  </div>
</template>

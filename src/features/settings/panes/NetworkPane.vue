<script setup lang="ts">
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { useShellContext } from '@/app/shellKeys'

const { settings } = useShellContext()
</script>

<template>
  <div class="grid max-w-2xl gap-6">
    <section class="grid gap-2">
      <h2 class="text-sm font-semibold">网络代理</h2>
      <p class="text-xs text-muted-foreground">所有书源请求默认使用此代理，书源单独配置的代理会覆盖这里的设置。</p>
      <form class="flex gap-2" @submit.prevent="settings.save()">
        <Input
          v-model="settings.proxyUrl"
          class="h-8"
          placeholder="如 http://127.0.0.1:7890 或 socks5://127.0.0.1:1080"
          aria-label="全局代理 URL"
        />
        <Button type="submit" size="sm" :disabled="settings.saving">{{ settings.saving ? '保存中…' : '保存' }}</Button>
        <Button type="button" variant="outline" size="sm" @click="settings.clear()">清空</Button>
      </form>
    </section>

    <section class="grid gap-2">
      <h2 class="text-sm font-semibold">User-Agent</h2>
      <p class="text-xs leading-relaxed text-muted-foreground">
        留空即跟随内置浏览器，这是推荐值：Cloudflare 会把通过验证后发放的 <code>cf_clearance</code> 绑定到
        User-Agent，只有认证窗口与后续请求完全一致时该 Cookie 才有效。填写此项会改写请求头，但改不了认证窗口发出的
        <code>Sec-CH-UA</code>，两者反而会不一致 —— 只在站点明确拒绝当前 UA 时才需要设置。
      </p>
      <form class="flex gap-2" @submit.prevent="settings.save()">
        <Input
          v-model="settings.userAgent"
          class="h-8"
          placeholder="留空 = 跟随内置浏览器（推荐）"
          aria-label="User-Agent 覆盖值"
        />
        <Button type="submit" size="sm" :disabled="settings.saving">{{ settings.saving ? '保存中…' : '保存' }}</Button>
        <Button type="button" variant="outline" size="sm" @click="settings.userAgent = ''">恢复默认</Button>
      </form>
      <p v-if="settings.effectiveUserAgent" class="text-xs text-muted-foreground">
        当前生效：<code>{{ settings.effectiveUserAgent }}</code>
      </p>
    </section>
  </div>
</template>

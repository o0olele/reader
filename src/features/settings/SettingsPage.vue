<script setup lang="ts">
import { inject } from 'vue'
import { settingsKey } from '../../app/shellKeys'

const settings = inject(settingsKey)!
</script>

<template>
  <div class="search-results">
    <section class="source-editor">
      <h2>网络代理</h2>
      <p>所有书源请求默认使用此代理，书源单独配置的代理会覆盖这里的设置。</p>
      <form class="source-import" @submit.prevent="settings.save()">
        <input
          v-model="settings.proxyUrl"
          placeholder="如 http://127.0.0.1:7890 或 socks5://127.0.0.1:1080"
          aria-label="全局代理 URL"
        />
        <button type="submit" class="primary" :disabled="settings.saving">
          {{ settings.saving ? '保存中...' : '保存代理' }}
        </button>
        <button type="button" class="secondary" @click="settings.clear()">清空</button>
      </form>
    </section>

    <section class="source-editor">
      <h2>User-Agent</h2>
      <p>
        留空即跟随内置浏览器，这是推荐值：Cloudflare 会把通过验证后发放的
        <code>cf_clearance</code> 绑定到 User-Agent，只有认证窗口与后续请求完全一致时该 Cookie
        才有效。填写此项会改写请求头，但改不了认证窗口发出的 <code>Sec-CH-UA</code>，两者反而会不一致 ——
        只在站点明确拒绝当前 UA 时才需要设置。
      </p>
      <form class="source-import" @submit.prevent="settings.save()">
        <input
          v-model="settings.userAgent"
          placeholder="留空 = 跟随内置浏览器（推荐）"
          aria-label="User-Agent 覆盖值"
        />
        <button type="submit" class="primary" :disabled="settings.saving">
          {{ settings.saving ? '保存中...' : '保存 UA' }}
        </button>
        <button type="button" class="secondary" @click="settings.userAgent = ''">恢复默认</button>
      </form>
      <p v-if="settings.effectiveUserAgent">
        当前生效：<code>{{ settings.effectiveUserAgent }}</code>
      </p>
    </section>
  </div>
</template>

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
  </div>
</template>

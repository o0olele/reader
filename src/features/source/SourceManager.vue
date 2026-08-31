<script setup lang="ts">
/* global HTMLInputElement */
import { inject, ref } from 'vue'
import { sourcesKey } from '../../app/shellKeys'

defineProps<{
  /** Reused as the probe keyword when testing a source. */
  query: string
}>()

const sources = inject(sourcesKey)!
const sourceFile = ref<HTMLInputElement>()

const FIELDS = [
  { key: 'name', label: '名称', placeholder: '名称' },
  { key: 'base_url', label: '基础 URL', placeholder: '基础 URL，如 https://example.com' },
  { key: 'search_url', label: '搜索 URL', placeholder: '搜索 URL，使用 {{key}}' },
  { key: 'item', label: '结果项 CSS', placeholder: '结果项 CSS，如 .book' },
  { key: 'title', label: '标题 CSS', placeholder: '标题 CSS，如 .title' },
  { key: 'author', label: '作者 CSS', placeholder: '作者 CSS（可选）' },
  { key: 'url', label: '链接 CSS', placeholder: '链接 CSS，如 a::attr(href)' },
  { key: 'login_url', label: '登录 URL', placeholder: '登录 URL（可选）' },
  { key: 'login_body', label: '登录 Body', placeholder: '登录 Body' },
  { key: 'token_path', label: 'Token 路径', placeholder: 'Token 路径' },
  { key: 'sign_script', label: '签名表达式', placeholder: '签名表达式' },
  { key: 'proxy_url', label: '代理 URL', placeholder: '代理 URL（可选）' },
  { key: 'next_toc_url_selector', label: '目录下一页 CSS', placeholder: '目录下一页 CSS，如 .next::attr(href)' },
  { key: 'next_content_url_selector', label: '正文下一页 CSS', placeholder: '正文下一页 CSS，如 .next::attr(href)' },
] as const
</script>

<template>
  <details class="source-editor">
    <summary>添加或导入书源</summary>
    <div class="source-import">
      <button type="button" class="secondary" :disabled="sources.importing" @click="sourceFile?.click()">
        导入 JSON 文件
      </button>
      <input
        ref="sourceFile"
        class="visually-hidden"
        type="file"
        accept=".json,application/json"
        @change="sources.importFromFile"
      />
      <form class="source-url-form" @submit.prevent="sources.importFromUrl()">
        <input v-model="sources.sourceUrl" type="url" placeholder="从 URL 导入 JSON" aria-label="书源 JSON URL" />
        <button type="submit" class="secondary" :disabled="sources.importing">导入 URL</button>
      </form>
    </div>

    <form @submit.prevent="sources.save()">
      <div class="source-fields">
        <input
          v-for="field in FIELDS"
          :key="field.key"
          v-model="sources.form[field.key]"
          :placeholder="field.placeholder"
          :aria-label="field.label"
        />
        <select v-model="sources.form.login_method" aria-label="登录方法">
          <option>POST</option>
          <option>GET</option>
          <option>PUT</option>
        </select>
      </div>
      <button type="submit" class="secondary" :disabled="sources.saving">
        {{ sources.saving ? '保存中...' : '保存书源' }}
      </button>
    </form>
  </details>

  <details v-if="sources.sources.length" class="source-editor">
    <summary>已配置的书源（{{ sources.sources.length }}）</summary>
    <div class="source-list">
      <div v-for="source in sources.sources" :key="source.id" class="source-list-item">
        <span>{{ source.name }}</span>
        <button
          type="button"
          class="secondary"
          :disabled="sources.testing === source.id"
          @click="sources.test(source, query)"
        >
          {{ sources.testing === source.id ? '测试中...' : '测试' }}
        </button>
      </div>
    </div>
  </details>

  <details v-if="sources.sources.some((source) => source.login_url)" class="source-editor">
    <summary>登录书源</summary>
    <form class="source-import" @submit.prevent="sources.login()">
      <select v-model.number="sources.loginForm.sourceId" aria-label="登录书源">
        <option :value="0">选择书源</option>
        <option v-for="source in sources.sources.filter((item) => item.login_url)" :key="source.id" :value="source.id">
          {{ source.name }}
        </option>
      </select>
      <input v-model="sources.loginForm.username" placeholder="用户名" aria-label="用户名" />
      <input v-model="sources.loginForm.password" type="password" placeholder="密码" aria-label="密码" />
      <button type="submit" class="secondary" :disabled="sources.loggingIn">
        {{ sources.loggingIn ? '登录中...' : '登录并保存会话' }}
      </button>
    </form>
  </details>

  <div v-if="sources.sources.some((source) => source.session_cookie || source.access_token)" class="source-sessions">
    <span v-for="source in sources.sources.filter((item) => item.session_cookie || item.access_token)" :key="source.id">
      {{ source.name }}：已认证
      <button type="button" class="secondary" @click="sources.clearSession(source)">清除</button>
    </span>
  </div>
</template>

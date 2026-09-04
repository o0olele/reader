<script setup lang="ts">
/* global HTMLInputElement */
import { inject, ref } from 'vue'
import { sourceDebugKey, sourcesKey } from '../../app/shellKeys'

defineProps<{
  /** Reused as the probe keyword when testing a source. */
  query: string
}>()

const sources = inject(sourcesKey)!
const sourceDebug = inject(sourceDebugKey)!
const sourceFile = ref<HTMLInputElement>()

function sessionState(source: { access_token?: string; session_cookie?: string; session_expires_at?: string }) {
  if (!source.access_token && !source.session_cookie) return '未认证'
  if (source.session_expires_at) {
    const raw = source.session_expires_at
    const expiry = /^\d+$/.test(raw) ? Number(raw) * 1000 : Date.parse(raw)
    if (Number.isFinite(expiry) && expiry <= Date.now()) return '已过期'
  }
  return '已认证'
}

const FIELDS = [
  { key: 'name', label: '名称', placeholder: '名称' },
  { key: 'base_url', label: '基础 URL', placeholder: '基础 URL，如 https://example.com' },
  { key: 'search_url', label: '搜索 URL', placeholder: '搜索 URL，使用 {{key}}' },
  { key: 'explore_url', label: '发现页 URL', placeholder: '发现页 URL，可用 名称::URL 多行配置' },
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
        <button type="button" class="secondary" @click="sourceDebug.openFor(source.id)">调试</button>
        <button type="button" class="secondary" @click="sources.browserAuth(source)">浏览器认证</button>
        <button type="button" class="secondary" @click="sources.saveBrowserSession(source)">读取浏览器会话</button>
      </div>
    </div>
  </details>

  <div v-if="sources.lastProbe" class="source-probe-status" role="status">
    <strong>{{ sources.lastProbe.source_name }}</strong>
    <span>HTTP {{ sources.lastProbe.status }}</span>
    <span>会话：{{ sources.lastProbe.session_state }}</span>
    <span>Cookie：{{ sources.lastProbe.has_cookie ? '已携带' : '未携带' }}</span>
    <span>Token：{{ sources.lastProbe.has_token ? '已携带' : '未携带' }}</span>
    <span>耗时：{{ sources.lastProbe.duration_ms }} ms</span>
    <span v-if="sources.lastProbe.cloudflare_challenge">需要浏览器完成 JavaScript 验证</span>
    <span v-else-if="sources.lastProbe.auth_required">需要重新认证</span>
    <span v-else>解析 {{ sources.lastProbe.result_count }} 条</span>
    <code>{{ sources.lastProbe.request_url }}</code>
    <code>UA={{ sources.lastProbe.user_agent }}</code>
  </div>

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
      <button type="button" class="secondary" :disabled="sources.loggingIn" @click="sources.refreshSession()">
        刷新会话
      </button>
    </form>
  </details>

  <div v-if="sources.sources.some((source) => source.session_cookie || source.access_token)" class="source-sessions">
    <span v-for="source in sources.sources.filter((item) => item.session_cookie || item.access_token)" :key="source.id">
      {{ source.name }}：{{ sessionState(source) }}
      <button
        v-if="sessionState(source) === '已过期'"
        type="button"
        class="secondary"
        @click="sources.loginForm.sourceId = source.id"
      >
        选择刷新
      </button>
      <button type="button" class="secondary" @click="sources.clearSession(source)">清除</button>
    </span>
  </div>
</template>

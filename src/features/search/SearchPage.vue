<script setup lang="ts">
import type { BookSearchResult, BookSource } from '../../services/api'

defineProps<{
  query: string
  results: BookSearchResult[]
  sources: BookSource[]
  searching: boolean
  importingSources: boolean
  savingSource: boolean
  addingResult?: string
  loggingIn: boolean
  sourceUrl: string
  loginSourceId: number
  username: string
  password: string
  sourceForm: Record<string, string>
}>()

const emit = defineEmits<{
  'update:query': [value: string]
  'update:source-url': [value: string]
  'update:login-source-id': [value: number]
  'update:username': [value: string]
  'update:password': [value: string]
  search: []
  importFile: [event: Event]
  importUrl: []
  saveSource: []
  login: []
  clearSession: [source: BookSource]
  add: [result: BookSearchResult]
}>()
</script>

<template>
  <div class="search-results">
    <details class="source-editor">
      <summary>添加或导入书源</summary>
      <div class="source-import">
        <button type="button" class="secondary" :disabled="importingSources" @click="($refs.sourceFile as HTMLInputElement).click()">
          导入 JSON 文件
        </button>
        <input ref="sourceFile" class="visually-hidden" type="file" accept=".json,application/json" @change="emit('importFile', $event)" />
        <form class="source-url-form" @submit.prevent="emit('importUrl')">
          <input :value="sourceUrl" type="url" placeholder="从 URL 导入 JSON" aria-label="书源 JSON URL" @input="emit('update:source-url', ($event.target as HTMLInputElement).value)" />
          <button type="submit" class="secondary" :disabled="importingSources">导入 URL</button>
        </form>
      </div>
      <form @submit.prevent="emit('saveSource')">
        <div class="source-fields">
          <input v-model="sourceForm.name" placeholder="名称" aria-label="书源名称" />
          <input v-model="sourceForm.base_url" placeholder="基础 URL，如 https://example.com" aria-label="基础 URL" />
          <input v-model="sourceForm.search_url" placeholder="搜索 URL，使用 {{key}}" aria-label="搜索 URL" />
          <input v-model="sourceForm.item" placeholder="结果项 CSS，如 .book" aria-label="结果项 CSS" />
          <input v-model="sourceForm.title" placeholder="标题 CSS，如 .title" aria-label="标题 CSS" />
          <input v-model="sourceForm.author" placeholder="作者 CSS（可选）" aria-label="作者 CSS" />
          <input v-model="sourceForm.url" placeholder="链接 CSS，如 a::attr(href)" aria-label="链接 CSS" />
          <input v-model="sourceForm.login_url" placeholder="登录 URL（可选）" aria-label="登录 URL" />
          <select v-model="sourceForm.login_method" aria-label="登录方法"><option>POST</option><option>GET</option><option>PUT</option></select>
          <input v-model="sourceForm.login_body" placeholder="登录 Body" aria-label="登录 Body" />
          <input v-model="sourceForm.token_path" placeholder="Token 路径" aria-label="Token 路径" />
          <input v-model="sourceForm.sign_script" placeholder="签名表达式" aria-label="签名表达式" />
          <input v-model="sourceForm.proxy_url" placeholder="代理 URL（可选）" aria-label="代理 URL" />
        </div>
        <button type="submit" class="secondary" :disabled="savingSource">{{ savingSource ? '保存中...' : '保存书源' }}</button>
      </form>
    </details>
    <details v-if="sources.some((source) => source.login_url)" class="source-editor">
      <summary>登录书源</summary>
      <form class="source-import" @submit.prevent="emit('login')">
        <select :value="loginSourceId" aria-label="登录书源" @change="emit('update:login-source-id', Number(($event.target as HTMLSelectElement).value))">
          <option :value="0">选择书源</option>
          <option v-for="source in sources.filter((item) => item.login_url)" :key="source.id" :value="source.id">{{ source.name }}</option>
        </select>
        <input :value="username" placeholder="用户名" aria-label="用户名" @input="emit('update:username', ($event.target as HTMLInputElement).value)" />
        <input :value="password" type="password" placeholder="密码" aria-label="密码" @input="emit('update:password', ($event.target as HTMLInputElement).value)" />
        <button type="submit" class="secondary" :disabled="loggingIn">{{ loggingIn ? '登录中...' : '登录并保存会话' }}</button>
      </form>
    </details>
    <div v-if="sources.some((source) => source.session_cookie || source.access_token)" class="source-sessions">
      <span v-for="source in sources.filter((item) => item.session_cookie || item.access_token)" :key="source.id">
        {{ source.name }}：已认证 <button type="button" class="secondary" @click="emit('clearSession', source)">清除</button>
      </span>
    </div>
    <div v-if="!searching && !results.length" class="search-empty">输入关键词后开始搜索</div>
    <article v-for="result in results" :key="result.url" class="search-result">
      <div class="book-cover">{{ result.title.slice(0, 1) }}</div>
      <div class="search-result-meta">
        <h2>{{ result.title }}</h2>
        <p>{{ result.author || '作者未知' }} · {{ result.source_name }}</p>
        <button type="button" class="secondary" :disabled="addingResult === result.url" @click="emit('add', result)">{{ addingResult === result.url ? '加入中...' : '加入书架' }}</button>
      </div>
    </article>
  </div>
</template>

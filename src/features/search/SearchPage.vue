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
  'update:source-form': [key: string, value: string]
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
          <input :value="sourceForm.name" placeholder="名称" aria-label="书源名称" @input="emit('update:source-form', 'name', ($event.target as HTMLInputElement).value)" />
          <input :value="sourceForm.base_url" placeholder="基础 URL，如 https://example.com" aria-label="基础 URL" @input="emit('update:source-form', 'base_url', ($event.target as HTMLInputElement).value)" />
          <input :value="sourceForm.search_url" placeholder="搜索 URL，使用 {{key}}" aria-label="搜索 URL" @input="emit('update:source-form', 'search_url', ($event.target as HTMLInputElement).value)" />
          <input :value="sourceForm.item" placeholder="结果项 CSS，如 .book" aria-label="结果项 CSS" @input="emit('update:source-form', 'item', ($event.target as HTMLInputElement).value)" />
          <input :value="sourceForm.title" placeholder="标题 CSS，如 .title" aria-label="标题 CSS" @input="emit('update:source-form', 'title', ($event.target as HTMLInputElement).value)" />
          <input :value="sourceForm.author" placeholder="作者 CSS（可选）" aria-label="作者 CSS" @input="emit('update:source-form', 'author', ($event.target as HTMLInputElement).value)" />
          <input :value="sourceForm.url" placeholder="链接 CSS，如 a::attr(href)" aria-label="链接 CSS" @input="emit('update:source-form', 'url', ($event.target as HTMLInputElement).value)" />
          <input :value="sourceForm.login_url" placeholder="登录 URL（可选）" aria-label="登录 URL" @input="emit('update:source-form', 'login_url', ($event.target as HTMLInputElement).value)" />
          <select :value="sourceForm.login_method" aria-label="登录方法" @change="emit('update:source-form', 'login_method', ($event.target as HTMLSelectElement).value)"><option>POST</option><option>GET</option><option>PUT</option></select>
          <input :value="sourceForm.login_body" placeholder="登录 Body" aria-label="登录 Body" @input="emit('update:source-form', 'login_body', ($event.target as HTMLInputElement).value)" />
          <input :value="sourceForm.token_path" placeholder="Token 路径" aria-label="Token 路径" @input="emit('update:source-form', 'token_path', ($event.target as HTMLInputElement).value)" />
          <input :value="sourceForm.sign_script" placeholder="签名表达式" aria-label="签名表达式" @input="emit('update:source-form', 'sign_script', ($event.target as HTMLInputElement).value)" />
          <input :value="sourceForm.proxy_url" placeholder="代理 URL（可选）" aria-label="代理 URL" @input="emit('update:source-form', 'proxy_url', ($event.target as HTMLInputElement).value)" />
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

<script setup lang="ts">
import { useReplaceRules } from './useReplaceRules'

const { rules, draft, busy, error, message, sampleTitle, sampleContent, preview, edit, save, remove, toggle, test } =
  useReplaceRules()
</script>

<template>
  <section class="source-editor replace-rules">
    <h2>净化替换</h2>
    <p>去除广告或替换阅读文字。规则按顺序执行，适用于本地和在线书籍；停用后重新打开章节即可恢复原文。</p>
    <p v-if="error" role="alert">{{ error }}</p>
    <p v-if="message" role="status">{{ message }}</p>
    <ul v-if="rules.length" class="rule-list">
      <li v-for="rule in rules" :key="rule.id">
        <span
          >{{ rule.sort_order }} · {{ rule.name }} <small v-if="rule.group">（{{ rule.group }}）</small></span
        >
        <button type="button" :disabled="busy" :aria-pressed="rule.enabled" @click="toggle(rule)">
          {{ rule.enabled ? '已启用' : '已停用' }}
        </button>
        <button type="button" :disabled="busy" @click="edit(rule)">编辑</button>
        <button type="button" :disabled="busy" @click="remove(rule)">删除</button>
      </li>
    </ul>
    <p v-else>还没有净化规则。</p>
    <form @submit.prevent="save">
      <fieldset :disabled="busy">
        <legend>{{ draft.id ? '编辑规则' : '新增规则' }}</legend>
        <div class="rule-fields">
          <label>名称<input v-model="draft.name" required /></label>
          <label>分组<input v-model="draft.group" /></label>
          <label>匹配内容<textarea v-model="draft.pattern" required rows="3" /></label>
          <label>替换为<textarea v-model="draft.replacement" rows="3" placeholder="留空则删除匹配内容" /></label>
          <label>作用范围<input v-model="draft.scope" placeholder="书名或书源地址；留空为全部" /></label>
          <label>排除范围<input v-model="draft.exclude_scope" placeholder="不应用规则的书名或书源地址" /></label>
          <label>执行顺序<input v-model.number="draft.sort_order" type="number" step="1" required /></label>
        </div>
        <p>范围可填写多个完整书名或书源地址，以换行或逗号分隔；排除范围优先。</p>
        <div class="rule-options">
          <label><input v-model="draft.is_regex" type="checkbox" />正则表达式</label>
          <label><input v-model="draft.scope_title" type="checkbox" />章节标题</label>
          <label><input v-model="draft.scope_content" type="checkbox" />正文</label>
          <label><input v-model="draft.enabled" type="checkbox" />启用</label>
        </div>
        <div class="rule-options">
          <button type="submit" class="primary">保存规则</button>
          <button type="button" @click="edit()">新增规则</button>
        </div>
      </fieldset>
    </form>
    <details>
      <summary>预览当前规则</summary>
      <p>使用下方样本文字测试当前编辑的规则，忽略启用状态和作用范围。</p>
      <div class="rule-fields">
        <label>样本标题<input v-model="sampleTitle" /></label>
        <label>样本正文<textarea v-model="sampleContent" rows="5" /></label>
      </div>
      <button type="button" :disabled="busy" @click="test">预览</button>
      <div v-if="preview" aria-live="polite">
        <h3>{{ preview.title }}</h3>
        <pre>{{ preview.content || '（替换后正文为空）' }}</pre>
      </div>
    </details>
  </section>
</template>

<style scoped>
.replace-rules {
  display: grid;
  gap: 12px;
}
.rule-list {
  list-style: none;
  padding: 0;
  display: grid;
  gap: 8px;
}
.rule-list li,
.rule-options {
  display: flex;
  gap: 12px;
  align-items: center;
  flex-wrap: wrap;
}
.rule-list li > span {
  flex: 1;
  min-width: 160px;
}
.rule-fields {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: 12px;
}
.rule-fields label {
  display: grid;
  gap: 6px;
}
fieldset {
  display: grid;
  gap: 12px;
  border: 1px solid currentColor;
  border-radius: 8px;
  padding: 16px;
}
input,
textarea {
  min-width: 0;
}
textarea {
  resize: vertical;
}
pre {
  white-space: pre-wrap;
  overflow-wrap: anywhere;
}
details > * {
  margin-top: 12px;
}
</style>

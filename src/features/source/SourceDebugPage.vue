<script setup lang="ts">
import { computed, inject } from 'vue'
import { sourceDebugKey } from '../../app/shellKeys'

const debug = inject(sourceDebugKey)!

const stageHint = computed(
  () => debug.stages.find((item) => item.value === debug.stage)?.hint ?? '输入 URL 或关键词',
)
const parsedText = computed(() =>
  debug.result?.parsed === undefined ? '' : JSON.stringify(debug.result.parsed, null, 2),
)
</script>

<template>
  <div class="debug-page">
    <div class="debug-toolbar">
      <label>
        书源
        <select v-model.number="debug.sourceId" aria-label="调试书源">
          <option :value="0">选择书源</option>
          <option v-for="source in debug.sourceOptions" :key="source.id" :value="source.id">
            {{ source.name }}
          </option>
        </select>
      </label>
      <label>
        阶段
        <select v-model="debug.stage" aria-label="调试阶段">
          <option v-for="item in debug.stages" :key="item.value" :value="item.value">
            {{ item.label }}
          </option>
        </select>
      </label>
      <input v-model="debug.input" class="debug-input" :placeholder="stageHint" aria-label="调试输入" />
      <button type="button" class="primary" :disabled="debug.running || !debug.sourceId" @click="debug.run()">
        {{ debug.running ? '执行中...' : '单步执行' }}
      </button>
      <button type="button" class="secondary" :disabled="debug.savingRules || !debug.sourceId" @click="debug.saveRules()">
        {{ debug.savingRules ? '保存中...' : '保存规则' }}
      </button>
    </div>

    <div class="debug-grid">
      <section class="debug-rules">
        <h2>规则编辑</h2>
        <label v-for="item in debug.stages" :key="item.value" class="debug-rule-field">
          {{ item.label }}
          <textarea v-model="debug.rules[item.value]" rows="5" spellcheck="false" :aria-label="`${item.label}规则`" />
        </label>
        <p class="debug-hint">“单步执行”使用编辑器中的当前规则；“保存规则”写回书源。</p>
      </section>

      <section class="debug-output">
        <h2>执行结果</h2>
        <div v-if="debug.running" class="search-empty">正在执行「{{ debug.stageLabel }}」阶段...</div>
        <div v-else-if="!debug.result" class="search-empty">选择书源和输入后点击「单步执行」</div>
        <template v-else>
          <div v-if="debug.result.error" class="error-banner">{{ debug.result.error }}</div>
          <details v-if="debug.steps.length" :open="!debug.result.raw_html && !parsedText">
            <summary>每步中间结果（{{ debug.steps.length }}）</summary>
            <table class="debug-steps">
              <thead>
                <tr>
                  <th>规则</th>
                  <th>输入片段</th>
                  <th>匹配</th>
                  <th>输出</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="(step, index) in debug.steps" :key="`${step.rule}-${index}`">
                  <td><code>{{ step.rule }}</code></td>
                  <td><code>{{ step.input ?? '' }}</code></td>
                  <td>{{ step.matched }}</td>
                  <td><code>{{ step.error ?? step.output ?? '' }}</code></td>
                </tr>
              </tbody>
            </table>
          </details>
          <details v-if="debug.result.raw_html">
            <summary>原始 HTML</summary>
            <pre class="debug-pre">{{ debug.result.raw_html }}</pre>
          </details>
          <details v-if="parsedText" :open="true">
            <summary>最终解析结果（JSON）</summary>
            <pre class="debug-pre">{{ parsedText }}</pre>
          </details>
          <div v-if="debug.result.request" class="source-probe-status" role="status">
            <strong>{{ debug.result.request.method }} {{ debug.result.request.url }}</strong>
            <span>HTTP {{ debug.result.request.status }}</span>
            <span>耗时：{{ debug.result.request.duration_ms }} ms</span>
            <span v-for="(value, key) in debug.result.request.headers" :key="key">{{ key }}: {{ value }}</span>
          </div>
          <div v-if="!debug.result.request && !debug.result.error" class="search-empty">该阶段没有发起网络请求</div>
        </template>
      </section>
    </div>
  </div>
</template>
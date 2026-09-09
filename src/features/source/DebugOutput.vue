<script setup lang="ts">
import { computed } from 'vue'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import type { SourceDebugResult } from '@/services/api'

const props = defineProps<{ result: SourceDebugResult }>()

const finalText = computed(() =>
  props.result.final_json === null || props.result.final_json === undefined
    ? ''
    : JSON.stringify(props.result.final_json, null, 2),
)
const headerText = computed(() =>
  props.result.request ? props.result.request.headers.map(([key, value]) => `${key}: ${value}`).join('\n') : '',
)
const responseHeaderText = computed(() =>
  props.result.response_headers.map(([key, value]) => `${key}: ${value}`).join('\n'),
)
</script>

<template>
  <div class="grid gap-3">
    <div v-if="result.error" class="rounded-md border border-destructive/50 bg-destructive/10 px-3 py-2 text-xs">
      {{ result.error }}
    </div>

    <div v-if="result.request" class="rounded-md border bg-card p-3 text-xs">
      <div class="font-semibold">{{ result.request.method }} {{ result.request.url }}</div>
      <div class="mt-1 flex flex-wrap gap-x-4 gap-y-1 text-muted-foreground">
        <span v-if="result.status !== undefined">HTTP {{ result.status }}</span>
        <span>耗时：{{ result.duration_ms }} ms</span>
        <span>会话：{{ result.session_state }}</span>
        <span v-if="result.request.auth_attached">已附加认证</span>
        <span v-if="result.request.charset">编码：{{ result.request.charset }}</span>
      </div>
    </div>
    <p v-else-if="!result.error" class="text-xs text-muted-foreground">该阶段没有发起网络请求。</p>

    <Tabs default-value="steps">
      <TabsList>
        <TabsTrigger value="steps">中间步骤（{{ result.steps.length }}）</TabsTrigger>
        <TabsTrigger value="json">最终 JSON</TabsTrigger>
        <TabsTrigger value="html">原始 HTML</TabsTrigger>
        <TabsTrigger value="http">请求 / 响应头</TabsTrigger>
      </TabsList>

      <TabsContent value="steps" class="mt-3">
        <table class="w-full border-collapse text-left text-xs">
          <thead>
            <tr class="border-b text-muted-foreground">
              <th class="py-1.5 pr-3 font-medium">规则</th>
              <th class="py-1.5 pr-3 font-medium">输入片段</th>
              <th class="py-1.5 pr-3 font-medium">匹配节点</th>
              <th class="py-1.5 font-medium">输出值</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(step, index) in result.steps" :key="`${step.field}-${index}`" class="border-b border-border/60">
              <td class="py-1.5 pr-3">
                <code>{{ step.field }}</code>
              </td>
              <td class="py-1.5 pr-3">
                <code class="text-muted-foreground">{{ step.input_preview }}</code>
              </td>
              <td class="py-1.5 pr-3">{{ step.node_count }}</td>
              <td class="py-1.5">
                <code :class="step.error ? 'text-destructive' : ''">{{ step.error ?? step.output_preview }}</code>
              </td>
            </tr>
            <tr v-if="!result.steps.length">
              <td colspan="4" class="py-3 text-center text-muted-foreground">该阶段没有产生规则步骤。</td>
            </tr>
          </tbody>
        </table>
      </TabsContent>

      <TabsContent value="json" class="mt-3">
        <pre v-if="finalText" class="max-h-[420px] overflow-auto rounded-md border bg-muted/40 p-3 text-xs">{{
          finalText
        }}</pre>
        <p v-else class="text-xs text-muted-foreground">该阶段没有结构化输出。</p>
      </TabsContent>

      <TabsContent value="html" class="mt-3">
        <pre
          v-if="result.raw_html"
          class="max-h-[420px] overflow-auto rounded-md border bg-muted/40 p-3 text-xs whitespace-pre-wrap"
          >{{ result.raw_html }}</pre>
        <p v-else class="text-xs text-muted-foreground">没有响应正文。</p>
      </TabsContent>

      <TabsContent value="http" class="mt-3 grid gap-3">
        <div>
          <div class="mb-1 text-xs font-medium">请求头</div>
          <pre class="max-h-52 overflow-auto rounded-md border bg-muted/40 p-3 text-xs">{{
            headerText || '（无）'
          }}</pre>
        </div>
        <div v-if="result.request?.body">
          <div class="mb-1 text-xs font-medium">请求体</div>
          <pre class="max-h-52 overflow-auto rounded-md border bg-muted/40 p-3 text-xs whitespace-pre-wrap">{{
            result.request.body
          }}</pre>
        </div>
        <div>
          <div class="mb-1 text-xs font-medium">响应头</div>
          <pre class="max-h-52 overflow-auto rounded-md border bg-muted/40 p-3 text-xs">{{
            responseHeaderText || '（无）'
          }}</pre>
        </div>
      </TabsContent>
    </Tabs>
  </div>
</template>

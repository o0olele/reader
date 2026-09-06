import { onMounted, ref } from 'vue'
import { getErrorMessage } from '../../services/api/error'
import {
  deleteReplaceRule,
  listReplaceRules,
  previewReplaceRule,
  saveReplaceRule,
  type ProcessedContent,
  type ReplaceRule,
} from '../../services/api/replaceRules'

function emptyRule(): ReplaceRule {
  return {
    id: 0,
    name: '',
    group: '',
    pattern: '',
    replacement: '',
    is_regex: true,
    scope: '',
    scope_title: false,
    scope_content: true,
    exclude_scope: '',
    sort_order: 0,
    enabled: true,
  }
}

export function useReplaceRules() {
  const rules = ref<ReplaceRule[]>([])
  const draft = ref(emptyRule())
  const busy = ref(false)
  const error = ref('')
  const message = ref('')
  const sampleTitle = ref('第一章')
  const sampleContent = ref('')
  const preview = ref<ProcessedContent>()

  async function run(action: () => Promise<void>) {
    busy.value = true
    error.value = ''
    message.value = ''
    try {
      await action()
    } catch (cause) {
      error.value = getErrorMessage(cause)
    } finally {
      busy.value = false
    }
  }

  function edit(rule?: ReplaceRule) {
    draft.value = rule ? { ...rule } : emptyRule()
    preview.value = undefined
    error.value = ''
    message.value = ''
  }

  async function save() {
    await run(async () => {
      draft.value = await saveReplaceRule(draft.value)
      rules.value = await listReplaceRules()
      message.value = '规则已保存，下次打开章节时生效。'
    })
  }

  async function remove(rule: ReplaceRule) {
    await run(async () => {
      await deleteReplaceRule(rule.id)
      if (draft.value.id === rule.id) edit()
      rules.value = await listReplaceRules()
      message.value = '规则已删除。'
    })
  }

  async function toggle(rule: ReplaceRule) {
    await run(async () => {
      const saved = await saveReplaceRule({ ...rule, enabled: !rule.enabled })
      if (draft.value.id === rule.id) draft.value.enabled = saved.enabled
      rules.value = await listReplaceRules()
      message.value = saved.enabled ? '规则已启用。' : '规则已停用，再次打开章节可恢复原文。'
    })
  }

  async function test() {
    preview.value = undefined
    await run(async () => {
      preview.value = await previewReplaceRule(draft.value, sampleTitle.value, sampleContent.value)
    })
  }

  onMounted(
    () =>
      void run(async () => {
        rules.value = await listReplaceRules()
      }),
  )
  return { rules, draft, busy, error, message, sampleTitle, sampleContent, preview, edit, save, remove, toggle, test }
}

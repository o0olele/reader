import { invoke } from '@tauri-apps/api/core'

export interface ReplaceRule {
  id: number
  name: string
  group: string | null
  pattern: string
  replacement: string
  is_regex: boolean
  scope: string | null
  scope_title: boolean
  scope_content: boolean
  exclude_scope: string | null
  sort_order: number
  enabled: boolean
}

export interface ProcessedContent {
  title: string
  content: string
}

export const listReplaceRules = (): Promise<ReplaceRule[]> => invoke('list_replace_rules')
export const saveReplaceRule = (rule: ReplaceRule): Promise<ReplaceRule> => invoke('save_replace_rule', { rule })
export const deleteReplaceRule = (id: number): Promise<void> => invoke('delete_replace_rule', { id })
export const previewReplaceRule = (rule: ReplaceRule, title: string, content: string): Promise<ProcessedContent> =>
  invoke('preview_replace_rule', { rule, title, content })

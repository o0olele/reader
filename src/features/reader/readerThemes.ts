/** 阅读背景主题的唯一定义处：底栏主题 popover 与阅读样式面板共用。
 *  `value` 会拼成 `theme-<value>` 类名（见 `src/styles.css`），改名要同步样式。 */
export const READER_THEMES = [
  { value: 'light', label: '浅色', swatch: 'bg-[oklch(0.99_0.003_106)]' },
  { value: 'sepia', label: '护眼', swatch: 'bg-[oklch(0.95_0.03_70)]' },
  { value: 'dark', label: '深色', swatch: 'bg-[oklch(0.27_0.02_265)]' },
  { value: 'black', label: '黑夜', swatch: 'bg-black' },
] as const

export type ReaderTheme = (typeof READER_THEMES)[number]['value']

/** 底栏按钮要显示「当前主题」，存储值意外失配时回落到浅色。 */
export function readerTheme(value: string) {
  return READER_THEMES.find((theme) => theme.value === value) ?? READER_THEMES[0]
}

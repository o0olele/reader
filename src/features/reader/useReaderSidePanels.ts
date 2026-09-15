import { computed, reactive, ref } from 'vue'
import type { useChangeSource } from './useChangeSource'

/** 右侧只有一个 320px 槽位（ROADMAP-v3 §4 panel），同一时刻最多一块面板。 */
export type ReaderPanel = 'book' | 'search' | 'settings'

/**
 * 右侧槽位的开关状态。
 *
 * 书籍信息 / 正文搜索 / 阅读设置 由这里持有，换源面板的开关则属于
 * `useChangeSource`（换源搜索要能跨面板开关继续跑），所以这里只负责
 * 「开一块就收起其余几块」这条唯一的约束。
 */
export function useReaderSidePanels(changeSource: ReturnType<typeof useChangeSource>, initial?: ReaderPanel) {
  const panel = ref<ReaderPanel | undefined>(initial)
  const bookOpen = computed(() => panel.value === 'book')
  const searchOpen = computed(() => panel.value === 'search')
  const settingsOpen = computed(() => panel.value === 'settings')
  const sourceOpen = computed(() => changeSource.open)

  function toggle(target: ReaderPanel) {
    changeSource.close()
    panel.value = panel.value === target ? undefined : target
  }

  /** 换源面板和另外三块共用一个槽位，所以打开它要先把别的收起来。 */
  function toggleSource() {
    if (changeSource.open) {
      changeSource.close()
      return
    }
    panel.value = undefined
    changeSource.start()
  }

  function close() {
    panel.value = undefined
  }

  return reactive({ panel, bookOpen, searchOpen, settingsOpen, sourceOpen, toggle, toggleSource, close })
}

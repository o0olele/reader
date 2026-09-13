import { computed, nextTick, ref, watch, type Ref } from 'vue'
import type { SearchContentHit } from '@/services/api'
import { revealParagraph } from './readerPosition'

/** The range to mark, addressed the way the backend reports a hit. */
export interface HighlightRange {
  /** `null` marks the chapter title instead of a body paragraph. */
  paragraphIndex: number | null
  offset: number
  length: number
}

/**
 * Landing on a 正文搜索 hit: switch to its chapter, mark the exact range and
 * scroll it into view.
 *
 * The mark survives until another chapter is opened — a highlight that vanishes
 * on the next scroll would be gone the moment the jump animation ends.
 */
export function useContentHighlight(
  element: Ref<HTMLElement | null>,
  chapterId: () => number | undefined,
  mode: () => 'scroll' | 'paged',
  goToChapter: (chapterId: number) => Promise<void>,
) {
  const highlight = ref<HighlightRange | null>(null)
  /** True while a jump owns the reading position. */
  const busy = ref(false)

  watch(chapterId, () => {
    if (!busy.value) highlight.value = null
  })

  const titleRange = computed(() => (highlight.value?.paragraphIndex === null ? highlight.value : undefined))

  function paragraphRange(index: number) {
    return highlight.value?.paragraphIndex === index ? highlight.value : undefined
  }

  async function jumpToHit(hit: SearchContentHit) {
    busy.value = true
    try {
      if (hit.chapter_id !== chapterId()) {
        // Never carry the previous chapter's mark into the new one.
        highlight.value = null
        await goToChapter(hit.chapter_id)
      }
      highlight.value = {
        paragraphIndex: hit.paragraph_index,
        offset: hit.match_offset,
        length: hit.match_length,
      }
      await nextTick()
      const container = element.value
      if (!container) return
      const target =
        hit.paragraph_index === null
          ? container.querySelector<HTMLElement>('.reader-title')
          : container.querySelector<HTMLElement>(`[data-reader-index="${hit.paragraph_index}"]`)
      if (target) revealParagraph(container, target, mode())
    } finally {
      busy.value = false
    }
  }

  return { highlight, titleRange, paragraphRange, busy, jumpToHit }
}

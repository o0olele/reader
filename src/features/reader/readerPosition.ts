/* global HTMLElement, getComputedStyle */

export interface ReadingLocator {
  index: number
  ratio: number
}

const clampRatio = (value: number) => Math.min(1, Math.max(0, Number.isFinite(value) ? value : 0))

function pageStep(element: HTMLElement): number {
  const style = getComputedStyle(element)
  const gap = Number.parseFloat(style.columnGap || '0') || 0
  const horizontalPadding = (Number.parseFloat(style.paddingLeft) || 0) + (Number.parseFloat(style.paddingRight) || 0)
  return Math.max(1, element.clientWidth - horizontalPadding + gap)
}

export function captureReadingLocator(element: HTMLElement, mode: 'scroll' | 'paged'): ReadingLocator {
  const viewport = element.getBoundingClientRect()
  const paragraphs = Array.from(element.querySelectorAll<HTMLElement>('[data-reader-index]'))

  for (const paragraph of paragraphs) {
    const index = Number(paragraph.dataset.readerIndex ?? 0)
    if (mode === 'scroll') {
      const rect = paragraph.getBoundingClientRect()
      if (rect.bottom > viewport.top) {
        return { index, ratio: clampRatio((viewport.top - rect.top) / Math.max(1, rect.height)) }
      }
      continue
    }

    const fragments = Array.from(paragraph.getClientRects())
    const fragmentIndex = fragments.findIndex(
      (rect) => rect.right > viewport.left + 1 && rect.left < viewport.right - 1,
    )
    if (fragmentIndex >= 0) {
      return { index, ratio: fragments.length > 1 ? fragmentIndex / (fragments.length - 1) : 0 }
    }
  }

  const last = paragraphs.at(-1)
  return { index: Number(last?.dataset.readerIndex ?? 0), ratio: 1 }
}

export function restoreReadingLocator(element: HTMLElement, mode: 'scroll' | 'paged', locator: ReadingLocator) {
  const paragraph = element.querySelector<HTMLElement>(`[data-reader-index="${Math.max(0, locator.index)}"]`)
  if (!paragraph) return
  const viewport = element.getBoundingClientRect()

  if (mode === 'scroll') {
    const rect = paragraph.getBoundingClientRect()
    element.scrollTop += rect.top + clampRatio(locator.ratio) * rect.height - viewport.top
    return
  }

  const fragments = Array.from(paragraph.getClientRects())
  const fragment = fragments[Math.round(clampRatio(locator.ratio) * Math.max(0, fragments.length - 1))]
  if (!fragment) return
  const target = element.scrollLeft + fragment.left - viewport.left
  element.scrollLeft = Math.round(target / pageStep(element)) * pageStep(element)
}

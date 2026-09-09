import { onBeforeUnmount, onMounted, watch, type Ref } from 'vue'
import { captureReadingLocator, restoreReadingLocator } from './readerPosition'

export type ReaderMode = 'scroll' | 'paged'

/**
 * Paged-mode geometry and navigation: page width/height measured from the live
 * element (a fixed `column-width` drifts when the side panel or window
 * changes), page turning, auto-page advancing, and the arrow-key layer.
 */
export function useReaderPaging(
  contentRef: Ref<HTMLElement | null>,
  readerMode: () => ReaderMode,
  onModeChange: (mode: ReaderMode) => void,
  onScroll: () => void,
) {
  let resizeObserver: ResizeObserver | undefined

  function syncPageMetrics() {
    const element = contentRef.value
    if (!element) return
    const style = getComputedStyle(element)
    const horizontalPadding = (Number.parseFloat(style.paddingLeft) || 0) + (Number.parseFloat(style.paddingRight) || 0)
    const verticalPadding = (Number.parseFloat(style.paddingTop) || 0) + (Number.parseFloat(style.paddingBottom) || 0)
    const pageWidth = Math.max(1, element.clientWidth - horizontalPadding)
    const pageHeight = Math.max(1, element.clientHeight - verticalPadding - 72)
    element.style.setProperty('--reader-page-width', `${pageWidth}px`)
    element.style.setProperty('--reader-page-height', `${pageHeight}px`)
  }

  function turnPage(direction: number) {
    const element = contentRef.value
    if (!element || readerMode() !== 'paged') return
    const gap = Number.parseFloat(getComputedStyle(element).columnGap || '48') || 48
    const style = getComputedStyle(element)
    const pageWidth =
      Number.parseFloat(style.getPropertyValue('--reader-page-width')) ||
      Math.max(
        1,
        element.clientWidth -
          (Number.parseFloat(style.paddingLeft) || 0) -
          (Number.parseFloat(style.paddingRight) || 0),
      )
    element.scrollBy({ left: direction * (pageWidth + gap), behavior: 'smooth' })
  }

  /** Auto page-turn: paged mode flips a column, scroll mode advances a viewport. */
  function advance() {
    const element = contentRef.value
    if (!element) return
    if (readerMode() === 'paged') turnPage(1)
    else element.scrollBy({ top: element.clientHeight * 0.9, behavior: 'smooth' })
  }

  function changeMode(mode: ReaderMode) {
    const element = contentRef.value
    const locator = element ? captureReadingLocator(element, readerMode()) : undefined
    onModeChange(mode)
    void Promise.resolve().then(() => {
      if (!contentRef.value || !locator) return
      restoreReadingLocator(contentRef.value, mode, locator)
      onScroll()
    })
  }

  function onKeydown(event: KeyboardEvent) {
    if (readerMode() !== 'paged') return
    if (event.key === 'ArrowRight' || event.key === 'PageDown' || event.key === ' ') {
      event.preventDefault()
      turnPage(1)
    } else if (event.key === 'ArrowLeft' || event.key === 'PageUp') {
      event.preventDefault()
      turnPage(-1)
    }
  }

  onMounted(() => {
    syncPageMetrics()
    window.addEventListener('keydown', onKeydown)
  })

  // The content element is behind a `v-if`, so it appears after mount.
  watch(
    contentRef,
    (element) => {
      resizeObserver?.disconnect()
      if (!element || typeof ResizeObserver === 'undefined') return
      resizeObserver = new ResizeObserver(() => syncPageMetrics())
      resizeObserver.observe(element)
      syncPageMetrics()
    },
    { immediate: true },
  )

  onBeforeUnmount(() => {
    window.removeEventListener('keydown', onKeydown)
    resizeObserver?.disconnect()
  })

  return { syncPageMetrics, turnPage, advance, changeMode }
}

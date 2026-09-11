import { onBeforeUnmount, onMounted, ref, watch, type Ref } from 'vue'
import { captureReadingLocator, pageStep, restoreReadingLocator } from './readerPosition'

export type ReaderMode = 'scroll' | 'paged'

/** Prototype :871–885 — below either threshold the spread collapses. */
const SPREAD_MIN_HEIGHT = 820
const SPREAD_MIN_WIDTH = 760
const COLUMN_GAP = 64

type ReaderLayout = 'spread' | 'single-scroll' | 'scroll'

/**
 * Paged-mode geometry and navigation.
 *
 * The prototype renders the text stage as a two-column spread with a vertical
 * column rule (`desktop-ui.html:834–836`) and falls back to a single scrolling
 * column when the viewport is shorter than 820px or the text width is below
 * 760px (`:870–885`). Page geometry is measured from the live element, because a
 * fixed `column-width` drifts whenever the side panel, the window or the
 * font size changes.
 */
export function useReaderPaging(
  contentRef: Ref<HTMLElement | null>,
  readerMode: () => ReaderMode,
  onModeChange: (mode: ReaderMode) => void,
  onScroll: () => void,
) {
  const spreadEligible = ref(true)
  const pageIndex = ref(0)
  const pageCount = ref(1)
  const chapterPercent = ref(0)

  let resizeObserver: ResizeObserver | undefined
  let frame = 0

  function layout(): ReaderLayout {
    if (readerMode() !== 'paged') return 'scroll'
    return spreadEligible.value ? 'spread' : 'single-scroll'
  }

  /** Reading position within the chapter, along whichever axis is scrolling. */
  function progressPercent(element: HTMLElement): number {
    const [position, max] =
      layout() === 'spread'
        ? [element.scrollLeft, element.scrollWidth - element.clientWidth]
        : [element.scrollTop, element.scrollHeight - element.clientHeight]
    if (max <= 0) return 100
    return Math.min(100, Math.max(0, Math.round((position / max) * 100)))
  }

  function syncPosition() {
    const element = contentRef.value
    if (!element) return
    chapterPercent.value = progressPercent(element)
    if (layout() !== 'spread') {
      pageCount.value = 1
      pageIndex.value = 0
      return
    }
    const step = pageStep(element)
    pageCount.value = Math.max(1, Math.ceil(element.scrollWidth / step))
    pageIndex.value = Math.min(pageCount.value - 1, Math.max(0, Math.round(element.scrollLeft / step)))
  }

  function syncPageMetrics() {
    const element = contentRef.value
    if (!element) return
    const style = getComputedStyle(element)
    const horizontalPadding = (Number.parseFloat(style.paddingLeft) || 0) + (Number.parseFloat(style.paddingRight) || 0)
    const verticalPadding = (Number.parseFloat(style.paddingTop) || 0) + (Number.parseFloat(style.paddingBottom) || 0)
    const pageWidth = Math.max(1, element.clientWidth - horizontalPadding)
    const pageHeight = Math.max(1, element.clientHeight - verticalPadding)

    spreadEligible.value = window.innerHeight >= SPREAD_MIN_HEIGHT && pageWidth >= SPREAD_MIN_WIDTH
    const columnWidth = spreadEligible.value ? Math.max(1, (pageWidth - COLUMN_GAP) / 2) : pageWidth
    element.style.setProperty('--reader-page-width', `${pageWidth}px`)
    element.style.setProperty('--reader-page-height', `${pageHeight}px`)
    element.style.setProperty('--reader-page-gap', `${COLUMN_GAP}px`)
    element.style.setProperty('--reader-column-width', `${columnWidth}px`)
    syncPosition()
  }

  function turnPage(direction: number) {
    const element = contentRef.value
    if (!element || layout() !== 'spread') return
    element.scrollBy({ left: direction * pageStep(element), behavior: 'smooth' })
  }

  /** Auto page-turn: the spread flips a page, scrolling modes advance a viewport. */
  function advance() {
    const element = contentRef.value
    if (!element) return
    if (layout() === 'spread') turnPage(1)
    else element.scrollBy({ top: element.clientHeight * 0.9, behavior: 'smooth' })
  }

  function changeMode(mode: ReaderMode) {
    const element = contentRef.value
    const locator = element ? captureReadingLocator(element, readerMode()) : undefined
    onModeChange(mode)
    void Promise.resolve().then(() => {
      syncPageMetrics()
      if (!contentRef.value || !locator) return
      restoreReadingLocator(contentRef.value, mode, locator)
      syncPosition()
      onScroll()
    })
  }

  function handleScroll() {
    if (frame) return
    if (typeof window.requestAnimationFrame !== 'function') {
      syncPosition()
      onScroll()
      return
    }
    frame = window.requestAnimationFrame(() => {
      frame = 0
      syncPosition()
      onScroll()
    })
  }

  function onKeydown(event: KeyboardEvent) {
    if (layout() !== 'spread') return
    if (event.key === 'ArrowRight' || event.key === 'PageDown' || event.key === ' ') {
      event.preventDefault()
      turnPage(1)
    } else if (event.key === 'ArrowLeft' || event.key === 'PageUp') {
      event.preventDefault()
      turnPage(-1)
    }
  }

  let observed: HTMLElement | null = null

  function observe(element: HTMLElement | null) {
    resizeObserver?.disconnect()
    observed?.removeEventListener('scroll', handleScroll)
    observed = element
    if (!element) return
    element.addEventListener('scroll', handleScroll, { passive: true })
    if (typeof ResizeObserver === 'undefined') return
    resizeObserver = new ResizeObserver(() => syncPageMetrics())
    resizeObserver.observe(element)
  }

  onMounted(() => {
    window.addEventListener('keydown', onKeydown)
    window.addEventListener('resize', syncPageMetrics)
  })

  // The content element sits behind a `v-if`, so it appears after mount.
  watch(
    contentRef,
    (element) => {
      observe(element)
      if (element) syncPageMetrics()
    },
    { immediate: true },
  )

  // A font / margin change reflows the columns; re-measure after the paint.
  watch(
    () => [readerMode(), spreadEligible.value],
    () => void Promise.resolve().then(() => syncPageMetrics()),
  )

  onBeforeUnmount(() => {
    window.removeEventListener('keydown', onKeydown)
    window.removeEventListener('resize', syncPageMetrics)
    if (frame && typeof window.cancelAnimationFrame === 'function') window.cancelAnimationFrame(frame)
    observed?.removeEventListener('scroll', handleScroll)
    resizeObserver?.disconnect()
  })

  return { syncPageMetrics, turnPage, advance, changeMode, spreadEligible, pageIndex, pageCount, chapterPercent }
}

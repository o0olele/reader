import { computed, nextTick, ref, watch } from 'vue'

export interface WindowedListOptions {
  /** A mounted row is the ruler for every row, so the scroller needs one selector. */
  rowSelector: string
  /** Space between rows in px; must match the grid gap in the markup. */
  gap?: number
  /** Rows mounted beyond the viewport on each side. */
  overscan?: number
  /** Row height used until a mounted row has been measured. */
  fallbackRowHeight?: number
}

/**
 * Windowing for a list whose rows share one box model: only the rows around the
 * viewport are mounted, and the window is pure arithmetic — one measured row
 * height plus the gap — with no per-row measurement bookkeeping.
 *
 * A book-source collection routinely runs to several hundred rows, and a row can
 * carry inputs, a switch and a Reka dropdown, so mounting all of them at once
 * costs thousands of nodes on first paint. This keeps that cost proportional to
 * the viewport instead.
 *
 * `items` is a getter so callers can window a prop or any derived array. A new
 * array resets the scroll position, because the window can otherwise sit past
 * the end of a shorter list.
 */
export function useWindowedList<T>(items: () => readonly T[], options: WindowedListOptions) {
  const gap = options.gap ?? 8
  const overscan = options.overscan ?? 8
  const { rowSelector } = options

  const rows = computed(items)
  const viewport = ref<HTMLElement>()
  const scrollTop = ref(0)
  const viewportHeight = ref(0)
  const rowHeight = ref(options.fallbackRowHeight ?? 56)

  const stride = computed(() => rowHeight.value + gap)
  const trackHeight = computed(() => (rows.value.length ? rows.value.length * stride.value - gap : 0))
  const startIndex = computed(() => Math.max(0, Math.floor(scrollTop.value / stride.value) - overscan))
  const endIndex = computed(() =>
    Math.min(rows.value.length, Math.ceil((scrollTop.value + viewportHeight.value) / stride.value) + overscan),
  )
  const visible = computed(() => rows.value.slice(startIndex.value, endIndex.value))
  const offset = computed(() => startIndex.value * stride.value)

  /** Scroll handler: the hot path, so it only reads the one value the window needs. */
  function syncScroll() {
    const element = viewport.value
    if (element) scrollTop.value = element.scrollTop
  }

  /** A mounted row is the ruler for every row, and the box decides how many fit. */
  function measure() {
    const element = viewport.value
    if (!element) return
    viewportHeight.value = element.clientHeight
    const row = element.querySelector<HTMLElement>(rowSelector)
    if (row?.offsetHeight) rowHeight.value = row.offsetHeight
  }

  // The scroller only exists once there is something to scroll, which can happen
  // long after mount (the first refresh resolving), so bind on the ref instead.
  watch(
    viewport,
    (element, _previous, onCleanup) => {
      if (!element) return
      syncScroll()
      measure()
      const observer = new ResizeObserver(measure)
      observer.observe(element)
      onCleanup(() => observer.disconnect())
    },
    { flush: 'post' },
  )

  watch(rows, () => {
    scrollTop.value = 0
    if (viewport.value) viewport.value.scrollTop = 0
    void nextTick(measure)
  })

  return { viewport, trackHeight, visible, offset, syncScroll }
}

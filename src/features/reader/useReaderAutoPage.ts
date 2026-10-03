import { onBeforeUnmount, ref, watch } from 'vue'

const PAGE_INTERVAL_SECONDS = 12
const INTERVAL_KEY = 'reader-auto-page-interval'
const validInterval = (seconds: number) => Number.isInteger(seconds) && seconds >= 1 && seconds <= 300

/** Wait a full interval for each loaded page; never overlap chapter requests. */
export function useReaderAutoPage(
  ready: () => boolean,
  position: () => string,
  advance: () => boolean | Promise<boolean>,
  reportError: (cause: unknown) => void,
) {
  const saved =
    typeof localStorage === 'undefined'
      ? PAGE_INTERVAL_SECONDS
      : Number(localStorage.getItem(INTERVAL_KEY) ?? PAGE_INTERVAL_SECONDS)
  const intervalSeconds = ref(validInterval(saved) ? saved : PAGE_INTERVAL_SECONDS)
  const active = ref(false)
  const secondsLeft = ref(intervalSeconds.value)
  const advancing = ref(false)
  let timer: ReturnType<typeof setInterval> | undefined
  let session = 0

  function clearTimer() {
    if (timer !== undefined) clearInterval(timer)
    timer = undefined
  }

  function stop() {
    session++
    active.value = false
    clearTimer()
  }

  function start(seconds = intervalSeconds.value) {
    if (!ready() || !validInterval(seconds)) return false
    intervalSeconds.value = seconds
    if (typeof localStorage !== 'undefined') localStorage.setItem(INTERVAL_KEY, String(seconds))
    session++
    active.value = true
    return true
  }

  async function tick() {
    if (!active.value || !ready() || advancing.value) return
    if (--secondsLeft.value > 0) return
    const request = session
    advancing.value = true
    try {
      if (!(await advance()) && request === session) stop()
    } catch (cause) {
      if (request === session) {
        stop()
        reportError(cause)
      }
    } finally {
      advancing.value = false
    }
  }

  watch(
    [active, ready, position, advancing, intervalSeconds],
    () => {
      clearTimer()
      secondsLeft.value = intervalSeconds.value
      if (active.value && ready() && !advancing.value) timer = setInterval(() => void tick(), 1000)
    },
    { flush: 'sync' },
  )
  onBeforeUnmount(stop)

  return { active, secondsLeft, intervalSeconds, start, stop }
}

import { computed, reactive, ref, watch } from 'vue'
import {
  getErrorMessage,
  previewBookSource,
  searchBooks,
  setBookSourceEnabled,
  type Book,
  type BookSearchResult,
  type SourceBookPreview,
} from '@/services/api'
import { useShellContext } from '@/app/shellKeys'
import { notifyError, notifyInfo, notifySuccess } from '@/app/useToast'

/** How many sources are probed at once. The search page uses the same width. */
const SEARCH_CONCURRENCY = 6
/** How many candidates may load 目录/详情 in parallel. */
const PREVIEW_CONCURRENCY = 3
const SETTINGS_KEY = 'change-source-settings'
/** `AppPattern.authorRegex` of the reference app: 「作者：」前缀与「著」后缀。 */
const AUTHOR_NOISE = /^\s*作\s*者[:：\s]+|\s+著/g

export interface ChangeSourceSettings {
  /** 校验作者：书名相同还要作者对得上，避免同名书换错源。 */
  checkAuthor: boolean
  /** 加载详情：换源前先读封面/简介，列表里也就能显示这些字段。 */
  loadInfo: boolean
  /** 加载目录：换源前先取目录（同时验证书源可用），换源时直接复用。 */
  loadToc: boolean
}

/** One row of the 换源 sheet: a source that has this book. */
export interface ChangeSourceCandidate {
  result: BookSearchResult
  preview?: SourceBookPreview
  /** A preview call in flight for this row. */
  previewing: boolean
  /** Why this source cannot be switched to — shown instead of hiding the row. */
  error?: string
}

function loadSettings(): ChangeSourceSettings {
  try {
    const stored = JSON.parse(localStorage.getItem(SETTINGS_KEY) ?? '{}') as Partial<ChangeSourceSettings>
    return { checkAuthor: false, loadInfo: false, loadToc: false, ...stored }
  } catch {
    return { checkAuthor: false, loadInfo: false, loadToc: false }
  }
}

const normalizeTitle = (value: string) => value.replace(/\s+/g, '').toLowerCase()

/**
 * 换源：对**所有启用的书源**重新搜这本书，列出各源给出的书名/作者/最新章节，
 * 由用户挑一个换过去。
 *
 * 对位参考项目的 `ChangeSourceSheet` + `ChangeBookSourceComposeViewModel`：
 * 逐源并发搜索、边搜边出结果、按书源评分排序、可按源名/最新章节筛选，并有
 * 校验作者 / 加载详情 / 加载目录 三个选项。每行的「置顶 / 置底」记在本地偏好里
 * （对应参考项目的 `ObservableSourceConfig` 每本书独立评分），不会改动书源的
 * 全局权重；「禁用」直接停用该书源。参考项目里的「编辑 / 删除书源」落在书源
 * 管理页，这里不做，以免在阅读器里误删书源。
 */
export function useChangeSource() {
  const { reader, sources, bookshelf } = useShellContext()

  const open = ref(false)
  const searching = ref(false)
  const filter = ref('')
  const candidates = ref<ChangeSourceCandidate[]>([])
  const completed = ref(0)
  const total = ref(0)
  const lastSourceName = ref('')
  const switchingUrl = ref<string>()
  const settings = reactive(loadSettings())
  /** 这本书的源偏好，置顶/置底改的就是它。 */
  const pinned = ref<number[]>([])

  let token = 0
  let bookId: number | undefined
  let keyword = ''
  let oldAuthor = ''

  watch(settings, () => localStorage.setItem(SETTINGS_KEY, JSON.stringify(settings)), { deep: true })

  const book = computed<Book | undefined>(() => reader.selectedBook)
  const bookTitle = computed(() => book.value?.title ?? '')
  const currentSourceId = computed(() => book.value?.source_id)
  const currentSourceName = computed(
    () => sources.sources.find((source) => source.id === currentSourceId.value)?.name ?? '未知书源',
  )
  const isCurrent = (candidate: ChangeSourceCandidate) => candidate.result.source_id === currentSourceId.value

  /** 参考项目 `filterResults()`：先按筛选词过滤，再按「置顶 > 权重 > 章节数」排。 */
  const visible = computed(() => {
    const key = filter.value.trim()
    const matched = key
      ? candidates.value.filter(
          (candidate) =>
            candidate.result.source_name.includes(key) ||
            (candidate.preview?.latest_chapter ?? candidate.result.latest_chapter ?? '').includes(key),
        )
      : candidates.value
    return [...matched].sort((left, right) => {
      const pinLeft = pinned.value.indexOf(left.result.source_id)
      const pinRight = pinned.value.indexOf(right.result.source_id)
      const pinOrder =
        (pinLeft < 0 ? Number.MAX_SAFE_INTEGER : pinLeft) - (pinRight < 0 ? Number.MAX_SAFE_INTEGER : pinRight)
      if (pinOrder !== 0) return pinOrder
      const weight = sourceWeight(right.result.source_id) - sourceWeight(left.result.source_id)
      if (weight !== 0) return weight
      return chapterCount(right) - chapterCount(left)
    })
  })

  function sourceWeight(sourceId: number): number {
    const source = sources.sources.find((item) => item.id === sourceId)
    return source?.weight ?? 0
  }

  function chapterCount(candidate: ChangeSourceCandidate): number {
    return candidate.preview?.chapters.length ?? -1
  }

  const hasToc = (candidate: ChangeSourceCandidate) => Boolean(candidate.preview?.chapters.length)

  function pinKey(id: number) {
    return `change-source-pin:${id}`
  }

  function readPinned(id: number): number[] {
    try {
      const stored = JSON.parse(localStorage.getItem(pinKey(id)) ?? '[]') as unknown
      return Array.isArray(stored) ? stored.filter((value): value is number => typeof value === 'number') : []
    } catch {
      return []
    }
  }

  function writePinned() {
    if (bookId !== undefined) localStorage.setItem(pinKey(bookId), JSON.stringify(pinned.value))
  }

  /** 参考项目的 `topSource` / `bottomSource`，作用域是这一本书。 */
  function pinCandidate(candidate: ChangeSourceCandidate, toTop: boolean) {
    const id = candidate.result.source_id
    const rest = pinned.value.filter((value) => value !== id)
    pinned.value = toTop ? [id, ...rest] : [...rest, id]
    writePinned()
  }

  async function disableSource(candidate: ChangeSourceCandidate) {
    const wasCurrent = isCurrent(candidate)
    try {
      await setBookSourceEnabled(candidate.result.source_id, false)
      candidates.value = candidates.value.filter((item) => item.result.source_id !== candidate.result.source_id)
      await sources.refresh()
      notifyInfo(
        wasCurrent
          ? `${candidate.result.source_name} 已停用；这本书还在用它的缓存正文，请另选一个书源换源`
          : `${candidate.result.source_name} 已停用`,
      )
    } catch (cause) {
      notifyError(getErrorMessage(cause))
    }
  }

  function stop() {
    token++
    searching.value = false
  }

  function close() {
    stop()
    open.value = false
  }

  /** The sheet binds to these instead of writing the prop it was handed. */
  function setFilter(value: string | number) {
    filter.value = String(value)
  }

  function toggleOption(key: keyof ChangeSourceSettings) {
    settings[key] = !settings[key]
  }

  /** Opens the sheet for the book the reader has open and searches every source. */
  function start() {
    const current = reader.selectedBook
    if (!current?.source_id) {
      notifyInfo('本地书籍没有在线书源')
      return
    }
    bookId = current.id
    keyword = current.title.trim()
    oldAuthor = (current.author ?? '').replace(AUTHOR_NOISE, '').trim()
    pinned.value = readPinned(current.id)
    filter.value = ''
    candidates.value = []
    open.value = true
    void run()
  }

  /** 逐源搜索，结果边到边显示；参考项目 `ChangeSourceSearchUseCase.search`。 */
  async function run() {
    if (bookId === undefined) return
    const runToken = ++token
    searching.value = true
    candidates.value = []
    completed.value = 0
    lastSourceName.value = ''
    const enabled = sources.sources.filter((source) => source.enabled)
    if (!enabled.length) {
      searching.value = false
      notifyInfo('没有启用的书源')
      return
    }
    total.value = enabled.length
    let cursor = 0
    const worker = async () => {
      while (runToken === token) {
        const source = enabled[cursor++]
        if (!source) return
        lastSourceName.value = source.name
        try {
          const response = await searchBooks(keyword, source.id)
          if (runToken !== token) return
          for (const group of response.groups) {
            for (const result of group.sources) {
              if (matches(result)) candidates.value.push({ result, previewing: false })
            }
          }
        } catch {
          // 单个书源失败不该打断整轮换源搜索：它只是不出现在结果里。
        } finally {
          if (runToken === token) completed.value++
        }
      }
    }
    await Promise.all(Array.from({ length: Math.min(SEARCH_CONCURRENCY, enabled.length) }, worker))
    if (runToken !== token) return
    searching.value = false
    await previewMissing()
  }

  /** 参考项目的 `fName == name && (!checkAuthor || fAuthor.contains(author))`。 */
  function matches(result: BookSearchResult): boolean {
    if (normalizeTitle(result.title) !== normalizeTitle(keyword)) return false
    if (!settings.checkAuthor || !oldAuthor) return true
    return (result.author ?? '').includes(oldAuthor)
  }

  /** 加载详情 / 加载目录：按需读取，已经读到的部分不重复请求。 */
  async function previewMissing() {
    if (!bookId) return
    const runToken = token
    const pending = candidates.value.filter(
      (candidate) =>
        !candidate.error &&
        ((settings.loadInfo && !candidate.preview?.info) || (settings.loadToc && !hasToc(candidate))),
    )
    let cursor = 0
    const worker = async () => {
      while (runToken === token) {
        const candidate = pending[cursor++]
        if (!candidate) return
        candidate.previewing = true
        try {
          const preview = await previewBookSource(
            bookId as number,
            candidate.result,
            settings.loadInfo,
            settings.loadToc,
          )
          if (runToken !== token) return
          candidate.preview = {
            info: preview.info ?? candidate.preview?.info ?? null,
            chapters: settings.loadToc ? preview.chapters : (candidate.preview?.chapters ?? []),
            latest_chapter: preview.latest_chapter ?? candidate.preview?.latest_chapter ?? null,
            info_error: preview.info_error ?? candidate.preview?.info_error ?? null,
          }
        } catch (cause) {
          if (runToken === token) candidate.error = getErrorMessage(cause)
        } finally {
          candidate.previewing = false
        }
      }
    }
    await Promise.all(Array.from({ length: Math.min(PREVIEW_CONCURRENCY, pending.length) }, worker))
  }

  /** 换源：带上已加载的目录，换完停在原来那一章（`useReader.switchSource`）。 */
  async function switchTo(candidate: ChangeSourceCandidate) {
    const current = reader.selectedBook
    if (!current) return
    if (candidate.error) return
    // 参考项目换源前会 `stopSearch()`：搜索的结果已经没有意义了。
    stop()
    switchingUrl.value = candidate.result.url
    try {
      await reader.switchSource(
        candidate.result,
        candidate.preview?.chapters?.length ? candidate.preview.chapters : undefined,
      )
      await bookshelf.refresh()
      notifySuccess(`已换源到 ${candidate.result.source_name}`)
      open.value = false
    } catch (cause) {
      notifyError(getErrorMessage(cause))
    } finally {
      switchingUrl.value = undefined
    }
  }

  // 校验作者会改变结果集合，参考项目同样重跑一轮（`refresh()`）。
  watch(
    () => settings.checkAuthor,
    () => {
      if (open.value) void run()
    },
  )
  // 打开选项后才去读详情/目录，不必事先替用户把所有书源都拉一遍。
  watch([() => settings.loadInfo, () => settings.loadToc], () => {
    if (open.value) void previewMissing()
  })

  return reactive({
    open,
    searching,
    filter,
    candidates,
    visible,
    completed,
    total,
    lastSourceName,
    switchingUrl,
    settings,
    bookTitle,
    currentSourceName,
    isCurrent,
    start,
    stop,
    close,
    run,
    setFilter,
    toggleOption,
    switchTo,
    pinCandidate,
    disableSource,
  })
}

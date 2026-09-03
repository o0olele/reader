import { computed, onBeforeUnmount, onMounted, reactive, ref, watch } from 'vue'
import { useRouter } from 'vue-router'
import {
  debugSourceStage,
  updateBookSourceRules,
  type RawSourceRules,
  type SourceDebugProgress,
  type SourceDebugResult,
  type SourceDebugStage,
} from '../../services/api'
import { on as onAppEvent } from '../../services/events'
import type { useSources } from './useSources'

/** Debug stage options, in the order the rule engine executes them. */
export const DEBUG_STAGES: { value: SourceDebugStage; label: string; hint: string }[] = [
  { value: 'search', label: '搜索', hint: '搜索关键词' },
  { value: 'book_info', label: '详情', hint: '详情页 URL' },
  { value: 'toc', label: '目录', hint: '目录页 URL' },
  { value: 'content', label: '正文', hint: '正文页 URL' },
]

/**
 * Drives the source debugger: stage selection, rule editing, one-stage
 * execution with streamed progress, and persisting edited rules back to the
 * source so the next run uses them.
 */
export function useSourceDebug(
  report: (cause: unknown) => void,
  notify: (message: string) => void,
  sources: ReturnType<typeof useSources>,
) {
  const router = useRouter()
  const sourceId = ref<number>()
  const stage = ref<SourceDebugStage>('search')
  const input = ref('')
  const rules = ref<RawSourceRules>({})
  const running = ref(false)
  const savingRules = ref(false)
  const result = ref<SourceDebugResult>()
  const progressState = ref<SourceDebugProgress['state']>()

  const sourceOptions = computed(() => sources.sources)
  const currentSource = computed(() => sourceOptions.value.find((item) => item.id === sourceId.value))
  const stageLabel = computed(() => DEBUG_STAGES.find((item) => item.value === stage.value)?.label ?? '')

  let stopProgress: (() => void) | undefined
  onMounted(() => {
    stopProgress = onAppEvent('source-test-progress', (payload) => {
      const event = payload as SourceDebugProgress | undefined
      if (!event || event.source_id !== sourceId.value) return
      progressState.value = event.state
    })
  })
  onBeforeUnmount(() => stopProgress?.())

  /** Load a source's saved rules into the editor and drop stale results. */
  function loadRules(id: number) {
    const source = sourceOptions.value.find((item) => item.id === id)
    if (!source) return
    rules.value = {
      search: source.raw_rules.search ?? '',
      book_info: source.raw_rules.book_info ?? '',
      toc: source.raw_rules.toc ?? '',
      content: source.raw_rules.content ?? '',
    }
    result.value = undefined
    progressState.value = undefined
  }

  watch(sourceId, (id) => {
    if (id) loadRules(id)
  })

  /** Debug entry from the source list: load the source's rules, open the debug view. */
  function openFor(id: number) {
    if (!sourceOptions.value.some((item) => item.id === id)) return
    sourceId.value = id
    loadRules(id)
    input.value = ''
    void router.push({ name: 'sources' })
  }

  async function run() {
    if (!sourceId.value) {
      notify('请先选择要调试的书源')
      return
    }
    running.value = true
    progressState.value = 'started'
    result.value = undefined
    try {
      const current = await debugSourceStage(sourceId.value, stage.value, input.value.trim())
      result.value = current
      const name = currentSource.value?.name ?? current.source_name
      notify(
        current.error
          ? `${name} 的${stageLabel.value}阶段调试失败：${current.error}`
          : `${name} 的${stageLabel.value}阶段执行完成，耗时 ${current.duration_ms} ms`,
      )
    } catch (cause) {
      report(cause)
    } finally {
      running.value = false
      progressState.value = undefined
    }
  }

  async function saveRules() {
    if (!sourceId.value) {
      notify('请先选择要调试的书源')
      return
    }
    savingRules.value = true
    try {
      await updateBookSourceRules(sourceId.value, rules.value)
      await sources.refresh()
      notify(`${currentSource.value?.name ?? '书源'} 的规则已保存，单步执行将使用新规则`)
    } catch (cause) {
      report(cause)
    } finally {
      savingRules.value = false
    }
  }

  return reactive({
    stages: DEBUG_STAGES,
    sourceId,
    sourceOptions,
    currentSource,
    stage,
    input,
    rules,
    running,
    savingRules,
    result,
    progressState,
    stageLabel,
    openFor,
    run,
    saveRules,
  })
}
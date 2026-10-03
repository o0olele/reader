import { computed, watch, type Ref } from 'vue'
import { useShellContext } from '@/app/shellKeys'
import { notifyError, notifyInfo } from '@/app/useToast'
import { getErrorMessage } from '@/services/api'
import type ReaderPane from './ReaderPane.vue'
import { useReaderAutoPage } from './useReaderAutoPage'

/** Connect the timer to the current viewport and the reader's async chapter loader. */
export function useReaderAutoAdvance(pane: Ref<InstanceType<typeof ReaderPane> | undefined>) {
  const { reader } = useShellContext()
  const ready = computed(() =>
    Boolean(reader.selectedChapter?.content.trim() && !reader.loadingChapter && !reader.switchingSource && pane.value),
  )
  const auto = useReaderAutoPage(
    () => ready.value,
    () => `${reader.selectedBook?.id}:${reader.selectedChapter?.id}:${reader.readerMode}`,
    async () => {
      const result = pane.value?.advance()
      if (result !== 'end') return result === 'moved'
      const index = reader.chapters.findIndex((chapter) => chapter.id === reader.selectedChapter?.id)
      const next = reader.chapters[index + 1]
      if (next) {
        const loaded = await reader.selectChapter(next)
        if (loaded && reader.selectedChapter?.content.trim()) return true
        notifyInfo('正文未加载成功，自动翻页已停止')
        return false
      }
      notifyInfo('已到最后一章，自动翻页已停止')
      return false
    },
    (cause) => notifyError(getErrorMessage(cause)),
  )
  watch(() => reader.selectedBook?.id, auto.stop)

  function start(seconds: number) {
    const wasActive = auto.active.value
    if (auto.start(seconds))
      notifyInfo(`${wasActive ? '已调整自动翻页' : '已开启自动翻页'}：每 ${seconds} 秒翻一页，章末自动续读`)
  }

  function pause() {
    auto.stop()
    notifyInfo('自动翻页已暂停')
  }

  return { ...auto, ready, start, pause }
}

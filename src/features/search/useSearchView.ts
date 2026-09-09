import { computed, ref } from 'vue'
import type { SearchResultGroup } from '@/services/api'
import { useShellContext } from '@/app/shellKeys'

export type SearchType = 'all' | 'novel' | 'comic' | 'audio'

function matchesType(kind: string | undefined, type: SearchType) {
  if (type === 'all') return true
  const value = (kind || '').toLowerCase()
  if (type === 'comic') return value.includes('漫画') || value.includes('comic') || value.includes('manhua')
  if (type === 'audio') return value.includes('音频') || value.includes('有声') || value.includes('audio')
  return (
    !value || value.includes('小说') || value.includes('novel') || (!value.includes('漫画') && !value.includes('音频'))
  )
}

/**
 * Mirrors legado's SearchResultMerger: exact title/author matches first, then
 * kind tags containing the keyword, then partial matches, then the rest. Inside
 * the first three buckets, books returned by more sources rank higher.
 */
function rankBucket(group: SearchResultGroup, keyword: string) {
  const title = group.title.trim().toLowerCase()
  const author = (group.author ?? '').trim().toLowerCase()
  const kind = (group.sources[0]?.kind ?? '').toLowerCase()
  if (title === keyword || author === keyword) return 0
  if (kind.includes(keyword)) return 1
  if (title.includes(keyword) || author.includes(keyword)) return 2
  return 3
}

/** Search-page view state: type filter, source-group filter and result ranking. */
export function useSearchView() {
  const { search, sources } = useShellContext()
  const typeFilter = ref<SearchType>('all')
  const sourceGroup = ref('全部书源')

  search.setSourceIdsProvider(() =>
    sources.sources
      .filter(
        (source) =>
          source.enabled &&
          (sourceGroup.value === '全部书源' || (source.source_group || '未分组') === sourceGroup.value),
      )
      .map((source) => source.id),
  )

  const sourceGroups = computed(() => {
    const groups = new Set(
      sources.sources.filter((source) => source.enabled).map((source) => source.source_group || '未分组'),
    )
    return ['全部书源', ...groups]
  })
  const enabledSourceIds = computed(() => sources.sources.filter((source) => source.enabled).map((source) => source.id))
  const selectedSourceCount = computed(() =>
    search.selectedSourceIds === null ? enabledSourceIds.value.length : search.selectedSourceIds.length,
  )

  const visibleGroups = computed(() =>
    search.groups.filter((group) => {
      if (!group.sources.some((source) => matchesType(source.kind, typeFilter.value))) return false
      if (
        sourceGroup.value !== '全部书源' &&
        !group.sources.some(
          (source) => sources.sources.find((item) => item.id === source.source_id)?.source_group === sourceGroup.value,
        )
      ) {
        return false
      }
      return true
    }),
  )

  const rankedGroups = computed(() => {
    const keyword = search.query.trim().toLowerCase()
    if (!keyword) return visibleGroups.value
    return [...visibleGroups.value].sort((a, b) => {
      const rankA = rankBucket(a, keyword)
      const rankB = rankBucket(b, keyword)
      if (rankA !== rankB) return rankA - rankB
      return rankA === 3 ? 0 : b.sources.length - a.sources.length
    })
  })

  function selectSourceGroup(group: string) {
    sourceGroup.value = group
    search.selectedSourceIds = null
  }

  const canOpenBrowserAuth = (reason: string, authRequired: boolean) =>
    authRequired || reason.includes('Cloudflare challenge') || reason.includes('需要浏览器执行 JavaScript 验证')

  function openBrowserAuth(sourceId: number) {
    const source = sources.sources.find((item) => item.id === sourceId)
    if (source) void sources.browserAuth(source)
  }

  return {
    search,
    sources,
    typeFilter,
    sourceGroup,
    sourceGroups,
    enabledSourceIds,
    selectedSourceCount,
    visibleGroups,
    rankedGroups,
    selectSourceGroup,
    canOpenBrowserAuth,
    openBrowserAuth,
  }
}

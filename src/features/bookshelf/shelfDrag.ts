/** Custom MIME type used when dragging shelf cards onto a sidebar group. */
export const BOOK_DRAG_TYPE = 'application/x-bookshelf-books'

export function writeDraggedBookIds(event: DragEvent, ids: number[]) {
  if (!event.dataTransfer) return
  event.dataTransfer.setData(BOOK_DRAG_TYPE, JSON.stringify(ids))
  event.dataTransfer.setData('text/plain', ids.length > 1 ? `${ids.length} 本书` : String(ids[0] ?? ''))
  event.dataTransfer.effectAllowed = 'move'
}

export function readDraggedBookIds(event: DragEvent): number[] {
  const raw = event.dataTransfer?.getData(BOOK_DRAG_TYPE)
  if (!raw) return []
  try {
    const parsed: unknown = JSON.parse(raw)
    return Array.isArray(parsed) ? parsed.filter((id: unknown) => typeof id === 'number' && Number.isFinite(id)) : []
  } catch {
    return []
  }
}

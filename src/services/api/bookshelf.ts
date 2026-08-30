import { invoke } from '@tauri-apps/api/core'
import type { BookshelfGroup } from './types'

export function listGroups(): Promise<BookshelfGroup[]> {
  return invoke<BookshelfGroup[]>('list_groups')
}

export function createGroup(name: string): Promise<BookshelfGroup> {
  return invoke<BookshelfGroup>('create_group', { name })
}

export function moveBookToGroup(bookId: number, groupId: number): Promise<void> {
  return invoke<void>('move_book_to_group', { bookId, groupId })
}

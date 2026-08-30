/* global HTMLElement, HTMLInputElement, localStorage */
import { ref, type Ref } from 'vue'
import type { Book, BookSearchResult, BookSource, Chapter, BookshelfGroup } from '../services/api'

export type SourceForm = Record<string, string>
export type AppShellState = {
  status: Ref<string>; error: Ref<string>; books: Ref<Book[]>; groups: Ref<BookshelfGroup[]>; activeGroup: Ref<number | null>
  fileInput: Ref<HTMLInputElement | undefined>; selectedBook: Ref<Book | undefined>; chapters: Ref<Chapter[]>; selectedChapter: Ref<Chapter | undefined>
  loadingChapter: Ref<boolean>; onlineSearch: Ref<boolean>; settingsTab: Ref<boolean>; proxyUrl: Ref<string>; savingSettings: Ref<boolean>
  searchQuery: Ref<string>; searchResults: Ref<BookSearchResult[]>; searching: Ref<boolean>; addingResult: Ref<string | undefined>
  sourceForm: Ref<SourceForm>; savingSource: Ref<boolean>; sourceUrl: Ref<string>; importingSources: Ref<boolean>; bookSources: Ref<BookSource[]>
  testingSource: Ref<number | undefined>; loginForm: Ref<{ sourceId: number; username: string; password: string }>; loggingIn: Ref<boolean>
  readerContent: Ref<HTMLElement | null>; fontSize: Ref<number>; theme: Ref<string>
}

export function createAppShellState(): AppShellState {
  return {
    status: ref('检查中...'), error: ref(''), books: ref([]), groups: ref([]), activeGroup: ref(null), fileInput: ref(), selectedBook: ref(),
    chapters: ref([]), selectedChapter: ref(), loadingChapter: ref(false), onlineSearch: ref(false), settingsTab: ref(false), proxyUrl: ref(''),
    savingSettings: ref(false), searchQuery: ref(''), searchResults: ref([]), searching: ref(false), addingResult: ref(), sourceForm: ref({
      name: '', base_url: '', search_url: '', item: '', title: '', author: '', url: '', login_url: '', login_method: 'POST', login_body: '', token_path: '', sign_script: '', proxy_url: '',
    }), savingSource: ref(false), sourceUrl: ref(''), importingSources: ref(false), bookSources: ref([]), testingSource: ref(),
    loginForm: ref({ sourceId: 0, username: '', password: '' }), loggingIn: ref(false), readerContent: ref(null),
    fontSize: ref(Number(localStorage.getItem('reader-font-size') ?? '17')), theme: ref(localStorage.getItem('reader-theme') ?? 'light'),
  }
}

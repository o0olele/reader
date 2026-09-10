import type { LucideIcon } from 'lucide-vue-next'
import {
  BookOpen,
  Bookmark,
  Clock3,
  Compass,
  Database,
  Download,
  Home,
  Rss,
  Search,
  Settings,
  User,
} from 'lucide-vue-next'

export interface ShellNavItem {
  to: string
  label: string
  icon: LucideIcon
  /** Live counter rendered as a badge; `undefined` renders nothing. */
  badge?: 'books'
}

export interface ShellNavGroup {
  label: string
  items: ShellNavItem[]
}

/** Prototype rail groups (`desktop-ui.html:1420–1432`), extended with the real
 *  search / source / download pages this build already has. */
export const shellNavGroups: ShellNavGroup[] = [
  {
    label: '浏览',
    items: [
      { to: '/', label: '首页', icon: Home },
      { to: '/bookshelf', label: '书架', icon: BookOpen, badge: 'books' },
      { to: '/explore', label: '发现', icon: Compass },
      { to: '/rss', label: 'RSS', icon: Rss },
    ],
  },
  {
    label: '检索',
    items: [
      { to: '/search', label: '搜索', icon: Search },
      { to: '/sources', label: '书源', icon: Database },
      { to: '/downloads', label: '下载', icon: Download },
      { to: '/history', label: '历史', icon: Clock3 },
      { to: '/bookmarks', label: '书签', icon: Bookmark },
    ],
  },
  {
    label: '阅读',
    items: [{ to: '/read', label: '阅读器', icon: BookOpen }],
  },
]

export const shellFooterNav: ShellNavItem[] = [
  { to: '/my', label: '我的', icon: User },
  { to: '/settings', label: '设置', icon: Settings },
]

/** Breadcrumb label per route name; unknown names fall back to the raw name. */
export const routeTitles: Record<string, string> = {
  home: '首页',
  bookshelf: '书架',
  explore: '发现',
  rss: 'RSS',
  read: '阅读器',
  search: '搜索',
  sources: '书源',
  downloads: '下载',
  history: '历史',
  bookmarks: '书签',
  my: '我的',
  settings: '设置',
}

/** Prototype digit shortcuts 1–5 (`desktop-ui.html:2768`). */
export const shortcutRoutes: Record<string, string> = {
  '1': 'home',
  '2': 'bookshelf',
  '3': 'explore',
  '4': 'rss',
  '5': 'my',
}

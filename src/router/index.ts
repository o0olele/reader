import { createRouter, createWebHashHistory } from 'vue-router'
import AppLayout from '../app/AppLayout.vue'

/**
 * Real routes: every page is its own component under a single layout, and the
 * URL is the only navigation state (ROADMAP-v3 F0). The reader is deep-linkable
 * as `#/read/<bookId>?chapter=<id>&toc=0&panel=1`.
 */
export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: '/',
      component: AppLayout,
      children: [
        { path: '', name: 'home', component: () => import('../features/home/HomePage.vue') },
        { path: 'bookshelf', name: 'bookshelf', component: () => import('../features/bookshelf/BookshelfPage.vue') },
        { path: 'explore', name: 'explore', component: () => import('../features/search/ExplorePage.vue') },
        { path: 'rss', name: 'rss', component: () => import('../features/rss/RssPage.vue') },
        { path: 'read/:bookId?', name: 'read', component: () => import('../features/reader/ReaderPage.vue') },
        { path: 'search', name: 'search', component: () => import('../features/search/SearchPage.vue') },
        { path: 'sources', name: 'sources', component: () => import('../features/source/SourceManagerPage.vue') },
        {
          path: 'sources/debug',
          name: 'source-debug',
          component: () => import('../features/source/SourceDebugPage.vue'),
        },
        { path: 'downloads', name: 'downloads', component: () => import('../features/download/DownloadPage.vue') },
        { path: 'history', name: 'history', component: () => import('../features/history/HistoryPage.vue') },
        { path: 'bookmarks', name: 'bookmarks', component: () => import('../features/bookmark/BookmarksPage.vue') },
        { path: 'my', name: 'my', component: () => import('../features/my/MyPage.vue') },
        { path: 'settings/:pane?', name: 'settings', component: () => import('../features/settings/SettingsPage.vue') },
      ],
    },
    { path: '/:pathMatch(.*)*', redirect: '/' },
  ],
})

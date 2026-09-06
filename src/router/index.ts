import { createRouter, createWebHashHistory } from 'vue-router'
import AppShell from '../app/AppShell.vue'

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', name: 'bookshelf', component: AppShell },
    { path: '/search', name: 'search', component: AppShell },
    { path: '/explore', name: 'explore', component: AppShell },
    { path: '/sources', name: 'sources', component: AppShell },
    { path: '/settings', name: 'settings', component: AppShell },
    { path: '/downloads', name: 'downloads', component: AppShell },
    { path: '/rss', name: 'rss', component: AppShell },
    { path: '/history', name: 'history', component: AppShell },
    { path: '/bookmarks', name: 'bookmarks', component: AppShell },
  ],
})

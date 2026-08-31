import { createRouter, createWebHashHistory } from 'vue-router'
import AppShell from '../app/AppShell.vue'

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', name: 'bookshelf', component: AppShell },
    { path: '/search', name: 'search', component: AppShell },
    { path: '/settings', name: 'settings', component: AppShell },
  ],
})

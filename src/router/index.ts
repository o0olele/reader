import { createRouter, createWebHashHistory } from 'vue-router'
import AppShell from '../app/AppShell.vue'

// The shell currently owns the MVP views. Keeping a router boundary in place
// lets feature pages move out incrementally without changing the Tauri entrypoint.
export const router = createRouter({
  history: createWebHashHistory(),
  routes: [{ path: '/', component: AppShell }],
})

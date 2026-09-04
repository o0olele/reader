import { createApp } from 'vue'
import App from './app/AppRoot.vue'
import { router } from './router'
import { reportWebviewUserAgent } from './services/api'
import './styles.css'

createApp(App).use(router).mount('#app')

// Fire and forget: every HTTP request should impersonate this webview, but
// none of them should wait on it. Until it lands the backend runs on the value
// cached from the previous launch, so this only matters when the webview
// runtime updated between sessions.
void reportWebviewUserAgent().catch(() => {
  /* browser preview mode has no Tauri backend */
})

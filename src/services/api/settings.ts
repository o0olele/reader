import { invoke } from '@tauri-apps/api/core'
import type { AppSettings } from './types'

export function healthCheck(): Promise<string> {
  return invoke<string>('health_check')
}

export function getAppSettings(): Promise<AppSettings> {
  return invoke<AppSettings>('get_app_settings')
}

export function saveAppSettings(proxyUrl: string, userAgent = ''): Promise<AppSettings> {
  return invoke<AppSettings>('save_app_settings', {
    input: { proxy_url: proxyUrl || undefined, user_agent: userAgent || undefined },
  })
}

/**
 * Tells the backend what this webview calls itself, so outbound requests can
 * present the same identity as the browser-auth window. Cloudflare binds
 * `cf_clearance` to the User-Agent: if the window solves the challenge as
 * Edge/WebView2 and reqwest then replays the cookie as something else, the
 * cookie is rejected and the source looks broken for no visible reason.
 */
export function reportWebviewUserAgent(): Promise<void> {
  return invoke<void>('report_webview_user_agent', { userAgent: window.navigator.userAgent })
}

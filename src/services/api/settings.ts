import { invoke } from '@tauri-apps/api/core'
import type { AppSettings } from './types'

export function healthCheck(): Promise<string> {
  return invoke<string>('health_check')
}

export function getAppSettings(): Promise<AppSettings> {
  return invoke<AppSettings>('get_app_settings')
}

export function saveAppSettings(proxyUrl: string): Promise<AppSettings> {
  return invoke<AppSettings>('save_app_settings', { input: { proxy_url: proxyUrl || undefined } })
}

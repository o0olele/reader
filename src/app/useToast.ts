import { toast } from 'vue-sonner'

/** The single place the app surfaces a message; replaces the old inline
 *  `error-banner` div in `AppShell.vue`. */
export function notifyError(message: string) {
  toast.error(message)
}

export function notifyInfo(message: string) {
  toast(message)
}

export function notifySuccess(message: string) {
  toast.success(message)
}

import { defineStore } from 'pinia'

export const useAppStore = defineStore('app', {
  state: () => ({ status: '就绪' as string }),
  actions: {
    setStatus(status: string) {
      this.status = status
    },
  },
})

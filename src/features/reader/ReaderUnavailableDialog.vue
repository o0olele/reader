<script setup lang="ts">
import { computed } from 'vue'
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import NotConnected from '@/components/NotConnected.vue'

const props = defineProps<{
  /** The prototype tool that this build deliberately does not fake (纪律 F0). */
  tool?: { title: string; description: string; capabilities: string[] }
}>()
const emit = defineEmits<{ close: [] }>()

const open = computed({
  get: () => Boolean(props.tool),
  set: (value: boolean) => {
    if (!value) emit('close')
  },
})
</script>

<template>
  <Dialog v-model:open="open">
    <DialogContent class="sm:max-w-md">
      <DialogHeader><DialogTitle>未接入</DialogTitle></DialogHeader>
      <NotConnected v-if="tool" :title="tool.title" :description="tool.description" :capabilities="tool.capabilities" />
    </DialogContent>
  </Dialog>
</template>

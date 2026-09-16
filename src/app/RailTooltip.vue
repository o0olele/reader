<script setup lang="ts">
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip'

/** Rail affordance hint. Rendered on the right of the icon and switched off
 *  while the rail is extended, where the icons already carry their labels. */
defineProps<{ label: string; disabled?: boolean }>()
</script>

<template>
  <Tooltip :disabled="disabled">
    <TooltipTrigger as-child>
      <slot />
    </TooltipTrigger>
    <TooltipContent side="right" align="center">{{ label }}</TooltipContent>
  </Tooltip>
</template>

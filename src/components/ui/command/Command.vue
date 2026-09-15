<script setup lang="ts">
import type { ListboxRootEmits, ListboxRootProps } from "reka-ui"
import type { HTMLAttributes } from "vue"
import { reactiveOmit } from "@vueuse/core"
import { ListboxRoot, useFilter, useForwardPropsEmits } from "reka-ui"
import { computed, reactive, ref } from "vue"
import { cn } from "@/lib/utils"
import { provideCommandContext } from "."

const props = withDefaults(defineProps<ListboxRootProps & { class?: HTMLAttributes["class"] }>(), {
  modelValue: "",
})

const emits = defineEmits<ListboxRootEmits>()

const delegatedProps = reactiveOmit(props, "class")

const forwarded = useForwardPropsEmits(delegatedProps, emits)

const allItems = ref<Map<string, string>>(new Map())
const allGroups = ref<Map<string, Set<string>>>(new Map())

const { contains } = useFilter({ sensitivity: "base" })

const search = ref("")

/**
 * The visible set is derived, not stored: an item that mounts *while* a query
 * is typed — a book or source row produced by the caller's own matching — must
 * be scored in the same tick it registers. A watcher over the search string
 * alone ran before that registration, so such rows kept the "first render"
 * exemption, `count` stayed 0, and the empty state reported no matches while
 * result rows were on screen.
 */
const filtered = computed(() => {
  const items = new Map<string, number>()
  const groups = new Set<string>()
  if (!search.value) return { count: allItems.value.size, items, groups }

  let count = 0
  for (const [id, value] of allItems.value) {
    const matched = contains(value, search.value)
    items.set(id, matched ? 1 : 0)
    if (matched) count += 1
  }

  // Keep only the groups that still have at least one visible item.
  for (const [groupId, members] of allGroups.value) {
    for (const itemId of members) {
      if ((items.get(itemId) ?? 0) > 0) {
        groups.add(groupId)
        break
      }
    }
  }

  return { count, items, groups }
})

const filterState = reactive({ search, filtered })

provideCommandContext({
  allItems,
  allGroups,
  filterState,
})
</script>

<template>
  <ListboxRoot
    v-bind="forwarded"
    :class="cn('flex h-full w-full flex-col overflow-hidden rounded-md bg-popover text-popover-foreground', props.class)"
  >
    <slot />
  </ListboxRoot>
</template>

<script setup lang="ts">
import type { ListboxItemEmits, ListboxItemProps } from "reka-ui"
import type { HTMLAttributes } from "vue"
import { reactiveOmit, useCurrentElement } from "@vueuse/core"
import { ListboxItem, useForwardPropsEmits, useId } from "reka-ui"
import { computed, onMounted, onUnmounted, ref, watch } from "vue"
import { cn } from "@/lib/utils"
import { useCommand, useCommandGroup } from "."

/**
 * `keywords` is extra text the item matches on, on top of its rendered label.
 * It carries the fields a row is built from but does not print (a book's
 * author, a source's group), so a row whose owner already matched the query can
 * never be hidden by the list filter, and the filter text can be replaced when
 * the label is dynamic (e.g. it quotes the current query).
 */
const props = defineProps<ListboxItemProps & { class?: HTMLAttributes["class"], keywords?: string }>()
const emits = defineEmits<ListboxItemEmits>()

const delegatedProps = reactiveOmit(props, "class", "keywords")

const forwarded = useForwardPropsEmits(delegatedProps, emits)

const id = useId()
const { filterState, allItems, allGroups } = useCommand()
const groupContext = useCommandGroup()

const isRender = computed(() => {
  if (!filterState.search) {
    return true
  }
  else {
    const filteredCurrentItem = filterState.filtered.items.get(id)
    // If the filtered items is undefined means not in the all times map yet
    // Do the first render to add into the map
    if (filteredCurrentItem === undefined) {
      return true
    }

    // Check with filter
    return filteredCurrentItem > 0
  }
})

const itemRef = ref()
const currentElement = useCurrentElement(itemRef)

function register() {
  if (!(currentElement.value instanceof HTMLElement))
    return

  // textValue to perform filter
  const label = currentElement.value.textContent ?? props?.value?.toString() ?? ""
  allItems.value.set(id, props.keywords ? `${label} ${props.keywords}` : label)

  const groupId = groupContext?.id
  if (groupId) {
    if (!allGroups.value.has(groupId)) {
      allGroups.value.set(groupId, new Set([id]))
    }
    else {
      allGroups.value.get(groupId)?.add(id)
    }
  }
}

onMounted(register)
// A changing `keywords` means the row now filters on different text; without
// re-registering, the list would keep scoring it against its first render.
watch(() => props.keywords, register, { flush: "post" })
onUnmounted(() => {
  allItems.value.delete(id)
})
</script>

<template>
  <ListboxItem
    v-if="isRender"
    v-bind="forwarded"
    :id="id"
    ref="itemRef"
    :class="cn('relative flex cursor-default gap-2 select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50 [&_svg]:size-4 [&_svg]:shrink-0', props.class)"
    @select="() => {
      filterState.search = ''
    }"
  >
    <slot />
  </ListboxItem>
</template>

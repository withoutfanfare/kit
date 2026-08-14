<script setup lang="ts">
/**
 * How a skill's link state is drawn, everywhere it appears.
 *
 * A coloured pill carries its meaning entirely in the colour, which fails for
 * anyone who cannot separate the hues and reads as generic web chrome besides.
 * A schedule marks state with a symbol: a closed circuit, an open one, a space
 * with nothing wired to it. The glyph is the signal, the colour agrees with
 * it, and the label says it in words.
 */
import { computed } from "vue";
import type { LinkState } from "@/types";
import { linkStateLabels, linkStateMeaning } from "@/utils/statusLabels";
import PanelIcon, { type IconName } from "@/components/base/PanelIcon.vue";

const props = withDefaults(
  defineProps<{
    state: LinkState;
    /** Symbol only, for dense rows where the column header carries the words. */
    compact?: boolean;
  }>(),
  { compact: false }
);

const icon = computed<IconName>(
  () =>
    (
      {
        linked: "linked",
        local_only: "local",
        declared_only: "declared",
        broken_link: "broken",
      } as const
    )[props.state]
);

const label = computed(() => linkStateLabels[props.state]);
const meaning = computed(() => linkStateMeaning(props.state));
</script>

<template>
  <span class="mark" :class="[`is-${state}`, { compact }]" :title="meaning">
    <PanelIcon :name="icon" :size="12" />
    <span v-if="!compact" class="mark-label">{{ label }}</span>
    <span v-else class="sr-only">{{ label }}</span>
  </span>
</template>

<style scoped>
.mark {
  display: inline-flex;
  align-items: center;
  gap: var(--space-3);
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  color: var(--k-text-3);
  border-radius: var(--radius-sm);
  padding: 2px var(--space-3);
  white-space: nowrap;
  background: var(--k-layer-2);
}

.mark.compact {
  padding: 0;
  gap: 0;
  background: transparent;
}

/* Colour agrees with the glyph; it never carries the meaning by itself. */
.is-linked :deep(.icon) {
  color: var(--k-ok);
}

.is-local_only :deep(.icon),
.is-declared_only :deep(.icon) {
  color: var(--k-warn);
}

.is-broken_link :deep(.icon) {
  color: var(--k-danger);
}
</style>

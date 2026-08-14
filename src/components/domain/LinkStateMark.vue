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
  gap: var(--space-2);
  font-family: var(--font-plate);
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  letter-spacing: 0.1em;
  text-transform: uppercase;
  color: var(--text-secondary);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-xs);
  padding: 1px var(--space-2);
  white-space: nowrap;
}

.mark.compact {
  border: 0;
  padding: 0;
  gap: 0;
}

/* Colour agrees with the glyph; it never carries the meaning by itself. */
.is-linked {
  color: var(--success);
  border-color: color-mix(in srgb, var(--success) 32%, transparent);
}

.is-local_only,
.is-declared_only {
  color: var(--warning);
  border-color: color-mix(in srgb, var(--warning) 32%, transparent);
}

.is-broken_link {
  color: var(--danger);
  border-color: color-mix(in srgb, var(--danger) 32%, transparent);
}
</style>

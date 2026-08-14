<script setup lang="ts">
/**
 * One sub-panel on the source list.
 *
 * A schedule row earns its width: the name, how many circuits it carries, and
 * whether anything is wrong. The old row spent its second line on a truncated
 * path, which is the least useful fact about a location you have already named
 * — the count is what you actually compare down the list, so that is what the
 * row shows, right-aligned and tabular so the column reads.
 */
import { computed } from "vue";
import type { SavedLocationSummary } from "@/types";
import { requestRemoveLocation } from "@/composables/useRemoveLocation";
import { SDropdownMenu, SIconButton } from "@stuntrocket/ui";
import PanelIcon from "@/components/base/PanelIcon.vue";

const props = defineProps<{
  location: SavedLocationSummary;
  selected: boolean;
}>();

const isGlobal = computed(() => props.location.kind === "global");
const isMissing = computed(() => !isGlobal.value && !props.location.pathExists);

function onRowAction(action: string) {
  if (action === "remove") requestRemoveLocation(props.location);
}
</script>

<template>
  <div
    class="loc-row"
    :class="{ selected, global: isGlobal, missing: isMissing }"
  >
    <!-- Global is the main panel: it feeds everything, so it is marked, not
         labelled twice. -->
      <span class="loc-name">{{ location.label }}</span>

    <span v-if="isGlobal" class="badge loc-plate">Every session</span>

    <span v-if="isMissing" class="badge badge-warn loc-state">
      <PanelIcon name="broken" :size="11" />
      Gone
    </span>
    <span
      v-else-if="location.issueCount > 0"
      class="loc-issues"
      :title="`${location.issueCount} issue${location.issueCount === 1 ? '' : 's'}`"
    >
      <PanelIcon name="caution" :size="12" />
      <span class="tabular">{{ location.issueCount }}</span>
      <span class="sr-only">
        {{ location.issueCount === 1 ? "issue" : "issues" }}
      </span>
    </span>

    <span v-if="!isMissing" class="loc-count num">
      {{ location.installedSkillCount }}
    </span>

    <SDropdownMenu
      v-if="!isGlobal"
      class="row-menu"
      :items="[{ label: 'Remove…', value: 'remove', danger: true }]"
      align="right"
      @click.stop
      @select="onRowAction"
    >
      <template #trigger="{ toggle, open }">
        <SIconButton
          size="sm"
          :active="open"
          :tooltip="`Actions for ${location.label}`"
          aria-haspopup="menu"
          :aria-expanded="open"
          @click.stop="toggle"
        >
          <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor" aria-hidden="true">
            <circle cx="8" cy="3" r="1.2" />
            <circle cx="8" cy="8" r="1.2" />
            <circle cx="8" cy="13" r="1.2" />
          </svg>
        </SIconButton>
      </template>
    </SDropdownMenu>
  </div>
</template>

<style scoped>
.loc-row {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-4) var(--space-5);
  border-radius: var(--radius-sm);
  cursor: default;
  user-select: none;
  border-bottom: 1px solid var(--border-subtle);
  transition: background var(--duration-fast) var(--ease-default);
}

.loc-row:hover {
  background: var(--surface-hover);
}

.loc-row.selected {
  background: var(--surface-selected);
}

.loc-row.selected:hover {
  background: var(--surface-selected-strong);
}

.loc-name {
  font-size: var(--text-md);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.loc-plate {
  font-size: 9px;
  padding: 1px var(--space-2) 0;
  flex-shrink: 0;
}

.loc-state {
  flex-shrink: 0;
}

.loc-issues {
  display: inline-flex;
  align-items: center;
  gap: 3px;
  font-size: var(--text-xs);
  color: var(--warning);
  flex-shrink: 0;
}

.loc-count {
  margin-left: auto;
  font-size: var(--text-md);
  color: var(--text-secondary);
  flex-shrink: 0;
}

.loc-row.missing .loc-name {
  color: var(--text-tertiary);
  text-decoration: line-through;
  text-decoration-thickness: 1px;
  text-decoration-color: var(--border-strong);
}

.row-menu {
  opacity: 0;
  flex-shrink: 0;
  margin-left: var(--space-1);
}

.loc-row:hover .row-menu,
.loc-row:focus-within .row-menu {
  opacity: 1;
}
</style>

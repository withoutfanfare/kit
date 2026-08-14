<script setup lang="ts">
/**
 * One set on the library's source list.
 *
 * Scope is the fact that decides where a set can be used, so it gets a plate
 * rather than a coloured pill; the skill count sits right-aligned and tabular
 * so the column reads down the list.
 */
import type { SetSummary } from "@/types";

defineProps<{
  set: SetSummary;
  selected: boolean;
}>();
</script>

<template>
  <div class="set-row" :class="{ selected }">
    <div class="row-text">
      <span class="row-name">{{ set.name }}</span>
      <span v-if="set.description" class="row-desc">{{ set.description }}</span>
    </div>
    <span class="plate row-scope">
      {{ set.scope === "global" ? "Everywhere" : "This project" }}
    </span>
    <span class="row-count rating tabular">{{ set.skillCount }}</span>
  </div>
</template>

<style scoped>
.set-row {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-3);
  border-bottom: 1px solid var(--border-subtle);
  cursor: default;
  user-select: none;
  transition: background var(--duration-fast) var(--ease-default);
}

.set-row:hover {
  background: var(--surface-hover);
}

.set-row.selected {
  background: var(--surface-selected);
}

.set-row.selected:hover {
  background: var(--surface-selected-strong);
}

.row-text {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.row-name {
  font-size: var(--text-md);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.row-desc {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.row-scope {
  font-size: 9px;
  padding: 1px var(--space-2) 0;
  flex-shrink: 0;
}

.row-count {
  font-size: var(--text-md);
  color: var(--text-secondary);
  flex-shrink: 0;
  min-width: 2ch;
  text-align: right;
}
</style>

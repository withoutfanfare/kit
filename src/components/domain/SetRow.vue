<script setup lang="ts">
import type { SetSummary } from "@/types";
import { SBadge } from "@stuntrocket/ui";

defineProps<{
  set: SetSummary;
  selected: boolean;
}>();
</script>

<template>
  <div class="set-row" :class="{ selected }">
    <div class="row-content">
      <span class="row-name">{{ set.name }}</span>
      <span v-if="set.description" class="row-description">{{ set.description }}</span>
    </div>
    <div class="row-meta">
      <SBadge variant="count">{{ set.skillCount }}</SBadge>
      <SBadge :variant="set.scope === 'global' ? 'accent' : 'default'">
        {{ set.scope === 'global' ? 'Global' : 'Project' }}
      </SBadge>
    </div>
  </div>
</template>

<style scoped>
.set-row {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-sm);
  cursor: default;
  transition: background var(--duration-fast) var(--ease-default);
  user-select: none;
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

.row-content {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.row-name {
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.row-description {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.row-meta {
  display: flex;
  align-items: center;
  gap: var(--space-1);
  flex-shrink: 0;
}
</style>

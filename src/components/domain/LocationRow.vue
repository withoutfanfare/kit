<script setup lang="ts">
import type { SavedLocationSummary } from "@/types";
import { SBadge } from "@stuntrocket/ui";

defineProps<{
  location: SavedLocationSummary;
  selected: boolean;
}>();

function truncatePath(path: string, maxLen = 32): string {
  if (path.length <= maxLen) return path;
  const parts = path.split("/");
  if (parts.length <= 2) return "..." + path.slice(-maxLen);
  return ".../" + parts.slice(-2).join("/");
}
</script>

<template>
  <div class="location-row" :class="{ selected }">
    <div class="row-content">
      <span class="row-label">{{ location.label }}</span>
      <span class="row-path">{{ truncatePath(location.path) }}</span>
    </div>
    <SBadge v-if="location.issueCount > 0" variant="warning">
      {{ location.issueCount }}
    </SBadge>
  </div>
</template>

<style scoped>
.location-row {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-sm);
  cursor: default;
  transition: background var(--duration-fast) var(--ease-default);
  user-select: none;
}

.location-row:hover {
  background: var(--surface-hover);
}

.location-row.selected {
  background: var(--surface-selected);
}

.location-row.selected:hover {
  background: var(--surface-selected-strong);
}

.row-content {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.row-label {
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.row-path {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>

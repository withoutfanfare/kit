<script setup lang="ts">
import { linkStateTerms } from "@/utils/statusLabels";

defineProps<{
  linkedCount: number;
  localOnlyCount: number;
  brokenCount: number;
}>();

/** One row per state, in the order that matters: normal, precious, broken. */
const rows = [
  { key: "linked", term: linkStateTerms.linked, dot: "linked" },
  { key: "local_only", term: linkStateTerms.local_only, dot: "local-only" },
  { key: "broken_link", term: linkStateTerms.broken_link, dot: "broken" },
] as const;
</script>

<template>
  <div class="overview-group">
    <div
      v-for="row in rows"
      :key="row.key"
      class="overview-row"
      :title="row.term.meaning"
    >
      <span class="status-dot" :class="row.dot" />
      <span class="overview-text">
        <span class="overview-label">{{ row.term.label }}</span>
        <span class="overview-meaning">{{ row.term.meaning }}</span>
      </span>
      <span class="overview-value">
        {{
          row.key === "linked"
            ? linkedCount
            : row.key === "local_only"
              ? localOnlyCount
              : brokenCount
        }}
      </span>
    </div>
  </div>
</template>

<style scoped>
.overview-group {
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  background: var(--surface-panel);
  overflow: hidden;
}

.overview-row {
  display: flex;
  align-items: flex-start;
  gap: var(--space-2);
  padding: var(--space-3);
}

.overview-row + .overview-row {
  border-top: 1px solid var(--border-subtle);
}

.status-dot {
  width: 7px;
  height: 7px;
  border-radius: var(--radius-full);
  flex-shrink: 0;
  /* Sits on the label's first line rather than the block's top edge. */
  margin-top: 5px;
}

.status-dot.linked {
  background: var(--color-success);
}

.status-dot.local-only {
  background: var(--accent);
}

.status-dot.broken {
  background: var(--color-danger);
}

.overview-text {
  display: flex;
  flex-direction: column;
  gap: 2px;
  flex: 1;
  min-width: 0;
}

.overview-label {
  font-size: var(--text-sm);
  color: var(--text-primary);
}

.overview-meaning {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  line-height: 1.45;
  text-wrap: pretty;
}

.overview-value {
  font-size: var(--text-md);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  font-variant-numeric: tabular-nums;
}
</style>

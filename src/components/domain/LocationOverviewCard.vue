<script setup lang="ts">
defineProps<{
  linkedCount: number;
  localOnlyCount: number;
  brokenCount: number;
}>();
</script>

<template>
  <div class="overview-group">
    <div class="overview-row">
      <span class="status-dot linked" />
      <span class="overview-label">Linked</span>
      <span class="overview-value">{{ linkedCount }}</span>
    </div>
    <div class="overview-row">
      <span class="status-dot local-only" />
      <span class="overview-label">Local only</span>
      <span class="overview-value">{{ localOnlyCount }}</span>
    </div>
    <div v-if="brokenCount > 0" class="overview-row">
      <span class="status-dot broken" />
      <span class="overview-label">Broken</span>
      <span class="overview-value">{{ brokenCount }}</span>
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
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  font-size: var(--text-sm);
}

.overview-row + .overview-row {
  border-top: 1px solid var(--border-subtle);
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.status-dot.linked {
  background: var(--success);
}

.status-dot.local-only {
  background: var(--accent);
}

.status-dot.broken {
  background: var(--danger);
}

.overview-label {
  flex: 1;
  color: var(--text-secondary);
}

.overview-value {
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  font-variant-numeric: tabular-nums;
}
</style>

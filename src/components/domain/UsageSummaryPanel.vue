<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  usage: {
    lastUsedAt: string | null;
    useCount30d: number;
  };
}>();

const lastUsedDisplay = computed(() => {
  if (!props.usage.lastUsedAt) return "Never";
  const date = new Date(props.usage.lastUsedAt);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));

  if (diffDays === 0) return "Today";
  if (diffDays === 1) return "Yesterday";
  if (diffDays < 7) return `${diffDays} days ago`;
  if (diffDays < 30) return `${Math.floor(diffDays / 7)} weeks ago`;
  return date.toLocaleDateString(undefined, {
    day: "numeric",
    month: "short",
    year: "numeric",
  });
});
</script>

<template>
  <div class="usage-summary">
    <div class="usage-row">
      <span class="usage-label">Last used</span>
      <span class="usage-value">{{ lastUsedDisplay }}</span>
    </div>
    <div class="usage-row">
      <span class="usage-label">Uses (30d)</span>
      <span class="usage-value">{{ usage.useCount30d }}</span>
    </div>
  </div>
</template>

<style scoped>
.usage-summary {
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  background: var(--surface-panel);
  overflow: hidden;
}

.usage-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-2) var(--space-3);
  font-size: var(--text-sm);
}

.usage-row + .usage-row {
  border-top: 1px solid var(--border-subtle);
}

.usage-label {
  color: var(--text-secondary);
}

.usage-value {
  font-weight: var(--weight-medium);
  color: var(--text-primary);
  font-variant-numeric: tabular-nums;
}
</style>

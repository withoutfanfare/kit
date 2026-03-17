<script setup lang="ts">
defineProps<{
  variant?: 'default' | 'accent' | 'success' | 'warning' | 'danger'
  compact?: boolean
}>()
</script>

<template>
  <span class="badge" :class="[variant ?? 'default', { compact }]">
    <slot />
  </span>
</template>

<style scoped>
.badge {
  display: inline-flex;
  align-items: center;
  height: 20px;
  padding: 0 var(--space-2);
  font-family: var(--font-sans);
  font-size: var(--text-xs);
  font-weight: var(--weight-medium);
  border-radius: var(--radius-sm);
  white-space: nowrap;
  line-height: 1;
}

.badge.compact {
  height: 16px;
  padding: 0 var(--space-1);
  font-size: 10px;
}

.default {
  color: var(--text-secondary);
  background: var(--surface-hover);
}

.accent {
  color: var(--accent);
  background: var(--accent-subtle);
}

.success {
  color: var(--success);
  background: var(--success-subtle);
}

.warning {
  color: var(--warning);
  background: var(--warning-subtle);
}

.danger {
  color: var(--danger);
  background: var(--danger-subtle);
}
</style>

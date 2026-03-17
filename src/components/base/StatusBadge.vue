<script setup lang="ts">
import StatusDot from './StatusDot.vue'

defineProps<{
  status: 'linked' | 'broken' | 'local' | 'declared' | 'archived'
  label?: string
}>()
</script>

<template>
  <span class="status-badge">
    <StatusDot :status="status" />
    <span class="label">{{ label ?? status }}</span>
  </span>
</template>

<style scoped>
.status-badge {
  display: inline-flex;
  align-items: center;
  gap: var(--space-1);
  font-family: var(--font-sans);
  font-size: var(--text-xs);
  font-weight: var(--weight-medium);
  color: var(--text-secondary);
  white-space: nowrap;
}

.label {
  text-transform: capitalize;
}
</style>

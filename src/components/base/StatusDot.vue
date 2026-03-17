<script setup lang="ts">
defineProps<{
  status: 'linked' | 'broken' | 'local' | 'declared' | 'archived'
}>()
</script>

<template>
  <span class="status-dot" :class="status" />
</template>

<style scoped>
.status-dot {
  display: inline-block;
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.linked {
  background: var(--success);
}

.broken {
  background: var(--danger);
}

.local {
  background: var(--accent);
}

.declared {
  background: var(--warning);
}

.archived {
  background: var(--text-tertiary);
}
</style>

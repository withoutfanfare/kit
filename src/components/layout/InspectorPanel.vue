<script setup lang="ts">
defineProps<{
  title?: string;
}>();
</script>

<template>
  <div class="inspector-panel">
    <h3 v-if="title" class="inspector-title">{{ title }}</h3>
    <slot />
  </div>
</template>

<style scoped>
.inspector-panel {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.inspector-title {
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}
</style>

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
  gap: var(--space-6);
}

.inspector-title {
  font-size: var(--text-sm);
  font-weight: var(--weight-semibold);
  color: var(--text-tertiary);
}
</style>

<script setup lang="ts">
import Badge from './Badge.vue'

defineProps<{
  title: string
  count?: number
  actionLabel?: string
}>()

defineEmits<{
  action: []
}>()
</script>

<template>
  <div class="section-header">
    <div class="left">
      <span class="title">{{ title }}</span>
      <Badge v-if="count != null" compact>{{ count }}</Badge>
    </div>
    <button
      v-if="actionLabel"
      class="action"
      @click="$emit('action')"
    >
      {{ actionLabel }}
    </button>
  </div>
</template>

<style scoped>
.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-2) 0;
}

.left {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.title {
  font-family: var(--font-sans);
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.04em;
}

.action {
  font-family: var(--font-sans);
  font-size: var(--text-xs);
  font-weight: var(--weight-medium);
  color: var(--accent);
  background: transparent;
  border: none;
  cursor: pointer;
  padding: 0;
  transition: color var(--duration-fast) var(--ease-default);
}

.action:hover {
  color: var(--accent-hover);
}

.action:focus-visible {
  outline: none;
  box-shadow: 0 0 0 2px var(--accent);
  border-radius: 2px;
}
</style>

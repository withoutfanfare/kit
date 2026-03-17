<script setup lang="ts">
defineProps<{
  title: string;
  description?: string;
  actionLabel?: string;
}>();

defineEmits<{
  action: [];
}>();
</script>

<template>
  <div class="empty-state">
    <div class="empty-content">
      <slot name="icon" />
      <h3 class="empty-title">{{ title }}</h3>
      <p v-if="description" class="empty-description">{{ description }}</p>
      <button
        v-if="actionLabel"
        class="empty-action"
        @click="$emit('action')"
      >
        {{ actionLabel }}
      </button>
    </div>
  </div>
</template>

<style scoped>
.empty-state {
  display: flex;
  align-items: center;
  justify-content: center;
  flex: 1;
  padding: var(--space-10);
}

.empty-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-3);
  max-width: 280px;
  text-align: center;
}

.empty-title {
  font-size: var(--text-lg);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
}

.empty-description {
  font-size: var(--text-sm);
  color: var(--text-secondary);
  line-height: 1.5;
}

.empty-action {
  margin-top: var(--space-2);
  padding: var(--space-2) var(--space-4);
  background: var(--accent);
  color: var(--text-inverse);
  border: none;
  border-radius: var(--radius-sm);
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  cursor: pointer;
  transition: background var(--duration-fast) var(--ease-default);
}

.empty-action:hover {
  background: var(--accent-hover);
}
</style>

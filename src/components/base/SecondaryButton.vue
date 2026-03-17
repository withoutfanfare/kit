<script setup lang="ts">
defineProps<{
  label?: string
  disabled?: boolean
  loading?: boolean
}>()
</script>

<template>
  <button
    class="secondary-button"
    :disabled="disabled || loading"
    :class="{ loading }"
  >
    <span v-if="loading" class="spinner" />
    <slot>{{ label }}</slot>
  </button>
</template>

<style scoped>
.secondary-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-1);
  height: 28px;
  padding: 0 var(--space-3);
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
  background: transparent;
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all var(--duration-fast) var(--ease-default);
  white-space: nowrap;
}

.secondary-button:hover:not(:disabled) {
  background: var(--surface-hover);
  border-color: var(--border-strong);
}

.secondary-button:active:not(:disabled) {
  opacity: 0.85;
}

.secondary-button:focus-visible {
  outline: none;
  box-shadow: 0 0 0 2px var(--surface-panel), 0 0 0 4px var(--accent);
}

.secondary-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.secondary-button.loading {
  cursor: wait;
}

.spinner {
  width: 12px;
  height: 12px;
  border: 1.5px solid var(--border-default);
  border-top-color: var(--text-primary);
  border-radius: 50%;
  animation: spin 600ms linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style>

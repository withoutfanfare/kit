<script setup lang="ts">
defineProps<{
  label?: string
  disabled?: boolean
  loading?: boolean
}>()
</script>

<template>
  <button
    class="primary-button"
    :disabled="disabled || loading"
    :class="{ loading }"
  >
    <span v-if="loading" class="spinner" />
    <slot>{{ label }}</slot>
  </button>
</template>

<style scoped>
.primary-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-1);
  height: 28px;
  padding: 0 var(--space-3);
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  color: var(--text-inverse);
  background: var(--accent);
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: background var(--duration-fast) var(--ease-default),
    opacity var(--duration-fast) var(--ease-default);
  white-space: nowrap;
}

.primary-button:hover:not(:disabled) {
  background: var(--accent-hover);
}

.primary-button:active:not(:disabled) {
  opacity: 0.85;
}

.primary-button:focus-visible {
  outline: none;
  box-shadow: 0 0 0 2px var(--surface-panel), 0 0 0 4px var(--accent);
}

.primary-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.primary-button.loading {
  cursor: wait;
}

.spinner {
  width: 12px;
  height: 12px;
  border: 1.5px solid rgba(255, 255, 255, 0.3);
  border-top-color: currentColor;
  border-radius: 50%;
  animation: spin 600ms linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style>

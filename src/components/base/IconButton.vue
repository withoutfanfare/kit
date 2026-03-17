<script setup lang="ts">
defineProps<{
  title?: string
  disabled?: boolean
}>()
</script>

<template>
  <button
    class="icon-button"
    :title="title"
    :disabled="disabled"
  >
    <slot />
  </button>
</template>

<style scoped>
.icon-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 26px;
  height: 26px;
  padding: 0;
  color: var(--text-secondary);
  background: transparent;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all var(--duration-fast) var(--ease-default);
  flex-shrink: 0;
}

.icon-button:hover:not(:disabled) {
  color: var(--text-primary);
  background: var(--surface-hover);
}

.icon-button:active:not(:disabled) {
  opacity: 0.75;
}

.icon-button:focus-visible {
  outline: none;
  box-shadow: 0 0 0 2px var(--accent);
}

.icon-button:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}
</style>

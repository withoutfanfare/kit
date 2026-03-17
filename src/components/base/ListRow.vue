<script setup lang="ts">
defineProps<{
  selected?: boolean
  disabled?: boolean
}>()

defineEmits<{
  click: [event: MouseEvent]
}>()
</script>

<template>
  <div
    class="list-row"
    :class="{ selected, disabled }"
    role="button"
    :tabindex="disabled ? -1 : 0"
    @click="!disabled && $emit('click', $event)"
    @keydown.enter="!disabled && $emit('click', $event as any)"
  >
    <div class="left">
      <slot name="left" />
    </div>
    <div class="right">
      <slot name="right" />
    </div>
  </div>
</template>

<style scoped>
.list-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  min-height: var(--list-row-height);
  padding: 0 var(--space-3);
  gap: var(--space-2);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: background var(--duration-fast) var(--ease-default);
  user-select: none;
}

.list-row:hover:not(.disabled) {
  background: var(--surface-hover);
}

.list-row.selected {
  background: var(--surface-selected);
}

.list-row.selected:hover {
  background: var(--surface-selected-strong);
}

.list-row:focus-visible {
  outline: none;
  box-shadow: inset 0 0 0 2px var(--accent);
}

.list-row.disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.left {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  min-width: 0;
  flex: 1;
}

.right {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  flex-shrink: 0;
}
</style>

<script setup lang="ts">
export interface SegmentOption {
  label: string
  value: string
}

defineProps<{
  options: SegmentOption[]
  modelValue: string
}>()

defineEmits<{
  'update:modelValue': [value: string]
}>()
</script>

<template>
  <div class="segmented-control">
    <button
      v-for="option in options"
      :key="option.value"
      class="segment"
      :class="{ active: modelValue === option.value }"
      @click="$emit('update:modelValue', option.value)"
    >
      {{ option.label }}
    </button>
  </div>
</template>

<style scoped>
.segmented-control {
  display: inline-flex;
  background: var(--surface-hover);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-sm);
  padding: 2px;
  gap: 1px;
}

.segment {
  padding: 0 var(--space-3);
  height: 24px;
  font-family: var(--font-sans);
  font-size: var(--text-xs);
  font-weight: var(--weight-medium);
  color: var(--text-secondary);
  background: transparent;
  border: none;
  border-radius: calc(var(--radius-sm) - 2px);
  cursor: pointer;
  transition: all var(--duration-fast) var(--ease-default);
  white-space: nowrap;
}

.segment:hover:not(.active) {
  color: var(--text-primary);
}

.segment.active {
  background: var(--surface-panel);
  color: var(--text-primary);
  box-shadow: var(--shadow-sm);
}
</style>

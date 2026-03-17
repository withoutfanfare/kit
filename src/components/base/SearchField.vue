<script setup lang="ts">
defineProps<{
  modelValue?: string
  placeholder?: string
  compact?: boolean
}>()

defineEmits<{
  'update:modelValue': [value: string]
}>()
</script>

<template>
  <div class="search-field" :class="{ compact }">
    <svg
      class="search-icon"
      width="14"
      height="14"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <circle cx="11" cy="11" r="8" />
      <line x1="21" y1="21" x2="16.65" y2="16.65" />
    </svg>
    <input
      type="text"
      :value="modelValue"
      :placeholder="placeholder ?? 'Search…'"
      @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
    />
  </div>
</template>

<style scoped>
.search-field {
  position: relative;
  display: flex;
  align-items: center;
}

.search-icon {
  position: absolute;
  left: var(--space-2);
  color: var(--text-tertiary);
  pointer-events: none;
  flex-shrink: 0;
}

input {
  width: 100%;
  height: 30px;
  padding: 0 var(--space-2) 0 var(--space-8);
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--text-primary);
  background: var(--surface-hover);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-sm);
  outline: none;
  transition: border-color var(--duration-fast) var(--ease-default),
    box-shadow var(--duration-fast) var(--ease-default);
}

input::placeholder {
  color: var(--text-tertiary);
}

input:focus {
  border-color: var(--accent);
  box-shadow: 0 0 0 2px var(--accent-subtle);
}

.compact input {
  height: 26px;
  font-size: var(--text-xs);
}

.compact .search-icon {
  width: 12px;
  height: 12px;
}
</style>

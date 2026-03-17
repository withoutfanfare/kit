<script setup lang="ts">
defineProps<{
  variant?: 'info' | 'warning' | 'danger'
  dismissible?: boolean
}>()

defineEmits<{
  dismiss: []
}>()
</script>

<template>
  <div class="notice-banner" :class="variant ?? 'info'" role="alert">
    <div class="content">
      <slot />
    </div>
    <button
      v-if="dismissible"
      class="dismiss"
      aria-label="Dismiss"
      @click="$emit('dismiss')"
    >
      <svg
        width="12"
        height="12"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <line x1="18" y1="6" x2="6" y2="18" />
        <line x1="6" y1="6" x2="18" y2="18" />
      </svg>
    </button>
  </div>
</template>

<style scoped>
.notice-banner {
  display: flex;
  align-items: flex-start;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  line-height: 1.5;
  border-radius: var(--radius-sm);
}

.info {
  color: var(--accent);
  background: var(--accent-subtle);
}

.warning {
  color: var(--warning);
  background: var(--warning-subtle);
}

.danger {
  color: var(--danger);
  background: var(--danger-subtle);
}

.content {
  flex: 1;
  min-width: 0;
}

.dismiss {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  padding: 0;
  color: inherit;
  opacity: 0.6;
  background: transparent;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
  flex-shrink: 0;
  transition: opacity var(--duration-fast) var(--ease-default);
}

.dismiss:hover {
  opacity: 1;
}
</style>

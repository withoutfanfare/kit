<script setup lang="ts">
defineProps<{
  open: boolean
}>()

defineEmits<{
  close: []
}>()
</script>

<template>
  <Teleport to="body">
    <Transition name="sheet">
      <div v-if="open" class="sheet-backdrop" @click.self="$emit('close')">
        <div class="sheet-panel" role="dialog" aria-modal="true">
          <slot />
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.sheet-backdrop {
  position: fixed;
  inset: 0;
  z-index: 200;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.25);
}

.sheet-panel {
  width: 100%;
  max-width: 800px;
  max-height: 85vh;
  overflow-y: auto;
  background: var(--surface-panel);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-sheet);
}

/* Transition */
.sheet-enter-active {
  transition: opacity var(--duration-normal) var(--ease-out);
}

.sheet-leave-active {
  transition: opacity var(--duration-fast) var(--ease-default);
}

.sheet-enter-from,
.sheet-leave-to {
  opacity: 0;
}

.sheet-enter-active .sheet-panel {
  animation: slide-up var(--duration-slow) var(--ease-out);
}

.sheet-leave-active .sheet-panel {
  animation: slide-down var(--duration-normal) var(--ease-default);
}

@keyframes slide-up {
  from {
    transform: translateY(24px);
  }
  to {
    transform: translateY(0);
  }
}

@keyframes slide-down {
  from {
    transform: translateY(0);
  }
  to {
    transform: translateY(24px);
  }
}
</style>

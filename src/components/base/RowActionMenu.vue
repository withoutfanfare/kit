<script setup lang="ts">
import { ref, onBeforeUnmount } from 'vue'

export interface RowAction {
  label: string
  handler: () => void
  danger?: boolean
}

defineProps<{
  actions: RowAction[]
}>()

const open = ref(false)

function toggle() {
  open.value = !open.value
  if (open.value) {
    document.addEventListener('click', closeOnOutside, { once: true })
  }
}

function closeOnOutside() {
  open.value = false
}

function run(action: RowAction) {
  open.value = false
  action.handler()
}

onBeforeUnmount(() => {
  document.removeEventListener('click', closeOnOutside)
})
</script>

<template>
  <div class="row-action-menu" @click.stop>
    <button class="trigger" :class="{ active: open }" @click="toggle">
      <svg
        width="14"
        height="14"
        viewBox="0 0 24 24"
        fill="currentColor"
      >
        <circle cx="12" cy="5" r="2" />
        <circle cx="12" cy="12" r="2" />
        <circle cx="12" cy="19" r="2" />
      </svg>
    </button>

    <Transition name="menu">
      <div v-if="open" class="dropdown">
        <button
          v-for="action in actions"
          :key="action.label"
          class="menu-item"
          :class="{ danger: action.danger }"
          @click="run(action)"
        >
          {{ action.label }}
        </button>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.row-action-menu {
  position: relative;
}

.trigger {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  padding: 0;
  color: var(--text-tertiary);
  background: transparent;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all var(--duration-fast) var(--ease-default);
}

.trigger:hover,
.trigger.active {
  color: var(--text-primary);
  background: var(--surface-hover);
}

.dropdown {
  position: absolute;
  top: 100%;
  right: 0;
  z-index: 100;
  min-width: 140px;
  margin-top: 2px;
  padding: var(--space-1);
  background: var(--surface-panel);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-md);
}

.menu-item {
  display: block;
  width: 100%;
  padding: var(--space-1) var(--space-2);
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--text-primary);
  background: transparent;
  border: none;
  border-radius: calc(var(--radius-md) - 4px);
  cursor: pointer;
  text-align: left;
  transition: background var(--duration-fast) var(--ease-default);
}

.menu-item:hover {
  background: var(--surface-hover);
}

.menu-item.danger {
  color: var(--danger);
}

.menu-enter-active {
  transition: opacity var(--duration-fast) var(--ease-out),
    transform var(--duration-fast) var(--ease-out);
}

.menu-leave-active {
  transition: opacity var(--duration-fast) var(--ease-default);
}

.menu-enter-from {
  opacity: 0;
  transform: translateY(-4px);
}

.menu-leave-to {
  opacity: 0;
}
</style>

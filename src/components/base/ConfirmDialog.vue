<script setup lang="ts">
import PrimaryButton from './PrimaryButton.vue'
import SecondaryButton from './SecondaryButton.vue'

defineProps<{
  open: boolean
  title: string
  message?: string
  confirmLabel?: string
  danger?: boolean
}>()

defineEmits<{
  confirm: []
  cancel: []
}>()
</script>

<template>
  <Teleport to="body">
    <Transition name="dialog">
      <div v-if="open" class="dialog-backdrop" @click.self="$emit('cancel')">
        <div class="dialog" role="alertdialog" aria-modal="true">
          <h3 class="dialog-title">{{ title }}</h3>
          <p v-if="message" class="dialog-message">{{ message }}</p>
          <div class="dialog-actions">
            <SecondaryButton label="Cancel" @click="$emit('cancel')" />
            <PrimaryButton
              :label="confirmLabel ?? 'Confirm'"
              :class="{ danger }"
              @click="$emit('confirm')"
            />
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.dialog-backdrop {
  position: fixed;
  inset: 0;
  z-index: 300;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.25);
}

.dialog {
  width: 100%;
  max-width: 360px;
  background: var(--surface-panel);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-lg);
  padding: var(--space-5);
}

.dialog-title {
  font-family: var(--font-sans);
  font-size: var(--text-md);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  margin: 0;
}

.dialog-message {
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--text-secondary);
  margin: var(--space-2) 0 0;
  line-height: 1.5;
}

.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-2);
  margin-top: var(--space-5);
}

.danger :deep(.primary-button) {
  background: var(--danger);
}

.danger :deep(.primary-button:hover) {
  background: var(--danger);
  opacity: 0.9;
}

/* Danger applied directly to PrimaryButton */
:deep(.danger) {
  background: var(--danger);
}

:deep(.danger:hover:not(:disabled)) {
  background: var(--danger);
  opacity: 0.9;
}

/* Transition */
.dialog-enter-active {
  transition: opacity var(--duration-fast) var(--ease-out);
}

.dialog-leave-active {
  transition: opacity var(--duration-fast) var(--ease-default);
}

.dialog-enter-from,
.dialog-leave-to {
  opacity: 0;
}

.dialog-enter-active .dialog {
  animation: scale-in var(--duration-normal) var(--ease-out);
}

@keyframes scale-in {
  from {
    transform: scale(0.96);
  }
  to {
    transform: scale(1);
  }
}
</style>

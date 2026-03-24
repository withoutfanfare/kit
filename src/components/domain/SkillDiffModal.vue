<script setup lang="ts">
import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { SkillContentDiff } from "@/types";
import { SButton } from "@stuntrocket/ui";

const props = defineProps<{
  locationId: string;
  skillId: string;
  skillName: string;
  open: boolean;
}>();

const emit = defineEmits<{
  close: [];
}>();

const diff = ref<SkillContentDiff | null>(null);
const isLoading = ref(false);
const error = ref<string | null>(null);

watch(
  () => props.open,
  async (open) => {
    if (!open) return;
    isLoading.value = true;
    error.value = null;
    try {
      diff.value = await invoke<SkillContentDiff>("get_skill_content_diff", {
        locationId: props.locationId,
        skillId: props.skillId,
      });
    } catch (e) {
      error.value = String(e);
    } finally {
      isLoading.value = false;
    }
  }
);

function handleKeydown(event: KeyboardEvent) {
  if (event.key === "Escape") {
    emit("close");
  }
}

function handleBackdropClick(event: MouseEvent) {
  if (event.target === event.currentTarget) {
    emit("close");
  }
}
</script>

<template>
  <Teleport to="body">
    <Transition name="modal">
      <div
        v-if="open"
        class="diff-backdrop"
        @click="handleBackdropClick"
        @keydown="handleKeydown"
      >
        <div class="diff-modal" role="dialog" aria-modal="true" tabindex="-1">
          <div class="diff-header">
            <h3 class="diff-title">Changes: {{ skillName }}</h3>
            <button class="close-btn" @click="$emit('close')">
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <line x1="18" y1="6" x2="6" y2="18" />
                <line x1="6" y1="6" x2="18" y2="18" />
              </svg>
            </button>
          </div>

          <div v-if="isLoading" class="diff-loading">Loading...</div>
          <div v-else-if="error" class="diff-error">{{ error }}</div>
          <template v-else-if="diff">
            <div v-if="!diff.hasChanged && diff.assignedContent" class="diff-unchanged">
              Content has not changed since assignment.
            </div>
            <div v-else-if="!diff.assignedContent" class="diff-unchanged">
              No snapshot was recorded at assignment time.
            </div>
            <div v-else class="diff-columns">
              <div class="diff-column">
                <span class="column-label">At assignment</span>
                <pre class="diff-content">{{ diff.assignedContent }}</pre>
              </div>
              <div class="diff-column">
                <span class="column-label">Current</span>
                <pre class="diff-content">{{ diff.currentContent }}</pre>
              </div>
            </div>
          </template>

          <div class="diff-footer">
            <SButton variant="secondary" size="sm" @click="$emit('close')">Close</SButton>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.diff-backdrop {
  position: fixed;
  inset: 0;
  z-index: 300;
  background: rgba(0, 0, 0, 0.3);
  display: flex;
  align-items: center;
  justify-content: center;
}

.diff-modal {
  width: min(90vw, 900px);
  max-height: 80vh;
  background: var(--surface-panel);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-sheet);
  display: flex;
  flex-direction: column;
  outline: none;
}

.diff-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-3) var(--space-4);
  border-bottom: 1px solid var(--border-subtle);
  flex-shrink: 0;
}

.diff-title {
  font-size: var(--text-md);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  margin: 0;
}

.close-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border: none;
  background: transparent;
  color: var(--text-tertiary);
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.close-btn:hover {
  background: var(--surface-hover);
  color: var(--text-primary);
}

.diff-loading,
.diff-error,
.diff-unchanged {
  padding: var(--space-6);
  text-align: center;
  font-size: var(--text-sm);
  color: var(--text-tertiary);
}

.diff-error {
  color: var(--danger);
}

.diff-columns {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1px;
  background: var(--border-subtle);
  flex: 1;
  overflow: hidden;
  min-height: 0;
}

.diff-column {
  display: flex;
  flex-direction: column;
  background: var(--surface-base);
  overflow: hidden;
}

.column-label {
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.04em;
  padding: var(--space-2) var(--space-3);
  border-bottom: 1px solid var(--border-subtle);
  flex-shrink: 0;
}

.diff-content {
  font-size: var(--text-xs);
  font-family: ui-monospace, "SF Mono", SFMono-Regular, monospace;
  color: var(--text-secondary);
  padding: var(--space-3);
  margin: 0;
  overflow: auto;
  white-space: pre-wrap;
  word-break: break-word;
  flex: 1;
  line-height: 1.5;
}

.diff-footer {
  display: flex;
  justify-content: flex-end;
  padding: var(--space-3) var(--space-4);
  border-top: 1px solid var(--border-subtle);
  flex-shrink: 0;
}

.modal-enter-active {
  transition: opacity var(--duration-normal) var(--ease-out);
}

.modal-leave-active {
  transition: opacity var(--duration-fast) var(--ease-default);
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}
</style>

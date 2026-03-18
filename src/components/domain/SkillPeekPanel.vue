<script setup lang="ts">
import { watch, ref, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useSkillPeekStore } from "@/stores/skillPeekStore";
import { usePreferencesStore } from "@/stores/preferencesStore";
import Badge from "@/components/base/Badge.vue";
import SecondaryButton from "@/components/base/SecondaryButton.vue";

const peekStore = useSkillPeekStore();
const preferencesStore = usePreferencesStore();
const panelRef = ref<HTMLElement | null>(null);
const previousFocus = ref<HTMLElement | null>(null);

function handleBackdropClick(event: MouseEvent) {
  event.stopPropagation();
  if (event.target === event.currentTarget) {
    peekStore.close();
  }
}

function handleKeydown(event: KeyboardEvent) {
  if (event.key === "Escape") {
    peekStore.close();
  }
}

async function openInEditor() {
  if (!peekStore.detail) return;
  await invoke("open_path_in_editor", {
    path: peekStore.detail.path,
    editorCommand: preferencesStore.editorCommand ?? "code",
  });
}

async function revealInFinder() {
  if (!peekStore.detail) return;
  await invoke("reveal_in_finder", { path: peekStore.detail.path });
}

watch(
  () => peekStore.isOpen,
  async (open) => {
    if (open) {
      previousFocus.value = document.activeElement as HTMLElement | null;
      await nextTick();
      panelRef.value?.focus();
      document.addEventListener("keydown", handleKeydown);
    } else {
      document.removeEventListener("keydown", handleKeydown);
      previousFocus.value?.focus();
      previousFocus.value = null;
    }
  }
);
</script>

<template>
  <Teleport to="body">
    <Transition name="peek">
      <div
        v-if="peekStore.isOpen"
        class="peek-backdrop"
        @click="handleBackdropClick"
      >
        <div
          ref="panelRef"
          class="peek-panel"
          tabindex="-1"
          role="dialog"
          aria-modal="true"
          :aria-label="peekStore.detail ? peekStore.detail.name : 'Skill details'"
        >
          <!-- Close button -->
          <button class="close-button" @click="peekStore.close()">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18" />
              <line x1="6" y1="6" x2="18" y2="18" />
            </svg>
          </button>

          <!-- Loading state -->
          <div v-if="peekStore.isLoading" class="peek-loading">
            <span class="spinner" />
          </div>

          <!-- Error state -->
          <div v-else-if="peekStore.error" class="peek-error">
            <p class="error-message">{{ peekStore.error }}</p>
            <SecondaryButton label="Close" @click="peekStore.close()" />
          </div>

          <!-- Loaded state -->
          <template v-else-if="peekStore.detail">
            <div class="peek-header">
              <h3 class="peek-name">{{ peekStore.detail.name }}</h3>
              <Badge v-if="peekStore.detail.archived" variant="default" compact>Archived</Badge>
            </div>

            <span class="peek-path">{{ peekStore.detail.path }}</span>

            <div v-if="peekStore.detail.summary" class="peek-section">
              <span class="section-label">Summary</span>
              <p class="summary-text">{{ peekStore.detail.summary }}</p>
            </div>

            <div class="peek-section">
              <span class="section-label">Linked locations</span>
              <div v-if="peekStore.detail.linkedLocations.length > 0" class="compact-list">
                <span
                  v-for="loc in peekStore.detail.linkedLocations"
                  :key="loc.id"
                  class="compact-item"
                >
                  {{ loc.label }}
                </span>
              </div>
              <span v-else class="empty-text">None</span>
            </div>

            <div class="peek-section">
              <span class="section-label">Included in sets</span>
              <div v-if="peekStore.detail.includedInSets.length > 0" class="compact-list">
                <span
                  v-for="set in peekStore.detail.includedInSets"
                  :key="set.id"
                  class="compact-item"
                >
                  {{ set.name }}
                </span>
              </div>
              <span v-else class="empty-text">None</span>
            </div>

            <div class="peek-section">
              <span class="section-label">Usage (30 days)</span>
              <span class="usage-count">{{ peekStore.detail.usage.useCount30d }} uses</span>
            </div>

            <div class="peek-actions">
              <SecondaryButton label="Open in Editor" @click="openInEditor" />
              <SecondaryButton label="Reveal in Finder" @click="revealInFinder" />
            </div>
          </template>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.peek-backdrop {
  position: fixed;
  inset: 0;
  z-index: 250;
  background: rgba(0, 0, 0, 0.08);
}

.peek-panel {
  position: absolute;
  top: 0;
  right: 0;
  bottom: 0;
  width: 320px;
  background: var(--surface-panel);
  box-shadow: var(--shadow-sheet);
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  padding: var(--space-4);
  overflow-y: auto;
  outline: none;
}

.close-button {
  position: absolute;
  top: var(--space-3);
  right: var(--space-3);
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
  transition: all var(--duration-fast) var(--ease-default);
}

.close-button:hover {
  background: var(--surface-hover);
  color: var(--text-primary);
}

.peek-loading {
  display: flex;
  align-items: center;
  justify-content: center;
  flex: 1;
}

.spinner {
  width: 16px;
  height: 16px;
  border: 1.5px solid var(--border-default);
  border-top-color: var(--accent);
  border-radius: 50%;
  animation: spin 600ms linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.peek-error {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  flex: 1;
  gap: var(--space-3);
}

.error-message {
  font-size: var(--text-sm);
  color: var(--text-secondary);
  margin: 0;
}

.peek-header {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding-right: var(--space-6);
}

.peek-name {
  font-size: var(--text-lg);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.peek-path {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  font-family: ui-monospace, "SF Mono", SFMono-Regular, monospace;
  word-break: break-all;
}

.peek-section {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.section-label {
  font-size: var(--text-xs);
  font-weight: var(--weight-medium);
  color: var(--text-tertiary);
}

.summary-text {
  font-size: var(--text-sm);
  color: var(--text-secondary);
  line-height: 1.5;
  margin: 0;
}

.compact-list {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.compact-item {
  font-size: var(--text-sm);
  color: var(--text-secondary);
  padding: 1px 0;
}

.empty-text {
  font-size: var(--text-sm);
  color: var(--text-tertiary);
}

.usage-count {
  font-size: var(--text-sm);
  color: var(--text-primary);
}

.peek-actions {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  margin-top: auto;
  padding-top: var(--space-3);
}

.peek-actions :deep(.secondary-button) {
  width: 100%;
  justify-content: center;
}

.peek-enter-active {
  transition: opacity var(--duration-normal) var(--ease-out);
}

.peek-leave-active {
  transition: opacity var(--duration-fast) var(--ease-default);
}

.peek-enter-from,
.peek-leave-to {
  opacity: 0;
}

.peek-enter-active .peek-panel {
  animation: slide-in var(--duration-normal) var(--ease-out);
}

.peek-leave-active .peek-panel {
  animation: slide-out var(--duration-fast) var(--ease-default);
}

@keyframes slide-in {
  from { transform: translateX(100%); }
  to { transform: translateX(0); }
}

@keyframes slide-out {
  from { transform: translateX(0); }
  to { transform: translateX(100%); }
}
</style>

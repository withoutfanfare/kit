<script setup lang="ts">
import { ref, computed } from "vue";
import type { LocationDetail } from "@/types";
import { useLocationsStore } from "@/stores/locationsStore";
import { useAssignmentStore } from "@/stores/assignmentStore";
import { useAppStore } from "@/stores/appStore";
import { invoke } from "@tauri-apps/api/core";
import { SButton, SInlineTextField } from "@stuntrocket/ui";

const props = defineProps<{
  detail: LocationDetail;
}>();

const locationsStore = useLocationsStore();
const assignmentStore = useAssignmentStore();
const appStore = useAppStore();

type SyncState = "idle" | "syncing" | "done" | "failed";
const syncState = ref<SyncState>("idle");

const syncLabel = computed(() => {
  switch (syncState.value) {
    case "syncing": return "Syncing…";
    case "done": return "Synced";
    case "failed": return "Failed";
    default: return "Sync";
  }
});

async function updateLabel(label: string) {
  await locationsStore.updateLocation(props.detail.id, { label });
  appStore.toast("Label updated", "success");
}

async function syncLocation() {
  if (syncState.value === "syncing") return;
  syncState.value = "syncing";
  try {
    await locationsStore.syncLocation(props.detail.id);
    const detail = locationsStore.detailCache[props.detail.id];
    const skillCount = detail?.skills.length ?? 0;
    const issueCount = detail?.issues.length ?? 0;
    syncState.value = "done";
    if (issueCount > 0) {
      appStore.toast(`Synced — ${skillCount} skills, ${issueCount} issues found`, "info");
    } else {
      appStore.toast(`Synced — ${skillCount} skills, no issues`, "success");
    }
  } catch {
    syncState.value = "failed";
    appStore.toast("Sync failed", "error");
  }
  setTimeout(() => {
    syncState.value = "idle";
  }, 2000);
}

async function openInFinder() {
  try {
    await invoke("reveal_in_finder", { path: props.detail.path });
  } catch {
    appStore.toast("Could not open Finder", "error");
  }
}

function addSkills() {
  assignmentStore.open(props.detail.id);
}
</script>

<template>
  <div class="location-header">
    <div class="header-info">
      <SInlineTextField
        :model-value="detail.label"
        placeholder="Location name"
        @update:model-value="updateLabel"
      />
      <span class="header-path">{{ detail.path }}</span>
    </div>
    <div class="header-actions">
      <SButton @click="addSkills">Add Skills</SButton>
      <button
        class="sync-button"
        :class="syncState"
        :disabled="syncState === 'syncing'"
        @click="syncLocation"
      >
        <svg v-if="syncState === 'syncing'" class="sync-spinner" width="14" height="14" viewBox="0 0 14 14" fill="none">
          <circle cx="7" cy="7" r="5.5" stroke="currentColor" stroke-width="1.5" opacity="0.3"/>
          <path d="M12.5 7a5.5 5.5 0 00-5.5-5.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
        <svg v-else-if="syncState === 'done'" class="sync-icon" width="14" height="14" viewBox="0 0 14 14" fill="none">
          <path d="M3 7.5l3 3 5-6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <svg v-else-if="syncState === 'failed'" class="sync-icon" width="14" height="14" viewBox="0 0 14 14" fill="none">
          <path d="M4 4l6 6M10 4l-6 6" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
        </svg>
        <svg v-else class="sync-icon" width="14" height="14" viewBox="0 0 16 16" fill="none">
          <path d="M13.5 2.5v4h-4M2.5 13.5v-4h4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M3.5 6a5 5 0 018.2-1.8l1.8 1.8M12.5 10a5 5 0 01-8.2 1.8l-1.8-1.8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <span>{{ syncLabel }}</span>
      </button>
      <button class="action-button" @click="openInFinder">
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
          <path d="M2 4.5C2 3.67 2.67 3 3.5 3H6.29a1 1 0 01.7.29L8 4.3a1 1 0 00.71.29H12.5c.83 0 1.5.67 1.5 1.5v5.4c0 .83-.67 1.5-1.5 1.5h-9A1.5 1.5 0 012 11.5v-7z" fill="none" stroke="currentColor" stroke-width="1.3"/>
          <path d="M6 9l2-2 2 2" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <span>Reveal</span>
      </button>
    </div>
  </div>
</template>

<style scoped>
.location-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--space-4);
  padding: var(--space-4) var(--space-5);
  border-bottom: 1px solid var(--border-subtle);
  flex-shrink: 0;
}

.header-info {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  min-width: 0;
  flex: 1;
}

.header-path {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-family: ui-monospace, "SF Mono", SFMono-Regular, monospace;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  flex-shrink: 0;
}

/* Shared button base for sync and action */
.sync-button,
.action-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-1);
  height: 28px;
  padding: 0 var(--space-3);
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
  background: transparent;
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all var(--duration-fast) var(--ease-default);
  white-space: nowrap;
}

.sync-button:hover:not(:disabled),
.action-button:hover {
  background: var(--surface-hover);
  border-color: var(--border-strong);
}

.sync-button:active:not(:disabled),
.action-button:active {
  transform: scale(0.97);
}

.sync-button:disabled {
  cursor: wait;
}

/* Sync states */
.sync-button.syncing {
  color: var(--accent);
  border-color: var(--accent);
  background: var(--accent-subtle);
}

.sync-button.done {
  color: var(--success);
  border-color: var(--success);
  background: var(--success-subtle);
}

.sync-button.failed {
  color: var(--danger);
  border-color: var(--danger);
  background: var(--danger-subtle);
}

.sync-spinner {
  animation: spin 800ms linear infinite;
  flex-shrink: 0;
}

.sync-icon {
  flex-shrink: 0;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style>

<script setup lang="ts">
import { computed } from "vue";
import type { LocationDetail } from "@/types";
import { useLocationsStore } from "@/stores/locationsStore";
import { usePreferencesStore } from "@/stores/preferencesStore";
import { useAppStore } from "@/stores/appStore";
import { invoke } from "@tauri-apps/api/core";
import InspectorPanel from "@/components/layout/InspectorPanel.vue";
import { SButton } from "@stuntrocket/ui";

const props = defineProps<{
  detail: LocationDetail;
}>();

const locationsStore = useLocationsStore();
const preferencesStore = usePreferencesStore();
const appStore = useAppStore();

const lastSyncedDisplay = computed(() => {
  const summary = locationsStore.locationList.find(
    (l) => l.id === props.detail.id
  );
  if (!summary?.lastSyncedAt) return "Never";
  const date = new Date(summary.lastSyncedAt);
  return date.toLocaleString(undefined, {
    day: "numeric",
    month: "short",
    hour: "2-digit",
    minute: "2-digit",
  });
});

const manifestStatus = computed(() =>
  props.detail.manifestPath ? "Present" : "Not found"
);

async function syncLocation() {
  try {
    await locationsStore.syncLocation(props.detail.id);
    appStore.toast("Location synced", "success");
  } catch {
    appStore.toast("Sync failed", "error");
  }
}

async function removeLocation() {
  try {
    await locationsStore.removeLocation(props.detail.id);
    appStore.toast("Location removed", "success");
  } catch {
    appStore.toast("Failed to remove location", "error");
  }
}

async function openManifest() {
  if (props.detail.manifestPath) {
    await invoke("open_path_in_editor", { path: props.detail.manifestPath, editorCommand: preferencesStore.editorCommand ?? "code" });
  }
}
</script>

<template>
  <InspectorPanel title="Location">
    <div class="inspector-section">
      <div class="inspector-field">
        <span class="field-label">Path</span>
        <span class="field-value path">{{ detail.path }}</span>
      </div>
      <div class="inspector-field">
        <span class="field-label">Manifest</span>
        <span class="field-value" :class="{ muted: !detail.manifestPath }">
          {{ manifestStatus }}
        </span>
      </div>
      <div class="inspector-field">
        <span class="field-label">Last synced</span>
        <span class="field-value">{{ lastSyncedDisplay }}</span>
      </div>
    </div>
    <div class="inspector-actions">
      <SButton variant="secondary" @click="syncLocation">Sync</SButton>
      <SButton
        v-if="detail.manifestPath"
        variant="secondary"
        @click="openManifest"
      >Open Manifest</SButton>
      <SButton
        variant="secondary"
        @click="removeLocation"
      >Remove Location</SButton>
    </div>
  </InspectorPanel>
</template>

<style scoped>
.inspector-section {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}

.inspector-field {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.field-label {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  font-weight: var(--weight-medium);
}

.field-value {
  font-size: var(--text-sm);
  color: var(--text-primary);
  word-break: break-all;
}

.field-value.path {
  font-size: var(--text-xs);
  color: var(--text-secondary);
  font-family: ui-monospace, "SF Mono", SFMono-Regular, monospace;
}

.field-value.muted {
  color: var(--text-tertiary);
}

.inspector-actions {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.inspector-actions :deep(button) {
  width: 100%;
  justify-content: center;
}
</style>

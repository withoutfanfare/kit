<script setup lang="ts">
import type { SetDetail } from "@/types";
import { useSetsStore } from "@/stores/setsStore";
import { usePreferencesStore } from "@/stores/preferencesStore";
import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";
import { useRouter } from "vue-router";
import InspectorPanel from "@/components/layout/InspectorPanel.vue";
import SecondaryButton from "@/components/base/SecondaryButton.vue";
import Badge from "@/components/base/Badge.vue";
import ConfirmDialog from "@/components/base/ConfirmDialog.vue";

const props = defineProps<{
  detail: SetDetail;
}>();

const setsStore = useSetsStore();
const preferencesStore = usePreferencesStore();
const router = useRouter();
const showDeleteConfirm = ref(false);

async function openInEditor() {
  await invoke("open_path_in_editor", {
    path: props.detail.path,
    editorCommand: preferencesStore.editorCommand ?? "code",
  });
}

async function revealInFinder() {
  await invoke("reveal_in_finder", { path: props.detail.path });
}

async function confirmDelete() {
  await setsStore.deleteSet(
    props.detail.id,
    props.detail.scope,
    props.detail.ownerLocationId ?? undefined
  );
  showDeleteConfirm.value = false;
  router.push("/sets");
}
</script>

<template>
  <InspectorPanel title="Set">
    <div class="inspector-section">
      <div class="inspector-field">
        <span class="field-label">Source path</span>
        <span class="field-value path">{{ detail.path }}</span>
      </div>
      <div class="inspector-field">
        <span class="field-label">Scope</span>
        <span class="field-value">
          <Badge :variant="detail.scope === 'global' ? 'accent' : 'default'" compact>
            {{ detail.scope === 'global' ? 'Global' : 'Project' }}
          </Badge>
        </span>
      </div>
      <div class="inspector-field">
        <span class="field-label">Skills</span>
        <span class="field-value">{{ detail.skills.length }}</span>
      </div>
    </div>

    <div class="inspector-section">
      <span class="field-label">Assigned locations</span>
      <div v-if="detail.assignedLocations.length > 0" class="compact-list">
        <span
          v-for="loc in detail.assignedLocations"
          :key="loc.id"
          class="compact-item"
        >
          {{ loc.label }}
        </span>
      </div>
      <span v-else class="field-value muted">None</span>
    </div>

    <div v-if="detail.description" class="inspector-section">
      <span class="field-label">Description</span>
      <span class="field-value">{{ detail.description }}</span>
    </div>

    <div class="inspector-actions">
      <SecondaryButton label="Open in Editor" @click="openInEditor" />
      <SecondaryButton label="Reveal in Finder" @click="revealInFinder" />
      <SecondaryButton label="Delete Set" @click="showDeleteConfirm = true" />
    </div>
  </InspectorPanel>

  <ConfirmDialog
    :open="showDeleteConfirm"
    title="Delete set?"
    :message="`This will permanently remove '${detail.name}' and unlink it from all locations.`"
    confirm-label="Delete"
    danger
    @confirm="confirmDelete"
    @cancel="showDeleteConfirm = false"
  />
</template>

<style scoped>
.inspector-section {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
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

.inspector-actions {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.inspector-actions :deep(.secondary-button) {
  width: 100%;
  justify-content: center;
}
</style>

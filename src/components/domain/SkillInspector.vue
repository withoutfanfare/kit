<script setup lang="ts">
import type { SkillDetail } from "@/types";
import { useLibraryStore } from "@/stores/libraryStore";
import { usePreferencesStore } from "@/stores/preferencesStore";
import { invoke } from "@tauri-apps/api/core";
import InspectorPanel from "@/components/layout/InspectorPanel.vue";
import { SBadge, SButton } from "@stuntrocket/ui";

const props = defineProps<{
  detail: SkillDetail;
}>();

const libraryStore = useLibraryStore();
const preferencesStore = usePreferencesStore();

async function toggleArchive() {
  if (props.detail.archived) {
    await libraryStore.unarchiveSkill(props.detail.id);
  } else {
    await libraryStore.archiveSkill(props.detail.id);
  }
}

async function openInEditor() {
  await invoke("open_path_in_editor", { path: props.detail.path, editorCommand: preferencesStore.editorCommand ?? "code" });
}

async function revealInFinder() {
  await invoke("reveal_in_finder", { path: props.detail.path });
}
</script>

<template>
  <InspectorPanel title="Skill">
    <div class="inspector-section">
      <div class="inspector-field">
        <span class="field-label">Source path</span>
        <span class="field-value path">{{ detail.path }}</span>
      </div>
      <div class="inspector-field">
        <span class="field-label">State</span>
        <span class="field-value">
          <SBadge v-if="detail.archived" variant="count">Archived</SBadge>
          <span v-else class="state-active">Active</span>
        </span>
      </div>
      <div v-if="detail.summary" class="inspector-field">
        <span class="field-label">Summary</span>
        <span class="field-value">{{ detail.summary }}</span>
      </div>
    </div>

    <div class="inspector-section">
      <span class="field-label">Linked locations</span>
      <div v-if="detail.linkedLocations.length > 0" class="compact-list">
        <span
          v-for="loc in detail.linkedLocations"
          :key="loc.id"
          class="compact-item"
        >
          {{ loc.label }}
        </span>
      </div>
      <span v-else class="field-value muted">None</span>
    </div>

    <div class="inspector-section">
      <span class="field-label">Included in sets</span>
      <div v-if="detail.includedInSets.length > 0" class="compact-list">
        <span
          v-for="set in detail.includedInSets"
          :key="set.id"
          class="compact-item"
        >
          {{ set.name }}
        </span>
      </div>
      <span v-else class="field-value muted">None</span>
    </div>

    <div class="inspector-section">
      <span class="field-label">Usage (30 days)</span>
      <span class="field-value">{{ detail.usage.useCount30d }} uses</span>
    </div>

    <div class="inspector-actions">
      <SButton variant="secondary" size="sm" @click="toggleArchive">
        {{ detail.archived ? 'Unarchive' : 'Archive' }}
      </SButton>
      <SButton variant="secondary" size="sm" @click="openInEditor">Open in Editor</SButton>
      <SButton variant="secondary" size="sm" @click="revealInFinder">Reveal in Finder</SButton>
    </div>
  </InspectorPanel>
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

.state-active {
  font-size: var(--text-sm);
  color: var(--success);
  font-weight: var(--weight-medium);
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

.inspector-actions :deep(button) {
  width: 100%;
  justify-content: center;
}
</style>

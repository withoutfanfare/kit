<script setup lang="ts">
import { watch, computed, onMounted, onUnmounted, ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useSetsStore } from "@/stores/setsStore";
import { useLibraryStore } from "@/stores/libraryStore";
import { usePreferencesStore } from "@/stores/preferencesStore";
import { useLocationsStore } from "@/stores/locationsStore";
import { useAppStore } from "@/stores/appStore";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import type { UnlistenFn } from "@tauri-apps/api/event";
import Badge from "@/components/base/Badge.vue";
import InlineTextField from "@/components/base/InlineTextField.vue";
import SectionHeader from "@/components/base/SectionHeader.vue";
import SecondaryButton from "@/components/base/SecondaryButton.vue";
import ConfirmDialog from "@/components/base/ConfirmDialog.vue";
import SheetPanel from "@/components/base/SheetPanel.vue";
import SearchField from "@/components/base/SearchField.vue";

const route = useRoute();
const router = useRouter();
const setsStore = useSetsStore();
const libraryStore = useLibraryStore();
const locationsStore = useLocationsStore();
const preferencesStore = usePreferencesStore();

const setId = computed(() => route.params.setId as string);
const detail = computed(() => setsStore.selectedDetail);

const appStore = useAppStore();
const showDeleteConfirm = ref(false);
const showSkillPicker = ref(false);
const skillPickerQuery = ref("");
const isDraggingSkill = ref(false);
let unlistenDragDrop: UnlistenFn | null = null;

const availableSkills = computed(() => {
  if (!detail.value) return [];
  const existingIds = new Set(detail.value.skills.map((s) => s.id));
  let skills = libraryStore.items.filter(
    (i) => i.kind === "skill" && !existingIds.has(i.id)
  );
  if (skillPickerQuery.value.trim()) {
    const q = skillPickerQuery.value.trim().toLowerCase();
    skills = skills.filter(
      (s) =>
        s.name.toLowerCase().includes(q) ||
        (s.summary && s.summary.toLowerCase().includes(q))
    );
  }
  return skills;
});

function loadDetail() {
  const id = setId.value;
  if (id) {
    setsStore.selectSet(id);
  }
}

async function updateName(name: string) {
  if (!detail.value) return;
  await setsStore.updateSet(
    detail.value.id,
    detail.value.scope,
    detail.value.ownerLocationId ?? undefined,
    { name }
  );
}

async function updateDescription(description: string) {
  if (!detail.value) return;
  await setsStore.updateSet(
    detail.value.id,
    detail.value.scope,
    detail.value.ownerLocationId ?? undefined,
    { description: description || null }
  );
}

async function removeSkill(skillId: string) {
  if (!detail.value) return;
  await setsStore.removeSkillFromSet(
    detail.value.id,
    skillId,
    detail.value.scope,
    detail.value.ownerLocationId ?? undefined
  );
}

async function addSkill(skillId: string) {
  if (!detail.value) return;
  await setsStore.addSkillToSet(
    detail.value.id,
    skillId,
    detail.value.scope,
    detail.value.ownerLocationId ?? undefined
  );
}

function openSkillPicker() {
  skillPickerQuery.value = "";
  libraryStore.fetchItems();
  showSkillPicker.value = true;
}

function navigateToLocation(id: string) {
  locationsStore.selectLocation(id);
  router.push(`/locations/${id}`);
}

async function openInEditor() {
  if (!detail.value) return;
  await invoke("open_path_in_editor", {
    path: detail.value.path,
    editorCommand: preferencesStore.editorCommand ?? "code",
  });
}

async function confirmDelete() {
  if (!detail.value) return;
  await setsStore.deleteSet(
    detail.value.id,
    detail.value.scope,
    detail.value.ownerLocationId ?? undefined
  );
  showDeleteConfirm.value = false;
  router.push("/sets");
}

async function addSkillByPath(path: string) {
  if (!detail.value) return;
  try {
    const skillId = await invoke<string>("resolve_skill_path", { path });
    // Check not already in the set
    if (detail.value.skills.some((s) => s.id === skillId)) {
      appStore.toast(`'${skillId}' is already in this set`, "info");
      return;
    }
    await setsStore.addSkillToSet(
      detail.value.id,
      skillId,
      detail.value.scope,
      detail.value.ownerLocationId ?? undefined
    );
    appStore.toast(`Added '${skillId}' to set`, "success");
  } catch {
    appStore.toast("Could not resolve to a library skill", "error");
  }
}

onMounted(async () => {
  loadDetail();

  // Tauri native drag-drop for skill folders/symlinks
  const webview = getCurrentWebviewWindow();
  unlistenDragDrop = await webview.onDragDropEvent((event) => {
    if (event.payload.type === "enter") {
      isDraggingSkill.value = true;
    } else if (event.payload.type === "drop") {
      isDraggingSkill.value = false;
      for (const path of event.payload.paths) {
        addSkillByPath(path);
      }
    } else if (event.payload.type === "leave") {
      isDraggingSkill.value = false;
    }
  });
});

onUnmounted(() => {
  unlistenDragDrop?.();
});

watch(setId, loadDetail);
</script>

<template>
  <div class="set-detail-wrapper">
  <div v-if="detail" class="set-detail">
    <!-- Drop overlay for dragging skills -->
    <div v-if="isDraggingSkill" class="drop-overlay">
      <div class="drop-zone">
        <svg width="28" height="28" viewBox="0 0 16 16" fill="none">
          <path d="M8 3v10M3 8h10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
        <span class="drop-label">Drop skill folder to add it to this set</span>
      </div>
    </div>
    <div class="detail-header">
      <div class="header-title-row">
        <InlineTextField
          :model-value="detail.name"
          placeholder="Set name"
          class="header-name-field"
          @update:model-value="updateName"
        />
        <Badge :variant="detail.scope === 'global' ? 'accent' : 'default'">
          {{ detail.scope === 'global' ? 'Global' : 'Project' }}
        </Badge>
        <Badge compact>{{ detail.skills.length }} skills</Badge>
      </div>
      <InlineTextField
        :model-value="detail.description ?? ''"
        placeholder="Add a description..."
        class="header-description-field"
        @update:model-value="updateDescription"
      />
      <span class="header-path">{{ detail.path }}</span>
    </div>

    <div class="detail-content">
      <!-- Skills section -->
      <div class="detail-section">
        <SectionHeader
          title="Skills"
          :count="detail.skills.length"
          action-label="Add"
          @action="openSkillPicker"
        />
        <div v-if="detail.skills.length > 0" class="section-group">
          <div
            v-for="skill in detail.skills"
            :key="skill.id"
            class="skill-row"
          >
            <div class="skill-row-content">
              <span class="skill-name">{{ skill.name }}</span>
              <Badge v-if="skill.archived" variant="default" compact>Archived</Badge>
            </div>
            <button class="remove-button" title="Remove from set" @click="removeSkill(skill.id)">
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <line x1="18" y1="6" x2="6" y2="18" />
                <line x1="6" y1="6" x2="18" y2="18" />
              </svg>
            </button>
          </div>
        </div>
        <div v-else class="section-empty">
          <span class="section-empty-text">No skills yet — click Add or drag skill folders here</span>
        </div>
      </div>

      <!-- Assigned Locations section -->
      <div class="detail-section">
        <SectionHeader
          title="Assigned Locations"
          :count="detail.assignedLocations.length"
        />
        <div v-if="detail.assignedLocations.length > 0" class="section-group">
          <div
            v-for="loc in detail.assignedLocations"
            :key="loc.id"
            class="location-row"
            @click="navigateToLocation(loc.id)"
          >
            <div class="location-row-content">
              <span class="location-label">{{ loc.label }}</span>
              <span class="location-path">{{ loc.path }}</span>
            </div>
            <svg
              class="chevron"
              width="12"
              height="12"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <polyline points="9 18 15 12 9 6" />
            </svg>
          </div>
        </div>
        <div v-else class="section-empty">
          <span class="section-empty-text">Not assigned to any locations</span>
        </div>
      </div>

      <!-- Actions section -->
      <div class="detail-section">
        <SectionHeader title="Actions" />
        <div class="actions-row">
          <SecondaryButton label="Open in Editor" @click="openInEditor" />
          <SecondaryButton label="Delete Set" @click="showDeleteConfirm = true" />
        </div>
      </div>
    </div>
  </div>
  <div v-else class="loading-state">
    <span class="loading-text">Loading...</span>
  </div>

  <!-- Delete Confirm -->
  <ConfirmDialog
    :open="showDeleteConfirm"
    title="Delete set?"
    :message="detail ? `This will permanently remove '${detail.name}' and unlink it from all locations.` : ''"
    confirm-label="Delete"
    danger
    @confirm="confirmDelete"
    @cancel="showDeleteConfirm = false"
  />

  <!-- Skill Picker Sheet -->
  <SheetPanel :open="showSkillPicker" @close="showSkillPicker = false">
    <div class="picker-content">
      <h3 class="picker-title">Add skills to set</h3>
      <SearchField
        v-model="skillPickerQuery"
        placeholder="Search skills..."
      />
      <div class="picker-list">
        <div
          v-for="skill in availableSkills"
          :key="skill.id"
          class="picker-row"
          @click="addSkill(skill.id)"
        >
          <div class="picker-row-content">
            <span class="picker-row-name">{{ skill.name }}</span>
            <span v-if="skill.summary" class="picker-row-summary">{{ skill.summary }}</span>
          </div>
          <svg
            class="add-icon"
            width="14"
            height="14"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <line x1="12" y1="5" x2="12" y2="19" />
            <line x1="5" y1="12" x2="19" y2="12" />
          </svg>
        </div>
        <div v-if="availableSkills.length === 0" class="picker-empty">
          <span class="picker-empty-text">No skills available to add</span>
        </div>
      </div>
    </div>
  </SheetPanel>
  </div>
</template>

<style scoped>
.set-detail-wrapper {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.set-detail {
  display: flex;
  flex-direction: column;
  flex: 1;
  min-height: 0;
  position: relative;
}

.drop-overlay {
  position: absolute;
  inset: 0;
  z-index: 50;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.3);
  backdrop-filter: blur(2px);
}

.drop-zone {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-8) var(--space-10);
  border: 2px dashed var(--accent);
  border-radius: var(--radius-lg);
  background: var(--surface-panel);
  color: var(--accent);
  box-shadow: var(--shadow-lg);
}

.drop-label {
  font-size: var(--text-md);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
}

.detail-header {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  padding: var(--space-4) var(--space-5);
  border-bottom: 1px solid var(--border-subtle);
  flex-shrink: 0;
}

.header-title-row {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.header-name-field :deep(.field-display),
.header-name-field :deep(.field-input) {
  font-size: var(--text-xl);
  font-weight: var(--weight-semibold);
}

.header-description-field :deep(.field-display),
.header-description-field :deep(.field-input) {
  font-size: var(--text-sm);
  color: var(--text-secondary);
}

.header-path {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  font-family: ui-monospace, "SF Mono", SFMono-Regular, monospace;
}

.detail-content {
  flex: 1;
  overflow-y: auto;
  padding: var(--space-4) var(--space-5);
  display: flex;
  flex-direction: column;
  gap: var(--space-5);
}

.detail-section {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.section-group {
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  background: var(--surface-panel);
  overflow: hidden;
}

/* Skill rows */
.skill-row {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  transition: background var(--duration-fast) var(--ease-default);
}

.skill-row + .skill-row {
  border-top: 1px solid var(--border-subtle);
}

.skill-row:hover {
  background: var(--surface-hover);
}

.skill-row-content {
  flex: 1;
  display: flex;
  align-items: center;
  gap: var(--space-2);
  min-width: 0;
}

.skill-name {
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.remove-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  border: none;
  background: transparent;
  color: var(--text-tertiary);
  border-radius: var(--radius-sm);
  cursor: pointer;
  flex-shrink: 0;
  transition: all var(--duration-fast) var(--ease-default);
}

.remove-button:hover {
  background: var(--danger-subtle);
  color: var(--danger);
}

/* Location rows */
.location-row {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  cursor: pointer;
  transition: background var(--duration-fast) var(--ease-default);
  user-select: none;
}

.location-row + .location-row {
  border-top: 1px solid var(--border-subtle);
}

.location-row:hover {
  background: var(--surface-hover);
}

.location-row-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 1px;
  min-width: 0;
}

.location-label {
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.location-path {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.chevron {
  color: var(--text-tertiary);
  flex-shrink: 0;
}

/* Empty states */
.section-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--space-4);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  background: var(--surface-panel);
}

.section-empty-text {
  font-size: var(--text-sm);
  color: var(--text-tertiary);
}

/* Actions */
.actions-row {
  display: flex;
  gap: var(--space-2);
}

/* Loading */
.loading-state {
  display: flex;
  align-items: center;
  justify-content: center;
  flex: 1;
}

.loading-text {
  font-size: var(--text-sm);
  color: var(--text-tertiary);
}

/* Skill Picker */
.picker-content {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  padding: var(--space-5);
}

.picker-title {
  font-family: var(--font-sans);
  font-size: var(--text-lg);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  margin: 0;
}

.picker-list {
  display: flex;
  flex-direction: column;
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  background: var(--surface-panel);
  overflow: hidden;
  max-height: 400px;
  overflow-y: auto;
}

.picker-row {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  cursor: pointer;
  transition: background var(--duration-fast) var(--ease-default);
  user-select: none;
}

.picker-row + .picker-row {
  border-top: 1px solid var(--border-subtle);
}

.picker-row:hover {
  background: var(--surface-hover);
}

.picker-row-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 1px;
  min-width: 0;
}

.picker-row-name {
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.picker-row-summary {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.add-icon {
  color: var(--accent);
  flex-shrink: 0;
}

.picker-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--space-6);
}

.picker-empty-text {
  font-size: var(--text-sm);
  color: var(--text-tertiary);
}
</style>

<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { useAssignmentStore } from "@/stores/assignmentStore";
import { useLibraryStore } from "@/stores/libraryStore";
import { useLocationsStore } from "@/stores/locationsStore";
import { useSkillPeekStore } from "@/stores/skillPeekStore";
import type { LibraryListItem, SkillAssignment } from "@/types";
import { SModal, SSearchInput, SButton, SBadge } from "@stuntrocket/ui";
import SelectionPreview from "./SelectionPreview.vue";

const assignmentStore = useAssignmentStore();
const libraryStore = useLibraryStore();
const locationsStore = useLocationsStore();
const skillPeekStore = useSkillPeekStore();

const searchQuery = ref("");

const locationDetail = computed(() =>
  assignmentStore.locationId
    ? locationsStore.detailCache[assignmentStore.locationId] ?? null
    : null
);

const installedSkillIds = computed(() => {
  if (!locationDetail.value) return new Set<string>();
  return new Set(locationDetail.value.skills.map((s: SkillAssignment) => s.skillId));
});

const installedSetIds = computed(() => {
  if (!locationDetail.value) return new Set<string>();
  return new Set(locationDetail.value.sets.map((s) => s.setId));
});

const filteredItems = computed(() => {
  const query = searchQuery.value.toLowerCase().trim();
  if (!query) return libraryStore.items;
  return libraryStore.items.filter(
    (item: LibraryListItem) =>
      item.name.toLowerCase().includes(query) ||
      (item.summary && item.summary.toLowerCase().includes(query))
  );
});

const skills = computed(() =>
  filteredItems.value.filter((i: LibraryListItem) => i.kind === "skill")
);

const sets = computed(() =>
  filteredItems.value.filter((i: LibraryListItem) => i.kind === "set")
);

const installedSkills = computed(() => {
  if (!locationDetail.value) return [];
  return locationDetail.value.skills.filter(
    (s: SkillAssignment) => s.source === "library"
  );
});

function isSkillSelected(id: string): boolean {
  return assignmentStore.selectedSkillIds.has(id);
}

function isSetSelected(id: string): boolean {
  return assignmentStore.selectedSetIds.has(id);
}

function isMarkedForRemoval(id: string): boolean {
  return assignmentStore.removeSkillIds.has(id);
}

function isInstalled(item: LibraryListItem): boolean {
  if (item.kind === "skill") return installedSkillIds.value.has(item.id);
  if (item.kind === "set") return installedSetIds.value.has(item.id);
  return false;
}

function toggleItem(item: LibraryListItem) {
  if (item.kind === "skill") {
    assignmentStore.toggleSkill(item.id);
  } else {
    assignmentStore.toggleSet(item.id);
  }
}

function toggleRemove(skillId: string) {
  assignmentStore.toggleRemoveSkill(skillId);
}

function handleApply() {
  assignmentStore.apply();
}

function handleClose() {
  assignmentStore.close();
}

watch(
  () => assignmentStore.isOpen,
  (isOpen) => {
    if (isOpen) {
      searchQuery.value = "";
      libraryStore.fetchItems();
    }
  }
);
</script>

<template>
  <SModal :open="assignmentStore.isOpen" max-width="max-w-4xl" @close="handleClose">
    <div class="assignment-sheet">
      <!-- Header -->
      <div class="sheet-header">
        <h2 class="sheet-title">Manage Skills</h2>
        <span v-if="locationDetail" class="sheet-subtitle">
          {{ locationDetail.label }}
        </span>
      </div>

      <!-- Body: two-column layout -->
      <div class="sheet-body">
        <!-- Left: Library list -->
        <div class="library-panel">
          <div class="library-search">
            <SSearchInput
              v-model="searchQuery"
              placeholder="Search skills and sets..."
            />
          </div>

          <div class="library-list">
            <!-- Skills to add -->
            <div v-if="skills.length > 0" class="list-group">
              <div class="group-label">
                Skills
                <SBadge>{{ skills.length }}</SBadge>
              </div>
              <label
                v-for="item in skills"
                :key="item.id"
                class="library-item"
                :class="{
                  installed: isInstalled(item),
                  selected: isSkillSelected(item.id),
                  archived: item.archived,
                }"
              >
                <input
                  type="checkbox"
                  class="item-checkbox"
                  :checked="isSkillSelected(item.id)"
                  :disabled="isInstalled(item)"
                  @change="toggleItem(item)"
                />
                <div class="item-info">
                  <span class="item-name">{{ item.name }}</span>
                  <span v-if="item.summary" class="item-summary">{{ item.summary }}</span>
                </div>
                <button
                  class="info-button"
                  title="View skill details"
                  @click.prevent.stop="skillPeekStore.peek(item.id)"
                >
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <circle cx="12" cy="12" r="10" />
                    <path d="M12 16v-4M12 8h.01" />
                  </svg>
                </button>
                <SBadge v-if="isInstalled(item)" variant="success">Installed</SBadge>
                <SBadge v-if="item.archived" variant="warning">Archived</SBadge>
              </label>
            </div>

            <!-- Sets to add -->
            <div v-if="sets.length > 0" class="list-group">
              <div class="group-label">
                Sets
                <SBadge>{{ sets.length }}</SBadge>
              </div>
              <label
                v-for="item in sets"
                :key="item.id"
                class="library-item"
                :class="{
                  installed: isInstalled(item),
                  selected: isSetSelected(item.id),
                }"
              >
                <input
                  type="checkbox"
                  class="item-checkbox"
                  :checked="isSetSelected(item.id)"
                  :disabled="isInstalled(item)"
                  @change="toggleItem(item)"
                />
                <div class="item-info">
                  <span class="item-name">{{ item.name }}</span>
                  <span v-if="item.summary" class="item-summary">{{ item.summary }}</span>
                </div>
                <SBadge v-if="isInstalled(item)" variant="success">Installed</SBadge>
              </label>
            </div>

            <!-- Installed skills (removable) -->
            <div v-if="installedSkills.length > 0" class="list-group">
              <div class="group-label">
                Installed
                <SBadge>{{ installedSkills.length }}</SBadge>
              </div>
              <label
                v-for="skill in installedSkills"
                :key="'remove-' + skill.skillId"
                class="library-item removable"
                :class="{ 'marked-remove': isMarkedForRemoval(skill.skillId) }"
              >
                <input
                  type="checkbox"
                  class="item-checkbox remove-checkbox"
                  :checked="isMarkedForRemoval(skill.skillId)"
                  @change="toggleRemove(skill.skillId)"
                />
                <div class="item-info">
                  <span class="item-name">{{ skill.name }}</span>
                </div>
                <button
                  class="info-button"
                  title="View skill details"
                  @click.prevent.stop="skillPeekStore.peek(skill.skillId)"
                >
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <circle cx="12" cy="12" r="10" />
                    <path d="M12 16v-4M12 8h.01" />
                  </svg>
                </button>
                <SBadge v-if="isMarkedForRemoval(skill.skillId)" variant="error">Remove</SBadge>
              </label>
            </div>

            <!-- Empty search -->
            <div v-if="filteredItems.length === 0 && searchQuery" class="list-empty">
              <p class="empty-label">No skills or sets match "{{ searchQuery }}"</p>
            </div>
          </div>
        </div>

        <!-- Right: Preview -->
        <div class="preview-panel">
          <SelectionPreview
            :preview="assignmentStore.preview"
            :is-loading="assignmentStore.isPreviewLoading"
          />
        </div>
      </div>

      <!-- Footer -->
      <div class="sheet-footer">
        <SButton variant="secondary" @click="handleClose">Cancel</SButton>
        <SButton
          :disabled="!assignmentStore.hasSelections"
          :loading="assignmentStore.isApplying"
          @click="handleApply"
        >Apply Changes</SButton>
      </div>
    </div>
  </SModal>
</template>

<style scoped>
.assignment-sheet {
  display: flex;
  flex-direction: column;
  height: 70vh;
  max-height: 70vh;
}

/* Header */
.sheet-header {
  display: flex;
  align-items: baseline;
  gap: var(--space-3);
  padding: var(--space-5) var(--space-5) var(--space-3);
  border-bottom: 1px solid var(--border-subtle);
  flex-shrink: 0;
}

.sheet-title {
  font-family: var(--font-sans);
  font-size: var(--text-lg);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  margin: 0;
}

.sheet-subtitle {
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--text-tertiary);
}

/* Body */
.sheet-body {
  display: flex;
  flex: 1;
  min-height: 0;
}

/* Library panel (left) */
.library-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
  border-right: 1px solid var(--border-subtle);
}

.library-search {
  padding: var(--space-3) var(--space-4);
  border-bottom: 1px solid var(--border-subtle);
  flex-shrink: 0;
}

.library-list {
  flex: 1;
  overflow-y: auto;
  padding: var(--space-2) var(--space-3);
}

.list-group {
  margin-bottom: var(--space-3);
}

.group-label {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  font-family: var(--font-sans);
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.04em;
  padding: var(--space-2) var(--space-2) var(--space-1);
}

.library-item {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: background var(--duration-fast) var(--ease-default);
  user-select: none;
}

.library-item:hover {
  background: var(--surface-hover);
}

.library-item.installed {
  opacity: 0.6;
}

.library-item.selected {
  background: var(--accent-subtle);
}

.library-item.archived {
  opacity: 0.7;
}

.library-item.marked-remove {
  background: var(--danger-subtle);
}

.item-checkbox {
  flex-shrink: 0;
  width: 16px;
  height: 16px;
  margin: 0;
  accent-color: var(--accent);
  cursor: pointer;
}

.item-checkbox:disabled {
  cursor: not-allowed;
}

.remove-checkbox {
  accent-color: var(--danger);
}

.item-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.item-name {
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.item-summary {
  font-family: var(--font-sans);
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.list-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--space-8) var(--space-4);
}

.empty-label {
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--text-tertiary);
  margin: 0;
}

/* Preview panel (right) */
.preview-panel {
  width: 280px;
  flex-shrink: 0;
  padding: var(--space-4);
  overflow-y: auto;
}

.info-button {
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
  flex-shrink: 0;
  transition: all var(--duration-fast) var(--ease-default);
}

.info-button:hover {
  background: var(--surface-hover);
  color: var(--accent);
}

/* Footer */
.sheet-footer {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: var(--space-2);
  padding: var(--space-3) var(--space-5);
  border-top: 1px solid var(--border-subtle);
  flex-shrink: 0;
}
</style>

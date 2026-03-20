<script setup lang="ts">
import { onMounted, ref, computed } from "vue";
import { useLibraryStore } from "@/stores/libraryStore";
import { useLocationsStore } from "@/stores/locationsStore";
import { useAppStore } from "@/stores/appStore";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import SplitPaneLayout from "@/components/layout/SplitPaneLayout.vue";
import SkillInspector from "@/components/domain/SkillInspector.vue";
import { SBadge, SSearchInput, SSegmentedControl, SEmptyState } from "@stuntrocket/ui";

const libraryStore = useLibraryStore();
const locationsStore = useLocationsStore();
const appStore = useAppStore();
const router = useRouter();

const filterOptions = [
  { label: "All", value: "all" },
  { label: "Skills", value: "skill" },
  { label: "Sets", value: "set" },
];

const sortOptions = [
  { label: "Name", value: "name" },
  { label: "Most used", value: "most_used" },
  { label: "Least used", value: "least_used" },
];

// Inline preview state
const previewSkillId = ref<string | null>(null);
const previewContent = ref<string | null>(null);
const isLoadingPreview = ref(false);

const activeLocationId = computed(() => locationsStore.selectedLocationId);
const activeLocation = computed(() => locationsStore.selectedLocation);

function selectItem(id: string, kind: string) {
  if (kind === "skill") {
    libraryStore.selectSkill(id);
    router.push(`/skills/${id}`);
  }
}

async function togglePreview(id: string, path?: string) {
  if (previewSkillId.value === id) {
    previewSkillId.value = null;
    previewContent.value = null;
    return;
  }
  previewSkillId.value = id;
  isLoadingPreview.value = true;
  try {
    // Find path from library items or use library root
    const item = libraryStore.items.find((i) => i.id === id);
    const skillPath = path ?? (item ? undefined : undefined);
    const content = await invoke<string>("read_skill_content", {
      skillPath: skillPath ?? id,
    });
    if (previewSkillId.value === id) {
      previewContent.value = content;
    }
  } catch {
    if (previewSkillId.value === id) {
      previewContent.value = null;
    }
  } finally {
    isLoadingPreview.value = false;
  }
}

async function quickAssign(skillId: string) {
  if (!activeLocationId.value) return;
  try {
    await invoke("apply_assignment", {
      locationId: activeLocationId.value,
      skillIdsToAdd: [skillId],
      setIdsToAdd: [],
      skillIdsToRemove: [],
      setIdsToRemove: [],
      updateManifest: true,
    });
    appStore.toast(
      `Assigned '${skillId}' to ${activeLocation.value?.label ?? "location"}`,
      "success"
    );
    // Refresh the location detail
    await locationsStore.fetchDetail(activeLocationId.value);
    // Refresh library to update linked counts
    await libraryStore.fetchItems();
  } catch {
    appStore.toast("Failed to assign skill", "error");
  }
}

function isAssignedToActive(id: string): boolean {
  if (!activeLocationId.value) return false;
  const detail = locationsStore.detailCache[activeLocationId.value];
  if (!detail) return false;
  return detail.skills.some((s) => s.skillId === id && s.linkState === "linked");
}

onMounted(() => {
  libraryStore.fetchItems();
});
</script>

<template>
  <SplitPaneLayout :show-inspector="false">
    <template #sidebar>
      <div class="library-sidebar">
        <div class="sidebar-controls">
          <SSearchInput
            v-model="libraryStore.searchQuery"
            placeholder="Search library..."
            compact
          />
          <SSegmentedControl
            v-model="libraryStore.filterKind"
            :options="filterOptions"
          />
          <div class="filter-row">
            <SSegmentedControl
              v-model="libraryStore.sortBy"
              :options="sortOptions"
            />
          </div>
          <label v-if="libraryStore.unusedCount > 0" class="unused-filter">
            <input
              type="checkbox"
              :checked="libraryStore.filterUnused"
              @change="libraryStore.filterUnused = !libraryStore.filterUnused"
            />
            <span>Unused only</span>
            <SBadge variant="warning" compact>{{ libraryStore.unusedCount }}</SBadge>
          </label>
        </div>
        <div class="sidebar-items">
          <div
            v-for="item in libraryStore.filteredItems"
            :key="item.id"
            class="library-row"
            :class="{
              selected: item.id === libraryStore.selectedSkillId,
              archived: item.archived,
            }"
          >
            <div class="row-main" @click="selectItem(item.id, item.kind)">
              <div class="row-content">
                <span class="row-name">{{ item.name }}</span>
                <span v-if="item.summary" class="row-summary">{{ item.summary }}</span>
              </div>
              <div class="row-meta">
                <SBadge v-if="item.useCount30d > 0" variant="count" compact>
                  {{ item.useCount30d }} uses
                </SBadge>
                <span v-if="item.kind === 'skill' && item.useCount30d === 0 && item.isUnusedEverywhere" class="unused-dot" title="Not assigned anywhere" />
                <SBadge v-if="item.archived" variant="default" compact>archived</SBadge>
                <SBadge :variant="item.kind === 'skill' ? 'accent' : 'default'" compact>
                  {{ item.kind }}
                </SBadge>
              </div>
            </div>
            <div v-if="item.kind === 'skill'" class="row-actions">
              <button
                class="preview-button"
                :class="{ active: previewSkillId === item.id }"
                title="Preview SKILL.md"
                @click.stop="togglePreview(item.id)"
              >
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" />
                  <circle cx="12" cy="12" r="3" />
                </svg>
              </button>
              <button
                v-if="activeLocationId && !isAssignedToActive(item.id)"
                class="quick-assign-button"
                :title="`Assign to ${activeLocation?.label ?? 'active location'}`"
                @click.stop="quickAssign(item.id)"
              >
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <line x1="12" y1="5" x2="12" y2="19" />
                  <line x1="5" y1="12" x2="19" y2="12" />
                </svg>
              </button>
              <SBadge v-if="activeLocationId && isAssignedToActive(item.id)" variant="success" compact>
                assigned
              </SBadge>
            </div>

            <!-- Inline preview panel -->
            <div v-if="previewSkillId === item.id" class="inline-preview" @click.stop>
              <div v-if="isLoadingPreview" class="preview-loading">Loading...</div>
              <pre v-else-if="previewContent" class="preview-content">{{ previewContent.length > 4000 ? previewContent.slice(0, 4000) + '\n\n... (truncated)' : previewContent }}</pre>
              <div v-else class="preview-error">Could not load SKILL.md</div>
            </div>
          </div>
          <div v-if="libraryStore.filteredItems.length === 0 && !libraryStore.isLoading" class="list-empty">
            <span class="list-empty-text">No items found</span>
          </div>
        </div>
      </div>
    </template>
    <template #main>
      <router-view v-if="libraryStore.selectedSkillId" />
      <SEmptyState
        v-else-if="libraryStore.items.length === 0 && !libraryStore.isLoading"
        title="No skills in library"
        description="Set your skill library root in Settings to browse and manage skills."
      />
      <SEmptyState
        v-else-if="!libraryStore.isLoading"
        title="Select a skill"
        description="Choose a skill from the sidebar to see where it's used and manage it."
      />
    </template>
    <template #inspector>
      <SkillInspector
        v-if="libraryStore.selectedDetail"
        :detail="libraryStore.selectedDetail"
      />
    </template>
  </SplitPaneLayout>
</template>

<style scoped>
.library-sidebar {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.sidebar-controls {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  padding: var(--space-3);
  border-bottom: 1px solid var(--border-subtle);
  flex-shrink: 0;
}

.filter-row {
  display: flex;
  gap: var(--space-2);
}

.unused-filter {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  font-size: var(--text-xs);
  color: var(--text-secondary);
  cursor: pointer;
  user-select: none;
}

.unused-filter input {
  margin: 0;
  accent-color: var(--warning);
}

.sidebar-items {
  flex: 1;
  overflow-y: auto;
  padding: var(--space-1);
}

.library-row {
  display: flex;
  flex-direction: column;
  border-radius: var(--radius-sm);
  transition: background var(--duration-fast) var(--ease-default);
  user-select: none;
}

.library-row:hover {
  background: var(--surface-hover);
}

.library-row.selected {
  background: var(--surface-selected);
}

.library-row.selected:hover {
  background: var(--surface-selected-strong);
}

.library-row.archived {
  opacity: 0.6;
}

.row-main {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  cursor: default;
  flex: 1;
}

.row-content {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.row-name {
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.row-summary {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.row-meta {
  display: flex;
  align-items: center;
  gap: var(--space-1);
  flex-shrink: 0;
}

.unused-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--warning);
  flex-shrink: 0;
}

.row-actions {
  display: flex;
  align-items: center;
  gap: var(--space-1);
  padding: 0 var(--space-3) var(--space-1);
}

.preview-button,
.quick-assign-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 22px;
  height: 22px;
  border: none;
  background: transparent;
  color: var(--text-tertiary);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all var(--duration-fast) var(--ease-default);
}

.preview-button:hover {
  background: var(--accent-subtle);
  color: var(--accent);
}

.preview-button.active {
  background: var(--accent-subtle);
  color: var(--accent);
}

.quick-assign-button:hover {
  background: var(--success-subtle);
  color: var(--success);
}

/* Inline preview */
.inline-preview {
  padding: var(--space-2) var(--space-3);
  border-top: 1px solid var(--border-subtle);
  max-height: 200px;
  overflow-y: auto;
  background: var(--surface-panel);
  border-radius: 0 0 var(--radius-sm) var(--radius-sm);
}

.preview-content {
  font-family: ui-monospace, "SF Mono", SFMono-Regular, monospace;
  font-size: 10px;
  line-height: 1.5;
  color: var(--text-secondary);
  white-space: pre-wrap;
  word-break: break-word;
  margin: 0;
}

.preview-loading,
.preview-error {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  text-align: center;
  padding: var(--space-2);
}

.list-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--space-6);
}

.list-empty-text {
  font-size: var(--text-sm);
  color: var(--text-tertiary);
}
</style>

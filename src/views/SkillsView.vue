<script setup lang="ts">
import { onMounted, ref, computed, watch } from "vue";
import { useLibraryStore } from "@/stores/libraryStore";
import { useLocationsStore } from "@/stores/locationsStore";
import { useAppStore } from "@/stores/appStore";
import { useRoute, useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import SplitPaneLayout from "@/components/layout/SplitPaneLayout.vue";
import SkillInspector from "@/components/domain/SkillInspector.vue";
import PanelIcon from "@/components/base/PanelIcon.vue";
import LibraryTabs from "@/components/domain/LibraryTabs.vue";
import { SBadge, SSearchInput, SEmptyState } from "@stuntrocket/ui";

const libraryStore = useLibraryStore();
const locationsStore = useLocationsStore();
const appStore = useAppStore();
const router = useRouter();
const route = useRoute();

// Inline preview state
const previewSkillId = ref<string | null>(null);
const previewContent = ref<string | null>(null);
const isLoadingPreview = ref(false);

const activeLocationId = computed(() => locationsStore.selectedLocationId);
const activeLocation = computed(() => locationsStore.selectedLocation);
const compactPane = computed(() =>
  route.params.skillId
    ? "detail"
    : libraryStore.items.length > 0
      ? "list"
      : "main"
);

function showSkillList() {
  libraryStore.selectSkill(null);
  router.push("/skills");
}

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
  libraryStore.filterKind = "skill";
  libraryStore.fetchItems();
});

/**
 * Keep the selection in step with the URL. The route has always accepted a
 * skill id, but only a click ever set the selection — so opening
 * `/skills/<id>` directly, or coming back through history, landed on the empty
 * state with the id sitting in the address bar.
 */
watch(
  () => route.params.skillId as string | undefined,
  (id) => {
    if (id && id !== libraryStore.selectedSkillId) libraryStore.selectSkill(id);
  },
  { immediate: true }
);
</script>

<template>
  <SplitPaneLayout
    :show-inspector="!!(route.params.skillId && libraryStore.selectedDetail)"
    :compact-pane="compactPane"
    back-label="Skills"
    @back="showSkillList"
  >
    <template #sidebar>
      <div class="library-sidebar">
        <LibraryTabs />
        <div class="sidebar-controls">
          <SSearchInput
            v-model="libraryStore.searchQuery"
            data-local-filter
            placeholder="Filter skills"
            compact
          />
          <div class="status-controls">
            <label v-if="libraryStore.unusedCount > 0" class="unused-filter">
              <input
                type="checkbox"
                :checked="libraryStore.filterUnused"
                @change="libraryStore.filterUnused = !libraryStore.filterUnused"
              />
              <span>Unused only</span>
              <SBadge variant="warning" compact>{{ libraryStore.unusedCount }} unused</SBadge>
            </label>
          </div>
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

              <!-- One state mark, not a rank of pills. A row can only be in
                   one condition worth flagging, and the worst one wins. -->
              <div class="row-meta">
                <span
                  v-if="item.validationIssues.some((i: any) => i.severity === 'error')"
                  class="mark mark-bad"
                  :title="item.validationIssues.filter((i: any) => i.severity === 'error').map((i: any) => i.message).join(', ')"
                >
                  <PanelIcon name="caution" :size="12" />
                  <span class="sr-only">Has an error</span>
                </span>
                <span
                  v-else-if="item.kind === 'set' && item.brokenSkillCount > 0"
                  class="mark mark-bad"
                  :title="`${item.brokenSkillCount} skill(s) in this set are missing from the library`"
                >
                  <PanelIcon name="broken" :size="12" />
                  <span class="sr-only">{{ item.brokenSkillCount }} skills missing</span>
                </span>
                <span
                  v-else-if="item.validationIssues.some((i: any) => i.severity === 'warning')"
                  class="mark mark-warn"
                  :title="item.validationIssues.filter((i: any) => i.severity === 'warning').map((i: any) => i.message).join(', ')"
                >
                  <PanelIcon name="caution" :size="12" />
                  <span class="sr-only">Has a warning</span>
                </span>

                <span v-if="item.archived" class="badge row-plate">Archived</span>
                <span v-if="item.kind === 'set'" class="badge row-plate">Set</span>

                <!-- Never run anywhere: the fact this screen exists to surface. -->
                <span
                  v-if="item.kind === 'skill' && item.isUnusedEverywhere"
                  class="row-unused"
                  title="No recorded run anywhere"
                >never run</span>
                <span v-else class="row-uses num" title="Runs in the last 30 days">
                  {{ item.useCount30d }}
                </span>
              </div>
            </div>
            <div v-if="item.kind === 'skill'" class="row-actions">
              <button
                class="preview-button"
                :class="{ active: previewSkillId === item.id }"
                :title="`Preview SKILL.md for ${item.name}`"
                :aria-label="`Preview SKILL.md for ${item.name}`"
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
                :title="`Assign ${item.name} to ${activeLocation?.label ?? 'active location'}`"
                :aria-label="`Assign ${item.name} to ${activeLocation?.label ?? 'active location'}`"
                @click.stop="quickAssign(item.id)"
              >
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <line x1="12" y1="5" x2="12" y2="19" />
                  <line x1="5" y1="12" x2="19" y2="12" />
                </svg>
              </button>
              <SBadge v-if="activeLocationId && isAssignedToActive(item.id)" variant="success" compact>
                Assigned
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
            <span class="list-empty-text">
              {{ libraryStore.searchQuery ? 'No items match your search' : 'No items found' }}
            </span>
            <button
              v-if="libraryStore.searchQuery"
              class="list-empty-clear"
              @click="libraryStore.searchQuery = ''"
            >Clear search</button>
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

.unused-filter {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  font-size: var(--text-xs);
  color: var(--text-secondary);
  cursor: pointer;
  user-select: none;
}

.status-controls {
  display: flex;
  align-items: flex-start;
  flex-wrap: wrap;
  gap: var(--space-2);
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
  position: relative;
  border-bottom: 1px solid var(--border-subtle);
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

.library-row.archived .row-name {
  color: var(--text-tertiary);
}

.row-main {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-3);
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
  font-size: var(--text-md);
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
  gap: var(--space-2);
  flex-shrink: 0;
}

.row-plate {
  font-size: 9px;
  padding: 1px var(--space-2) 0;
}

/* The state mark is a glyph, not a pill; the title says it in words and the
   screen-reader text says it again. */
.mark {
  display: inline-flex;
  align-items: center;
}

.mark-bad {
  color: var(--danger);
}

.mark-warn {
  color: var(--warning);
}

/* Runs in the last 30 days, right-aligned so the column reads down the list. */
.row-uses {
  font-size: var(--text-md);
  color: var(--text-secondary);
  min-width: 2.5ch;
  text-align: right;
}

/* "Never run" is a word, not a zero: a zero here would read as a measurement
   when it is really the absence of one. */
.row-unused {
  font-weight: var(--weight-medium);
  font-size: var(--text-sm);
  color: var(--warning);
  white-space: nowrap;
}

/* Actions sit over the right of the row on hover. A permanent second line of
   buttons per row cost more vertical space than the content it served. */
.row-actions {
  position: absolute;
  right: var(--space-3);
  top: 50%;
  transform: translateY(-50%);
  display: flex;
  align-items: center;
  gap: var(--space-1);
  padding: 2px;
  background: var(--surface-panel);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-sm);
  opacity: 0;
  pointer-events: none;
}

.library-row:hover .row-actions,
.library-row:focus-within .row-actions {
  opacity: 1;
  pointer-events: auto;
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

.preview-button:focus-visible,
.quick-assign-button:focus-visible {
  outline: 2px solid var(--accent);
  outline-offset: 2px;
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

/* Tags were dropped from the row: a third line of chips inside a source
   list is noise, and the search field already matches on them. */

.list-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
  padding: var(--space-6);
}

.list-empty-text {
  font-size: var(--text-sm);
  color: var(--text-tertiary);
}

.list-empty-clear {
  font-family: var(--font-sans);
  font-size: var(--text-xs);
  color: var(--accent);
  background: none;
  border: none;
  cursor: pointer;
  padding: 0;
}

.list-empty-clear:hover {
  text-decoration: underline;
}
</style>

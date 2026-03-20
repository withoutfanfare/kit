<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useSetsStore } from "@/stores/setsStore";
import { useLocationsStore } from "@/stores/locationsStore";
import { useRouter } from "vue-router";
import SplitPaneLayout from "@/components/layout/SplitPaneLayout.vue";
import EmptyState from "@/components/layout/EmptyState.vue";
import SetRow from "@/components/domain/SetRow.vue";
import SetInspector from "@/components/domain/SetInspector.vue";
import SearchField from "@/components/base/SearchField.vue";
import SegmentedControl from "@/components/base/SegmentedControl.vue";
import PrimaryButton from "@/components/base/PrimaryButton.vue";
import SecondaryButton from "@/components/base/SecondaryButton.vue";
import type { SetScope, SetSummary } from "@/types";
import { setKeyFromSummary } from "@/utils/setKey";

const setsStore = useSetsStore();
const locationsStore = useLocationsStore();
const router = useRouter();

const scopeOptions = [
  { label: "All", value: "all" },
  { label: "Global", value: "global" },
  { label: "Project", value: "project" },
];

// New set dialog state
const showNewSetDialog = ref(false);
const newSetName = ref("");
const newSetScope = ref<SetScope>("global");
const newSetDescription = ref("");
const newSetOwnerLocationId = ref<string | undefined>(undefined);
const isCreating = ref(false);

function selectSet(set: SetSummary) {
  const key = setKeyFromSummary(set);
  setsStore.selectSet(key);
  router.push(`/sets/${encodeURIComponent(key)}`);
}

function openNewSetDialog() {
  newSetName.value = "";
  newSetScope.value = "global";
  newSetDescription.value = "";
  newSetOwnerLocationId.value = undefined;
  showNewSetDialog.value = true;
}

async function handleCreateSet() {
  if (!newSetName.value.trim()) return;
  if (newSetScope.value === "project" && !newSetOwnerLocationId.value) return;
  isCreating.value = true;
  try {
    await setsStore.createSet(
      newSetName.value.trim(),
      newSetScope.value,
      newSetScope.value === "project" ? newSetOwnerLocationId.value : undefined,
      newSetDescription.value.trim() || undefined
    );
    showNewSetDialog.value = false;
    if (setsStore.selectedSetKey) {
      router.push(`/sets/${encodeURIComponent(setsStore.selectedSetKey)}`);
    }
  } finally {
    isCreating.value = false;
  }
}

onMounted(() => {
  setsStore.fetchSets();
  locationsStore.fetchList();
});
</script>

<template>
  <SplitPaneLayout :show-inspector="!!setsStore.selectedDetail">
    <template #sidebar>
      <div class="sets-sidebar">
        <div class="sidebar-controls">
          <SearchField
            v-model="setsStore.searchQuery"
            placeholder="Search sets..."
            compact
          />
          <SegmentedControl
            v-model="setsStore.scopeFilter"
            :options="scopeOptions"
          />
        </div>
        <div class="sidebar-items">
          <SetRow
            v-for="item in setsStore.filteredItems"
            :key="setKeyFromSummary(item)"
            :set="item"
            :selected="setKeyFromSummary(item) === setsStore.selectedSetKey"
            @click="selectSet(item)"
          />
          <div
            v-if="setsStore.filteredItems.length === 0 && !setsStore.isLoading"
            class="list-empty"
          >
            <span class="list-empty-text">No sets found</span>
          </div>
        </div>
        <div class="sidebar-footer">
          <PrimaryButton label="New Set" @click="openNewSetDialog" />
        </div>
      </div>
    </template>
    <template #main>
      <router-view v-if="setsStore.selectedSetKey" />
      <EmptyState
        v-else-if="setsStore.items.length === 0 && !setsStore.isLoading"
        title="No sets yet"
        description="Create a set to group related skills together and assign them to locations."
        action-label="Create Set"
        @action="openNewSetDialog"
      />
      <EmptyState
        v-else-if="!setsStore.isLoading"
        title="Select a set"
        description="Choose a set from the sidebar to view its skills and manage assignments."
      />
    </template>
    <template #inspector>
      <SetInspector
        v-if="setsStore.selectedDetail"
        :detail="setsStore.selectedDetail"
      />
    </template>
  </SplitPaneLayout>

  <!-- New Set Dialog -->
  <Teleport to="body">
    <Transition name="dialog">
      <div
        v-if="showNewSetDialog"
        class="dialog-backdrop"
        @click.self="showNewSetDialog = false"
      >
        <div class="dialog" role="dialog" aria-modal="true">
          <h3 class="dialog-title">New Set</h3>
          <div class="dialog-form">
            <div class="form-field">
              <label class="form-label" for="set-name">Name</label>
              <input
                id="set-name"
                v-model="newSetName"
                type="text"
                class="form-input"
                placeholder="e.g. code-review-tools"
                @keydown.enter="handleCreateSet"
              />
            </div>
            <div class="form-field">
              <label class="form-label">Scope</label>
              <SegmentedControl
                v-model="(newSetScope as string)"
                :options="[
                  { label: 'Global', value: 'global' },
                  { label: 'Project', value: 'project' },
                ]"
              />
            </div>
            <div v-if="newSetScope === 'project'" class="form-field">
              <label class="form-label" for="set-owner">Location</label>
              <select
                id="set-owner"
                v-model="newSetOwnerLocationId"
                class="form-input"
              >
                <option :value="undefined" disabled>Select a location...</option>
                <option
                  v-for="loc in locationsStore.locationList"
                  :key="loc.id"
                  :value="loc.id"
                >
                  {{ loc.label }}
                </option>
              </select>
            </div>
            <div class="form-field">
              <label class="form-label" for="set-description">Description (optional)</label>
              <input
                id="set-description"
                v-model="newSetDescription"
                type="text"
                class="form-input"
                placeholder="Brief description of this set"
                @keydown.enter="handleCreateSet"
              />
            </div>
          </div>
          <div class="dialog-actions">
            <SecondaryButton label="Cancel" @click="showNewSetDialog = false" />
            <PrimaryButton
              label="Create"
              :disabled="!newSetName.trim() || (newSetScope === 'project' && !newSetOwnerLocationId)"
              :loading="isCreating"
              @click="handleCreateSet"
            />
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.sets-sidebar {
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

.sidebar-items {
  flex: 1;
  overflow-y: auto;
  padding: var(--space-1);
}

.sidebar-footer {
  padding: var(--space-3);
  border-top: 1px solid var(--border-subtle);
  flex-shrink: 0;
}

.sidebar-footer :deep(.primary-button) {
  width: 100%;
  justify-content: center;
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

/* New Set Dialog */
.dialog-backdrop {
  position: fixed;
  inset: 0;
  z-index: 300;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.25);
}

.dialog {
  width: 100%;
  max-width: 400px;
  background: var(--surface-panel);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-lg);
  padding: var(--space-5);
}

.dialog-title {
  font-family: var(--font-sans);
  font-size: var(--text-md);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  margin: 0;
}

.dialog-form {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
  margin-top: var(--space-4);
}

.form-field {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.form-label {
  font-family: var(--font-sans);
  font-size: var(--text-xs);
  font-weight: var(--weight-medium);
  color: var(--text-secondary);
}

.form-input {
  width: 100%;
  height: 30px;
  padding: 0 var(--space-2);
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--text-primary);
  background: var(--surface-hover);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-sm);
  outline: none;
  transition: border-color var(--duration-fast) var(--ease-default),
    box-shadow var(--duration-fast) var(--ease-default);
}

.form-input::placeholder {
  color: var(--text-tertiary);
}

.form-input:focus {
  border-color: var(--accent);
  box-shadow: 0 0 0 2px var(--accent-subtle);
}

.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-2);
  margin-top: var(--space-5);
}

/* Dialog transition */
.dialog-enter-active {
  transition: opacity var(--duration-fast) var(--ease-out);
}

.dialog-leave-active {
  transition: opacity var(--duration-fast) var(--ease-default);
}

.dialog-enter-from,
.dialog-leave-to {
  opacity: 0;
}

.dialog-enter-active .dialog {
  animation: scale-in var(--duration-normal) var(--ease-out);
}

@keyframes scale-in {
  from {
    transform: scale(0.96);
  }
  to {
    transform: scale(1);
  }
}
</style>

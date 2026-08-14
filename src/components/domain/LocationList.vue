<script setup lang="ts">
import { onMounted } from "vue";
import { useLocationsStore } from "@/stores/locationsStore";
import { useAppStore } from "@/stores/appStore";
import { useRouter } from "vue-router";
import { open } from "@tauri-apps/plugin-dialog";
import LocationRow from "@/components/domain/LocationRow.vue";
import { SButton } from "@stuntrocket/ui";

const locationsStore = useLocationsStore();
const appStore = useAppStore();
const router = useRouter();

onMounted(() => locationsStore.refreshDiscovered());

async function addDiscovered(path: string, label: string) {
  await locationsStore.addLocation(path, label);
  await locationsStore.refreshDiscovered();
  appStore.toast(`Added ${label}`, "success");
}

async function clearMissing() {
  // Counted from what actually went, not from what was offered: a directory
  // that reappeared in the meantime is kept, and the message should say so.
  const before = locationsStore.locationList.length;
  await locationsStore.removeMissingLocations();
  const count = before - locationsStore.locationList.length;

  appStore.toast(
    count === 0
      ? "Nothing to forget — those locations are back."
      : `Forgot ${count} location${count === 1 ? "" : "s"}. No files were touched.`,
    count === 0 ? "info" : "success"
  );
}

function selectLocation(id: string) {
  locationsStore.selectLocation(id);
  router.push(`/locations/${id}`);
}

async function addLocation() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: "Choose a project location",
  });
  if (selected && typeof selected === "string") {
    await locationsStore.addLocation(selected);
    if (locationsStore.selectedLocationId) {
      router.push(`/locations/${locationsStore.selectedLocationId}`);
    }
  }
}
</script>

<template>
  <div class="location-list">
    <!-- A schedule names its columns. The count on the right is otherwise a
         bare number, and a bare number in a list is a guess. -->
    <div class="list-header">
      <span class="label">Locations</span>
      <span class="label list-col">Skills</span>
    </div>
    <div class="list-items">
      <LocationRow
        v-for="loc in locationsStore.locationList"
        :key="loc.id"
        :location="loc"
        :selected="loc.id === locationsStore.selectedLocationId"
        @click="selectLocation(loc.id)"
      />
      <div v-if="locationsStore.locationList.length === 0" class="list-empty">
        <span class="list-empty-text">No locations added yet</span>
      </div>

      <!-- Saved locations whose folder has gone. Kit only forgets its record. -->
      <div v-if="locationsStore.missingLocations.length" class="list-note">
        <span class="note-text">
          {{ locationsStore.missingLocations.length }}
          {{ locationsStore.missingLocations.length === 1 ? "location has" : "locations have" }}
          moved or been deleted.
        </span>
        <SButton size="sm" variant="secondary" @click="clearMissing">Forget them</SButton>
      </div>

      <!-- Projects on disk keeping skills that Kit doesn't track yet. -->
      <div v-if="locationsStore.discovered.length" class="list-discovered">
        <span class="discovered-title">Found nearby</span>
        <button
          v-for="found in locationsStore.discovered"
          :key="found.path"
          class="discovered-row"
          @click="addDiscovered(found.path, found.label)"
        >
          <span class="discovered-label">{{ found.label }}</span>
          <span class="discovered-count">{{ found.skillCount }} skills · Add</span>
        </button>
      </div>
    </div>
    <div v-if="locationsStore.locationList.length > 0" class="list-footer">
      <SButton variant="secondary" @click="addLocation">Add location</SButton>
    </div>
  </div>
</template>

<style scoped>
.location-list {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.list-header {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: var(--space-3);
  padding: var(--space-6) var(--space-5) var(--space-4);
  border-bottom: 1px solid var(--border-default);
  margin: 0 var(--space-1);
  flex-shrink: 0;
}

.list-col {
  flex-shrink: 0;
}

.list-items {
  flex: 1;
  overflow-y: auto;
  padding: 0 var(--space-2) var(--space-4);
}

.list-note {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-4) var(--space-5);
  margin: var(--space-2) var(--space-1) 0;
  border-radius: var(--radius-sm);
  background: var(--surface-hover);
}

.note-text {
  flex: 1;
  font-size: var(--text-xs);
  color: var(--text-secondary);
}

.list-discovered {
  margin: var(--space-5) var(--space-2) 0;
}

.discovered-title {
  display: block;
  padding: 0 var(--space-4) var(--space-2);
  font-size: var(--text-sm);
  font-weight: var(--weight-semibold);
  color: var(--text-tertiary);
}

.discovered-row {
  display: flex;
  width: 100%;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-3) var(--space-4);
  border: none;
  border-radius: var(--radius-sm);
  background: transparent;
  cursor: default;
  text-align: left;
}

.discovered-row:hover {
  background: var(--surface-hover);
}

.discovered-label {
  flex: 1;
  min-width: 0;
  font-size: var(--text-sm);
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.discovered-count {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
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

.list-footer {
  flex-shrink: 0;
  padding: var(--space-5);
  border-top: 1px solid var(--border-subtle);
  display: flex;
}

.list-footer :deep(button) {
  width: 100%;
}
</style>

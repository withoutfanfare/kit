<script setup lang="ts">
import { useLocationsStore } from "@/stores/locationsStore";
import { useRouter } from "vue-router";
import { open } from "@tauri-apps/plugin-dialog";
import LocationRow from "@/components/domain/LocationRow.vue";
import PrimaryButton from "@/components/base/PrimaryButton.vue";

const locationsStore = useLocationsStore();
const router = useRouter();

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
    <div class="list-header">
      <span class="list-title">Locations</span>
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
    </div>
    <div class="list-footer">
      <PrimaryButton label="Add Location" @click="addLocation" />
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
  padding: var(--space-3) var(--space-3) var(--space-2);
  flex-shrink: 0;
}

.list-title {
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.list-items {
  flex: 1;
  overflow-y: auto;
  padding: 0 var(--space-1);
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
  padding: var(--space-3);
  border-top: 1px solid var(--border-subtle);
  display: flex;
}

.list-footer :deep(.primary-button) {
  width: 100%;
}
</style>

import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type {
  SavedLocationSummary,
  LocationDetail,
  LocationId,
} from "@/types";

export const useLocationsStore = defineStore("locations", () => {
  const locationList = ref<SavedLocationSummary[]>([]);
  const selectedLocationId = ref<LocationId | null>(null);
  const detailCache = ref<Record<LocationId, LocationDetail>>({});
  const isLoadingDetail = ref(false);

  const selectedLocation = computed(() =>
    locationList.value.find((l) => l.id === selectedLocationId.value) ?? null
  );

  const selectedDetail = computed(() =>
    selectedLocationId.value
      ? detailCache.value[selectedLocationId.value] ?? null
      : null
  );

  async function fetchList() {
    try {
      locationList.value = await invoke<SavedLocationSummary[]>("list_locations");
    } catch {
      // Retain existing list on failure — bootstrap data is the initial fallback
    }
  }

  async function fetchDetail(id: LocationId) {
    isLoadingDetail.value = true;
    try {
      const detail = await invoke<LocationDetail>("get_location_detail", {
        id,
      });
      detailCache.value[id] = detail;
    } finally {
      isLoadingDetail.value = false;
    }
  }

  async function addLocation(path: string, label?: string) {
    const loc = await invoke<SavedLocationSummary>("add_location", {
      label: label ?? "",
      path,
    });
    locationList.value.push(loc);
    selectedLocationId.value = loc.id;
    await fetchDetail(loc.id);
  }

  async function updateLocation(
    id: LocationId,
    updates: { label?: string; notes?: string | null }
  ) {
    const loc = await invoke<SavedLocationSummary>("update_location", {
      id,
      label: updates.label,
      notes: updates.notes,
    });
    const idx = locationList.value.findIndex((l) => l.id === id);
    if (idx >= 0) locationList.value[idx] = loc;
    if (detailCache.value[id]) {
      if (updates.label !== undefined)
        detailCache.value[id].label = updates.label;
      if (updates.notes !== undefined)
        detailCache.value[id].notes = updates.notes ?? null;
    }
  }

  async function removeLocation(id: LocationId) {
    const list = await invoke<SavedLocationSummary[]>("remove_location", {
      id,
    });
    locationList.value = list;
    delete detailCache.value[id];
    if (selectedLocationId.value === id) {
      selectedLocationId.value = locationList.value[0]?.id ?? null;
    }
  }

  async function syncLocation(id: LocationId) {
    const detail = await invoke<LocationDetail>("sync_location", {
      id,
    });
    detailCache.value[id] = detail;
    const idx = locationList.value.findIndex((l) => l.id === id);
    if (idx >= 0) {
      locationList.value[idx] = {
        ...locationList.value[idx],
        issueCount: detail.issues.length,
        installedSkillCount: detail.skills.filter(
          (s) => s.linkState === "linked" || s.linkState === "local_only"
        ).length,
        installedSetCount: detail.sets.length,
        lastSyncedAt: new Date().toISOString(),
      };
    }
  }

  function selectLocation(id: LocationId | null) {
    selectedLocationId.value = id;
    if (id && !detailCache.value[id]) {
      fetchDetail(id);
    }
  }

  return {
    locationList,
    selectedLocationId,
    detailCache,
    isLoadingDetail,
    selectedLocation,
    selectedDetail,
    fetchList,
    fetchDetail,
    addLocation,
    updateLocation,
    removeLocation,
    syncLocation,
    selectLocation,
  };
});

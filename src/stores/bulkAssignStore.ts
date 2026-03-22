import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { LocationId, SkillId, BulkAssignResult } from "@/types";
import { useLocationsStore } from "./locationsStore";
import { useLibraryStore } from "./libraryStore";
import { useAppStore } from "./appStore";

export const useBulkAssignStore = defineStore("bulkAssign", () => {
  const isOpen = ref(false);
  const skillIds = ref<SkillId[]>([]);
  const selectedLocationIds = ref<Set<LocationId>>(new Set());
  const isApplying = ref(false);
  const results = ref<BulkAssignResult[]>([]);

  const hasSelections = computed(() => selectedLocationIds.value.size > 0);

  function open(ids: SkillId[]) {
    skillIds.value = ids;
    selectedLocationIds.value = new Set();
    results.value = [];
    isOpen.value = true;
  }

  function close() {
    isOpen.value = false;
    skillIds.value = [];
    selectedLocationIds.value = new Set();
    results.value = [];
  }

  function toggleLocation(id: LocationId) {
    if (selectedLocationIds.value.has(id)) {
      selectedLocationIds.value.delete(id);
    } else {
      selectedLocationIds.value.add(id);
    }
    selectedLocationIds.value = new Set(selectedLocationIds.value);
  }

  function selectAll() {
    const locations = useLocationsStore();
    selectedLocationIds.value = new Set(locations.locationList.map((l) => l.id));
  }

  function deselectAll() {
    selectedLocationIds.value = new Set();
  }

  async function apply() {
    if (!hasSelections.value || skillIds.value.length === 0) return;
    isApplying.value = true;
    try {
      results.value = await invoke<BulkAssignResult[]>("bulk_assign_skills", {
        locationIds: [...selectedLocationIds.value],
        skillIds: skillIds.value,
      });

      const succeeded = results.value.filter((r) => r.success).length;
      const failed = results.value.filter((r) => !r.success).length;
      const app = useAppStore();

      if (failed === 0) {
        app.toast(
          `Assigned ${skillIds.value.length} skill${skillIds.value.length === 1 ? "" : "s"} to ${succeeded} location${succeeded === 1 ? "" : "s"}`,
          "success",
        );
        // Refresh stores
        await useLocationsStore().fetchList();
        await useLibraryStore().fetchItems();
        close();
      } else {
        app.toast(
          `${succeeded} succeeded, ${failed} failed`,
          failed === results.value.length ? "error" : "info",
        );
      }
    } catch {
      useAppStore().toast("Bulk assignment failed", "error");
    } finally {
      isApplying.value = false;
    }
  }

  return {
    isOpen,
    skillIds,
    selectedLocationIds,
    isApplying,
    results,
    hasSelections,
    open,
    close,
    toggleLocation,
    selectAll,
    deselectAll,
    apply,
  };
});

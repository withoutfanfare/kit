import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type {
  LocationId,
  SkillId,
  SetId,
  AssignmentPreview,
  LocationDetail,
} from "@/types";
import { useLocationsStore } from "./locationsStore";
import { useAppStore } from "./appStore";

export const useAssignmentStore = defineStore("assignment", () => {
  const isOpen = ref(false);
  const locationId = ref<LocationId | null>(null);
  const selectedSkillIds = ref<Set<SkillId>>(new Set());
  const selectedSetIds = ref<Set<SetId>>(new Set());
  const removeSkillIds = ref<Set<SkillId>>(new Set());
  const preview = ref<AssignmentPreview | null>(null);
  const previewError = ref<string | null>(null);
  const isPreviewLoading = ref(false);
  const isApplying = ref(false);

  const hasSelections = computed(
    () =>
      selectedSkillIds.value.size > 0 ||
      selectedSetIds.value.size > 0 ||
      removeSkillIds.value.size > 0
  );

  function open(forLocationId: LocationId) {
    locationId.value = forLocationId;
    selectedSkillIds.value = new Set();
    selectedSetIds.value = new Set();
    removeSkillIds.value = new Set();
    preview.value = null;
    isOpen.value = true;
  }

  function close() {
    isOpen.value = false;
    locationId.value = null;
    preview.value = null;
  }

  function toggleSkill(id: SkillId) {
    if (selectedSkillIds.value.has(id)) {
      selectedSkillIds.value.delete(id);
    } else {
      selectedSkillIds.value.add(id);
    }
    selectedSkillIds.value = new Set(selectedSkillIds.value);
    fetchPreview();
  }

  function toggleSet(id: SetId) {
    if (selectedSetIds.value.has(id)) {
      selectedSetIds.value.delete(id);
    } else {
      selectedSetIds.value.add(id);
    }
    selectedSetIds.value = new Set(selectedSetIds.value);
    fetchPreview();
  }

  function toggleRemoveSkill(id: SkillId) {
    if (removeSkillIds.value.has(id)) {
      removeSkillIds.value.delete(id);
    } else {
      removeSkillIds.value.add(id);
    }
    removeSkillIds.value = new Set(removeSkillIds.value);
    fetchPreview();
  }

  async function fetchPreview() {
    if (!locationId.value || !hasSelections.value) {
      preview.value = null;
      previewError.value = null;
      return;
    }
    isPreviewLoading.value = true;
    previewError.value = null;
    try {
      preview.value = await invoke<AssignmentPreview>("preview_assignment", {
        locationId: locationId.value,
        skillIdsToAdd: [...selectedSkillIds.value],
        setIdsToAdd: [...selectedSetIds.value],
        skillIdsToRemove: [...removeSkillIds.value],
        setIdsToRemove: [],
      });
    } catch (err) {
      preview.value = null;
      previewError.value = err instanceof Error ? err.message : "Failed to generate preview";
    } finally {
      isPreviewLoading.value = false;
    }
  }

  async function apply() {
    if (!locationId.value) return;
    isApplying.value = true;
    try {
      const detail = await invoke<LocationDetail>("apply_assignment", {
        locationId: locationId.value,
        skillIdsToAdd: [...selectedSkillIds.value],
        setIdsToAdd: [...selectedSetIds.value],
        skillIdsToRemove: [...removeSkillIds.value],
        setIdsToRemove: [],
        updateManifest: true,
      });

      const locations = useLocationsStore();
      locations.detailCache[locationId.value] = detail;
      const idx = locations.locationList.findIndex(
        (l) => l.id === locationId.value
      );
      if (idx >= 0) {
        locations.locationList[idx] = {
          ...locations.locationList[idx],
          issueCount: detail.issues.length,
          installedSkillCount: detail.skills.filter(
            (s) => s.linkState === "linked" || s.linkState === "local_only"
          ).length,
          installedSetCount: detail.sets.length,
          lastSyncedAt: new Date().toISOString(),
        };
      }

      const added = selectedSkillIds.value.size + selectedSetIds.value.size;
      const removed = removeSkillIds.value.size;
      const parts: string[] = [];
      if (added > 0) parts.push(`${added} added`);
      if (removed > 0) parts.push(`${removed} removed`);
      useAppStore().toast(`Skills updated — ${parts.join(", ")}`, "success");

      close();
    } catch {
      useAppStore().toast("Failed to apply changes", "error");
    } finally {
      isApplying.value = false;
    }
  }

  return {
    isOpen,
    locationId,
    selectedSkillIds,
    selectedSetIds,
    removeSkillIds,
    preview,
    previewError,
    isPreviewLoading,
    isApplying,
    hasSelections,
    open,
    close,
    toggleSkill,
    toggleSet,
    toggleRemoveSkill,
    apply,
  };
});

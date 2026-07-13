import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { BrokenLinkRemovalPreview, HealthCheckResult, LocationId } from "@/types";
import { useAppStore } from "./appStore";

export type HealthSeverityFilter = "all" | "healthy" | "warning" | "error";

export const useHealthStore = defineStore("health", () => {
  const result = ref<HealthCheckResult | null>(null);
  const isLoading = ref(false);
  const isApplying = ref(false);
  const filterLocationId = ref<LocationId | null>(null);
  const severityFilter = ref<HealthSeverityFilter>("all");
  const selectedLocationIds = ref<Set<LocationId>>(new Set());
  const removalPreview = ref<BrokenLinkRemovalPreview[] | null>(null);

  const groupedIssues = computed(() => {
    const current = result.value;
    if (!current) return [];

    return current.locations.flatMap((location) => {
      if (filterLocationId.value && location.locationId !== filterLocationId.value) {
        return [];
      }

      const isHealthy =
        location.errorCount === 0 &&
        location.warningCount === 0 &&
        location.infoCount === 0;
      if (severityFilter.value === "healthy" && !isHealthy) return [];
      if (severityFilter.value === "warning" && location.warningCount === 0) return [];
      if (severityFilter.value === "error" && location.errorCount === 0) return [];

      const issues = current.issues.filter((issue) => {
        if (issue.locationId !== location.locationId) return false;
        if (severityFilter.value === "warning") return issue.severity === "warning";
        if (severityFilter.value === "error") return issue.severity === "error";
        return severityFilter.value !== "healthy";
      });

      if (severityFilter.value === "all" && issues.length === 0) return [];
      return [{ location, issues }];
    });
  });

  async function runCheck() {
    isLoading.value = true;
    try {
      result.value = await invoke<HealthCheckResult>("run_health_check");
    } finally {
      isLoading.value = false;
    }
  }

  function setFilter(locationId: LocationId | null) {
    filterLocationId.value = locationId;
  }

  function setSeverityFilter(filter: Exclude<HealthSeverityFilter, "all">) {
    severityFilter.value = severityFilter.value === filter ? "all" : filter;
  }

  function isSelected(locationId: LocationId) {
    return selectedLocationIds.value.has(locationId);
  }

  function toggleLocation(locationId: LocationId) {
    const selection = new Set(selectedLocationIds.value);
    if (selection.has(locationId)) {
      selection.delete(locationId);
    } else {
      selection.add(locationId);
    }
    selectedLocationIds.value = selection;
  }

  function clearSelection() {
    selectedLocationIds.value = new Set();
  }

  async function previewRemoval(locationIds = [...selectedLocationIds.value]) {
    if (locationIds.length === 0) return false;
    try {
      selectedLocationIds.value = new Set(locationIds);
      removalPreview.value = await invoke<BrokenLinkRemovalPreview[]>(
        "preview_broken_link_removal",
        { locationIds },
      );
      if (removalPreview.value.length === 0) {
        useAppStore().toast("No broken links remain", "info");
        clearSelection();
        return false;
      }
      return true;
    } catch {
      useAppStore().toast("Failed to preview broken links", "error");
      return false;
    }
  }

  function clearPreview() {
    removalPreview.value = null;
  }

  async function applyRemoval() {
    const locationIds = [...selectedLocationIds.value];
    if (locationIds.length === 0 || isApplying.value) return;

    isApplying.value = true;
    try {
      const currentPreview = await invoke<BrokenLinkRemovalPreview[]>(
        "preview_broken_link_removal",
        { locationIds },
      );
      const removedCount = currentPreview.reduce(
        (count, location) => count + location.paths.length,
        0,
      );
      result.value = await invoke<HealthCheckResult>("remove_broken_links", {
        locationIds,
      });
      clearSelection();
      clearPreview();
      useAppStore().toast(
        `Removed ${removedCount} broken link${removedCount === 1 ? "" : "s"}`,
        "success",
      );
    } catch {
      useAppStore().toast("Failed to remove broken links", "error");
    } finally {
      isApplying.value = false;
    }
  }

  return {
    result,
    isLoading,
    isApplying,
    filterLocationId,
    severityFilter,
    selectedLocationIds,
    removalPreview,
    groupedIssues,
    runCheck,
    setFilter,
    setSeverityFilter,
    isSelected,
    toggleLocation,
    clearSelection,
    previewRemoval,
    clearPreview,
    applyRemoval,
  };
});

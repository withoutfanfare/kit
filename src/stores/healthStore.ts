import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { HealthCheckResult, LocationId } from "@/types";

export const useHealthStore = defineStore("health", () => {
  const result = ref<HealthCheckResult | null>(null);
  const isLoading = ref(false);
  const filterLocationId = ref<LocationId | null>(null);

  const filteredIssues = computed(() => {
    if (!result.value) return [];
    if (!filterLocationId.value) return result.value.issues;
    return result.value.issues.filter(
      (i) => i.locationId === filterLocationId.value
    );
  });

  const errorIssues = computed(() =>
    filteredIssues.value.filter((i) => i.severity === "error")
  );

  const warningIssues = computed(() =>
    filteredIssues.value.filter((i) => i.severity === "warning")
  );

  const infoIssues = computed(() =>
    filteredIssues.value.filter((i) => i.severity === "info")
  );

  async function runCheck() {
    isLoading.value = true;
    try {
      result.value = await invoke<HealthCheckResult>("run_health_check");
    } finally {
      isLoading.value = false;
    }
  }

  async function fixBrokenLinks(locationId: LocationId) {
    isLoading.value = true;
    try {
      result.value = await invoke<HealthCheckResult>("fix_broken_links", {
        locationId,
      });
    } finally {
      isLoading.value = false;
    }
  }

  function setFilter(locationId: LocationId | null) {
    filterLocationId.value = locationId;
  }

  return {
    result,
    isLoading,
    filterLocationId,
    filteredIssues,
    errorIssues,
    warningIssues,
    infoIssues,
    runCheck,
    fixBrokenLinks,
    setFilter,
  };
});

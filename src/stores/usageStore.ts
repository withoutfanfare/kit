import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { UsageSummary } from "@/types";

export const useUsageStore = defineStore("usage", () => {
  const summary = ref<UsageSummary | null>(null);
  const isLoading = ref(false);

  async function fetchSummary() {
    isLoading.value = true;
    try {
      summary.value = await invoke<UsageSummary>("get_usage_summary");
    } finally {
      isLoading.value = false;
    }
  }

  return {
    summary,
    isLoading,
    fetchSummary,
  };
});

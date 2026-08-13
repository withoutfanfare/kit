import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { UsageReport, UsageSummary } from "@/types";

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

  const report = ref<UsageReport | null>(null);

  async function fetchReport() {
    isLoading.value = true;
    try {
      report.value = await invoke<UsageReport>("get_usage_report");
    } finally {
      isLoading.value = false;
    }
  }

  return {
    summary,
    report,
    isLoading,
    fetchSummary,
    fetchReport,
  };
});

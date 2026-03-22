import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { ChangelogEntry } from "@/types";

export const useChangelogStore = defineStore("changelog", () => {
  const entries = ref<ChangelogEntry[]>([]);
  const isLoading = ref(false);
  const filterDays = ref<number | null>(null);
  const searchQuery = ref("");

  const filteredEntries = computed(() => {
    if (!searchQuery.value.trim()) return entries.value;
    const q = searchQuery.value.trim().toLowerCase();
    return entries.value.filter((e) => e.name.toLowerCase().includes(q));
  });

  async function fetchEntries() {
    isLoading.value = true;
    try {
      entries.value = await invoke<ChangelogEntry[]>("get_skill_changelog", {
        days: filterDays.value,
      });
    } finally {
      isLoading.value = false;
    }
  }

  return {
    entries,
    isLoading,
    filterDays,
    searchQuery,
    filteredEntries,
    fetchEntries,
  };
});

import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { LibraryListItem, SkillDetail, SkillId } from "@/types";
import { useSkillPeekStore } from "./skillPeekStore";

export const useLibraryStore = defineStore("library", () => {
  const items = ref<LibraryListItem[]>([]);
  const selectedSkillId = ref<SkillId | null>(null);
  const skillDetailCache = ref<Record<SkillId, SkillDetail>>({});
  const isLoading = ref(false);
  const searchQuery = ref("");
  const filterArchived = ref(false);
  const filterKind = ref<"all" | "skill" | "set">("all");
  const totalSkills = ref(0);
  const totalSets = ref(0);
  const detailError = ref<string | null>(null);

  const filteredItems = computed(() => {
    let result = items.value;
    if (!filterArchived.value) {
      result = result.filter((i) => !i.archived);
    }
    if (filterKind.value !== "all") {
      result = result.filter((i) => i.kind === filterKind.value);
    }
    if (searchQuery.value.trim()) {
      const q = searchQuery.value.trim().toLowerCase();
      result = result.filter(
        (i) =>
          i.name.toLowerCase().includes(q) ||
          (i.summary && i.summary.toLowerCase().includes(q))
      );
    }
    return result;
  });

  const selectedDetail = computed(() =>
    selectedSkillId.value
      ? skillDetailCache.value[selectedSkillId.value] ?? null
      : null
  );

  async function fetchItems() {
    isLoading.value = true;
    try {
      items.value = await invoke<LibraryListItem[]>("list_library_items");
      useSkillPeekStore().clearCache();
    } finally {
      isLoading.value = false;
    }
  }

  async function fetchSkillDetail(id: SkillId) {
    detailError.value = null;
    try {
      const detail = await invoke<SkillDetail>("get_skill_detail", {
        skillId: id,
      });
      skillDetailCache.value[id] = detail;
    } catch (err) {
      detailError.value = err instanceof Error ? err.message : "Failed to load skill detail";
    }
  }

  async function archiveSkill(id: SkillId) {
    const detail = await invoke<SkillDetail>("archive_skill", {
      skillId: id,
    });
    skillDetailCache.value[id] = detail;
    const item = items.value.find((i) => i.id === id);
    if (item) item.archived = true;
  }

  async function unarchiveSkill(id: SkillId) {
    const detail = await invoke<SkillDetail>("unarchive_skill", {
      skillId: id,
    });
    skillDetailCache.value[id] = detail;
    const item = items.value.find((i) => i.id === id);
    if (item) item.archived = false;
  }

  function selectSkill(id: SkillId | null) {
    selectedSkillId.value = id;
    if (id && !skillDetailCache.value[id]) {
      fetchSkillDetail(id);
    }
  }

  return {
    items,
    selectedSkillId,
    skillDetailCache,
    isLoading,
    searchQuery,
    filterArchived,
    filterKind,
    totalSkills,
    totalSets,
    detailError,
    filteredItems,
    selectedDetail,
    fetchItems,
    fetchSkillDetail,
    archiveSkill,
    unarchiveSkill,
    selectSkill,
  };
});

import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type {
  SetSummary,
  SetDetail,
  SetId,
  SetScope,
  LocationId,
} from "@/types";
import {
  type SetKey,
  makeSetKey,
  parseSetKey,
  setKeyFromSummary,
} from "@/utils/setKey";

export const useSetsStore = defineStore("sets", () => {
  const items = ref<SetSummary[]>([]);
  const selectedSetKey = ref<SetKey | null>(null);
  const detailCache = ref<Record<SetKey, SetDetail>>({});
  const isLoading = ref(false);
  const searchQuery = ref("");
  const scopeFilter = ref<"all" | "global" | "project">("all");

  const filteredItems = computed(() => {
    let result = items.value;
    if (scopeFilter.value !== "all") {
      result = result.filter((s) => s.scope === scopeFilter.value);
    }
    if (searchQuery.value.trim()) {
      const q = searchQuery.value.trim().toLowerCase();
      result = result.filter(
        (s) =>
          s.name.toLowerCase().includes(q) ||
          (s.description && s.description.toLowerCase().includes(q))
      );
    }
    return result;
  });

  const selectedDetail = computed(() =>
    selectedSetKey.value
      ? detailCache.value[selectedSetKey.value] ?? null
      : null
  );

  async function fetchSets() {
    isLoading.value = true;
    try {
      items.value = await invoke<SetSummary[]>("list_sets");
    } finally {
      isLoading.value = false;
    }
  }

  async function fetchSetDetail(
    id: SetId,
    scope: SetScope,
    ownerLocationId?: LocationId
  ) {
    const detail = await invoke<SetDetail>("get_set_detail", {
      setId: id,
      scope,
      ownerLocationId: ownerLocationId ?? null,
    });
    const key = makeSetKey(scope, ownerLocationId, id);
    detailCache.value[key] = detail;
  }

  async function createSet(
    name: string,
    scope: SetScope,
    ownerLocationId?: LocationId,
    description?: string
  ) {
    const summary = await invoke<SetSummary>("create_set", {
      name,
      scope,
      ownerLocationId: ownerLocationId ?? null,
      description: description ?? null,
    });
    items.value.push(summary);
    const key = setKeyFromSummary(summary);
    selectedSetKey.value = key;
    await fetchSetDetail(summary.id, summary.scope, summary.ownerLocationId ?? undefined);
  }

  async function updateSet(
    id: SetId,
    scope: SetScope,
    ownerLocationId: LocationId | undefined,
    updates: { name?: string; description?: string | null }
  ) {
    const detail = await invoke<SetDetail>("update_set", {
      setId: id,
      scope,
      ownerLocationId: ownerLocationId ?? null,
      name: updates.name,
      description: updates.description,
    });
    const key = makeSetKey(scope, ownerLocationId, id);
    detailCache.value[key] = detail;
    const idx = items.value.findIndex(
      (s) => setKeyFromSummary(s) === key
    );
    if (idx >= 0) {
      if (updates.name !== undefined) items.value[idx].name = updates.name;
      if (updates.description !== undefined)
        items.value[idx].description = updates.description ?? null;
    }
  }

  async function deleteSet(
    id: SetId,
    scope: SetScope,
    ownerLocationId?: LocationId
  ) {
    await invoke("delete_set", {
      setId: id,
      scope,
      ownerLocationId: ownerLocationId ?? null,
    });
    const key = makeSetKey(scope, ownerLocationId, id);
    items.value = items.value.filter(
      (s) => setKeyFromSummary(s) !== key
    );
    delete detailCache.value[key];
    if (selectedSetKey.value === key) {
      selectedSetKey.value = null;
    }
  }

  async function addSkillToSet(
    setId: SetId,
    skillId: string,
    scope: SetScope,
    ownerLocationId?: LocationId
  ) {
    const detail = await invoke<SetDetail>("add_skill_to_set", {
      setId,
      skillId,
      scope,
      ownerLocationId: ownerLocationId ?? null,
    });
    const key = makeSetKey(scope, ownerLocationId, setId);
    detailCache.value[key] = detail;
    const idx = items.value.findIndex(
      (s) => setKeyFromSummary(s) === key
    );
    if (idx >= 0) {
      items.value[idx].skillCount = detail.skills.length;
    }
  }

  async function removeSkillFromSet(
    setId: SetId,
    skillId: string,
    scope: SetScope,
    ownerLocationId?: LocationId
  ) {
    const detail = await invoke<SetDetail>("remove_skill_from_set", {
      setId,
      skillId,
      scope,
      ownerLocationId: ownerLocationId ?? null,
    });
    const key = makeSetKey(scope, ownerLocationId, setId);
    detailCache.value[key] = detail;
    const idx = items.value.findIndex(
      (s) => setKeyFromSummary(s) === key
    );
    if (idx >= 0) {
      items.value[idx].skillCount = detail.skills.length;
    }
  }

  function selectSet(key: SetKey | null) {
    selectedSetKey.value = key;
    if (key && !detailCache.value[key]) {
      const { scope, ownerLocationId, id } = parseSetKey(key);
      fetchSetDetail(id, scope, ownerLocationId);
    }
  }

  return {
    items,
    selectedSetKey,
    detailCache,
    isLoading,
    searchQuery,
    scopeFilter,
    filteredItems,
    selectedDetail,
    fetchSets,
    fetchSetDetail,
    createSet,
    updateSet,
    deleteSet,
    addSkillToSet,
    removeSkillFromSet,
    selectSet,
  };
});

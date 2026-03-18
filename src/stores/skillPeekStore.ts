import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { SkillDetail, SkillId, SkillAssignment } from "@/types";

export const useSkillPeekStore = defineStore("skillPeek", () => {
  const peekSkillId = ref<SkillId | null>(null);
  const detail = ref<SkillDetail | null>(null);
  const isLoading = ref(false);
  const error = ref<string | null>(null);
  const cache = ref<Record<SkillId, SkillDetail>>({});

  const isOpen = computed(() => peekSkillId.value !== null);

  async function peek(id: SkillId) {
    peekSkillId.value = id;
    error.value = null;

    if (cache.value[id]) {
      detail.value = cache.value[id];
      return;
    }

    isLoading.value = true;
    detail.value = null;
    try {
      const result = await invoke<SkillDetail>("get_skill_detail", {
        skillId: id,
      });
      cache.value[id] = result;
      if (peekSkillId.value === id) {
        detail.value = result;
      }
    } catch {
      if (peekSkillId.value === id) {
        error.value = "Could not load skill details";
      }
    } finally {
      isLoading.value = false;
    }
  }

  function peekLocal(assignment: SkillAssignment) {
    peekSkillId.value = assignment.skillId;
    error.value = null;
    isLoading.value = false;
    detail.value = {
      id: assignment.skillId,
      name: assignment.name,
      path: assignment.path,
      archived: assignment.archived,
      summary: null,
      linkedLocations: [],
      includedInSets: [],
      usage: { lastUsedAt: null, useCount30d: 0 },
    };
  }

  function close() {
    peekSkillId.value = null;
  }

  function clearCache() {
    cache.value = {};
  }

  return {
    peekSkillId,
    detail,
    isLoading,
    error,
    isOpen,
    peek,
    peekLocal,
    close,
    clearCache,
  };
});

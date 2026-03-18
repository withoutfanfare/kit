import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { SkillDetail, SkillId } from "@/types";

export const useSkillPeekStore = defineStore("skillPeek", () => {
  const peekSkillId = ref<SkillId | null>(null);
  const detail = ref<SkillDetail | null>(null);
  const isLoading = ref(false);
  const error = ref<string | null>(null);
  const cache = ref<Record<SkillId, SkillDetail>>({});

  const isOpen = computed(() => peekSkillId.value !== null);

  async function peek(id: SkillId, path?: string) {
    peekSkillId.value = id;
    error.value = null;

    const cacheKey = path ? `${id}:${path}` : id;
    if (cache.value[cacheKey]) {
      detail.value = cache.value[cacheKey];
      return;
    }

    isLoading.value = true;
    detail.value = null;
    try {
      const result = await invoke<SkillDetail>("get_skill_detail", {
        skillId: id,
        skillPath: path ?? null,
      });
      cache.value[cacheKey] = result;
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
    close,
    clearCache,
  };
});

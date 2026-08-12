import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { LocationId, SessionLoadout } from "@/types";
import { useAppStore } from "./appStore";

export const useLoadoutStore = defineStore("loadout", () => {
  const loadout = ref<SessionLoadout | null>(null);
  const isLoading = ref(false);
  const locationId = ref<LocationId | null>(null);

  /** Groups Kit can vouch for, in the order they reach a session. */
  const countedGroups = computed(
    () => loadout.value?.groups.filter((g) => !g.enablementUnknown) ?? []
  );

  /** Present on disk, but Kit cannot say whether they are switched on. */
  const uncountedGroups = computed(
    () => loadout.value?.groups.filter((g) => g.enablementUnknown) ?? []
  );

  /**
   * Skills linked into a location but switched off globally. The symlink looks
   * active and isn't — the failure this view exists to catch.
   */
  const conflictCount = computed(() => loadout.value?.vetoed.length ?? 0);

  const hasOverrideProblems = computed(() => {
    const current = loadout.value;
    if (!current) return false;
    return (
      current.deadOverrides.length > 0 || current.unreachableOverrides.length > 0
    );
  });

  async function load(id: LocationId) {
    isLoading.value = true;
    locationId.value = id;
    try {
      loadout.value = await invoke<SessionLoadout>("resolve_session_loadout", {
        locationId: id,
      });
    } catch {
      useAppStore().toast("Couldn't work out what loads here", "error");
      loadout.value = null;
    } finally {
      isLoading.value = false;
    }
  }

  function reset() {
    loadout.value = null;
    locationId.value = null;
  }

  return {
    loadout,
    isLoading,
    locationId,
    countedGroups,
    uncountedGroups,
    conflictCount,
    hasOverrideProblems,
    load,
    reset,
  };
});

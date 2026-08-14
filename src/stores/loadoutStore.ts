import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { LocationId, LocationUsage, SessionLoadout } from "@/types";
import { useAppStore } from "./appStore";

export const useLoadoutStore = defineStore("loadout", () => {
  const loadout = ref<SessionLoadout | null>(null);
  const usage = ref<LocationUsage | null>(null);
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

  /**
   * Which request is the current one. Switching location while a scan is in
   * flight used to let the slower, older answer land last and overwrite the
   * newer one — the screen then showed one location's name over another's
   * numbers.
   */
  let latestRequest = 0;

  async function load(id: LocationId) {
    const request = ++latestRequest;
    isLoading.value = true;
    locationId.value = id;

    // Two independent questions. A usage failure used to throw away a perfectly
    // good loadout and show "couldn't work out what loads here", which was both
    // wrong and the more alarming of the two messages.
    const [resolved, used] = await Promise.allSettled([
      invoke<SessionLoadout>("resolve_session_loadout", { locationId: id }),
      invoke<LocationUsage>("get_location_usage", { locationId: id }),
    ]);

    if (request !== latestRequest) return;

    if (resolved.status === "fulfilled") {
      loadout.value = resolved.value;
    } else {
      useAppStore().toast("Couldn't work out what loads here", "error");
      loadout.value = null;
    }
    usage.value = used.status === "fulfilled" ? used.value : null;
    isLoading.value = false;
  }

  function reset() {
    // Also retires any scan still in flight, so it cannot repopulate the screen
    // after the user has navigated away.
    latestRequest += 1;
    loadout.value = null;
    usage.value = null;
    locationId.value = null;
    isLoading.value = false;
  }

  return {
    loadout,
    usage,
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

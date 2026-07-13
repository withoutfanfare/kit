import { ref } from "vue";
import type { LocationId, SkillId } from "@/types";

type DismissedRecommendations = Record<LocationId, SkillId[]>;

const STORAGE_KEY = "kit.dismissedRecommendations.v1";

function loadDismissed(): DismissedRecommendations {
  try {
    const parsed: unknown = JSON.parse(localStorage.getItem(STORAGE_KEY) ?? "{}");
    if (
      parsed &&
      typeof parsed === "object" &&
      !Array.isArray(parsed) &&
      Object.values(parsed).every(
        (ids) => Array.isArray(ids) && ids.every((id) => typeof id === "string")
      )
    ) {
      return parsed as DismissedRecommendations;
    }
  } catch {
    // Malformed or unavailable storage behaves like no dismissals.
  }
  return {};
}

const dismissed = ref<DismissedRecommendations>(loadDismissed());

function persist() {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(dismissed.value));
  } catch {
    // A storage failure should not break recommendations.
  }
}

export function useDismissedRecommendations() {
  function isDismissed(locationId: LocationId, skillId: SkillId) {
    return dismissed.value[locationId]?.includes(skillId) ?? false;
  }

  function dismiss(locationId: LocationId, skillId: SkillId) {
    if (isDismissed(locationId, skillId)) return;
    dismissed.value = {
      ...dismissed.value,
      [locationId]: [...(dismissed.value[locationId] ?? []), skillId],
    };
    persist();
  }

  function restore(locationId: LocationId, skillId: SkillId) {
    dismissed.value = {
      ...dismissed.value,
      [locationId]: (dismissed.value[locationId] ?? []).filter((id) => id !== skillId),
    };
    persist();
  }

  function restoreAll(locationId: LocationId) {
    const { [locationId]: _, ...rest } = dismissed.value;
    dismissed.value = rest;
    persist();
  }

  return { dismissed, isDismissed, dismiss, restore, restoreAll };
}

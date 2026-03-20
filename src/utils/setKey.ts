import type { SetScope, LocationId, SetId } from "@/types";

export type SetKey = string;

export function makeSetKey(
  scope: SetScope,
  ownerLocationId: LocationId | null | undefined,
  id: SetId
): SetKey {
  if (scope === "project" && ownerLocationId) {
    return `project:${ownerLocationId}:${id}`;
  }
  return `global::${id}`;
}

export function parseSetKey(key: SetKey): {
  scope: SetScope;
  ownerLocationId: LocationId | undefined;
  id: SetId;
} {
  if (key.startsWith("project:")) {
    const rest = key.slice("project:".length);
    const colonIdx = rest.indexOf(":");
    if (colonIdx === -1) {
      // Malformed key — treat entire rest as set ID with no owner
      return { scope: "project", ownerLocationId: undefined, id: rest };
    }
    return {
      scope: "project",
      ownerLocationId: rest.slice(0, colonIdx),
      id: rest.slice(colonIdx + 1),
    };
  }
  // "global::setId"
  return {
    scope: "global",
    ownerLocationId: undefined,
    id: key.slice("global::".length),
  };
}

export function setKeyFromSummary(s: {
  scope: SetScope;
  ownerLocationId?: LocationId | null;
  id: SetId;
}): SetKey {
  return makeSetKey(s.scope, s.ownerLocationId, s.id);
}

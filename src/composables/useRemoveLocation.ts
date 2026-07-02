import { ref } from "vue";
import type { LocationId } from "@/types";

/** Location pending removal confirmation, or null when no dialog is open. */
export const pendingRemoval = ref<{ id: LocationId; label: string } | null>(null);

/** Ask for confirmation before removing a location. The dialog lives in LocationsView. */
export function requestRemoveLocation(location: { id: LocationId; label: string }) {
  pendingRemoval.value = { id: location.id, label: location.label };
}

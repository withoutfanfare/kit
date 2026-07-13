<script setup lang="ts">
import { computed } from "vue";
import type { LocationDetail } from "@/types";
import { useLocationsStore } from "@/stores/locationsStore";
import InspectorPanel from "@/components/layout/InspectorPanel.vue";

const props = defineProps<{
  detail: LocationDetail;
}>();

const locationsStore = useLocationsStore();

const lastSyncedDisplay = computed(() => {
  const summary = locationsStore.locationList.find(
    (l) => l.id === props.detail.id
  );
  if (!summary?.lastSyncedAt) return "Never";
  const date = new Date(summary.lastSyncedAt);
  return date.toLocaleString(undefined, {
    day: "numeric",
    month: "short",
    hour: "2-digit",
    minute: "2-digit",
  });
});

const manifestStatus = computed(() =>
  props.detail.manifestPath ? "Present" : "Not found"
);
</script>

<template>
  <InspectorPanel title="Location">
    <div class="inspector-section">
      <div class="inspector-field">
        <span class="field-label">Path</span>
        <span class="field-value path">{{ detail.path }}</span>
      </div>
      <div class="inspector-field">
        <span class="field-label">Manifest</span>
        <span class="field-value" :class="{ muted: !detail.manifestPath }">
          {{ manifestStatus }}
        </span>
      </div>
      <div class="inspector-field">
        <span class="field-label">Last synced</span>
        <span class="field-value">{{ lastSyncedDisplay }}</span>
      </div>
    </div>
  </InspectorPanel>
</template>

<style scoped>
.inspector-section {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}

.inspector-field {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.field-label {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  font-weight: var(--weight-medium);
}

.field-value {
  font-size: var(--text-sm);
  color: var(--text-primary);
  word-break: break-all;
}

.field-value.path {
  font-size: var(--text-xs);
  color: var(--text-secondary);
  font-family: ui-monospace, "SF Mono", SFMono-Regular, monospace;
}

.field-value.muted {
  color: var(--text-tertiary);
}

</style>

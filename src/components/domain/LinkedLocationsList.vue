<script setup lang="ts">
import type { SavedLocationSummary } from "@/types";
import { useRouter } from "vue-router";
import { useLocationsStore } from "@/stores/locationsStore";

defineProps<{
  locations: SavedLocationSummary[];
}>();

const router = useRouter();
const locationsStore = useLocationsStore();

function navigateToLocation(id: string) {
  locationsStore.selectLocation(id);
  router.push(`/locations/${id}`);
}
</script>

<template>
  <div class="linked-locations">
    <div class="section-group">
      <div
        v-for="loc in locations"
        :key="loc.id"
        class="location-row"
        @click="navigateToLocation(loc.id)"
      >
        <div class="row-content">
          <span class="row-label">{{ loc.label }}</span>
          <span class="row-path">{{ loc.path }}</span>
        </div>
        <svg
          class="chevron"
          width="12"
          height="12"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <polyline points="9 18 15 12 9 6" />
        </svg>
      </div>
      <div v-if="locations.length === 0" class="list-empty">
        <span class="list-empty-text">Not linked to any locations</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.linked-locations {
  display: flex;
  flex-direction: column;
}

.section-group {
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  background: var(--surface-panel);
  overflow: hidden;
}

.location-row {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  cursor: pointer;
  transition: background var(--duration-fast) var(--ease-default);
  user-select: none;
}

.location-row + .location-row {
  border-top: 1px solid var(--border-subtle);
}

.location-row:hover {
  background: var(--surface-hover);
}

.row-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 1px;
  min-width: 0;
}

.row-label {
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.row-path {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.chevron {
  color: var(--text-tertiary);
  flex-shrink: 0;
}

.list-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--space-4);
}

.list-empty-text {
  font-size: var(--text-sm);
  color: var(--text-tertiary);
}
</style>

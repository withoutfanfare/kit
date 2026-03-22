<script setup lang="ts">
import { computed } from "vue";
import { useBulkAssignStore } from "@/stores/bulkAssignStore";
import { useLocationsStore } from "@/stores/locationsStore";
import { SModal, SBadge } from "@stuntrocket/ui";

const bulkStore = useBulkAssignStore();
const locationsStore = useLocationsStore();

const locations = computed(() => locationsStore.locationList);

function isSelected(id: string): boolean {
  return bulkStore.selectedLocationIds.has(id);
}

function resultFor(id: string) {
  return bulkStore.results.find((r) => r.locationId === id);
}
</script>

<template>
  <SModal
    :open="bulkStore.isOpen"
    title="Assign to multiple locations"
    @close="bulkStore.close()"
  >
    <div class="bulk-modal">
      <div class="skill-summary">
        <span class="label">Skills to assign:</span>
        <div class="skill-badges">
          <SBadge v-for="sid in bulkStore.skillIds" :key="sid" variant="accent" compact>
            {{ sid }}
          </SBadge>
        </div>
      </div>

      <div class="location-header">
        <span class="label">Select locations:</span>
        <div class="select-actions">
          <button class="text-btn" @click="bulkStore.selectAll()">Select all</button>
          <button class="text-btn" @click="bulkStore.deselectAll()">Deselect all</button>
        </div>
      </div>

      <div class="location-list">
        <label
          v-for="loc in locations"
          :key="loc.id"
          class="location-row"
          :class="{ selected: isSelected(loc.id) }"
        >
          <input
            type="checkbox"
            :checked="isSelected(loc.id)"
            :disabled="bulkStore.isApplying"
            @change="bulkStore.toggleLocation(loc.id)"
          />
          <div class="loc-info">
            <span class="loc-label">{{ loc.label }}</span>
            <span class="loc-path">{{ loc.path }}</span>
          </div>
          <SBadge v-if="loc.installedSkillCount > 0" variant="default" compact>
            {{ loc.installedSkillCount }} skills
          </SBadge>
          <!-- Show result status after apply -->
          <SBadge v-if="resultFor(loc.id)?.success" variant="success" compact>done</SBadge>
          <SBadge v-else-if="resultFor(loc.id)?.error" variant="error" compact :title="resultFor(loc.id)!.error!">
            failed
          </SBadge>
        </label>
        <div v-if="locations.length === 0" class="empty-msg">
          No locations saved yet.
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn-secondary" @click="bulkStore.close()">Cancel</button>
        <button
          class="btn-primary"
          :disabled="!bulkStore.hasSelections || bulkStore.isApplying"
          @click="bulkStore.apply()"
        >
          {{ bulkStore.isApplying ? 'Assigning...' : `Assign to ${bulkStore.selectedLocationIds.size} location${bulkStore.selectedLocationIds.size === 1 ? '' : 's'}` }}
        </button>
      </div>
    </div>
  </SModal>
</template>

<style scoped>
.bulk-modal {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
  min-width: 380px;
}

.label {
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.04em;
}

.skill-summary {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.skill-badges {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-1);
}

.location-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.select-actions {
  display: flex;
  gap: var(--space-2);
}

.text-btn {
  background: none;
  border: none;
  font-family: inherit;
  font-size: var(--text-xs);
  color: var(--accent);
  cursor: pointer;
  padding: 0;
}

.text-btn:hover {
  text-decoration: underline;
}

.location-list {
  display: flex;
  flex-direction: column;
  gap: 1px;
  max-height: 280px;
  overflow-y: auto;
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  padding: var(--space-1);
}

.location-row {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-2);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: background var(--duration-fast) var(--ease-default);
  user-select: none;
}

.location-row:hover {
  background: var(--surface-hover);
}

.location-row.selected {
  background: var(--surface-selected);
}

.location-row input[type="checkbox"] {
  margin: 0;
  flex-shrink: 0;
  accent-color: var(--accent);
}

.loc-info {
  display: flex;
  flex-direction: column;
  min-width: 0;
  flex: 1;
}

.loc-label {
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.loc-path {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.empty-msg {
  padding: var(--space-4);
  text-align: center;
  font-size: var(--text-sm);
  color: var(--text-tertiary);
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-2);
  padding-top: var(--space-2);
  border-top: 1px solid var(--border-subtle);
}

.btn-secondary,
.btn-primary {
  font-family: inherit;
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--duration-fast) var(--ease-default);
  border: none;
}

.btn-secondary {
  background: var(--surface-panel);
  color: var(--text-primary);
  border: 1px solid var(--border-default);
}

.btn-secondary:hover {
  background: var(--surface-hover);
}

.btn-primary {
  background: var(--accent);
  color: #fff;
}

.btn-primary:hover:not(:disabled) {
  filter: brightness(1.1);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: default;
}
</style>

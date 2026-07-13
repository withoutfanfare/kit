<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useLocationsStore } from "@/stores/locationsStore";
import { useAppStore } from "@/stores/appStore";
import { useAssignmentStore } from "@/stores/assignmentStore";
import { useRoute } from "vue-router";
import SkillStatusLegend from "@/components/domain/SkillStatusLegend.vue";
import {
  linkStateBadgeVariant,
  linkStateLabels,
} from "@/utils/statusLabels";
import { SButton, SBadge, SEmptyState } from "@stuntrocket/ui";
import type { LocationComparison, LocationId } from "@/types";

const locationsStore = useLocationsStore();
const appStore = useAppStore();
const assignmentStore = useAssignmentStore();
const route = useRoute();

const locationAId = ref<LocationId | null>(null);
const locationBId = ref<LocationId | null>(null);
const comparison = ref<LocationComparison | null>(null);
const isLoading = ref(false);

// Filter out the other selected location from each dropdown
const locationsForA = computed(() =>
  locationsStore.locationList.filter((l) => l.id !== locationBId.value)
);
const locationsForB = computed(() =>
  locationsStore.locationList.filter((l) => l.id !== locationAId.value)
);

const canCompare = computed(
  () => locationAId.value && locationBId.value && locationAId.value !== locationBId.value
);

async function runComparison() {
  if (!canCompare.value) return;
  isLoading.value = true;
  comparison.value = null;
  try {
    comparison.value = await invoke<LocationComparison>("compare_locations", {
      locationAId: locationAId.value,
      locationBId: locationBId.value,
    });
  } catch (err) {
    appStore.toast(
      err instanceof Error ? err.message : "Comparison failed",
      "error"
    );
  } finally {
    isLoading.value = false;
  }
}

function swapLocations() {
  const tmp = locationAId.value;
  locationAId.value = locationBId.value;
  locationBId.value = tmp;
  if (comparison.value) {
    runComparison();
  }
}

function quickAssign(skillId: string, toLocationId: string) {
  assignmentStore.open(toLocationId);
  assignmentStore.toggleSkill(skillId);
}

function exportComparison() {
  if (!comparison.value) return;
  const c = comparison.value;
  const lines: string[] = [
    `# Location Comparison`,
    ``,
    `**${c.locationA.label}** (${c.locationA.totalSkills} skills) vs **${c.locationB.label}** (${c.locationB.totalSkills} skills)`,
    ``,
  ];

  if (c.onlyInA.length > 0) {
    lines.push(`## Only in ${c.locationA.label} (${c.onlyInA.length})`);
    for (const s of c.onlyInA) {
      lines.push(`- ${s.name}`);
    }
    lines.push(``);
  }

  if (c.shared.length > 0) {
    lines.push(`## Shared (${c.shared.length})`);
    for (const s of c.shared) {
      const flag = s.versionDiffers ? " ⚠ different versions" : "";
      lines.push(`- ${s.name}${flag}`);
    }
    lines.push(``);
  }

  if (c.onlyInB.length > 0) {
    lines.push(`## Only in ${c.locationB.label} (${c.onlyInB.length})`);
    for (const s of c.onlyInB) {
      lines.push(`- ${s.name}`);
    }
    lines.push(``);
  }

  const text = lines.join("\n");
  navigator.clipboard.writeText(text).then(
    () => appStore.toast("Comparison copied to clipboard", "success"),
    () => appStore.toast("Failed to copy to clipboard", "error")
  );
}

onMounted(async () => {
  await locationsStore.fetchList();
  const requestedLocation = route.query.locationA;
  if (
    typeof requestedLocation === "string" &&
    locationsStore.locationList.some((location) => location.id === requestedLocation)
  ) {
    locationAId.value = requestedLocation;
  }
});
</script>

<template>
  <div class="compare-view">
    <div class="page-header">
      <h1 class="page-title">Compare Locations</h1>
      <div v-if="comparison" class="header-actions">
        <SButton variant="secondary" size="sm" @click="exportComparison">
          Copy as Markdown
        </SButton>
      </div>
    </div>

    <!-- Location selectors -->
    <div class="selector-row">
      <div class="selector-col">
        <label class="selector-label">Location A</label>
        <select
          v-model="locationAId"
          class="location-select"
          @change="comparison = null"
        >
          <option :value="null" disabled>Select a location...</option>
          <option v-for="loc in locationsForA" :key="loc.id" :value="loc.id">
            {{ loc.label || loc.path }}
          </option>
        </select>
      </div>

      <SButton
        variant="secondary"
        size="sm"
        class="swap-btn"
        :disabled="!locationAId || !locationBId"
        @click="swapLocations"
        title="Swap locations"
        aria-label="Swap locations"
      >
        ⇄
      </SButton>

      <div class="selector-col">
        <label class="selector-label">Location B</label>
        <select
          v-model="locationBId"
          class="location-select"
          @change="comparison = null"
        >
          <option :value="null" disabled>Select a location...</option>
          <option v-for="loc in locationsForB" :key="loc.id" :value="loc.id">
            {{ loc.label || loc.path }}
          </option>
        </select>
      </div>

      <SButton
        size="sm"
        :disabled="!canCompare"
        :loading="isLoading"
        @click="runComparison"
      >
        Compare
      </SButton>
    </div>

    <!-- Results -->
    <div v-if="isLoading && !comparison" class="loading-state">
      <span class="spinner" />
      <span class="loading-label">Comparing locations...</span>
    </div>

    <SEmptyState
      v-else-if="!comparison"
      title="Select two locations to compare"
      description="See which skills each location has, which are shared, and which differ."
    />

    <div v-else class="comparison-content">
      <!-- Summary -->
      <div class="summary-row">
        <div class="summary-card">
          <span class="summary-value">{{ comparison.onlyInA.length }}</span>
          <span class="summary-label">Only in {{ comparison.locationA.label }}</span>
        </div>
        <div class="summary-card shared-card">
          <span class="summary-value">{{ comparison.shared.length }}</span>
          <span class="summary-label">Shared</span>
        </div>
        <div class="summary-card">
          <span class="summary-value">{{ comparison.onlyInB.length }}</span>
          <span class="summary-label">Only in {{ comparison.locationB.label }}</span>
        </div>
      </div>

      <SkillStatusLegend />

      <!-- Three-column layout -->
      <div class="columns">
        <!-- Only in A -->
        <div class="column">
          <div class="column-header">
            <span class="column-title">{{ comparison.locationA.label }}</span>
            <SBadge variant="count">{{ comparison.onlyInA.length }}</SBadge>
          </div>
          <div v-if="comparison.onlyInA.length === 0" class="column-empty">
            No unique skills
          </div>
          <div v-else class="column-list">
            <div v-for="skill in comparison.onlyInA" :key="skill.skillId" class="skill-row">
              <div class="skill-info">
                <span class="skill-name">{{ skill.name }}</span>
                <SBadge :variant="linkStateBadgeVariant(skill.linkState)" size="sm">
                  {{ linkStateLabels[skill.linkState] }}
                </SBadge>
              </div>
              <SButton
                variant="secondary"
                size="sm"
                :title="`Assign ${skill.name} to ${comparison.locationB.label}`"
                :aria-label="`Assign ${skill.name} to ${comparison.locationB.label}`"
                @click="quickAssign(skill.skillId, comparison!.locationB.id)"
              >
                →
              </SButton>
            </div>
          </div>
        </div>

        <!-- Shared -->
        <div class="column shared-column">
          <div class="column-header">
            <span class="column-title">Shared</span>
            <SBadge variant="count">{{ comparison.shared.length }}</SBadge>
          </div>
          <div v-if="comparison.shared.length === 0" class="column-empty">
            No shared skills
          </div>
          <div v-else class="column-list">
            <div
              v-for="skill in comparison.shared"
              :key="skill.skillId"
              class="skill-row"
              :class="{ 'version-differs': skill.versionDiffers }"
            >
              <div class="skill-info">
                <span class="skill-name">{{ skill.name }}</span>
                <SBadge v-if="skill.versionDiffers" variant="warning" size="sm">
                  Different versions
                </SBadge>
              </div>
            </div>
          </div>
        </div>

        <!-- Only in B -->
        <div class="column">
          <div class="column-header">
            <span class="column-title">{{ comparison.locationB.label }}</span>
            <SBadge variant="count">{{ comparison.onlyInB.length }}</SBadge>
          </div>
          <div v-if="comparison.onlyInB.length === 0" class="column-empty">
            No unique skills
          </div>
          <div v-else class="column-list">
            <div v-for="skill in comparison.onlyInB" :key="skill.skillId" class="skill-row">
              <div class="skill-info">
                <span class="skill-name">{{ skill.name }}</span>
                <SBadge :variant="linkStateBadgeVariant(skill.linkState)" size="sm">
                  {{ linkStateLabels[skill.linkState] }}
                </SBadge>
              </div>
              <SButton
                variant="secondary"
                size="sm"
                :title="`Assign ${skill.name} to ${comparison.locationA.label}`"
                :aria-label="`Assign ${skill.name} to ${comparison.locationA.label}`"
                @click="quickAssign(skill.skillId, comparison!.locationA.id)"
              >
                ←
              </SButton>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.compare-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow-y: auto;
  padding: var(--space-5) var(--space-6);
}

.page-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--space-5);
  flex-shrink: 0;
}

.page-title {
  font-family: var(--font-sans);
  font-size: var(--text-xl);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  margin: 0;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

/* Selectors */
.selector-row {
  display: flex;
  align-items: flex-end;
  gap: var(--space-3);
  margin-bottom: var(--space-5);
  flex-shrink: 0;
}

.selector-col {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  flex: 1;
}

.selector-label {
  font-family: var(--font-sans);
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.04em;
}

.location-select {
  height: 32px;
  padding: 0 var(--space-6) 0 var(--space-2);
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--text-primary);
  background: var(--surface-hover);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  outline: none;
  cursor: pointer;
  appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='10' height='6' viewBox='0 0 10 6'%3E%3Cpath d='M1 1l4 4 4-4' stroke='%236b6b6b' stroke-width='1.5' fill='none' stroke-linecap='round' stroke-linejoin='round'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 8px center;
}

.location-select:focus {
  border-color: var(--accent);
  box-shadow: 0 0 0 2px var(--accent-subtle);
}

.swap-btn {
  flex-shrink: 0;
  margin-bottom: 1px;
}

/* Loading */
.loading-state {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-8) 0;
  justify-content: center;
}

.spinner {
  width: 14px;
  height: 14px;
  border: 1.5px solid var(--border-default);
  border-top-color: var(--accent);
  border-radius: 50%;
  animation: spin 600ms linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.loading-label {
  font-size: var(--text-sm);
  color: var(--text-secondary);
}

/* Summary */
.comparison-content {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
  flex: 1;
  min-height: 0;
}

.summary-row {
  display: flex;
  gap: var(--space-3);
  flex-shrink: 0;
}

.summary-card {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-1);
  padding: var(--space-3);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  background: var(--surface-panel);
}

.summary-card.shared-card {
  border-color: var(--accent-subtle);
}

.summary-value {
  font-size: var(--text-lg);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  font-variant-numeric: tabular-nums;
}

.summary-label {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  text-align: center;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 100%;
}

/* Columns */
.columns {
  display: flex;
  gap: var(--space-3);
  flex: 1;
  min-height: 0;
}

.column {
  flex: 1;
  display: flex;
  flex-direction: column;
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  background: var(--surface-panel);
  overflow: hidden;
  min-width: 0;
}

.shared-column {
  border-color: var(--accent-subtle);
}

.column-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-2) var(--space-3);
  border-bottom: 1px solid var(--border-subtle);
  flex-shrink: 0;
}

.column-title {
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.column-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--space-6);
  font-size: var(--text-sm);
  color: var(--text-tertiary);
  font-style: italic;
}

.column-list {
  flex: 1;
  overflow-y: auto;
}

/* Skill rows */
.skill-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  border-bottom: 1px solid var(--border-subtle);
}

.skill-row:last-child {
  border-bottom: none;
}

.skill-row.version-differs {
  background: var(--warning-subtle);
}

.skill-info {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  min-width: 0;
  flex: 1;
}

.skill-name {
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>

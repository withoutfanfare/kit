<script setup lang="ts">
import { onMounted, watch } from "vue";
import { useChangelogStore } from "@/stores/changelogStore";
import { SSearchInput, SSegmentedControl, SBadge, SEmptyState } from "@stuntrocket/ui";

const store = useChangelogStore();

const dayOptions = [
  { label: "All", value: "all" },
  { label: "7 days", value: "7" },
  { label: "30 days", value: "30" },
  { label: "90 days", value: "90" },
];

const dayFilter = defineModel<string>("dayFilter", { default: "all" });

watch(dayFilter, (val) => {
  store.filterDays = val === "all" ? null : parseInt(val, 10);
  store.fetchEntries();
});

function formatModifiedAt(iso: string): string {
  return new Date(iso).toLocaleString("en-GB", {
    dateStyle: "medium",
    timeStyle: "short",
  });
}

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

onMounted(() => {
  store.fetchEntries();
});
</script>

<template>
  <div class="changelog-view">
    <div class="changelog-header">
      <div class="header-title">
        <h2>Recently modified</h2>
        <span class="header-count">{{ store.filteredEntries.length }} skills</span>
      </div>
      <div class="header-controls">
        <SSearchInput
          v-model="store.searchQuery"
          placeholder="Filter skills..."
          compact
        />
        <SSegmentedControl v-model="dayFilter" :options="dayOptions" />
      </div>
    </div>

    <div v-if="store.isLoading" class="changelog-loading">Loading modified skills…</div>

    <div v-else-if="store.filteredEntries.length === 0" class="changelog-empty">
      <SEmptyState
        title="No modified skills"
        description="Modified skills will appear here."
      />
    </div>

    <div v-else class="changelog-list">
      <RouterLink
        v-for="entry in store.filteredEntries"
        :key="entry.skillId"
        :to="`/skills/${entry.skillId}`"
        class="changelog-row"
      >
        <div class="row-left">
          <span class="row-name">{{ entry.name }}</span>
          <span class="row-summary">SKILL.md edited</span>
        </div>
        <div class="row-right">
          <SBadge v-if="entry.assignedLocations.length > 0" variant="accent" compact>
            {{ entry.assignedLocations.slice(0, 2).map((location) => location.label).join(", ") }}
            <span v-if="entry.assignedLocations.length > 2">
              +{{ entry.assignedLocations.length - 2 }} more
            </span>
          </SBadge>
          <span class="row-size">{{ formatSize(entry.sizeBytes) }}</span>
          <time class="row-time" :datetime="entry.modifiedAt">{{ formatModifiedAt(entry.modifiedAt) }}</time>
          <span class="row-action">Open skill</span>
        </div>
      </RouterLink>
    </div>
  </div>
</template>

<style scoped>
.changelog-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.changelog-header {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  padding: var(--space-4) var(--space-5);
  border-bottom: 1px solid var(--border-subtle);
  flex-shrink: 0;
}

.header-title {
  display: flex;
  align-items: baseline;
  gap: var(--space-2);
}

.header-title h2 {
  font-size: var(--text-lg);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  margin: 0;
}

.header-count {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
}

.header-controls {
  display: flex;
  gap: var(--space-2);
  align-items: center;
}

.changelog-loading {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--space-8);
  color: var(--text-tertiary);
  font-size: var(--text-sm);
}

.changelog-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  flex: 1;
}

.changelog-list {
  flex: 1;
  overflow-y: auto;
  padding: var(--space-2);
}

.changelog-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-3);
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-sm);
  color: inherit;
  text-decoration: none;
  transition: background var(--duration-fast) var(--ease-default);
}

.changelog-row:hover {
  background: var(--surface-hover);
}

.changelog-row:focus-visible {
  outline: 2px solid var(--accent);
  outline-offset: -2px;
}

.row-left {
  display: flex;
  flex-direction: column;
  gap: 1px;
  min-width: 0;
  flex: 1;
}

.row-name {
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.row-summary {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.row-right {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  flex-shrink: 0;
}

.row-size {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  min-width: 48px;
  text-align: right;
}

.row-time {
  font-size: var(--text-xs);
  color: var(--text-secondary);
  min-width: 116px;
  text-align: right;
}

.row-action {
  font-size: var(--text-xs);
  font-weight: var(--weight-medium);
  color: var(--accent);
  white-space: nowrap;
}
</style>

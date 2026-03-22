<script setup lang="ts">
import { onMounted, watch } from "vue";
import { useChangelogStore } from "@/stores/changelogStore";
import { useRouter } from "vue-router";
import { SSearchInput, SSegmentedControl, SBadge, SEmptyState } from "@stuntrocket/ui";

const store = useChangelogStore();
const router = useRouter();

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

function goToSkill(skillId: string) {
  router.push(`/skills/${skillId}`);
}

function formatDate(iso: string): string {
  const d = new Date(iso);
  return d.toLocaleDateString("en-GB", {
    day: "numeric",
    month: "short",
    year: "numeric",
  });
}

function formatTime(iso: string): string {
  const d = new Date(iso);
  return d.toLocaleTimeString("en-GB", {
    hour: "2-digit",
    minute: "2-digit",
  });
}

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

function timeAgo(iso: string): string {
  const now = Date.now();
  const then = new Date(iso).getTime();
  const diff = now - then;
  const mins = Math.floor(diff / 60000);
  if (mins < 60) return `${mins}m ago`;
  const hours = Math.floor(mins / 60);
  if (hours < 24) return `${hours}h ago`;
  const days = Math.floor(hours / 24);
  return `${days}d ago`;
}

onMounted(() => {
  store.fetchEntries();
});
</script>

<template>
  <div class="changelog-view">
    <div class="changelog-header">
      <div class="header-title">
        <h2>Changelog</h2>
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

    <div v-if="store.isLoading" class="changelog-loading">Loading...</div>

    <div v-else-if="store.filteredEntries.length === 0" class="changelog-empty">
      <SEmptyState
        title="No recent changes"
        description="Skills will appear here when they are modified."
      />
    </div>

    <div v-else class="changelog-list">
      <div
        v-for="entry in store.filteredEntries"
        :key="entry.skillId"
        class="changelog-row"
        @click="goToSkill(entry.skillId)"
      >
        <div class="row-left">
          <span class="row-name">{{ entry.name }}</span>
          <span class="row-id">{{ entry.skillId }}</span>
        </div>
        <div class="row-right">
          <SBadge v-if="entry.assignedLocationCount > 0" variant="accent" compact>
            {{ entry.assignedLocationCount }} location{{ entry.assignedLocationCount === 1 ? '' : 's' }}
          </SBadge>
          <span class="row-size">{{ formatSize(entry.sizeBytes) }}</span>
          <span class="row-time" :title="`${formatDate(entry.modifiedAt)} ${formatTime(entry.modifiedAt)}`">
            {{ timeAgo(entry.modifiedAt) }}
          </span>
        </div>
      </div>
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
  cursor: default;
  transition: background var(--duration-fast) var(--ease-default);
  user-select: none;
}

.changelog-row:hover {
  background: var(--surface-hover);
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

.row-id {
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
  min-width: 48px;
  text-align: right;
}
</style>

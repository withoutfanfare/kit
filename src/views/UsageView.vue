<script setup lang="ts">
import { onMounted } from "vue";
import { useUsageStore } from "@/stores/usageStore";
import { useSkillPeekStore } from "@/stores/skillPeekStore";
import { SSectionHeader, SBadge } from "@stuntrocket/ui";

const usageStore = useUsageStore();
const skillPeekStore = useSkillPeekStore();

function navigateToSkill(skillId: string) {
  skillPeekStore.peek(skillId);
}

function formatDate(iso: string): string {
  const date = new Date(iso);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));

  if (diffDays === 0) return "Today";
  if (diffDays === 1) return "Yesterday";
  if (diffDays < 7) return `${diffDays} days ago`;

  return date.toLocaleDateString("en-GB", {
    day: "numeric",
    month: "short",
  });
}

onMounted(() => {
  usageStore.fetchSummary();
});
</script>

<template>
  <div class="usage-view">
    <!-- Page header -->
    <div class="page-header">
      <h1 class="page-title">Usage</h1>
      <span class="page-subtitle">Last 30 days</span>
    </div>

    <div v-if="usageStore.isLoading && !usageStore.summary" class="loading-state">
      <span class="spinner" />
      <span class="loading-label">Loading usage data...</span>
    </div>

    <div v-else-if="!usageStore.summary" class="empty-usage">
      <p class="empty-usage-title">No usage data yet</p>
      <p class="empty-usage-desc">Usage tracks how often each skill is invoked across your projects. Data will appear here once skills are in use.</p>
    </div>

    <div v-else class="usage-content">
      <!-- Most Used -->
      <section class="usage-section">
        <SSectionHeader title="Most Used" :count="usageStore.summary.mostUsed.length" />
        <div class="grouped-list">
          <div
            v-for="(item, index) in usageStore.summary.mostUsed"
            :key="item.id"
            class="list-item clickable"
            @click="navigateToSkill(item.id)"
          >
            <div class="item-left">
              <span class="rank">{{ index + 1 }}</span>
              <span class="item-name">{{ item.name }}</span>
            </div>
            <SBadge variant="count">{{ item.count }}</SBadge>
          </div>
          <div v-if="usageStore.summary.mostUsed.length === 0" class="list-empty">
            No usage data for this period
          </div>
        </div>
      </section>

      <!-- Recently Used -->
      <section class="usage-section">
        <SSectionHeader title="Recently Used" :count="usageStore.summary.recentlyUsed.length" />
        <div class="grouped-list">
          <div
            v-for="item in usageStore.summary.recentlyUsed"
            :key="item.id"
            class="list-item clickable"
            @click="navigateToSkill(item.id)"
          >
            <span class="item-name">{{ item.name }}</span>
            <span class="item-date">{{ formatDate(item.lastUsedAt) }}</span>
          </div>
          <div v-if="usageStore.summary.recentlyUsed.length === 0" class="list-empty">
            No recent activity
          </div>
        </div>
      </section>

      <!-- Unused -->
      <section class="usage-section">
        <SSectionHeader title="Unused" :count="usageStore.summary.unused.length" />
        <div class="grouped-list">
          <div
            v-for="item in usageStore.summary.unused"
            :key="item.id"
            class="list-item clickable"
            @click="navigateToSkill(item.id)"
          >
            <span class="item-name">{{ item.name }}</span>
          </div>
          <div v-if="usageStore.summary.unused.length === 0" class="list-empty">
            All skills have been used recently
          </div>
        </div>
      </section>

      <!-- Suggestions -->
      <section v-if="usageStore.summary.suggestions.length > 0" class="usage-section">
        <SSectionHeader title="Suggestions" />
        <div class="grouped-list">
          <div
            v-for="(suggestion, idx) in usageStore.summary.suggestions"
            :key="idx"
            class="list-item suggestion"
          >
            <span class="suggestion-text">{{ suggestion }}</span>
          </div>
        </div>
      </section>
    </div>
  </div>
</template>

<style scoped>
.usage-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow-y: auto;
  padding: var(--space-5) var(--space-6);
}

/* Page header */
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

.page-subtitle {
  font-size: var(--text-sm);
  color: var(--text-tertiary);
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
  to {
    transform: rotate(360deg);
  }
}

.loading-label {
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--text-secondary);
}

/* Content */
.usage-content {
  display: flex;
  flex-direction: column;
  gap: var(--space-5);
}

/* Sections */
.usage-section {
  display: flex;
  flex-direction: column;
}

/* Grouped list */
.grouped-list {
  background: var(--surface-panel);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  overflow: hidden;
}

.list-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-2) var(--space-3);
  min-height: var(--list-row-height);
  font-family: var(--font-sans);
  transition: background var(--duration-fast) var(--ease-default);
}

.list-item + .list-item {
  border-top: 1px solid var(--border-subtle);
}

.list-item.clickable {
  cursor: pointer;
}

.list-item.clickable:hover {
  background: var(--surface-hover);
}

.item-left {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  min-width: 0;
}

.rank {
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  color: var(--text-tertiary);
  width: 18px;
  text-align: right;
  flex-shrink: 0;
}

.item-name {
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.item-date {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  flex-shrink: 0;
}

.suggestion-text {
  font-size: var(--text-sm);
  color: var(--text-secondary);
  line-height: 1.5;
}

.list-empty {
  padding: var(--space-4) var(--space-3);
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--text-tertiary);
  text-align: center;
}

/* Empty state */
.empty-usage {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
  padding: var(--space-10) var(--space-6);
  text-align: center;
  max-width: 340px;
}

.empty-usage-title {
  font-family: var(--font-sans);
  font-size: var(--text-lg);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  margin: 0;
}

.empty-usage-desc {
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--text-secondary);
  line-height: 1.5;
  margin: 0;
}
</style>

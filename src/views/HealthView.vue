<script setup lang="ts">
import { onMounted, computed } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useHealthStore } from "@/stores/healthStore";
import { SButton, SBadge } from "@stuntrocket/ui";

const route = useRoute();
const router = useRouter();
const healthStore = useHealthStore();

const locationFilter = computed(() =>
  route.query.locationId ? String(route.query.locationId) : null
);

function navigateToLocation(id: string) {
  router.push(`/locations/${id}`);
}

function clearFilter() {
  healthStore.setFilter(null);
  router.replace({ path: "/health" });
}

onMounted(() => {
  if (locationFilter.value) {
    healthStore.setFilter(locationFilter.value);
  }
  healthStore.runCheck();
});
</script>

<template>
  <div class="health-view">
    <div class="page-header">
      <div class="header-left">
        <h1 class="page-title">Health Check</h1>
        <SBadge v-if="healthStore.result" variant="count">
          {{ healthStore.result.locationCount }} locations scanned
        </SBadge>
      </div>
      <div class="header-actions">
        <SButton
          v-if="healthStore.filterLocationId"
          variant="secondary"
          size="sm"
          @click="clearFilter"
        >
          Clear filter
        </SButton>
        <SButton
          :loading="healthStore.isLoading"
          size="sm"
          @click="healthStore.runCheck()"
        >
          Rescan
        </SButton>
      </div>
    </div>

    <div v-if="healthStore.isLoading && !healthStore.result" class="loading-state">
      <span class="spinner" />
      <span class="loading-label">Scanning all locations...</span>
    </div>

    <div v-else-if="healthStore.result" class="health-content">
      <!-- Summary cards -->
      <div class="summary-row">
        <div class="summary-card healthy">
          <span class="summary-value">{{ healthStore.result.healthyCount }}</span>
          <span class="summary-label">Healthy</span>
        </div>
        <div class="summary-card warning">
          <span class="summary-value">{{ healthStore.result.warningCount }}</span>
          <span class="summary-label">Warnings</span>
        </div>
        <div class="summary-card error">
          <span class="summary-value">{{ healthStore.result.errorCount }}</span>
          <span class="summary-label">Errors</span>
        </div>
      </div>

      <!-- Issues list -->
      <div v-if="healthStore.filteredIssues.length === 0" class="all-clear">
        <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
          <polyline points="22 4 12 14.01 9 11.01" />
        </svg>
        <p class="all-clear-title">All locations are healthy</p>
        <p class="all-clear-desc">No broken symlinks, manifest mismatches, or duplicate IDs found.</p>
      </div>

      <div v-else class="issues-section">
        <!-- Errors -->
        <div v-if="healthStore.errorIssues.length > 0" class="issue-group">
          <div class="group-banner severity-error">
            <span class="group-label">Errors</span>
            <span class="group-count">{{ healthStore.errorIssues.length }}</span>
          </div>
          <div class="group-items">
            <div
              v-for="(issue, idx) in healthStore.errorIssues"
              :key="'err-' + idx"
              class="issue-row"
            >
              <div class="issue-content">
                <span class="issue-location" @click="navigateToLocation(issue.locationId)">
                  {{ issue.locationLabel }}
                </span>
                <span class="issue-desc">{{ issue.description }}</span>
                <span class="issue-suggestion">{{ issue.suggestion }}</span>
              </div>
              <div class="issue-actions">
                <SButton
                  v-if="issue.autoFixable"
                  variant="secondary"
                  size="sm"
                  @click="healthStore.fixBrokenLinks(issue.locationId)"
                >Fix</SButton>
              </div>
            </div>
          </div>
        </div>

        <!-- Warnings -->
        <div v-if="healthStore.warningIssues.length > 0" class="issue-group">
          <div class="group-banner severity-warning">
            <span class="group-label">Warnings</span>
            <span class="group-count">{{ healthStore.warningIssues.length }}</span>
          </div>
          <div class="group-items">
            <div
              v-for="(issue, idx) in healthStore.warningIssues"
              :key="'warn-' + idx"
              class="issue-row"
            >
              <div class="issue-content">
                <span class="issue-location" @click="navigateToLocation(issue.locationId)">
                  {{ issue.locationLabel }}
                </span>
                <span class="issue-desc">{{ issue.description }}</span>
                <span class="issue-suggestion">{{ issue.suggestion }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Info -->
        <div v-if="healthStore.infoIssues.length > 0" class="issue-group">
          <div class="group-banner severity-info">
            <span class="group-label">Info</span>
            <span class="group-count">{{ healthStore.infoIssues.length }}</span>
          </div>
          <div class="group-items">
            <div
              v-for="(issue, idx) in healthStore.infoIssues"
              :key="'info-' + idx"
              class="issue-row"
            >
              <div class="issue-content">
                <span class="issue-location" @click="navigateToLocation(issue.locationId)">
                  {{ issue.locationLabel }}
                </span>
                <span class="issue-desc">{{ issue.description }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.health-view {
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

.header-left {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.header-actions {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.page-title {
  font-family: var(--font-sans);
  font-size: var(--text-xl);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  margin: 0;
}

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

.health-content {
  display: flex;
  flex-direction: column;
  gap: var(--space-5);
}

.summary-row {
  display: flex;
  gap: var(--space-3);
}

.summary-card {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-1);
  padding: var(--space-4);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  background: var(--surface-panel);
}

.summary-card.healthy { border-left: 3px solid var(--success); }
.summary-card.warning { border-left: 3px solid var(--warning); }
.summary-card.error { border-left: 3px solid var(--danger); }

.summary-value {
  font-size: var(--text-xl);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
}

.summary-label {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.all-clear {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-8);
  text-align: center;
  color: var(--success);
}

.all-clear-title {
  font-size: var(--text-lg);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  margin: 0;
}

.all-clear-desc {
  font-size: var(--text-sm);
  color: var(--text-secondary);
  margin: 0;
}

.issues-section {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}

.issue-group {
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  background: var(--surface-panel);
  overflow: hidden;
}

.group-banner {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
}

.group-banner.severity-error {
  background: var(--danger-subtle);
  color: var(--danger);
}

.group-banner.severity-warning {
  background: var(--warning-subtle);
  color: var(--warning);
}

.group-banner.severity-info {
  background: var(--accent-subtle);
  color: var(--accent);
}

.group-label { flex: 1; }
.group-count { font-variant-numeric: tabular-nums; }

.issue-row {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-2) var(--space-3);
  border-top: 1px solid var(--border-subtle);
}

.issue-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.issue-location {
  font-size: var(--text-sm);
  font-weight: var(--weight-semibold);
  color: var(--accent);
  cursor: pointer;
}

.issue-location:hover {
  text-decoration: underline;
}

.issue-desc {
  font-size: var(--text-sm);
  color: var(--text-primary);
}

.issue-suggestion {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  font-style: italic;
}

.issue-actions {
  flex-shrink: 0;
}
</style>

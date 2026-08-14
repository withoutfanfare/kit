<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import PanelIcon from "@/components/base/PanelIcon.vue";
import { SBadge, SButton, SConfirmDialog, SModal } from "@stuntrocket/ui";
import { useHealthStore } from "@/stores/healthStore";
import type { LocationId, LocationIssue } from "@/types";

const route = useRoute();
const router = useRouter();
const healthStore = useHealthStore();
const showPreview = ref(false);
const showConfirm = ref(false);

const locationFilter = computed(() =>
  route.query.locationId ? String(route.query.locationId) : null,
);
const previewCount = computed(() =>
  healthStore.removalPreview?.reduce(
    (count, location) => count + location.paths.length,
    0,
  ) ?? 0,
);

function clearFilter() {
  healthStore.setFilter(null);
  router.replace({ path: "/health" });
}

function issueCause(kind: LocationIssue["kind"]) {
  return {
    broken_link: "Broken link",
    declared_missing: "Declared skill missing",
    linked_undeclared: "Undeclared linked skill",
    stale: "Stale skill",
    missing_set: "Missing set",
  }[kind];
}

async function openPreview(locationIds?: LocationId[]) {
  showPreview.value = await healthStore.previewRemoval(locationIds);
}

function continueToConfirmation() {
  showPreview.value = false;
  showConfirm.value = true;
}

function cancelRemoval() {
  showPreview.value = false;
  showConfirm.value = false;
  healthStore.clearPreview();
}

async function confirmRemoval() {
  await healthStore.applyRemoval();
  showConfirm.value = false;
}

watch(locationFilter, (locationId) => {
  healthStore.setFilter(locationId);
  healthStore.setSeverityFilter("all");
  healthStore.clearSelection();
}, { immediate: true });

onMounted(() => {
  healthStore.runCheck();
});
</script>

<template>
  <div class="health-view">
    <div class="page-header">
      <div class="header-left">
        <h1 class="page-title">Health check</h1>
        <SBadge v-if="healthStore.result" variant="count">
          {{ healthStore.result.locationCount }} locations scanned
        </SBadge>
      </div>
      <div class="header-actions">
        <SButton
          v-if="healthStore.selectedLocationIds.size > 0"
          variant="secondary"
          size="sm"
          @click="openPreview()"
        >
          Remove selected…
        </SButton>
        <SButton
          v-if="healthStore.filterLocationId"
          variant="secondary"
          size="sm"
          @click="clearFilter"
        >
          Clear location filter
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
      <span class="loading-label">Scanning all locations…</span>
    </div>

    <div v-else-if="healthStore.result" class="health-content">
      <!-- A filter strip, not three equal cards. The counts are the point;
           the buttons are how you narrow to one of them. -->
      <div class="filters" role="group" aria-label="Filter health results">
        <button
          type="button"
          class="filter"
          :class="{ active: healthStore.severityFilter === 'healthy' }"
          :aria-pressed="healthStore.severityFilter === 'healthy'"
          @click="healthStore.setSeverityFilter('healthy')"
        >
          <span class="num">{{ healthStore.result.healthyCount }}</span>
          <span class="filter-label">
            {{ healthStore.result.healthyCount === 1 ? "location is" : "locations are" }} clear
          </span>
        </button>
        <button
          type="button"
          class="filter is-warning"
          :class="{ active: healthStore.severityFilter === 'warning' }"
          :aria-pressed="healthStore.severityFilter === 'warning'"
          @click="healthStore.setSeverityFilter('warning')"
        >
          <PanelIcon name="caution" :size="13" />
          <span class="num">{{ healthStore.result.warningCount }}</span>
          <span class="filter-label">
            {{ healthStore.result.warningCount === 1 ? "warning" : "warnings" }}
          </span>
        </button>
        <button
          type="button"
          class="filter is-error"
          :class="{ active: healthStore.severityFilter === 'error' }"
          :aria-pressed="healthStore.severityFilter === 'error'"
          @click="healthStore.setSeverityFilter('error')"
        >
          <PanelIcon name="broken" :size="13" />
          <span class="num">{{ healthStore.result.errorCount }}</span>
          <span class="filter-label">
            {{ healthStore.result.errorCount === 1 ? "error" : "errors" }}
          </span>
        </button>
      </div>

      <div v-if="healthStore.groupedIssues.length === 0" class="all-clear">
        <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
          <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
          <polyline points="22 4 12 14.01 9 11.01" />
        </svg>
        <p class="all-clear-title">
          {{ healthStore.result.issues.length === 0 ? "All locations are healthy" : "No locations match this filter" }}
        </p>
        <p class="all-clear-desc">
          {{ healthStore.result.issues.length === 0 ? "No broken symlinks or manifest mismatches found." : "Choose another summary filter to continue triage." }}
        </p>
      </div>

      <div v-else class="location-groups">
        <section
          v-for="group in healthStore.groupedIssues"
          :key="group.location.locationId"
          class="location-group"
        >
          <header class="location-header">
            <label
              v-if="group.location.brokenLinkCount > 0"
              class="selection-control"
            >
              <input
                type="checkbox"
                :checked="healthStore.isSelected(group.location.locationId)"
                :aria-label="`Select ${group.location.locationLabel} for broken-link removal`"
                @change="healthStore.toggleLocation(group.location.locationId)"
              />
            </label>
            <RouterLink
              class="location-link"
              :to="`/locations/${group.location.locationId}`"
            >
              {{ group.location.locationLabel }}
            </RouterLink>
            <div class="location-counts">
              <span v-if="group.location.errorCount" class="count is-error">
                <span class="num">{{ group.location.errorCount }}</span>
                {{ group.location.errorCount === 1 ? "error" : "errors" }}
              </span>
              <span v-if="group.location.warningCount" class="count is-warning">
                <span class="num">{{ group.location.warningCount }}</span>
                {{ group.location.warningCount === 1 ? "warning" : "warnings" }}
              </span>
              <span v-if="group.location.infoCount" class="count">
                <span class="num">{{ group.location.infoCount }}</span>
                to note
              </span>
              <span v-if="group.issues.length === 0" class="badge">Clear</span>
            </div>
            <SButton
              v-if="group.location.brokenLinkCount > 0"
              variant="secondary"
              size="sm"
              @click="openPreview([group.location.locationId])"
            >
              Remove {{ group.location.brokenLinkCount === 1 ? "broken link" : `${group.location.brokenLinkCount} broken links` }}
            </SButton>
          </header>

          <div v-if="group.issues.length === 0" class="healthy-row">
            No health issues found.
          </div>
          <div v-else class="issue-list">
            <div
              v-for="issue in group.issues"
              :key="`${issue.kind}-${issue.skillId ?? issue.description}`"
              class="issue-row"
            >
              <span
                class="badge issue-kind"
                :class="`is-${issue.severity}`"
              >{{ issueCause(issue.kind) }}</span>
              <div class="issue-content">
                <span class="issue-description">{{ issue.description }}</span>
                <span class="issue-suggestion">{{ issue.suggestion }}</span>
              </div>
            </div>
          </div>
        </section>
      </div>
    </div>

    <SModal :open="showPreview" max-width="max-w-3xl" @close="cancelRemoval">
      <template #header>
        <h2 class="modal-title">Review broken links</h2>
      </template>
      <div class="preview-content">
        <section
          v-for="location in healthStore.removalPreview ?? []"
          :key="location.locationId"
          class="preview-location"
        >
          <h3>{{ location.locationLabel }}</h3>
          <p>{{ location.paths.length }} broken {{ location.paths.length === 1 ? "link" : "links" }}</p>
          <ul>
            <li v-for="path in location.paths" :key="path">{{ path }}</li>
          </ul>
        </section>
        <div class="modal-actions">
          <SButton variant="secondary" @click="cancelRemoval">Cancel</SButton>
          <SButton @click="continueToConfirmation">Continue</SButton>
        </div>
      </div>
    </SModal>

    <SConfirmDialog
      :open="showConfirm"
      title="Remove broken links?"
      :message="`Kit will remove ${previewCount} broken link${previewCount === 1 ? '' : 's'}. Each location will be rescanned before anything is removed.`"
      confirm-label="Remove"
      danger
      @confirm="confirmRemoval"
      @cancel="cancelRemoval"
      @close="cancelRemoval"
    />
  </div>
</template>

<style scoped>
.health-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow-y: auto;
  padding: var(--space-9) var(--space-9) var(--space-12);
}

.page-header,
.header-left,
.header-actions,
.location-header,
.location-counts,
.issue-row,
.modal-actions {
  display: flex;
  align-items: center;
}

.page-header {
  justify-content: space-between;
  margin-bottom: var(--space-8);
}

.header-left,
.location-header,
.issue-row {
  gap: var(--space-3);
}

.header-actions,
.location-counts,
.modal-actions {
  gap: var(--space-2);
}

.page-title,
.modal-title,
.preview-location h3 {
  margin: 0;
  color: var(--text-primary);
}

.page-title {
  font-size: var(--text-xl);
  font-weight: var(--weight-semibold);
}

.loading-state,
.all-clear {
  display: flex;
  align-items: center;
  justify-content: center;
}

.loading-state {
  gap: var(--space-2);
  padding: var(--space-8) 0;
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

.loading-label,
.all-clear-desc,
.healthy-row,
.preview-location p {
  color: var(--text-secondary);
  font-size: var(--text-sm);
}

.health-content,
.location-groups,
.preview-content {
  display: flex;
  flex-direction: column;
}

.health-content { gap: var(--space-8); }
.location-groups,
.preview-content { gap: var(--space-3); }

/* One ruled strip of counts. Three equal cards made the filter look like the
   content, and the numbers are what you came to read. */
.filters {
  display: flex;
  align-items: stretch;
  gap: 0;
  border-top: 1px solid var(--border-default);
  border-bottom: 1px solid var(--border-default);
}

.filter {
  display: flex;
  align-items: baseline;
  gap: var(--space-2);
  padding: var(--space-4) var(--space-6) var(--space-4) 0;
  margin-right: var(--space-5);
  border: 0;
  border-right: 1px solid var(--border-subtle);
  background: none;
  color: var(--text-secondary);
  font: inherit;
  cursor: pointer;
}

.filter:last-child {
  border-right: 0;
  margin-right: 0;
}

.filter .num {
  font-size: var(--text-lg);
  color: var(--text-primary);
}

.filter-label {
  font-size: var(--text-sm);
}

.filter:hover .num,
.filter.active .num {
  color: var(--accent);
}

.filter.is-warning :deep(.icon) {
  color: var(--warning);
  align-self: center;
}

.filter.is-error :deep(.icon) {
  color: var(--danger);
  align-self: center;
}

.filter.is-warning:hover .num,
.filter.is-warning.active .num {
  color: var(--warning);
}

.filter.is-error:hover .num,
.filter.is-error.active .num {
  color: var(--danger);
}

.filter.active .filter-label {
  color: var(--text-primary);
}

.filter:focus-visible,
.location-link:focus-visible {
  outline: 2px solid var(--accent);
  outline-offset: 2px;
}

.count {
  display: inline-flex;
  align-items: baseline;
  gap: var(--space-1);
  font-size: var(--text-sm);
  color: var(--text-tertiary);
}

.count .num {
  color: var(--text-secondary);
}

.count.is-error .num {
  color: var(--danger);
}

.count.is-warning .num {
  color: var(--warning);
}

/* The cause is a plate, and severity tints its edge — never the pill-shaped
   colour block that made every row shout equally loudly. */
.issue-kind {
  align-self: flex-start;
  flex-shrink: 0;
}

.issue-kind.is-error {
  color: var(--danger);
  border-color: color-mix(in srgb, var(--danger) 40%, transparent);
}

.issue-kind.is-warning {
  color: var(--warning);
  border-color: color-mix(in srgb, var(--warning) 40%, transparent);
}

.all-clear {
  flex-direction: column;
  gap: var(--space-2);
  padding: var(--space-8);
  text-align: center;
  color: var(--success);
}

.all-clear-title {
  margin: 0;
  color: var(--text-primary);
  font-size: var(--text-lg);
  font-weight: var(--weight-semibold);
}

.all-clear-desc,
.preview-location p {
  margin: 0;
}

.location-group {
  overflow: hidden;
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  background: var(--surface-panel);
}

.location-header {
  min-height: 44px;
  padding: var(--space-4) var(--space-5);
  border-bottom: 1px solid var(--border-subtle);
}

.selection-control {
  display: flex;
  align-items: center;
}

.location-link {
  flex: 1;
  color: var(--accent);
  font-size: var(--text-sm);
  font-weight: var(--weight-semibold);
  text-decoration: none;
}

.location-link:hover { text-decoration: underline; }

.issue-list {
  display: flex;
  flex-direction: column;
}

.issue-row {
  padding: var(--space-4) var(--space-5);
  border-top: 1px solid var(--border-subtle);
}

.issue-row:first-child { border-top: 0; }

.issue-content {
  display: flex;
  flex: 1;
  min-width: 0;
  flex-direction: column;
  gap: 2px;
}

.issue-description {
  color: var(--text-primary);
  font-size: var(--text-sm);
}

.issue-suggestion {
  color: var(--text-tertiary);
  font-size: var(--text-xs);
}

.healthy-row { padding: var(--space-4) var(--space-5); }

.modal-title {
  font-size: var(--text-lg);
  font-weight: var(--weight-semibold);
}

.preview-location {
  padding: var(--space-3);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
}

.preview-location h3 { font-size: var(--text-sm); }

.preview-location ul {
  margin: var(--space-2) 0 0;
  padding-left: var(--space-5);
}

.preview-location li {
  color: var(--text-secondary);
  font-family: ui-monospace, "SF Mono", SFMono-Regular, monospace;
  font-size: var(--text-xs);
  overflow-wrap: anywhere;
}

.modal-actions { justify-content: flex-end; }

@media (max-width: 760px) {
  .page-header,
  .location-header {
    align-items: flex-start;
    flex-direction: column;
  }

  .summary-row { grid-template-columns: 1fr; }
  .location-counts { flex-wrap: wrap; }
}
</style>
